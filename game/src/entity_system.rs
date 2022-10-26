use nalgebra as na;
use nohash_hasher::IntMap;
use crate::math::*;


pub type EntityId = usize;
pub type EntityIndex = usize;

pub type MoveTargets = IntMap<EntityId, V3>;

pub type DamageMap = IntMap<EntityId, EntityDamage>;

#[derive(Debug, Default, Clone)]
pub struct Entities {

    next_id: usize,
    pub entities: Vec::<EntityId>,

    pub id_to_index: IntMap<EntityId, EntityIndex>,

    // COMPONENTS
    pub positions: Vec::<V3>,
    pub velocities: Vec::<V3>,
    pub z_rotations: Vec::<Rotation2>,

    pub move_targets: MoveTargets,

    pub damage: DamageMap,
}


#[derive(Debug, Default, Clone)]
pub struct EntityDamage {
    pub health: f32,
}


impl Entities {
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

    pub fn remove(&mut self, id: &EntityId) {

        if let Some(index) = self.id_to_index.remove(id) {
            // swap index so after swap delete everything is still cool
            if let Some(&last_id) = self.entities.last() {
                self.id_to_index.insert(last_id, index);
            }
            self.entities.swap_remove(index);
            self.positions.swap_remove(index);
            self.z_rotations.swap_remove(index);
            self.velocities.swap_remove(index);

        };

        self.move_targets.remove(id);
        self.damage.remove(id);
        self.id_to_index.remove(id);


    }
}
