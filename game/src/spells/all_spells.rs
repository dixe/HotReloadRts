use nohash_hasher::IntMap;
use crate::entity_system::*;
use crate::state;
use crate::math::*;
use crate::damage::*;


pub type SpellId = usize;
pub type InstantCastFn = fn(EntityId, &mut state::State);

#[derive(Clone)]
pub enum Spell {
    Instant(InstantSpell),
    OverTimeSpell(OverTimeSpell)
}

#[derive(Clone)]
pub struct InstantSpell {
    pub call_fn: InstantCastFn
}

#[derive(Debug, Default, Clone)]
pub struct OverTimeSpell {
    pub radius: f32,
    pub duration: f32,
}

#[derive(Default, Clone)]
pub struct AllSpells {
    pub spells : Vec::<Spell>
}

impl std::fmt::Debug for AllSpells {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AllSpells")
         .finish()
    }
}


pub fn create_all_spells() -> AllSpells {
    AllSpells {
        spells : vec![Spell::Instant(InstantSpell {
            call_fn: instant_attack,
        })]
    }
}


pub fn instant_attack(id: EntityId, state: &mut state::State) {

    println!("Instant_attack");
    if let Some(target_id) = state.entities.targets.get(&id) {
        apply_damage(*target_id, 0.2, &mut state.entities.damage);
    }
}
