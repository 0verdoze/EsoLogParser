use super::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DisplayQuality {
    MythicOverride,
    Legendary,
    Artifact,
    Arcane,
    Magic,
    Normal,
    Trash,
}
