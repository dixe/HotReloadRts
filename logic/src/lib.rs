extern crate game;
use game::commands::{Command, Target};

use nalgebra as na;


pub type V3 = na::Vector3::<f32>;


#[no_mangle]
pub extern "Rust" fn step(state: &mut game::State) {

    let dt = state.dt;
    let count = state.positions.len();

    println!("mp_step {:?}", state.mouse_pos);

    if state.selected.len() > 0 && state.command != Command::Empty {
        // apply state.command to every item in selection
        for &select_index in &state.selected {
            // for now everything is units, so just do the command for all

            match state.command {
                Command::DefaultRightClick(target) => {
                    match target {
                        Target::Position(x,y) => {
                            state.move_targets.insert(select_index, V3::new(x, y, 0.5));
                        },
                        _ => {
                            todo!();
                        }
                    }
                },
                _ => {
                    todo!();
                }
            }
        }
    }





    // Update positions and rotation
    for i in 0..count {
        if let Some(target) = state.move_targets.get(&i) {
            state.velocities[i] =  (target - state.positions[i]).normalize();
        }
        state.positions[i] += state.velocities[i] * dt;
    }
}
