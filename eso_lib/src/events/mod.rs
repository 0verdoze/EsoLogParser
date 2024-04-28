pub mod common;
mod ability_info;
mod begin_cast;
mod begin_combat;
mod begin_log;
mod begin_trial;
mod combat_event;
mod effect_changed;
mod effect_info;
mod end_cast;
mod end_combat;
mod end_log;
mod end_trial;
mod health_regen;
mod map_info;
mod player_info;
mod trial_init;
mod unit_added;
mod unit_changed;
mod unit_removed;
mod zone_info;

use eso_parser::eso_serde::SerializeError;
use rayon::{prelude::ParallelIterator, str::ParallelString};
use serde::{Deserialize, Serialize};
use getset::Getters;

pub use ability_info::*;
pub use begin_cast::*;
pub use begin_combat::*;
pub use begin_log::*;
pub use begin_trial::*;
pub use combat_event::*;
pub use effect_changed::*;
pub use effect_info::*;
pub use end_cast::*;
pub use end_combat::*;
pub use end_log::*;
pub use end_trial::*;
pub use health_regen::*;
pub use map_info::*;
pub use player_info::*;
pub use trial_init::*;
pub use unit_added::*;
pub use unit_changed::*;
pub use unit_removed::*;
pub use zone_info::*;


/// Event describing all possible events
/// 
/// you probably want to use one of the `Event::parse*` functions to deserialize it,
/// or `Event::dump` to serialize it
#[derive(Debug, Clone, Getters, Deserialize, Serialize)]
#[getset(get = "pub")]
pub struct Event {
    pub(crate) timestamp: eso_parser::eso_serde::newtypes::EsoDuration,
    pub(crate) event: EventType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventType {
    AbilityInfo(EventAbilityInfo),
    BeginCast(EventBeginCast),
    BeginCombat(EventBeginCombat),
    BeginLog(EventBeginLog),
    BeginTrial(EventBeginTrial),
    CombatEvent(EventCombatEvent),
    EffectChanged(EventEffectChanged),
    EffectInfo(EventEffectInfo),
    EndCast(EventEndCast),
    EndCombat(EventEndCombat),
    EndLog(EventEndLog),
    EndTrial(EventEndTrial),
    HealthRegen(EventHealthRegen),
    #[serde(alias = "MAP_INFO")]
    MapChanged(EventMapInfo),
    PlayerInfo(EventPlayerInfo),
    TrialInit(EventTrialInit),
    UnitAdded(EventUnitAdded),
    UnitChanged(EventUnitChanged),
    UnitRemoved(EventUnitRemoved),
    #[serde(alias = "ZONE_INFO")]
    ZoneChanged(EventZoneInfo),
}

impl Event {
    /// parse single event
    #[inline]
    pub fn parse(s: &str) -> Result<Self, eso_parser::eso_serde::Error> {
        let mut deserializer = eso_parser::eso_serde::Deserializer::new(s);

        let event = Event::deserialize(&mut deserializer)?;

        if deserializer.is_depleted() {
            Ok(event)
        } else {
            Err(eso_parser::eso_serde::Error::ReaderNotExhausted)
        }
    }

    /// parse many events
    pub fn parse_many<'a, T: AsRef<str> + 'a>(s: &'a T) -> impl Iterator<Item = Result<Self, eso_parser::eso_serde::Error>> + 'a {
        let s = s.as_ref();

        unsafe { Self::_parse_many(s, s.len()) }
    }
    
    /// parse many events from `String` (parsing from `String` gives opportunity for niche optimization)
    pub fn parse_many_string<'a>(s: &'a String) -> impl Iterator<Item = Result<Self, eso_parser::eso_serde::Error>> + 'a {
        unsafe { Self::_parse_many(s.as_str(), s.capacity()) }
    }

    /// parse many events in parallel (uses `rayon` under the hood)
    pub fn parse_many_par<'a, T: AsRef<str> + 'a>(s: &'a T) -> impl ParallelIterator<Item = Result<Self, eso_parser::eso_serde::Error>> + 'a {
        let s = s.as_ref();

        unsafe { Self::_parse_many_par(s, s.len()) }
    }

    /// parse many events in parallel from `String` (uses `rayon` under the hood, parsing from `String` gives opportunity for niche optimization)
    pub fn parse_many_string_par<'a>(s: &'a String) -> impl ParallelIterator<Item = Result<Self, eso_parser::eso_serde::Error>> + 'a {
        unsafe { Self::_parse_many_par(s.as_str(), s.capacity()) }
    }

    unsafe fn _parse_many<'a>(s: &'a str, capacity: usize) -> impl Iterator<Item = Result<Self, eso_parser::eso_serde::Error>> + 'a {
        // TODO: replace 32 with lane size
        let end = s.as_ptr().add(capacity.saturating_sub(32));

        s.lines()
         .map(move |s| {
            if s.as_ptr().add(s.len()) < end {
                Self::parse_unguarded(s)
            } else {
                Self::parse(s)
            }
         })
    }

    unsafe fn _parse_many_par<'a>(s: &'a str, capacity: usize) -> impl ParallelIterator<Item = Result<Self, eso_parser::eso_serde::Error>> + 'a {
        // TODO: replace 32 with lane size
        let end = s.as_ptr().add(capacity.saturating_sub(32)) as usize;

        s.par_lines()
         .map(move |s| {
            if (s.as_ptr().add(s.len()) as usize) < end {
                Self::parse_unguarded(s)
            } else {
                Self::parse(s)
            }
         })
    }

    #[allow(unused_unsafe)]
    #[inline]
    unsafe fn parse_unguarded(s: &str) -> Result<Self, eso_parser::eso_serde::Error> {
        let mut deserializer = eso_parser::eso_serde::UnguardedDeserializer::new(s);

        let event = Event::deserialize(&mut deserializer)?;

        if deserializer.is_depleted() {
            Ok(event)
        } else {
            Err(eso_parser::eso_serde::Error::ReaderNotExhausted)
        }
    }

    /// Serializes this event to `String`
    pub fn dump(&self) -> Result<String, SerializeError> {
        let mut serializer = eso_parser::eso_serde::Serializer::new();
        self.serialize(&mut serializer)?;
        
        Ok(serializer.into_string())
    }
}

// TODO: replace with macro
impl EventType {
    pub fn ability_info(&self) -> Option<&EventAbilityInfo> {
        if let Self::AbilityInfo(e) = self {
            Some(e)
        } else {
            None
        }
    }

    pub fn begin_cast(&self) -> Option<&EventBeginCast> {
        if let Self::BeginCast(e) = self {
            Some(e)
        } else {
            None
        }
    }

    pub fn begin_combat(&self) -> Option<&EventBeginCombat> {
        if let Self::BeginCombat(e) = self {
            Some(e)
        } else {
            None
        }
    }

    pub fn begin_log(&self) -> Option<&EventBeginLog> {
        if let Self::BeginLog(e) = self {
            Some(e)
        } else {
            None
        }
    }

    pub fn begin_trial(&self) -> Option<&EventBeginTrial> {
        if let Self::BeginTrial(e) = self {
            Some(e)
        } else {
            None
        }
    }

    pub fn combat_event(&self) -> Option<&EventCombatEvent> {
        if let Self::CombatEvent(e) = self {
            Some(e)
        } else {
            None
        }
    }

    pub fn effect_changed(&self) -> Option<&EventEffectChanged> {
        if let Self::EffectChanged(e) = self {
            Some(e)
        } else {
            None
        }
    }

    pub fn effect_info(&self) -> Option<&EventEffectInfo> {
        if let Self::EffectInfo(e) = self {
            Some(e)
        } else {
            None
        }
    }

    pub fn end_cast(&self) -> Option<&EventEndCast> {
        if let Self::EndCast(e) = self {
            Some(e)
        } else {
            None
        }
    }

    pub fn end_combat(&self) -> Option<&EventEndCombat> {
        if let Self::EndCombat(e) = self {
            Some(e)
        } else {
            None
        }
    }

    pub fn end_log(&self) -> Option<&EventEndLog> {
        if let Self::EndLog(e) = self {
            Some(e)
        } else {
            None
        }
    }

    pub fn end_trial(&self) -> Option<&EventEndTrial> {
        if let Self::EndTrial(e) = self {
            Some(e)
        } else {
            None
        }
    }

    pub fn health_regen(&self) -> Option<&EventHealthRegen> {
        if let Self::HealthRegen(e) = self {
            Some(e)
        } else {
            None
        }
    }

    pub fn map_info(&self) -> Option<&EventMapInfo> {
        if let Self::MapChanged(e) = self {
            Some(e)
        } else {
            None
        }
    }

    pub fn player_info(&self) -> Option<&EventPlayerInfo> {
        if let Self::PlayerInfo(e) = self {
            Some(e)
        } else {
            None
        }
    }

    pub fn trial_init(&self) -> Option<&EventTrialInit> {
        if let Self::TrialInit(e) = self {
            Some(e)
        } else {
            None
        }
    }

    pub fn unit_added(&self) -> Option<&EventUnitAdded> {
        if let Self::UnitAdded(e) = self {
            Some(e)
        } else {
            None
        }
    }

    pub fn unit_changed(&self) -> Option<&EventUnitChanged> {
        if let Self::UnitChanged(e) = self {
            Some(e)
        } else {
            None
        }
    }

    pub fn unit_removed(&self) -> Option<&EventUnitRemoved> {
        if let Self::UnitRemoved(e) = self {
            Some(e)
        } else {
            None
        }
    }

    pub fn zone_info(&self) -> Option<&EventZoneInfo> {
        if let Self::ZoneChanged(e) = self {
            Some(e)
        } else {
            None
        }
    }
}
