extern crate game;
use game::commands::{Command, Target};

use nalgebra as na;


pub type V3 = na::Vector3::<f32>;


#[no_mangle]
pub extern "Rust" fn step(state: &mut game::State) {

    let dt = state.dt;
    let count = state.positions.len();


    if state.selected.len() > 0 && state.command != Command::Empty {
        // apply state.command to every item in selection
        for &select_index in &state.selected {
            // for now everything is units, so just do the
            println!("Command: '{:?}' for :'{:?}'", state.command, state.positions[select_index]);

        }
    }



    // Update positions and rotation
    for i in 0..count {
        state.positions[i] += state.velocities[i] * dt;
    }
}
