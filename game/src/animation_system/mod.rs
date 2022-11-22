use crate::types::*;
use gl_lib::animations::*;
use crate::state;

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
    // given the current animation, update the entities bones


    for (e_id, anim) in &state.entities.current_animation {
        if let Some(animation) = animations.get(anim.animation_id) {

            let bones = state.entities.bones.get_mut(&e_id).unwrap();
            let skeleton = state.entities.skeletons.get_mut(&e_id).unwrap();

            animation.update_bones(bones, skeleton, anim.current_time);
        }
    }
}



fn keyframe_from_t(skeleton: &skeleton::Skeleton, _next_keyframe: usize, _t: f32) -> KeyFrame {


    let mut joints = Vec::new();


    for i in 0..skeleton.joints.len() {
        let translation = skeleton.joints[i].translation;
        let rotation = skeleton.joints[i].rotation;
        joints.push(Transformation {
            translation,
            rotation
        });
    }

    KeyFrame {
        joints
    }
    /*
    for i in 0..skeleton.joints.len() {
        let current_transformation = match next_keyframe {
            0 => {
                self.key_frames[0].joints[i]

            },
            n => {
                self.key_frames[n - 1].joints[i]
            }
        };

        let target_joint = &self.key_frames[next_keyframe].joints[i];

        let rotation = current_transformation.rotation.slerp(&target_joint.rotation, t);

        let translation = current_transformation.translation * (1.0 - t) + target_joint.translation * t;


        joints.push(Transformation {
            translation,
            rotation
        });
    }
*/
}
