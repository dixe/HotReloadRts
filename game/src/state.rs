use nalgebra as na;
use nalgebra::vector;
use crate::game;
use crate::commands::{ Command, Action };
use crate::entity_system::*;
use crate::spells::*;
use crate::math::*;


// All into regarding the simulation
#[derive(Debug)]
pub struct State {


    pub select_box: Option<game::SelectBox>,

    pub selected: Vec::<EntityId>,
    pub mouse_pos: na::Vector2::<f32>,
    pub entities: Entitites,

    pub spells: Spells,


    // GLOBAL STUFF
    pub select_pos: V3, // should be 1 for each entity, or in a int map maybe, or just not used, but render the move targets for selected units
    pub light: V3,
    pub dt: f32,
    pub command: Command,
    pub action: Action

}


impl State {
    pub fn new() -> Self {
        Self {
            entities: Default::default(),
            spells: Default::default(),
            light: vector![0.0, -30.0, 30.0],

            //commands: vec![],
            dt: 1.0/60.0,

            select_box: None,
            selected: vec![],

            command: Command::Empty,
            action : Action::Move,

            select_pos: V3::new(0.0, 0.0, -1.0),
            mouse_pos: Default::default(),
        }
    }

}



pub fn init() -> State {

    let mut state = State::new();

    for i in 1..5 {
        for j in 1..5 {
            state.entities.add_entity(vector![i as f32 * 1.0, j as f32 * 1.0, 0.0]);
        }
    }

    state
}
