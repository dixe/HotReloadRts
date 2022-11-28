use crate::types::*;
use gl_lib::animations::*;
use crate::state;
use crate::entity_system::StateChangeTo;


mod animation_map;
pub use self::animation_map::*;


#[derive(Debug, Clone)]
pub struct ActiveAnimation {
    pub animation_id: AnimationId,
    pub current_time: Sec
}



/// Update active animation time elapsed, done seperated from update animation, since setting a new animation,
/// fx from cast a spell, should start at 0 and not at dt.
pub fn step_animation(state: &mut state::State) {

    for anim in state.entities.current_animation.values_mut() {
        anim.current_time += state.dt;
    }
}



/// Update entities bones, from the current animation
pub fn update_animations(state: &mut state::State, animations: &Animations) {

    // loop over state changes and update each entities

    for (id, new_state) in &state.entities.state_change {
        // TODO: Some logic so we don't keep restarting animations.
        if let Some(map_id) = state.entities.animation_map.get(&id) {

            let map = state.animation_maps.get(*map_id);



            let animation_id = match new_state {
                StateChangeTo::Idle => map.idle,
                StateChangeTo::Move => map.walk,
                StateChangeTo::Attack => map.attack,
                _ => todo!(),
            };


            if let Some(cur_anim) = state.entities.current_animation.get(id) {
                if cur_anim.animation_id == animation_id {
                    continue;
                }
            }

            let _ = &state.entities.current_animation.insert(*id, ActiveAnimation { animation_id, current_time: 0.0});
        }
    }

    state.entities.state_change.clear();

    // given the current animation, update the entities bones




    for (e_id, anim) in &state.entities.current_animation {
        if let Some(animation) = animations.get(anim.animation_id) {

            let bones = state.entities.bones.get_mut(&e_id).unwrap();
            let skeleton = state.entities.skeletons.get_mut(&e_id).unwrap();

            animation.update_bones(bones, skeleton, anim.current_time);
        }
    }
}
