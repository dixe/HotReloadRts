use rand::{thread_rng, Rng};
use crate::entity_system::*;
use crate::state;

pub fn damage(state: &mut state::State) {

    let mut rng = thread_rng();

    for entity_id in &state.entities.ids {

        let damage = f32::max(rng.gen::<f32>() - 0.95, 0.0);

        if damage > 0.0 {
            apply_damage(*entity_id, damage, &mut state.entities.damage);
        }

    }

}


// maybe don't do this, but do cleanup when a entities dies
pub fn cleanup(state: &mut state::State) {
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


pub fn apply_damage(target_id: EntityId, damage: f32, dmg_map: &mut DamageMap) {

    if let Some(current_damage) = dmg_map.get_mut(&target_id) {
        current_damage.health -= damage;
    }
    else {
        dmg_map.insert(target_id, EntityDamage { health: 1.0 - damage});
    }

}
