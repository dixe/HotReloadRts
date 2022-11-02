use nohash_hasher::IntMap;
use crate::entity_system::*;
use crate::state;
use crate::math::*;


#[derive(Debug, Default, Clone)]
pub struct ActiveSpells {

    pub radius: Vec::<f32>,
    pub pos: Vec::<V3>,
    pub seconds_left: Vec::<f32>,
    pub team: Vec::<Team>,
}

pub struct SpellInfo {
    pub radius: f32,
    pub pos: V3,
    pub team: Team
}

pub struct SpellCast {
    pub caster: EntityId
}


impl ActiveSpells {

    pub fn cast_spell(&mut self, cs: CastSpell) {
        self.radius.push(cs.r);
        self.seconds_left.push(cs.dur_sec);
        self.pos.push(cs.pos);
        self.team.push(cs.team)

    }

    pub fn remove_ended_spells(&mut self) {
        let mut count = self.radius.len();
        let mut i = 0;

        while i < count {
            if self.seconds_left[i] <= 0.0 {
                self.radius.swap_remove(i);
                self.pos.swap_remove(i);
                self.seconds_left.swap_remove(i);
                self.team.swap_remove(i);
                count -= 1;
            }
            else {
                // only increment i when we pass a spell that is still active
                i += 1;
            }
        }
    }
}





//TODO: maybe take only entities from state and dt, also return a result instead of taking a mutable
pub fn heal_tick(si: SpellInfo, state: &mut state::State)  {

    let hps = 1.0;
    // find
    for i in 0..state.entities.positions.len() {
        let dist = (si.pos - state.entities.positions[i]).magnitude();

        if dist < si.radius && state.entities.team[i] == si.team {
            let id =  state.entities.ids[i];
            if let Some(dmg) = state.entities.damage.get_mut(&id) {
                dmg.health = f32::min(1.0, dmg.health + hps * state.dt);
            }
        }
    }
}

pub fn cast_heal(pos: V3, team: Team) -> CastSpell {
    CastSpell {
        r: 1.0,
        pos,
        dur_sec: 2.0,
        team
    }
}


pub struct CastSpell {
    r: f32,
    pos: V3,
    dur_sec: f32,
    team: Team
}
