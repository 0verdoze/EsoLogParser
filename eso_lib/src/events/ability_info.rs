use getset::Getters;

use super::common::*;

// ABILITY_INFO - abilityId, name, iconPath, interruptible, blockable
#[derive(Debug, Clone, Getters, Deserialize, Serialize)]
#[getset(get = "pub")]
pub struct EventAbilityInfo {
    ability_id: AbilityId,
    name: String,
    icon_path: String,
    interruptible: bool,
    blockable: bool,
}