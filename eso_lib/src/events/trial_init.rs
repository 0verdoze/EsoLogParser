use getset::Getters;

use super::common::*;

// TRIAL_INIT - id, inProgress, completed, startTimeMS, durationMS, success, finalScore
#[derive(Debug, Clone, Getters, Deserialize, Serialize)]
#[getset(get = "pub")]
pub struct EventTrialInit {
    id: Id,
    in_progress: bool,
    completed: bool,
    start_time: eso_serde::newtypes::EsoDuration,
    duration: eso_serde::newtypes::EsoDuration,
    success: bool,
    final_score: Attribute,
}
