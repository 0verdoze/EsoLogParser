use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub struct EsoDuration(pub Duration);

#[derive(Debug, Clone, Copy)]
pub struct EsoSystemTime(pub SystemTime);

impl<'de> Deserialize<'de> for EsoDuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let millis = u64::deserialize(deserializer)?;
        Ok(Self(Duration::from_millis(millis)))
    }
}

impl Serialize for EsoDuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (self.0.as_millis() as u64).serialize(serializer)    
    }
}

impl<'de> Deserialize<'de> for EsoSystemTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let duration = EsoDuration::deserialize(deserializer)?.0;

        Ok(Self(SystemTime::UNIX_EPOCH + duration))
    }
}

impl Serialize for EsoSystemTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        EsoDuration(self.0.duration_since(SystemTime::UNIX_EPOCH).unwrap()).serialize(serializer)
    }
}
