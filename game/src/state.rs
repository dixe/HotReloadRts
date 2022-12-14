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
use crate::animation_system::*;


// All info regarding the simulation
#[derive(Debug)]
pub struct State {


    pub select_box: Option<game::SelectBox>,

    pub selected: Vec::<EntityId>,
    pub mouse_pos: na::Vector2::<f32>,
    pub entities: Entities,

    pub active_aoe_spells: Vec::<ActiveAoeSpell>,

    pub animation_maps: AnimationMaps,
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


            animation_maps: Default::default(),

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

    // build animation maps

    // foreach mesh we can build an animation map
    let walk_animation_id = &render_data.animations.get_mesh_name_animation(&boid_unit.model_name, "idle");

    let id_to_name = render_data.animations.id_to_name();

    let mut animation_maps = AnimationMaps::default();

    for (mesh, anim_ids) in render_data.animations.get_animations() {

        let mut idle = None;
        let mut attack = None;
        let mut walk = None;

        for id in anim_ids {
            let name = id_to_name.get(&id).unwrap();
            if name == "idle"{
                idle = Some(*id);
            } else if name == "walk" {
                walk = Some(*id);
            }
            else if name == "attack_2"{
                attack = Some(*id);
            } else {
                // TODO: spells
            }
        }

        let map = AnimationMap {
            idle: idle.unwrap(),
            walk: walk.unwrap(),
            attack: attack.unwrap(),
            spells: Default::default()
        };

        animation_maps.add_map(mesh, map);
    }

    state.animation_maps = animation_maps;

    for i in 1..5 {
        for j in 1..5 {
            let id = state.entities.add_entity(vector![i as f32 * 1.0, j as f32 * 1.0, 0.0], i % 3, boid_index);

            if let Some(ma) = &model_animations {

                let skeleton = ma.skeleton.clone();

                let bones = skeleton.create_bones();

                state.entities.add_skeleton(id, skeleton);
                state.entities.add_bones(id, bones);

                // animations are stored in render data
                // Store animation Ids. Try to get for baisc, fx move
                // other animations are linked to a spell, so attack spell should also tell animation.
                // By the time we get here that should be just an animationId

                // For now just set everyting to loop the walk animation
                if let Some(wa) = walk_animation_id {

                    state.entities.set_active_animation(id, *wa);
                }
            }

            if let Some(map_id) = state.animation_maps.get_map_id(&boid_unit.model_name)  {
                state.entities.add_animation_map(id, *map_id);
            }
        }
    }
}



pub fn init() -> State {

    let mut state = State::new();

    state.behaviour_tree = heal_then_kill();

    state
}
