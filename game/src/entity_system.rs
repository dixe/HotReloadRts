use nalgebra as na;
use nohash_hasher::IntMap;
use crate::math::*;


pub type EntityId = usize;
pub type EntityIndex = usize;



#[derive(Debug, Default, Clone)]
pub struct Entitites {

    // COMPONENTS
    pub positions: Vec::<V3>,
    pub velocities: Vec::<V3>,
    pub z_rotations: Vec::<Rotation2>,

    pub move_targets: IntMap<EntityId, V3>,



}


impl Entitites {
    pub fn add_entity(&mut self, pos: V3) -> EntityId {
        self.positions.push(pos);
        self.velocities.push(na::Vector3::new(0.0, 0.0, 0.0));
        self.z_rotations.push(Default::default());
        self.positions.len() - 1
    }
}
