use nalgebra as na;
use nohash_hasher::IntMap;
use crate::math::*;


pub type EntityId = usize;
pub type EntityIndex = usize;

pub type MoveTargets = IntMap<EntityId, V3>;

#[derive(Debug, Default, Clone)]
pub struct Entitites {

    next_id: usize,
    pub entities: Vec::<EntityId>,

    pub id_to_index: IntMap<EntityId, EntityIndex>,

    // COMPONENTS
    pub positions: Vec::<V3>,
    pub velocities: Vec::<V3>,
    pub z_rotations: Vec::<Rotation2>,

    pub move_targets: MoveTargets,

    pub damage: IntMap<EntityId, EntityDamage>,
}


#[derive(Debug, Default, Clone)]
pub struct EntityDamage {
    pub health: f32,
}


impl Entitites {
    pub fn add_entity(&mut self, pos: V3) -> EntityId {
        let id = self.get_id();

        self.entities.push(id);
        self.positions.push(pos);
        self.velocities.push(na::Vector3::new(0.0, 0.0, 0.0));
        self.z_rotations.push(Default::default());

        self.id_to_index.insert(id, self.positions.len() - 1);

        id
    }

    fn get_id(&mut self) -> EntityId {
         let id = self.next_id;

        self.next_id += 1;

        id
    }
}
