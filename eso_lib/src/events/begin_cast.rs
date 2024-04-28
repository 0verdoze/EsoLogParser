use getset::Getters;

use super::common::*;

// BEGIN_CAST - durationMS, channeled, castTrackId, abilityId, <sourceUnitState>, <targetUnitState>
#[derive(Debug, Clone, Getters, Deserialize, Serialize)]
#[getset(get = "pub")]
pub struct EventBeginCast {
    duration: eso_serde::newtypes::EsoDuration,
    channeled: bool,
    cast_id: TrackId,
    ability_id: AbilityId,
    source_unit: UnitState,
    #[getset(skip)]
    target_unit: TargetUnitState,
}

impl EventBeginCast {
    pub fn target_unit(&self) -> &UnitState {
        self.target_unit.get(self.source_unit())
    }
}

