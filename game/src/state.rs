use nalgebra as na;
use nalgebra::vector;
use crate::game;
use crate::commands::Command;
use nohash_hasher::IntMap;


pub type EntityId = usize;
pub type V3 = na::Vector3::<f32>;

// All into regarding the simulation
#[derive(Debug)]
pub struct State {
    pub next_id: EntityId,

    pub select_box: Option<game::SelectBox>,

    pub selected: Vec::<EntityId>,
    pub mouse_pos: na::Vector2::<f32>,


    // ENTITY PROPERTIES
    pub positions: Vec::<V3>,
    pub velocities: Vec::<V3>,
    pub z_rotations: Vec::<f32>,
    pub move_targets: IntMap<EntityId, V3>,
    //pub commands: Vec::<Command>,


    // GLOBAL STUFF
    pub select_pos: V3, // should be 1 for each entity, or in a int map maybe, or just not used, but render the move targets for selected units
    pub light: V3,
    pub dt: f32,
    pub command: Command,
}


impl State {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            positions: vec![],
            velocities: vec![],
            z_rotations: vec![],
            light: vector![0.0, -30.0, 30.0],
            move_targets: Default::default(),

            //commands: vec![],
            dt: 1.0/60.0,

            select_box: None,
            selected: vec![],

            command: Command::Empty,
            select_pos: V3::new(0.0, 0.0, -1.0),
            mouse_pos: Default::default(),
        }
    }

    pub fn add_entity(&mut self, pos: V3) {
        self.positions.push(pos);
        self.velocities.push(na::Vector3::new(0.0, 0.0, 0.0));
        self.z_rotations.push(0.0);
        //self.commands.push(Command::Empty);

    }

}



pub fn init() -> State {

    let mut state = State::new();

    for i in 1..100 {
        for j in 1..5 {
            state.add_entity(vector![i as f32 * 1.0, j as f32 * 1.0, 0.5]);
        }
    }
    state
}
