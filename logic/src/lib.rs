extern crate game;

use nalgebra as na;


pub type EntityId = usize;
pub type V3 = na::Vector3::<f32>;


#[no_mangle]
pub extern "Rust" fn step(state: &mut game::State) {

    let dt = state.dt;
    let count = state.positions.len();


    for i in 0..count {
    }

    // Update positions and rotation
    for i in 0..count {
        state.positions[i] += state.velocities[i] * dt;
    }
}
