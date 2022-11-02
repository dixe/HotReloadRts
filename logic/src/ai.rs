extern crate game;
use nalgebra as na;
use game::entity_system::*;
use game::behaviour_tree::Decision;
use crate::V3;
use game::spells;

pub fn run_ais(state: &mut game::State) {

    let mut buffer : Vec::<(EntityId, Decision)>  = vec![];
    let count = state.entities.team.len();
    for i in 0..count {

        if state.entities.team[i] == 2 {
            let id = state.entities.ids[i];

            let decision = state.behaviour_tree.run(id, state);

            buffer.push((id, decision));

        }
    }

    for (id, des) in &buffer {
        match des {
            Decision::Target(target_id) => {
                state.entities.targets.insert(*id, *target_id);
            },

            Decision::MoveTo(target_id) => {
                if let Some(&idx) = state.entities.id_to_index.get(&target_id) {
                    let pos = state.entities.positions[idx];

                    state.entities.move_targets.insert(*id, pos);
                }
            },

            Decision::UnTarget => {
                state.entities.targets.remove(id);
            },

            Decision::AttackTarget => {
                state.entities.move_targets.remove(id);

                let idx = *state.entities.id_to_index.get(&id).unwrap();
                state.entities.velocities[idx] = V3::zeros();

                // TODO: Get id/index from entity base attack or something. Maybe from the Decision:Attack
                let spell = &state.all_spells.spells[0];

                // TODO: Call a centrelized cast function, that can check cooldown ect.
                match spell {
                    spells::Spell::Instant(is) => {
                        (is.call_fn)(*id, state);
                    },
                    _ => todo!()
                }
            },
            _ => {}
        }
    }
}
