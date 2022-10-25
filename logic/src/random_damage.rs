extern crate game;
use rand::{thread_rng, Rng};

use game::entity_system::EntityDamage;


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
