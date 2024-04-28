use super::*;

// <equipmentInfo> refers to the following fields for a piece of equipment: slot, id, isCP, level, trait, displayQuality, setId, enchantType, isEnchantCP, enchantLevel, enchantQuality.
#[derive(Debug, Clone, Deserialize, Serialize)]
// we're implementing this as tuple, because in combat log format this is represented as a sequence
pub struct EquipmentInfo (
    EquipSlot,
    Id,
    bool,
    Level,
    Trait,
    DisplayQuality,
    SetId,
    EnchantType,
    bool,
    Level,
    DisplayQuality,
);


#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EquipSlot {
    BackupMain,
    BackupOff,
    BackupPoison,
    Chest,
    Class1,
    Class2,
    Class3,
    Costume,
    Feet,
    Hand,
    Head,
    Legs,
    MainHand,
    Neck,
    None,
    OffHand,
    Poison,
    Ranged,
    Ring1,
    Ring2,
    Shoulders,
    Waist,
    Wrist,
}

impl EquipmentInfo {
    pub fn slot(&self) -> &EquipSlot {
        &self.0
    }

    pub fn id(&self) -> &Id {
        &self.1
    }

    pub fn is_cp(&self) -> &bool {
        &self.2
    }

    pub fn level(&self) -> &Level {
        &self.3
    }

    pub fn trait_(&self) -> &Trait {
        &self.4
    }

    pub fn display_quality(&self) -> &DisplayQuality {
        &self.5
    }

    pub fn set_id(&self) -> &SetId {
        &self.6
    }

    pub fn enchant_type(&self) -> &EnchantType {
        &self.7
    }

    pub fn is_enchant_cp(&self) -> &bool {
        &self.8
    }

    pub fn enchant_level(&self) -> &Level {
        &self.9
    }

    pub fn enchant_quality(&self) -> &DisplayQuality {
        &self.10
    }
}
