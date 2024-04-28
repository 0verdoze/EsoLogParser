use getset::Getters;

use super::common::*;

// ZONE_INFO - id, name, dungeonDifficulty
#[derive(Debug, Clone, Getters, Deserialize, Serialize)]
#[getset(get = "pub")]
pub struct EventZoneInfo {
    id: Id,
    name: String,
    dungeon_difficulty: DungeonDifficulty,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DungeonDifficulty {
    None,
    Normal,
    Veteran,
}
