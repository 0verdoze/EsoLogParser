use getset::Getters;

use super::common::*;

// UNIT_CHANGED - unitId, classId, raceId, name, displayName, characterId, level, championPoints, ownerUnitId, reaction, isGroupedWithLocalPlayer
#[derive(Debug, Clone, Getters, Deserialize, Serialize)]
#[getset(get = "pub")]
pub struct EventUnitChanged {
    unit_id: UnitId,
    class_id: ClassId,
    race_id: RaceId,
    name: String,
    display_name: String,
    character_id: Id,
    level: Attribute,
    champion_points: Attribute,
    owner_id: Id,
    reaction: super::UnitReactionType,
    is_grouped_with_local_player: bool,
}

