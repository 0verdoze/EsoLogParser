use getset::Getters;

use super::common::*;

// PLAYER_INFO - unitId, [longTermEffectAbilityId,...], [longTermEffectStackCounts,...], [<equipmentInfo>,...], [primaryAbilityId,...], [backupAbilityId,...]
#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct EventPlayerInfo {
    unit_id: UnitId,
    long_term_effects: Vec<LongTermEffect>,
    equipment_info: Vec<EquipmentInfo>,
    primary_abilities: Vec<AbilityId>,
    backup_abilities: Vec<AbilityId>,
}

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct LongTermEffect {
    ability: AbilityId,
    stack_count: StackCount,
}

#[derive(Deserialize, Serialize)]
struct PlayerInfoRaw {
    unit_id: UnitId,
    long_term_effect_ability_ids: Vec<AbilityId>,
    long_term_effect_stack_counts: Vec<StackCount>,
    equipment_info: Vec<EquipmentInfo>,
    primary_abilities: Vec<AbilityId>,
    backup_abilities: Vec<AbilityId>,
}

impl<'de> Deserialize<'de> for EventPlayerInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = PlayerInfoRaw::deserialize(deserializer)?;

        let long_term_effects: Vec<_> = raw
            .long_term_effect_ability_ids
            .iter()
            .zip(raw.long_term_effect_stack_counts.iter())
            .map(|(&ability, &stack_count)| LongTermEffect { ability, stack_count })
            .collect();

        Ok(Self {
            unit_id: raw.unit_id,
            long_term_effects,
            equipment_info: raw.equipment_info,
            primary_abilities: raw.primary_abilities,
            backup_abilities: raw.backup_abilities,
        })
    }
}

impl Serialize for EventPlayerInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let (lte_ability_ids, lte_stack_counts): (Vec<AbilityId>, Vec<StackCount>) = self
            .long_term_effects
            .iter()
            .map(|s| (s.ability, s.stack_count))
            .unzip();

        PlayerInfoRaw {
            unit_id: self.unit_id,
            long_term_effect_ability_ids: lte_ability_ids,
            long_term_effect_stack_counts: lte_stack_counts,
            equipment_info: self.equipment_info.clone(),
            primary_abilities: self.primary_abilities.clone(),
            backup_abilities: self.backup_abilities.clone(),
        }.serialize(serializer)
    }
}

// impl<'a> EsoParse<'a> for EventPlayerInfo {
//     fn parse(t: &mut impl EsoLogReaderTrait<'a>) -> EsoResult<Self> {
//         let unit_id = UnitId::parse(t)?;

//         let abilities = EsoLogReader::read_vec(t.next()?);
//         let stack_counts = EsoLogReader::read_vec(t.next()?);

//         let long_term_effects = abilities.zip(stack_counts)
//             .map(|(a,s)| {
//                 Ok(LongTermEffect {
//                     ability: EsoParse::parse(&mut EsoLogReader::new_raw(a))?,
//                     stack_count: EsoParse::parse(&mut EsoLogReader::new_raw(s))?,
//                 })
//             }).collect::<Result<Vec<_>, EsoError>>()?;

//         let equipment_info = EsoLogReader::read_vec(t.next()?)
//             .map(|s| Ok(EsoParse::parse(&mut EsoLogReader::read_vec(s))?))
//             .collect::<Result<Vec<_>, EsoError>>()?;

//         let primary_abilities = EsoLogReader::read_vec(t.next()?)
//             .map(|s| Ok(EsoParse::parse(&mut EsoLogReader::read_line(s))?))
//             .collect::<Result<Vec<_>, EsoError>>()?;

//         let backup_abilities = EsoLogReader::read_vec(t.next()?)
//             .map(|s| Ok(EsoParse::parse(&mut EsoLogReader::read_line(s))?))
//             .collect::<Result<Vec<_>, EsoError>>()?;

//         EsoResult::Ok(Self {
//             unit_id,
//             long_term_effects,
//             equipment_info,
//             primary_abilities,
//             backup_abilities
//         })
//     }
// }

