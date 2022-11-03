extern crate game;
use game::commands::{Command, Target, Action};
use game::math::*;
use nalgebra as na;
use rand::Rng;
use game::entity_system::*;
use game::spells;
use game::damage;

mod ai;

pub type V3 = na::Vector3::<f32>;
pub type Rot = Rotation2;//na::geometry::Rotation3::<f32>;

#[no_mangle]
pub extern "Rust" fn step(state: &mut game::State) {

    let dt = state.dt;

    update_selected_command(state);

    //damage::damage(state);

    // TODO: Update targets, remove where target is dead

    spells::update_spells(state);

    ai::run_ais(state);

    run_step(state);

    // remove dead entities
    let dead = damage::find_dead(&state.entities.damage);

    for id in &dead {
        state.entities.remove(id);
    }

    let count = state.entities.positions.len();
    // Update positions
    for i in 0..count {
        state.entities.positions[i] += state.entities.velocities[i] * dt;
    }
}


fn update_selected_command(state: &mut game::State) {
    if state.selected.len() > 0 && state.command != Command::Empty {
        // apply state.command to every item in selection
        'update: for &select_id in &state.selected {
            // for now everything is units, so just do the command for all
            match state.command {
                Command::DefaultRightClick(target) => {
                    match state.action {
                        Action::Move => {
                            update_move_target(&mut state.entities.move_targets, target, select_id);
                        },
                        Action::Spell => {

                            match target {
                                Target::Position(x,y) => {
                                    // TODO: maybe handle on cooldown
                                    let spell_id = 1;

                                    let cast_res = spells::cast_aoe_spell(V3::new(x, y, 0.0), select_id, spell_id, state);

                                 },
                                _ => {
                                    todo!();
                                }
                            }
                            state.action = Action::Move;
                            break 'update;
                        }
                    }
                },
                Command::Stop => {
                    // remove target
                    let _ = state.entities.move_targets.remove(&select_id);
                    if let Some(&id) = state.entities.id_to_index.get(&select_id) {
                        state.entities.velocities[id] = V3::zeros();
                    }
                }
                _ => {
                    todo!();
                }
            }
        }
    }
}


fn update_move_target(move_targets: &mut MoveTargets, target: Target, index: EntityId) {
    match target {
        Target::Position(x,y) => {
            move_targets.insert(index, V3::new(x, y, 0.0));
        },
        _ => {
            todo!();
        }
    }
}


// maybe return an manifold like thing for each entity. Instead of directly manipulation the state
fn run_step(state: &mut game::State) {
    let count = state.entities.positions.len();
    let dt = state.dt;

    // Run logic based on entity state
    // State is based on which tables entity has entry, fx move_target will trigger move to logic
    // nothing will trigger spread

    for i in 0..count {
        let id = state.entities.ids[i];

        if let Some(target) = state.entities.move_targets.get(&id) {

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
            //state.entities.velocities[i] = spread(i, &state.entities.positions, dt);
        }

    }
}



fn spread(i: usize, positions: &[V3], dt: f32) -> V3 {
    // standing and waiting
    // if anyone else is near, move away, only for same team

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

    let diff = target - pos;

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
