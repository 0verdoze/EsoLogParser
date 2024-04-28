use getset::Getters;

use super::common::*;

// END_CAST - endReason, castTrackId, interruptingAbilityId:optional, interruptingUnitId:optional
#[derive(Debug, Clone, Getters, Deserialize, Serialize)]
#[getset(get = "pub")]
pub struct EventEndCast {
    reason: EndCastReason,
    cast_id: TrackId,
    #[getset(skip)]
    interrupting_ability_id: Option<AbilityId>,
    #[getset(skip)]
    interrupting_unit_id: Option<UnitId>,
    unknown_field_4: Option<Id>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EndCastReason {
    Completed,
    Failed,
    Interrupted,
    PlayerCancelled,
}

impl EventEndCast {
    pub fn interrupting_ability_id(&self) -> Option<&AbilityId> {
        self.interrupting_ability_id.as_ref()
    }

    pub fn interrupting_unit_id(&self) -> Option<&UnitId> {
        self.interrupting_unit_id.as_ref()
    }
}