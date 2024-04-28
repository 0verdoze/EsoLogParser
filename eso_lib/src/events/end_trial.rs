use getset::Getters;

use super::common::*;

// END_TRIAL - id, durationMS, success, finalScore, finalVitalityBonus
#[derive(Debug, Clone, Getters, Deserialize, Serialize)]
#[getset(get = "pub")]
pub struct EventEndTrial {
    id: Id,
    duration: eso_serde::newtypes::EsoDuration,
    success: bool,
    final_score: Attribute,
    final_vitality_bonus: Attribute,
}
