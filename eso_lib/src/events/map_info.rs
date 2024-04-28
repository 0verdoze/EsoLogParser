use getset::Getters;

use super::common::*;


// MAP_INFO - id, name, texturePath
#[derive(Debug, Clone, Getters, Deserialize, Serialize)]
#[getset(get = "pub")]
pub struct EventMapInfo {
    id: Id,
    name: String,
    texture_path: String,
}