use nalgebra as na;
use nalgebra::vector;
use crate::game;

pub type EntityId = usize;
pub type V3 = na::Vector3::<f32>;

// All into regarding the simulation
#[derive(Debug)]
pub struct State {
    pub next_id: EntityId,

    pub select_box: Option<game::SelectBox>,

    pub selected: Vec::<usize>,

    pub positions: Vec::<V3>,
    pub velocities: Vec::<V3>,
    pub z_rotations: Vec::<f32>,
    pub steer: Vec::<Steer>,

    pub light: V3,
    pub sep_w: f32,
    pub align_w: f32,
    pub coh_w: f32,
    pub steer_force: f32,
    pub dt: f32,

}

#[derive(Debug, Clone, Copy, Default)]
pub struct Steer {
    pub seperate: V3,
    pub run_from: V3
}


impl State {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            positions: vec![],
            velocities: vec![],
            z_rotations: vec![],
            light: vector![0.0, -30.0, 30.0],
            steer: vec![],
            sep_w: 1.5,
            align_w: 1.0,
            coh_w: 1.0,
            steer_force: 5.0,
            dt: 1.0/60.0,
            select_box: None,
            selected: vec![]

        }
    }

    pub fn add_entity(&mut self, pos: V3) {
        self.positions.push(pos);
        self.velocities.push(na::Vector3::new(0.0, 0.0, 0.0));
        self.z_rotations.push(0.0);
        self.steer.push(Default::default());
    }

}



pub fn init() -> State {

    let mut state = State::new();

    for i in 1..3 {
        for j in 1..5 {
            state.add_entity(vector![i as f32 * 1.0, j as f32 * 1.0, 0.5]);
        }
    }
    state
}
