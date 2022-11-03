use crate::entity_system::*;
use crate::state;
use crate::damage::*;
use crate::math::V3;
use crate::types::*;


pub type SpellId = usize;


pub type TargetInstantFn = fn(EntityId, &mut state::State);
pub type TargetOverTimeFn = fn(EntityId, &mut state::State);

pub type AoeInstantFn = fn(V3, EntityId, &mut state::State);

pub type AoeTickFn = fn(ActiveAoeSpell, &mut state::State);



#[derive(Clone)]
pub struct ActiveAoeSpell {
    pub radius: f32,
    pub tick_fn: AoeTickFn,
    pub pos: V3,
    pub seconds_left: Sec,
    pub team: Team,
}



impl std::fmt::Debug for ActiveAoeSpell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActiveAoeSpells")
            .field("radius", &self.radius)
            .field("pos", &self.pos)
            .field("seconds_left", &self.seconds_left)
            .field("team", &self.team)
         .finish()
    }
}


#[derive(Clone)]
pub enum Spell {
    TargetInstant(TargetInstantSpell),
    AoeInstant(AoeInstantSpell),
    AoeOverTime(AoeOverTimeSpell)
}

#[derive(Clone)]
pub struct TargetInstantSpell {
    pub call_fn: TargetInstantFn,
    pub cooldown: Sec,

}

#[derive(Clone)]
pub struct AoeInstantSpell {
    pub call_fn: AoeInstantFn,
    pub radius: f32,
}

#[derive(Clone)]
pub struct AoeOverTimeSpell {
    pub tick_fn: AoeTickFn,
    pub duration: Sec,
    pub radius: f32,
    pub cooldown: Sec
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
        spells : vec![
            Spell::TargetInstant(TargetInstantSpell {
                call_fn: instant_attack,
                cooldown: 1.0,
            }),
            Spell::AoeOverTime(AoeOverTimeSpell {
                radius: 1.0,
                duration: 2.0,
                tick_fn: aoe_heal_tick,
                cooldown: 1.0,
            })
        ]
    }
}

pub fn aoe_heal_tick(si: ActiveAoeSpell, state: &mut state::State)  {    let hps = 1.0;

    // find
    for i in 0..state.entities.positions.len() {
        let dist = (si.pos - state.entities.positions[i]).magnitude();

        if dist < si.radius && state.entities.team[i] == si.team {
            let id =  state.entities.ids[i];
            if let Some(dmg) = state.entities.damage.get_mut(&id) {
                dmg.health = f32::min(1.0, dmg.health + hps * state.dt);
            }
        }
    }

}


pub fn instant_attack(id: EntityId, state: &mut state::State) {
    if let Some(target_id) = state.entities.targets.get(&id) {
        println!("Instant_attack {} -> {}", id, target_id);
        apply_damage(*target_id, 0.2, &mut state.entities.damage);
    }
}
