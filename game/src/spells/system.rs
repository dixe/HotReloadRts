use crate::state;
use crate::spells;
use crate::entity_system::*;
use crate::math::V3;


pub fn update_spells(state: &mut state::State) {

    for i in 0..state.active_aoe_spells.len() {

        let spell_info =state.active_aoe_spells[i].clone();


        (spell_info.tick_fn)(spell_info, state);

    }

    // Decrement all cooldowns
    for (_key, secs) in &mut state.entities.cooldown {
        *secs -= state.dt;
    }


    // remove cooldowns that are not on cooldown any more
    state.entities.cooldown.retain(|_,v| {
        *v >= 0.0
    });


    remove_ended_spells(&mut state.active_aoe_spells);
}

#[derive(Debug, Clone, Copy)]
pub enum SpellCastResult {
    Cast,
    OnCoolDown(f32),
    Failed

}

//TODO: If we get mutable state anyway, we might as well store cast position in state, and use same interface
pub fn cast_spell(caster_id: EntityId, spell_id: spells::SpellId, state: &mut state::State) -> SpellCastResult {

    let spell = state.all_spells.spells[spell_id].clone();
    match spell {
        spells::Spell::TargetInstant(s) => {

            // check cooldown
            if let Some(cd) = state.entities.cooldown.get(&(caster_id, spell_id)) {
                return SpellCastResult::OnCoolDown(*cd);
            }

            (s.call_fn)(caster_id, state);

            // Add Cooldown

            state.entities.cooldown.insert((caster_id, spell_id), s.cooldown);
                return SpellCastResult::Cast;
        },
        _ => {
            return SpellCastResult::Failed;
        }
    }
}


pub fn cast_aoe_spell(pos: V3, caster_id: EntityId, spell_id: spells::SpellId, state: &mut state::State) -> SpellCastResult {
    match &state.all_spells.spells[spell_id] {
        spells::Spell::AoeInstant(s) => {
            // check cooldown
            if let Some(cd) = state.entities.cooldown.get(&(caster_id, spell_id)) {
                return SpellCastResult::OnCoolDown(*cd);
            }

            (s.call_fn)(pos, caster_id, state);

            return SpellCastResult::Cast;
        },
        spells::Spell::AoeOverTime(s) => {

            // check cooldown

            if let Some(cd) = state.entities.cooldown.get(&(caster_id, spell_id)) {
                return SpellCastResult::OnCoolDown(*cd);
            }

            let caster_idx = match state.entities.id_to_index.get(&caster_id) {
                None => {return SpellCastResult::Failed;},
                Some(idx) => idx,
            };

            let team = state.entities.team[*caster_idx];


            let new_spell = spells::ActiveAoeSpell {
                radius: s.radius,
                pos,
                seconds_left: s.duration,
                tick_fn: s.tick_fn,
                team,
            };

            state.active_aoe_spells.push(new_spell);

            return SpellCastResult::Cast;
        },
        _ => {
            return SpellCastResult::Failed;
        }
    }
}





pub fn remove_ended_spells(active_spells: &mut Vec::<spells::ActiveAoeSpell>) {
    let mut count = active_spells.len();
    let mut i = 0;

    while i < count {
        if active_spells[i].seconds_left <= 0.0 {
            active_spells.swap_remove(i);
            count -= 1;
        }
        else {
            // only increment i when we pass a spell that is still active
            i += 1;
        }
    }
}

// Can be removed when confimed that using a struct is better than seperate arrays
/*
fn remove_from_active(index: usize, active: &mut spells::ActiveSpells) {
    active.radius.swap_remove(index);
    active.pos.swap_remove(index);
    active.seconds_left.swap_remove(index);
    active.team.swap_remove(index);
    active.team.push(cast.tick_fn);
}

fn add_to_active(cast: AddToActiveSpell, active: &mut spells::ActiveSpells) {
    active.radius.push(cast.r);
    active.pos.push(cast.pos);
    active.seconds_left.push(cast.dur);
    active.team.push(cast.team);
    active.team.push(cast.tick_fn);
}
*/
