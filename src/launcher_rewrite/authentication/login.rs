use std::collections::HashMap;
use std::sync::Arc;
use regex::Regex;
use reqwest::blocking;
use reqwest::blocking::ClientBuilder;
use reqwest::redirect::Policy;
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};
use crate::launcher_rewrite::authentication::account_data::{LoggedInAccount, MicrosoftTokenInfo, MinecraftAccountInfo, MinecraftTokenInfo, XboxLiveTokenInfo};
use crate::launcher_rewrite::authentication::LOGGED_IN_ACCOUNT_DATA;
use crate::launcher_rewrite::authentication::login::LoginState::{InvalidCredentials, LoggedIn, Requires2FA};
use crate::launcher_rewrite::error::LauncherError;
use crate::launcher_rewrite::installer::APP_USER_AGENT;

#[derive(Debug, Clone)]
#[must_use]
pub enum LoginState {
    Requires2FA(blocking::Client),
    InvalidCredentials,
    LoggedIn,
}

pub fn login(username: String, password: String) -> Result<LoginState, LauncherError> {
    let cookies = Arc::new(CookieStoreMutex::new(CookieStore::default()));

    let client = ClientBuilder::new().user_agent(APP_USER_AGENT).redirect(Policy::default()).cookie_provider(cookies).build().unwrap();

    let response = client.get("https://login.live.com/oauth20_authorize.srf?client_id=000000004C12AE6F&redirect_uri=https://login.live.com/oauth20_desktop.srf&scope=service::user.auth.xboxlive.com::MBI_SSL&display=touch&response_type=token&locale=en").send()?.text()?;
    //println!("\n\n\n\nResponse 1: {}\n\n\n\n", response);

    let sft_tag_regex = Regex::new("value=\"(.+?)\"").unwrap();
    let sft_tag = sft_tag_regex.find(response.as_str()).unwrap().as_str().strip_suffix("\"").unwrap().strip_prefix("value=\"").unwrap();
    //println!("Sft tag: {}", sft_tag);

    let ulr_post_regex = Regex::new("urlPost:'(.+?)'").unwrap();
    let url_post = ulr_post_regex.find(response.as_str()).unwrap().as_str().strip_suffix("'").unwrap().strip_prefix("urlPost:'").unwrap();
    //println!("Url Post: {}", url_post);

    let url_encoded_username = urlencoding::encode(username.as_str());
    let url_encoded_password = urlencoding::encode(password.as_str());
    let url_encoded_sft_tag = urlencoding::encode(sft_tag);

    let body = format!("login={}&loginfmt={}&passwd={}&PPFT={}", url_encoded_username, url_encoded_username, url_encoded_password, url_encoded_sft_tag);

    let response = client.post(url_post).body(body).header("Content-Type", "application/x-www-form-urlencoded").send()?;
    let final_url: String = response.url().as_str().into();
    let response = response.text().unwrap();

    //println!("\n\n\n\nResponse: {}\n\n\n\n", response);
    //println!("final url: {}", final_url);

    if response.contains("Sign in to") {
        // Wrong Credentials
        println!("Wrong Creds!");
        return Ok(InvalidCredentials);
    }

    if response.contains("Help us protect your account") {
        // 2FA
        println!("2FA Required");
        return Ok(Requires2FA(client))
    }

    let raw_login_data = final_url.split_once("#").unwrap().1;

    let data_pairs: HashMap<&str, &str> = raw_login_data.split("&").map(|i| i.split_once("=").unwrap()).collect();

    let ms_token_info = MicrosoftTokenInfo::from_map(data_pairs);

    // Sign in to Xbox Live

    let body = format!("{{\"Properties\": {{\"AuthMethod\": \"RPS\",\"SiteName\": \"user.auth.xboxlive.com\",\"RpsTicket\": \"{}\"}},\"RelyingParty\": \"http://auth.xboxlive.com\",\"TokenType\": \"JWT\"}}", ms_token_info.access_token());

    let response = client.post("https://user.auth.xboxlive.com/user/authenticate").header("Content-Type", "application/json").header("Accept", "application/json").body(body).send()?.text()?;

    let xbox_token_info: XboxLiveTokenInfo = serde_json::from_str(response.as_str()).unwrap();

    // get XSTS token

    let body = format!("{{\"Properties\": {{\"SandboxId\": \"RETAIL\",\"UserTokens\": [\"{}\"]}},\"RelyingParty\": \"rp://api.minecraftservices.com/\",\"TokenType\": \"JWT\"}}", xbox_token_info.token());

    let response = client.post("https://xsts.auth.xboxlive.com/xsts/authorize").header("Content-Type", "application/json").header("Accept", "application/json").body(body).send()?.text()?;

    //println!("Xsts Response: {}", response.as_str());

    let xsts_token_info: XboxLiveTokenInfo = serde_json::from_str(response.as_str())?;

    // get Minecraft Token

    let body = format!("{{\"identityToken\" : \"XBL3.0 x={};{}\",\"ensureLegacyEnabled\" : true}}", xsts_token_info.iter().next().unwrap().as_str(), xsts_token_info.token());

    let response = client.post("https://api.minecraftservices.com/authentication/login_with_xbox").header("Content-Type", "application/json").body(body).send()?.text()?;

    let minecraft_token_info: MinecraftTokenInfo = serde_json::from_str(response.as_str()).unwrap();

    //println!("Minecraft Token: {}", minecraft_token_info.access_token());

    let response = client.get("https://api.minecraftservices.com/minecraft/profile").header("Authorization", format!("Bearer {}", minecraft_token_info.access_token())).send()?.text()?;

    //println!("Account Info: {}", &response);

    let profile_info: MinecraftAccountInfo = serde_json::from_str(response.as_str()).unwrap();

    let complete_token_info = LoggedInAccount::new(ms_token_info, xbox_token_info, xsts_token_info, minecraft_token_info, profile_info);

    LOGGED_IN_ACCOUNT_DATA.write().unwrap().add_account_and_set_active(complete_token_info);
    Ok(LoggedIn)
}
