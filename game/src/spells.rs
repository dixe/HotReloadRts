use nohash_hasher::IntMap;
use crate::entity_system::*;
use crate::state;
use crate::math::*;

#[derive(Debug, Default, Clone)]
pub struct Spells {

    pub radius: Vec::<f32>,
    pub pos: Vec::<V3>,
    pub duration_sec: Vec::<f32>,

    /*
tick_interval: Vec::<f32>,
    next_tick: Vec::<f32>,
    caster: Vec::<EntityId>
*/
}

pub struct SpellInfo {
    radius: f32,
    time_remaining: f32,
}

impl Spells {

    pub fn cast_spell(&mut self, cs: CastSpell) {
        self.radius.push(cs.r);
        self.duration_sec.push(cs.dur_sec);
        self.pos.push(cs.pos);

    }
}


pub type SpellFn = fn (SpellInfo, &mut state::State);

pub fn heal_tick(si: SpellInfo, state: &mut state::State, dt: f32) {


}


pub fn cast_heal(pos: V3, spells: &mut Spells) {

    spells.cast_spell(CastSpell {
        r: 3.0,
        pos,
        dur_sec: 5.0
    });

}


pub struct CastSpell {
    r: f32,
    pos: V3,
    dur_sec: f32
}
