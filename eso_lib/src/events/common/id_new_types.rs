use super::*;

type IdType = u64;
pub type Attribute = u32;

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(pub IdType);

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnitId(pub IdType);

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SetId(pub IdType);

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Level(pub IdType);

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AbilityId(pub IdType);

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TrackId(pub IdType);

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MonsterId(pub IdType);

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StackCount(pub IdType);

// TODO: make this an enum
// PowertypeDaedric
// PowertypeHealth
// PowertypeHealthBonus
// PowertypeInvalid
// PowertypeMagicka
// PowertypeMountStamina
// PowertypeStamina
// PowertypeUltimate
// PowertypeWerewolf
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PowerType(pub IdType);

// TODO: make this an enum
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RaceId(pub IdType);

// TODO: make this an enum
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassId(pub IdType);


macro_rules! impl_getter {
    ($t:ty) => {
        impl $t {
            #[inline]
            pub fn get(&self) -> IdType {
                self.0
            }
        }
    };

    ($t:ty, $($ts:ty),+) => {
        impl_getter!( $t );
        impl_getter!( $($ts),+ );
    };
}

impl_getter! {
    Id,
    UnitId,
    SetId,
    Level,
    AbilityId,
    TrackId,
    MonsterId,
    StackCount,
    PowerType,
    RaceId,
    ClassId
}
