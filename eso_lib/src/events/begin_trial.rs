use getset::Getters;

use super::common::*;

// BEGIN_TRIAL - id, startTimeMS
#[derive(Debug, Clone, Getters, Deserialize, Serialize)]
#[getset(get = "pub")]
pub struct EventBeginTrial {
    id: Id,
    start_time: eso_serde::newtypes::EsoDuration,
}