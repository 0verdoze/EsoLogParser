use getset::Getters;

use super::common::*;

// BEGIN_LOG - timeSinceEpochMS, logVersion, realmName, language, gameVersion
#[derive(Debug, Clone, Getters, Deserialize, Serialize)]
#[getset(get = "pub")]
pub struct EventBeginLog {
    time: eso_serde::newtypes::EsoSystemTime,
    log_version: String,
    realm_name: String,
    language: String,
    game_version: String,
}

