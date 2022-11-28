use std::collections::HashMap;
use gl_lib::animations::*;
use crate::spells::SpellId;
use nohash_hasher::IntMap;


pub type AnimationMapId = usize;

#[derive(Default, Debug, Clone)]
pub struct AnimationMaps {
    mesh_to_map : HashMap::<String, AnimationMapId>,
    animation_maps: Vec::<AnimationMap>
}

impl AnimationMaps {
    pub fn add_map(&mut self, mesh_name: &str, map: AnimationMap) {
        self.animation_maps.push(map);
        self.mesh_to_map.insert(mesh_name.to_string(), self.animation_maps.len() - 1);
    }

    pub fn get_map_id(&self, mesh_name: &str) -> Option<&AnimationMapId> {
        self.mesh_to_map.get(mesh_name)
    }

    pub fn get(&self, id: AnimationMapId) -> &AnimationMap {
        &self.animation_maps[id]
    }

}

#[derive(Default, Debug,  Clone)]
pub struct AnimationMap {
    pub idle: AnimationId,
    pub walk: AnimationId,
    pub attack: AnimationId,
    pub spells: IntMap::<SpellId, AnimationId>,
}
