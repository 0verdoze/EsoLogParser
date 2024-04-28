use getset::Getters;

use super::common::*;

// HEALTH_REGEN - effectiveRegen, <unitState>
#[derive(Debug, Clone, Getters, Deserialize, Serialize)]
#[getset(get = "pub")]
pub struct EventHealthRegen {
    effective_regen: Attribute,
    unit: UnitState,
}
