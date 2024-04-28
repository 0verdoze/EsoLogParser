use std::{str::FromStr, fmt::Display};

use getset::Getters;
use serde::{de::{Error, Unexpected}, Deserializer};

use super::*;

#[derive(Debug, Getters, Clone)]
#[getset(get = "pub")]
pub struct CurrentMaxAttribute<T = Attribute>
where
    T: Clone,
{
    current: T,
    max: T,
}

// <unitState> unitId, health/max, magicka/max, stamina/max, ultimate/max, werewolf/max, shield, map NX, map NY, headingRadians
#[derive(Debug, Clone, Getters, Deserialize, Serialize)]
#[getset(get = "pub")]
pub struct UnitState {
    unit_id: UnitId,
    health: CurrentMaxAttribute,
    magicka: CurrentMaxAttribute,
    stamina: CurrentMaxAttribute,
    ultimate: CurrentMaxAttribute,
    werewolf: CurrentMaxAttribute,
    shield: Attribute,
    pos: Pos,
}

impl UnitState {
    pub(crate) fn new(unit_id: UnitId) -> Self {
        let empty_attr = CurrentMaxAttribute {
            current: 0,
            max: 0,
        };

        Self {
            unit_id,
            health: empty_attr.clone(),
            magicka: empty_attr.clone(),
            stamina: empty_attr.clone(),
            ultimate: empty_attr.clone(),
            werewolf: empty_attr.clone(),
            shield: 0,
            pos: Pos { x: 0.0, y: 0.0, rotation: 0.0 },
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TargetUnitState(Option<UnitState>);

#[derive(Debug, Clone, Getters, Deserialize, Serialize)]
#[getset(get = "pub")]
pub struct Pos {
    x: f32,
    y: f32,
    rotation: f32,
}

struct StrVisitor;

impl<'de> serde::de::Visitor<'de> for StrVisitor {
    type Value = &'de str;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string")
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v)
    }
}

impl<'de, T> Deserialize<'de> for CurrentMaxAttribute<T>
where
    T: Clone + FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let str: &'de str = deserializer.deserialize_str(StrVisitor)?;
        let mut reader = EsoLogReader::new_split(str, '/');
        
        let current_str = reader.next().ok_or_else(|| <D as Deserializer>::Error::invalid_length(1, &"eof while parsing value"))?;

        const ERROR_MSG: &str = "unable to parse CurrentMaxAttribute";
        let current: T = current_str
            .parse()
            .map_err(|_| <D as Deserializer>::Error::invalid_value(Unexpected::Other("unknown"), &ERROR_MSG))?;

        let max: T = reader
            .inner()
            .parse()
            .map_err(|_| <D as Deserializer>::Error::invalid_value(Unexpected::Other("unknown"), &ERROR_MSG))?;

        Ok(Self {
            current,
            max,
        })
    }
}

impl<'de, T> Serialize for CurrentMaxAttribute<T>
where
    T: Clone + Display,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}/{}", &self.current, &self.max))
    }
}

impl<'a> TargetUnitState {
    #[inline]
    pub fn get(&'a self, fallback: &'a UnitState) -> &'a UnitState {
        self.0.as_ref().unwrap_or(fallback)
    }
}

impl Serialize for TargetUnitState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.0.as_ref() {
            Some(v) => v.serialize(serializer),
            None => "*".serialize(serializer),
        }    
    }
}

impl Pos {
    pub fn distance(&self, other: &Self) -> f32 {
        let x = self.x - other.x;
        let y = self.y - other.y;

        x.mul_add(x, y*y).sqrt()
    }
}
