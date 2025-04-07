use crate::{
    Game,
    level::{interlude::Interlude, level1::Level1, level41::Level41, rules::Rules},
};

pub enum State {
    Rules(Rules),
    Interlude(Interlude),
    Level1(Level1),
    Level4(Level41),
}

impl State {
    pub fn update(&mut self, game: &mut Game) -> bool {
        let should_close = game.raylib.rl.window_should_close();

        let new_state = match self {
            State::Interlude(interlude) => interlude.update(game),
            State::Rules(rules) => rules.update(game),
            State::Level1(lvl) => lvl.update(game),
            State::Level4(lvl) => lvl.update(game),
        };

        if let Some(state) = new_state {
            *self = state;
        }

        cfg!(not(target_arch = "wasm32")) && should_close
    }
}
