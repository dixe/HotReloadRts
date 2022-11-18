use crate::types::*;
use gl_lib::animations::*;


#[allow(dead_code)]
#[derive(Debug)]
pub struct ActiveAnimation {
    animation: Animation, // list of key frames, and total duration
    current_time: Sec
}




pub fn update_skeleton(skel: &mut skeleton::Joints) {


}


pub fn keyframe_from_t(skeleton: &skeleton::Skeleton, next_keyframe: usize, t: f32) -> KeyFrame {


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
