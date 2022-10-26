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
}
