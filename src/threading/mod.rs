use crate::launcher_rewrite::authentication::account_data::{LoggedInAccount, MicrosoftTokenInfo, MinecraftAccountInfo, MinecraftTokenInfo, XboxLiveTokenInfo};
use crate::gui::account::AccountInteraction;
use crate::gui::LauncherMessage;
use crate::util::StripCanonicalization;
use crate::launcher_rewrite::installer::APP_USER_AGENT;
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::blocking::ClientBuilder;
use reqwest::redirect::Policy;
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};
use std::collections::HashMap;
use std::ops::Deref;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::sync::{mpsc, Arc};
use std::thread::JoinHandle;
use std::{fs, thread};
use tokio::sync::mpsc::UnboundedSender;
use crate::launcher_rewrite::authentication::LOGGED_IN_ACCOUNT_DATA;
use crate::launcher_rewrite::installer::Downloadable;
use crate::launcher_rewrite::launch_game;
use crate::launcher_rewrite::manifest::GAME_VERSION_MANIFEST;
use crate::launcher_rewrite::profiles::PROFILES;

pub enum WorkerThreadTask {
    LaunchGame(u128),
    DownloadVersionManifest,
    LoadProfiles,
    Shutdown,
    MicrosoftLogin { username: String, password: String },
}

pub struct WorkerThread {
    comms: Sender<WorkerThreadTask>,
    handle: JoinHandle<()>,
}

impl Deref for WorkerThread {
    type Target = Sender<WorkerThreadTask>;

    fn deref(&self) -> &Self::Target {
        &self.comms
    }
}

impl WorkerThread {
    pub fn new(message_send: UnboundedSender<LauncherMessage>) -> WorkerThread {
        let (sender, receiver) = mpsc::channel::<WorkerThreadTask>();

        let handle = thread::Builder::new().name("Worker Thread".into()).spawn(move || worker_thread(receiver, message_send)).unwrap();
        Self { comms: sender, handle }
    }
}

fn worker_thread(comms: Receiver<WorkerThreadTask>, message_send: UnboundedSender<LauncherMessage>) {
    //let dev_game_path = fs::canonicalize(PathBuf::from_str("run\\game").unwrap()).unwrap().strip_canonicalization();

    // Deref so this thread starts initializing them for us
    &*PROFILES;
    &*GAME_VERSION_MANIFEST;
    &*LOGGED_IN_ACCOUNT_DATA;


    'events: loop {
        match comms.try_recv() {
            Ok(v) => {
                match v {
                    WorkerThreadTask::LaunchGame(profile_id) => {
                        launch_game(profile_id);



                        /*
                        let profile_lock = PROFILES.lock().unwrap();
                        let profiles = profile_lock.as_ref().unwrap();
                        let profile = profiles.je_client_profiles().iter().find(|p| p.id() == profile_id).unwrap();
                        let versions_lock = VERSION_MANIFEST.read().unwrap();
                        let version_manifest = versions_lock.as_ref().unwrap();
                        let version = version_manifest.get_version(profile.version_name()).unwrap();
                        version.install_and_get_launch_properties(&launcher_path, profile.mod_loader());

                        // TODO installation dependent

                        let cmd_builder = build_launch_command(&dev_game_path, &launcher_path, version, profile.mod_loader());

                        let dev_clone = dev_game_path.clone();

                        thread::spawn(move || generate_args_and_launch_game(dev_clone, cmd_builder));*/
                    }
                    WorkerThreadTask::DownloadVersionManifest => {}
                    WorkerThreadTask::LoadProfiles => {}
                    WorkerThreadTask::Shutdown => break 'events,
                    WorkerThreadTask::MicrosoftLogin { username, password } => 'login: {
                        let cookies = Arc::new(CookieStoreMutex::new(CookieStore::default()));

                        let client = ClientBuilder::new().user_agent(APP_USER_AGENT).redirect(Policy::default()).cookie_provider(cookies).build().unwrap();

                        let response = client.get("https://login.live.com/oauth20_authorize.srf?client_id=000000004C12AE6F&redirect_uri=https://login.live.com/oauth20_desktop.srf&scope=service::user.auth.xboxlive.com::MBI_SSL&display=touch&response_type=token&locale=en").send().unwrap().text().unwrap();
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

                        let response = client.post(url_post).body(body).header("Content-Type", "application/x-www-form-urlencoded").send().unwrap();
                        let final_url: String = response.url().as_str().into();
                        let response = response.text().unwrap();

                        //println!("\n\n\n\nResponse: {}\n\n\n\n", response);
                        //println!("final url: {}", final_url);

                        if response.contains("Sign in to") {
                            // Wrong Credentials
                            println!("Wrong Creds!");
                            message_send.send(LauncherMessage::AccountTabInteraction(AccountInteraction::InvalidCreds)).unwrap();
                            break 'login;
                        }

                        if response.contains("Help us protect your account") {
                            // 2FA
                            println!("2FA Required");
                            message_send.send(LauncherMessage::AccountTabInteraction(AccountInteraction::_2FARequired)).unwrap();
                            break 'login;
                        }

                        let raw_login_data = final_url.split_once("#").unwrap().1;

                        let data_pairs: HashMap<&str, &str> = raw_login_data.split("&").map(|i| i.split_once("=").unwrap()).collect();

                        let ms_token_info = MicrosoftTokenInfo::from_map(data_pairs);

                        // Sign in to Xbox Live

                        let body = format!("{{\"Properties\": {{\"AuthMethod\": \"RPS\",\"SiteName\": \"user.auth.xboxlive.com\",\"RpsTicket\": \"{}\"}},\"RelyingParty\": \"http://auth.xboxlive.com\",\"TokenType\": \"JWT\"}}", ms_token_info.access_token());

                        let response = client.post("https://user.auth.xboxlive.com/user/authenticate").header("Content-Type", "application/json").header("Accept", "application/json").body(body).send().unwrap().text().unwrap();

                        let xbox_token_info: XboxLiveTokenInfo = serde_json::from_str(response.as_str()).unwrap();

                        // get XSTS token

                        let body = format!("{{\"Properties\": {{\"SandboxId\": \"RETAIL\",\"UserTokens\": [\"{}\"]}},\"RelyingParty\": \"rp://api.minecraftservices.com/\",\"TokenType\": \"JWT\"}}", xbox_token_info.token());

                        let response = client.post("https://xsts.auth.xboxlive.com/xsts/authorize").header("Content-Type", "application/json").header("Accept", "application/json").body(body).send().unwrap().text().unwrap();

                        //println!("Xsts Response: {}", response.as_str());

                        let xsts_token_info: XboxLiveTokenInfo = serde_json::from_str(response.as_str()).unwrap();

                        // get Minecraft Token

                        let body = format!("{{\"identityToken\" : \"XBL3.0 x={};{}\",\"ensureLegacyEnabled\" : true}}", xsts_token_info.iter().next().unwrap().as_str(), xsts_token_info.token());

                        let response = client.post("https://api.minecraftservices.com/authentication/login_with_xbox").header("Content-Type", "application/json").body(body).send().unwrap().text().unwrap();

                        let minecraft_token_info: MinecraftTokenInfo = serde_json::from_str(response.as_str()).unwrap();

                        //println!("Minecraft Token: {}", minecraft_token_info.access_token());

                        let response = client.get("https://api.minecraftservices.com/minecraft/profile").header("Authorization", format!("Bearer {}", minecraft_token_info.access_token())).send().unwrap().text().unwrap();

                        //println!("Account Info: {}", &response);

                        let profile_info: MinecraftAccountInfo = serde_json::from_str(response.as_str()).unwrap();

                        let complete_token_info = LoggedInAccount::new(ms_token_info, xbox_token_info, xsts_token_info, minecraft_token_info, profile_info);

                        LOGGED_IN_ACCOUNT_DATA.write().unwrap().add_account_and_set_active(complete_token_info);

                        message_send.send(LauncherMessage::AccountTabInteraction(AccountInteraction::LoginSuccess)).unwrap();
                    }
                }
            }
            Err(e) => {
                match e {
                    TryRecvError::Empty => { /* Continue Waiting */ }
                    TryRecvError::Disconnected => break,
                }
            }
        }
    }
}
