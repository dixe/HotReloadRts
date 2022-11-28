use nalgebra as na;
use nohash_hasher::IntMap;

use crate::math::*;
use crate::spells::SpellId;
use crate::types::*;
use gl_lib::animations::{self, skeleton};
use crate::animation_system::*;


pub type EntityId = usize;
pub type EntityIndex = usize;
pub type Team = u32;

pub type MoveTargets = IntMap<EntityId, V3>;

pub type DamageMap = IntMap<EntityId, EntityDamage>;

pub type CoolDownMap = std::collections::HashMap<(EntityId, SpellId), Sec>;

#[derive(Debug, Default, Clone)]
pub struct Entities {

    next_id: usize,
    pub ids: Vec::<EntityId>,

    pub id_to_index: IntMap<EntityId, EntityIndex>,

    // COMPONENTS
    pub positions: Vec::<V3>,
    pub velocities: Vec::<V3>,
    pub z_rotations: Vec::<Rotation2>,
    pub team: Vec::<Team>,
    pub(crate) state_change: IntMap<EntityId, StateChangeTo>,

    pub mesh_index: Vec::<usize>,
    pub targets: IntMap<EntityId, EntityId>,
    move_targets: MoveTargets,

    pub damage: DamageMap,

    pub skeletons: IntMap<EntityId, skeleton::Skeleton>,
    pub bones : IntMap::<EntityId, skeleton::Bones>,

    pub current_animation: IntMap<EntityId, ActiveAnimation>,
    pub animation_map: IntMap<EntityId, AnimationMapId>,

    pub cooldown: CoolDownMap
}

#[derive(Debug, Clone)]
pub enum StateChangeTo {
    None,
    Idle,
    Move,
    Attack,
    Spell(SpellId),
}

#[derive(Debug, Default, Clone)]
pub struct EntityDamage {
    pub health: f32,
}


#[derive(Debug, Default, Clone)]
pub struct CoolDown {
    pub secs: f32,
}

impl Entities {
    pub fn add_entity(&mut self, pos: V3, team: Team, mesh_index: usize) -> EntityId {
        let id = self.get_id();

        self.ids.push(id);
        self.positions.push(pos);
        self.velocities.push(na::Vector3::new(0.0, 0.0, 0.0));
        self.z_rotations.push(Default::default());
        self.team.push(team);
        self.mesh_index.push(mesh_index);
        self.state_change.insert(id, StateChangeTo::Idle);

        self.id_to_index.insert(id, self.positions.len() - 1);

        id
    }

    fn get_id(&mut self) -> EntityId {
         let id = self.next_id;

        self.next_id += 1;

        id
    }

    pub fn remove(&mut self, id: &EntityId) {

        if let Some(index) = self.id_to_index.remove(id) {
            // swap index so after swap delete everything is still cool
            if let Some(&last_id) = self.ids.last() {
                self.id_to_index.insert(last_id, index);
            }
            self.ids.swap_remove(index);
            self.positions.swap_remove(index);
            self.z_rotations.swap_remove(index);
            self.velocities.swap_remove(index);
            self.team.swap_remove(index);
            self.mesh_index.swap_remove(index);
        };


        self.state_change.remove(id);
        self.move_targets.remove(id);
        self.damage.remove(id);
        self.id_to_index.remove(id);
    }

    pub fn add_skeleton(&mut self, id: EntityId, skeleton: skeleton::Skeleton) {
        self.skeletons.insert(id, skeleton);
    }

    pub fn add_bones(&mut self, id: EntityId, bones: skeleton::Bones) {
        self.bones.insert(id, bones);
    }

    pub fn add_animation_map(&mut self, id: EntityId, map_id: AnimationMapId) {
        self.animation_map.insert(id, map_id);
    }

    pub fn set_active_animation(&mut self, id: EntityId, animation_id: animations::AnimationId) {
        self.current_animation.insert(id, ActiveAnimation { animation_id, current_time: 0.0});
    }

    pub fn move_to(&mut self, id: EntityId, pos: V3) {
        self.move_targets.insert(id, pos);
        self.animation_state_change(id, StateChangeTo::Move);
    }


    fn animation_state_change(&mut self, id: EntityId, new_state: StateChangeTo) {
        self.state_change.insert(id, new_state);
    }

    pub fn stop_move(&mut self, id: EntityId) {
        if let Some(__) = self.move_targets.remove(&id) {
            // transition to idle animation
            self.animation_state_change(id, StateChangeTo::Idle);
        };
    }

    pub fn move_target(&self, id: &EntityId) -> Option<&V3> {
        self.move_targets.get(id)
    }

    pub fn attack(&mut self, id: EntityId) {
        self.animation_state_change(id, StateChangeTo::Attack);
    }

}
