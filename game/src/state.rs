use nalgebra as na;
use nalgebra::vector;
use crate::game;
use crate::commands::{ Command, Action };
use crate::entity_system::*;
use crate::spells::{self, ActiveAoeSpell, AllSpells};
use crate::math::*;
use crate::behaviour_tree::{Tree, TreeBuilder, heal_then_kill};
use crate::render;
use crate::loading;


// All info regarding the simulation
#[derive(Debug)]
pub struct State {


    pub select_box: Option<game::SelectBox>,

    pub selected: Vec::<EntityId>,
    pub mouse_pos: na::Vector2::<f32>,
    pub entities: Entities,

    pub active_aoe_spells: Vec::<ActiveAoeSpell>,

    pub all_spells: AllSpells,
    pub behaviour_tree : Tree,

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
            active_aoe_spells: vec![],
            all_spells: spells::create_all_spells(),

            light: vector![0.0, -30.0, 30.0],


            //commands: vec![],
            dt: 1.0/60.0,

            behaviour_tree : TreeBuilder::new().build(),

            select_box: None,
            selected: vec![],

            command: Command::Empty,
            action : Action::Move,

            select_pos: V3::new(0.0, 0.0, -1.0),
            mouse_pos: Default::default(),
        }
    }

}


pub fn populate(state: &mut State, game_assets: &loading::GameAssets, render_data: &render::RenderData) {

    let units = &game_assets.units;

    // TODO: Define number of units in a scene asset file.
    // for now use this

    let boid_unit = units.get("boid").unwrap();
    let boid_index = render_data.get_mesh_index(&boid_unit.model_name);

    let model_animations = match game_assets.models.get(&boid_unit.model_name)  {
        Some(model) => {
            model.animations.clone()
        },
        None => None
    };

    for i in 1..5 {
        for j in 1..5 {
            let id = state.entities.add_entity(vector![i as f32 * 1.0, j as f32 * 1.0, 0.0], i % 3, boid_index);

            if let Some(ma) = &model_animations {
                state.entities.add_joints(id, ma.skeleton.joints.clone());
                // add animations too, but maybe we should just store animations on render_data, since duplicating them
                // for all entities seems wastefull. We only need them once, to do interpolation for the given entitys
                // skeleton. Which we then can render
            }
        }
    }
}



pub fn init() -> State {

    let mut state = State::new();

    state.behaviour_tree = heal_then_kill();

    state
}
