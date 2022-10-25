extern crate game;
use game::commands::{Command, Target};
use game::math::*;
use nalgebra as na;
use rand::Rng;


mod random_damage;

pub type V3 = na::Vector3::<f32>;
pub type Rot = Rotation2;//na::geometry::Rotation3::<f32>;

#[no_mangle]
pub extern "Rust" fn step(state: &mut game::State) {

    let dt = state.dt;
    let count = state.entities.positions.len();

    update_selected_command(state);

    random_damage::damage(state);

    run_step(state);

    // Update positions
    for i in 0..count {
        state.entities.positions[i] += state.entities.velocities[i] * dt;
    }
}


fn update_selected_command(state: &mut game::State) {
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
                    let _ = state.entities.move_targets.remove(&select_index);
                    state.entities.velocities[select_index] = V3::zeros();
                }
                _ => {
                    todo!();
                }
            }
        }
    }
}


// maybe return an manifold like thing for each entity. Instead of instaed
fn run_step(state: &mut game::State) {
    let count = state.entities.positions.len();
    let dt = state.dt;

    // Run logic based on entity state
    // State is based on which tables entity has entry, fx move_target will trigger move to logic
    // nothing will trigger spread
    // TODO: hold will trigger holding position
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

                    state.entities.velocities[i] = update_vel(move_to_target, dt);
                }
            }
        } else {
            state.entities.velocities[i] = spread(i, &state.entities.positions, dt);
        }

    }
}



fn spread(i: usize, positions: &[V3], dt: f32) -> V3 {
    // standing and waiting
    // if anyone else is near, move away

    let mut rng = rand::thread_rng();

    let pos_i = positions[i];
    let mut away_dir = V3::zeros();
    let count = positions.len();

    for j in 0..count {

        if j == i {
            continue;
        }
        let dist_other = pos_i - positions[j];

        let mag = dist_other.magnitude();

        if mag > 0.0 {
            if mag < 0.5 {
                away_dir += dist_other / mag;
            }
        } else {
            // move away in random dir from other on top
            let x : f32 = rng.gen();
            let y : f32 = rng.gen();

            away_dir += V3::new(x - 0.5, y - 0.5, 0.0);
        }
    }

    let mag = away_dir.magnitude();

    if mag > 0.0 {
        let dir = away_dir.normalize();
        update_vel(MoveToTarget { dir, dist: mag}, dt)

    } else {
        V3::zeros()
    }
}



fn update_vel(mtt: MoveToTarget, dt: f32) -> V3 {
    let move_speed = 1.0;
    // velocity should be updated such that we don't go over max speed. But keep momentum until we are at target, and not slowing down before
    // while still hitting the target as precisely as we can
    mtt.dir * f32::min((1.0/dt) * mtt.dist, move_speed)

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
