use getset::Getters;

use super::common::*;

// EFFECT_CHANGED - changeType, stackCount, castTrackId, abilityId, <sourceUnitState>, <targetUnitState>, playerInitiatedRemoveCastTrackId:optional
#[derive(Debug, Clone, Getters, Deserialize, Serialize)]
#[getset(get = "pub")]
pub struct EventEffectChanged {
    change_type: EffectChangeType,
    stack_count: StackCount,
    cast_id: TrackId,
    ability_id: AbilityId,
    source_unit: UnitState,
    #[getset(skip)]
    target_unit: TargetUnitState,
    player_initiated_remove_cast: Option<TrackId>,
}

// 41,22738/22738,32167/32167,14453/14453,73/500,1000/1000,0,509.9681,185.8353,4.4129,*

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EffectChangeType {
    Gained,
    Faded,
    Updated,
}

impl EventEffectChanged {
    pub fn target_unit(&self) -> &UnitState {
        self.target_unit.get(self.source_unit())
    }
}
