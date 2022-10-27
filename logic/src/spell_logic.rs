use crate::game;
use crate::game::spells;

pub fn update_spells(state: &mut game::State) {

    for i in 0..state.spells.pos.len() {
        let info = spells::SpellInfo {
            radius: state.spells.radius[i],
            pos: state.spells.pos[i],
        };

        // TODO: Get from tick function table for now just use heal

        spells::heal_tick(info, state);
    }

    let mut ended_spells = vec![];
    for i in 0..state.spells.seconds_left.len() {
        state.spells.seconds_left[i] -= state.dt;
        if state.spells.seconds_left[i] <= 0.0 {
            ended_spells.push(i);
        }
    }

    state.spells.remove_ended_spells();


}
