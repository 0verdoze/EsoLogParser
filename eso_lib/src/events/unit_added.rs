use getset::Getters;

use super::common::*;

// UNIT_ADDED - unitId, unitType, isLocalPlayer, playerPerSessionId, monsterId, isBoss, classId, raceId, name, displayName, characterId, level, championPoints, ownerUnitId, reaction, isGroupedWithLocalPlayer
#[derive(Debug, Clone, Getters, Deserialize, Serialize)]
#[getset(get = "pub")]
pub struct EventUnitAdded {
    unit_id: UnitId,
    unit_type: UnitType,
    is_local_player: bool,
    player_per_session_id: Id,
    monster_id: MonsterId,
    is_boss: bool,
    class_id: ClassId,
    race_id: RaceId,
    name: String,
    display_name: String,
    character_id: Id,
    level: Attribute,
    champion_points: Attribute,
    owner_id: UnitId,
    reaction: UnitReactionType,
    is_grouped_with_local_player: bool,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UnitType {
    Monster,
    Object,
    Player,
    SiegeWeapon,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UnitReactionType {
    Companion,
    Default,
    Friendly,
    Hostile,
    Neutral,
    NpcAlly,
    PlayerAlly,
}
