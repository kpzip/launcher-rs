use serde::Deserialize;

type FabricCompatibleVersionsResponse = Vec<FabricCompatibleVersionInfo>;

#[derive(Debug, Deserialize)]
pub struct FabricCompatibleVersionInfo {
    build: usize,
    version: String,
    stable: bool,
}