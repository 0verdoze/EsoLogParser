use getset::Getters;

use super::common::*;

// EFFECT_INFO - abilityId, effectType, statusEffectType, noEffectBar, grantsSynergyAbilityId:optional
#[derive(Debug, Clone, Getters, Deserialize, Serialize)]
#[getset(get = "pub")]
pub struct EventEffectInfo {
    ability_id: AbilityId,
    effect_type: EffectType,
    status_effect_type: StatusEffectType,
    // not used since U38
    // no_effect_bar: bool,
    effect_bar_display_behaviour: DisplayBehaviour,
    grants_synergy: Option<AbilityId>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EffectType {
    Buff,
    Debuff,
    NotAnEffect,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StatusEffectType {
    Bleed,
    Blind,
    Charm,
    Dazed,
    Disease,
    Environment,
    Fear,
    Levitate,
    Magic,
    Mesmerize,
    Nearsight,
    None,
    Pacify,
    Poison,
    Puncture,
    Root,
    Silence,
    Snare,
    Stun,
    Trauma,
    Weakness,
    Wound,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DisplayBehaviour {
    Default,
    Always,
    Never,
}
