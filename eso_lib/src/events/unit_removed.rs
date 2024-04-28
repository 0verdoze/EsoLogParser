use getset::Getters;

use super::common::*;

// UNIT_REMOVED - unitId
#[derive(Debug, Clone, Getters, Deserialize, Serialize)]
#[getset(get = "pub")]
pub struct EventUnitRemoved {
    pub(crate) unit_id: UnitId,
}
