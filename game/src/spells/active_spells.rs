use crate::entity_system::*;
use crate::state;
use crate::math::*;
use crate::spells::all_spells::{AoeTickFn, SpellId};
use crate::types::*;

#[derive(Default, Clone)]
pub struct ActiveSpell2s {

    pub radius: Vec::<f32>,
    pub tick_fn: Vec::<AoeTickFn>,
    pub pos: Vec::<V3>,
    pub seconds_left: Vec::<f32>,
    pub team: Vec::<Team>,
}
