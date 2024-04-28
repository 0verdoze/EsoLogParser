use std::{collections::HashMap, borrow::Borrow};

use getset::Getters;

use crate::events::{*, common::*};

/// Can be feeded events to save revelent informations
/// like:
/// - informations about abilities and effects
/// - current units
/// - active effects
/// - and more
#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct State {
    entities: HashMap<UnitId, Unit>,
    effects: EffectMap,
    ability_info: AbilityInfoMap<EventAbilityInfo>,
    effect_info: AbilityInfoMap<EventEffectInfo>,
    in_combat: bool,
}

/// holds informations about Unit
#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct Unit {
    unit_type: UnitType,
    state: UnitState,
    reaction: UnitReactionType,
    equipment: HashMap<EquipSlot, EquipmentInfo>,
    name: String,
    display_name: String,
    monster_id: MonsterId,
    race_id: RaceId,
    class_id: ClassId,
    is_boss: bool,
}

// TODO: manual Debug impl
// maybe FIXME: in game multiple units can share TrackId from single a source
/// holds informations about active effects
#[derive(Debug, Clone, Default)]
pub struct EffectMap {
    effects: HashMap<TrackId, EventEffectChanged>,
    recevied_effects: HashMap<UnitId, Vec<TrackId>>,
    granted_effects: HashMap<UnitId, Vec<TrackId>>,
}

/// holds informations about abilities and effects
#[derive(Debug, Clone)]
pub struct AbilityInfoMap<T>(HashMap<AbilityId, T>);

impl State {
    /// Create fresh instance, with single default entity (World)
    pub fn new() -> Self {
        let mut this = Self {
            entities: Default::default(),
            effects: Default::default(),
            ability_info: Default::default(),
            effect_info: Default::default(),
            in_combat: Default::default(),
        };

        let zero = UnitId(0);

        this.entities.insert(zero, Unit {
            unit_type: UnitType::Object, 
            state: UnitState::new(zero),
            reaction: UnitReactionType::Hostile,
            equipment: HashMap::new(),
            name: "World".into(),
            display_name: "World".into(), 
            monster_id: MonsterId(0), 
            race_id: RaceId(0),
            class_id: ClassId(0), 
            is_boss: false,
        });

        this
    }

    /// process passed event, update/store/remove data accourding to that event
    pub fn handle_event(&mut self, e: &Event) {
        use EventType::*;

        // fix for vscode extension
        let e: &EventType = e.event();

        match e {
            AbilityInfo(v) => {
                self.insert_ability_info(v);
            },
            BeginCombat(_) => {
                self.in_combat = true;
            },
            BeginLog(_) => {
                *self = Self::new();
            },
            CombatEvent(v) => {
                self.update_unit_state(v.source_unit());
                self.update_unit_state(v.target_unit());
            },
            EffectChanged(v) => {
                self.effects.handle_effect_changed(v);
            },
            EffectInfo(v) => {
                self.insert_effect_info(v);
            },
            EndCombat(_) => {
                self.remove_enemy_units();
                self.in_combat = false;
            },
            HealthRegen(v) => {
                self.update_unit_state(v.unit());
            },
            PlayerInfo(v) => {
                self.update_player(v);
            },
            UnitAdded(v) => {
                self.add_unit(v);
            },
            UnitChanged(v) => {
                self.update_unit(v);
            },
            UnitRemoved(v) => {
                self.remove_unit(v);
            },
            
            BeginCast(_) => { /* noop */ },
            BeginTrial(_) => { /* noop */ },
            EndCast(_) => { /* noop */ },
            EndLog(_) => { /* noop */ },
            EndTrial(_) => { /* noop */ },
            MapChanged(_) => { /* noop */ },
            TrialInit(_) => { /* noop */ },
            ZoneChanged(_) => { /* noop */ },
        }
    }

    /// process multiple events
    pub fn handle_events<Iter, T>(&mut self, e: Iter)
    where
        Iter: IntoIterator<Item = T>,
        T: Borrow<Event>,
    {
        e.into_iter()
         .for_each(|event| self.handle_event(event.borrow()));
    }

    fn add_unit(&mut self, e: &EventUnitAdded) {
        self.entities
            .insert(*e.unit_id(), Unit {
                unit_type: *e.unit_type(),
                state: UnitState::new(*e.unit_id()),
                reaction: *e.reaction(),
                equipment: Default::default(),
                name: e.name().clone(),
                display_name: e.display_name().clone(),
                monster_id: *e.monster_id(),
                race_id: *e.race_id(),
                class_id: *e.class_id(),
                is_boss: *e.is_boss(),
            });
    }

    fn update_unit(&mut self, e: &EventUnitChanged) {
        self.entities
            .get_mut(e.unit_id())
            .map(|unit| {
                unit.reaction = *e.reaction();
            });
    }

    fn remove_unit(&mut self, e: &EventUnitRemoved) {
        self.entities
            .remove(e.unit_id());

        let removed_effects = self.effects
            .recevied_effects
            .remove(e.unit_id());

        // iterate over `received_effects` and remove them, from `effects` and `granted_effects`
        removed_effects.map(|v| v.iter().for_each(|track_id| {
            self.effects
                .effects
                .remove(track_id)
                .map(|effect_changed| {
                    self.effects
                        .granted_effects
                        .remove(effect_changed.source_unit().unit_id());
                });
        }));
    }

    // i can't determine if this should be called
    // and if on combat end or combat begin
    fn remove_enemy_units(&mut self) {
        let hostile_units: Vec<_> = self.entities()
            .iter()
            .filter_map(|(id, unit)| {
                // (unit.reaction() == &UnitReactionType::Hostile).then_some(*id)
                (unit.monster_id() != &MonsterId(0)).then_some(*id)
            }).collect();

        hostile_units.into_iter()
            .for_each(|unit_id| {
                self.remove_unit(&EventUnitRemoved { unit_id })
            });
    }

    fn update_player(&mut self, e: &EventPlayerInfo) {
        self.entities
            .get_mut(e.unit_id())
            .map(|unit| {
                e.equipment_info()
                 .iter()
                 .for_each(|eq| {
                    unit.equipment
                        .insert(*eq.slot(), eq.clone());
                    
                    // TODO: add ability bars and long term effects
                 });
            });
    }

    fn update_unit_state(&mut self, unit_state: &UnitState) {
        self.entities
            .get_mut(unit_state.unit_id())
            .map(|unit| {
                unit.state = unit_state.clone();
            });
    }

    #[inline]
    fn insert_ability_info(&mut self, e: &EventAbilityInfo) {
        self.ability_info.insert(*e.ability_id(), e);
    }

    #[inline]
    fn insert_effect_info(&mut self, e: &EventEffectInfo) {
        self.effect_info.insert(*e.ability_id(), e);
    }
}

impl EffectMap {
    /// process EffectChanged event
    pub fn handle_effect_changed(&mut self, e: &EventEffectChanged) {
        match e.change_type() {
            EffectChangeType::Gained => {
                // MAYBE TODO: if effect with same ability id exists, remove old one
                self.insert(e.clone());
            },
            EffectChangeType::Updated => {
                self.insert(e.clone());
            },
            EffectChangeType::Faded => {
                self.remove(e.cast_id());
            }
        }
    }

    fn remove(&mut self, id: &TrackId) {
        self.effects
            .remove(&id)
            .map(|effect| {
                self.granted_effects
                    .get_mut(effect.source_unit().unit_id())
                    .map(|v| v.retain(|tid| tid != id));

                self.recevied_effects
                    .get_mut(effect.target_unit().unit_id())
                    .map(|v| v.retain(|tid| tid != id));

                effect
            });
    }

    fn insert(&mut self, e: EventEffectChanged) {
        let cast_id = *e.cast_id();
        let source_unit = *e.source_unit().unit_id();
        let target_unit = *e.target_unit().unit_id();

        let option = self.effects
            .insert(cast_id, e);

        match option {
            Some(_v) => {
                // was already stored, and was updated
                // we dont need to update another fields
            },
            None => {
                // was not stored
                // fill remaining data
                self.granted_effects
                    .entry(source_unit)
                    .or_default()
                    .push(cast_id);

                self.recevied_effects
                    .entry(target_unit)
                    .or_default()
                    .push(cast_id);
            }
        }
    }

    /// get effect by its `TrackId`
    pub fn get_by_id(&self, track_id: &TrackId) -> Option<&EventEffectChanged> {
        self.effects.get(track_id)
    }

    /// get all effects that unit granted (possible to other units)
    pub fn get_granted_effects(&self, unit_id: &UnitId) -> Option<&[TrackId]> {
        self.granted_effects.get(unit_id).map(Vec::as_slice)
    }

    /// get all effects that unit has received
    pub fn get_received_effects(&self, unit_id: &UnitId) -> Option<&[TrackId]> {
        self.recevied_effects.get(unit_id).map(Vec::as_slice)
    }

    /// get all active effects
    pub fn effects(&self) -> &HashMap<TrackId, EventEffectChanged> {
        &self.effects
    }
}

impl<T> AbilityInfoMap<T>
where
    T: Clone,
{
    fn insert(&mut self, id: AbilityId, info: &T) {
        self.0
            .entry(id)
            .or_insert_with(|| info.clone());
    }

    /// retrieve informations about ability/effect
    pub fn get_info(&self, ability_id: &AbilityId) -> Option<&T> {
        self.0.get(ability_id)
    }

    /// get raw access to underlying HashMap
    pub fn inner(&self) -> &HashMap<AbilityId, T> {
        &self.0
    }
}

impl<T> Default for AbilityInfoMap<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}
