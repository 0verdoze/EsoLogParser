use getset::Getters;

use super::common::*;

// COMBAT_EVENT - actionResult, damageType, powerType, hitValue, overflow, castTrackId, abilityId, <sourceUnitState>, <targetUnitState>
#[derive(Debug, Clone, Getters, Deserialize, Serialize)]
#[getset(get = "pub")]
pub struct EventCombatEvent {
    action_result: ActionResult,
    damage_type: DamageType,
    power_type: PowerType,
    hit_value: Attribute,
    overflow: Attribute,
    cast_id: TrackId,
    ability_id: AbilityId,
    source_unit: UnitState,
    #[getset(skip)]
    target_unit: TargetUnitState,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActionResult {
    AbilityOnCooldown,
    Absorbed,
    BadTarget,
    BattleStandardsDisabled,
    BattleStandardAlreadyExistsForGuild,
    BattleStandardLimit,
    BattleStandardNoPermission,
    BattleStandardTabardMismatch,
    BattleStandardTooCloseToCapturable,
    Bladeturn,
    Blocked,
    BlockedDamage,
    Busy,
    CannotUse,
    CantSeeTarget,
    CantSwapHotbarIsOverridden,
    CantSwapWhileChangingGear,
    CasterDead,
    CriticalDamage,
    CriticalHeal,
    Damage,
    DamageShielded,
    Defended,
    Died,
    DiedCompanionXp,
    DiedXp,
    Disarmed,
    Disoriented,
    Dodged,
    DotTick,
    DotTickCritical,
    Failed,
    FailedRequirements,
    FailedSiegeCreationRequirements,
    Falling,
    FallDamage,
    Feared,
    ForwardCampAlreadyExistsForGuild,
    ForwardCampNoPermission,
    ForwardCampTabardMismatch,
    GraveyardDisallowedInInstance,
    GraveyardTooClose,
    Heal,
    HealAbsorbed,
    HotTick,
    HotTickCritical,
    Immune,
    InsufficientResource,
    Intercepted,
    Interrupt,
    Invalid,
    InvalidFixture,
    InvalidJusticeTarget,
    InvalidTerrain,
    InAir,
    InCombat,
    InEnemyKeep,
    InEnemyOutpost,
    InEnemyResource,
    InEnemyTown,
    InHideyhole,
    KilledByDaedricWeapon,
    KilledBySubzone,
    KillingBlow,
    Knockback,
    Levitated,
    MercenaryLimit,
    Miss,
    MissingEmptySoulGem,
    MissingFilledSoulGem,
    MobileGraveyardLimit,
    Mounted,
    MustBeInOwnKeep,
    NotEnoughInventorySpace,
    NotEnoughInventorySpaceSoulGem,
    NotEnoughSpaceForSiege,
    NoLocationFound,
    NoRamAttackableTargetWithinRange,
    NoWeaponsToSwapTo,
    NpcTooClose,
    Offbalance,
    Pacified,
    Parried,
    PartialResist,
    PowerDrain,
    PowerEnergize,
    PreciseDamage,
    Queued,
    RamAttackableTargetsAllDestroyed,
    RamAttackableTargetsAllOccupied,
    Recalling,
    Reflected,
    Reincarnating,
    Resist,
    Resurrect,
    Rooted,
    SiegeLimit,
    SiegeNotAllowedInZone,
    SiegeTooClose,
    Silenced,
    Snared,
    SoulGemResurrectionAccepted,
    Sprinting,
    Staggered,
    Stunned,
    Swimming,
    TargetDead,
    TargetNotInView,
    TargetNotPvpFlagged,
    TargetOutOfRange,
    TargetTooClose,
    UnevenTerrain,
    Weaponswap,
    WreckingDamage,
    WrongWeapon,

    // not in docs
    LinkedCast,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DamageType {
    Bleed,
    Cold,
    Disease,
    Drown,
    Earth,
    Fire,
    Generic,
    Magic,
    None,
    Oblivion,
    Physical,
    Poison,
    Shock,
}

impl EventCombatEvent {
    pub fn target_unit(&self) -> &UnitState {
        self.target_unit.get(self.source_unit())
    }
}

impl ActionResult {
    #[inline]
    pub fn is_critical(&self) -> bool {
        match self {
            Self::CriticalDamage
           |Self::CriticalHeal
           |Self::DotTickCritical
           |Self::HotTickCritical => true,
           _ => false,
        }
    }
}
