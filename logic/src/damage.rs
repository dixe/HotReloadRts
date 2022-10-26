extern crate game;
use rand::{thread_rng, Rng};

use game::entity_system::*;


pub fn damage(state: &mut game::State) {

    let mut rng = thread_rng();

    for entity_id in &state.entities.entities {

        let damage = f32::max(rng.gen::<f32>() - 0.95, 0.0);

        if damage > 0.0 {
            if let Some(current_damage) = state.entities.damage.get_mut(entity_id) {
                current_damage.health -= damage;
            }
            else {
                state.entities.damage.insert(*entity_id, EntityDamage { health: 1.0 - damage});
            }
        }

    }

}


// maybe don't do this, but do cleanup when a entities dies
pub fn cleanup(state: &mut game::State) {
    let mut to_remove = vec![];
    for (&id, dmg) in &state.entities.damage {

        if dmg.health >= 1.0 {
            to_remove.push(id);
        }
    }

    for id in &to_remove {
        state.entities.damage.remove(id);
    }
}


pub fn find_dead(damage: &DamageMap) -> Vec::<EntityId> {

    let mut res = vec![];
    for (id, dmg) in damage.iter() {
        if dmg.health <= 0.0 {
            res.push(*id);
        }
    }

    res
}
