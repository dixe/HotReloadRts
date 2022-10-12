extern crate game;
use game::commands::{Command, Target};
use game::math::*;
use nalgebra as na;

pub type V3 = na::Vector3::<f32>;
pub type Rot = Rotation2;//na::geometry::Rotation3::<f32>;

#[no_mangle]
pub extern "Rust" fn step(state: &mut game::State) {

    let dt = state.dt;
    let count = state.entities.positions.len();


    if state.selected.len() > 0 && state.command != Command::Empty {
        // apply state.command to every item in selection
        for &select_index in &state.selected {
            // for now everything is units, so just do the command for all

            match state.command {
                Command::DefaultRightClick(target) => {
                    match target {
                        Target::Position(x,y) => {
                            state.entities.move_targets.insert(select_index, V3::new(x, y, 0.0));
                        },
                        _ => {
                            todo!();
                        }
                    }
                },
                Command::Stop => {
                    // remove target
                    let removed = state.entities.move_targets.remove(&select_index);
                    state.entities.velocities[select_index] = V3::zeros();
                }
                _ => {
                    todo!();
                }
            }
        }
    }

    // Update positions and rotation
    for i in 0..count {
        if let Some(target) = state.entities.move_targets.get(&i) {

            let move_res = move_to(state.entities.positions[i], *target, Rotation2 { radians: state.entities.z_rotations[i].radians });

            match move_res {
                MoveUpdate::AtTarget => {

                    state.entities.move_targets.remove(&i);
                    state.entities.velocities[i] = V3::zeros();
                },
                MoveUpdate::Rotate(rot) => {
                    // reset velocity so we don't rotate and move
                    state.entities.velocities[i] = V3::zeros();

                    // maybe change this to be an angular momentum, and then update rotation at the end like position and velocity
                    let rot_speed_angle_pr_sec = 36.0;
                    state.entities.z_rotations[i] += rot.interpolate(dt, rot_speed_angle_pr_sec);

                },
                MoveUpdate::Move(move_to_target) => {
                    let move_speed = 1.0;
                    // velocity should be updated such that we don't go over max speed. But keep momentum until we are at target, and not slowing down before
                    // while still hitting the target as precisely as we can
                    state.entities.velocities[i] = move_to_target.dir * f32::min((1.0/dt) * move_to_target.dist, move_speed);

                }
            }
        }

        state.entities.positions[i] += state.entities.velocities[i] * dt;
    }
}

#[derive(Debug, Clone, Copy)]
enum MoveUpdate {
    AtTarget,
    Rotate(Rot),
    Move(MoveToTarget)
}

#[derive(Debug, Clone, Copy)]
struct MoveToTarget {
    dir: V3,
    dist: f32
}

fn move_to(pos: V3, target: V3, z_rot: Rot) -> MoveUpdate {

    let mut diff = target - pos;

    let dist = diff.magnitude();

    if dist <= 0.001 {
        return MoveUpdate::AtTarget;
    }

    let dir = diff.normalize();

    let target_angle = Rotation2 {radians: update_rotation_z(dir)} ;

    let rot_diff = z_rot.diff(target_angle);

    if (rot_diff.radians.abs()) > 0.01 {
        return MoveUpdate::Rotate(rot_diff);
    }

    return MoveUpdate::Move(MoveToTarget {dir, dist});
}
