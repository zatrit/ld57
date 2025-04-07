use crate::{
    Game,
    level::{level1::Level1, level4::Level4},
};

pub enum State {
    Level1(Level1),
    Level4(Level4),
}

impl State {
    pub fn update(&mut self, game: &mut Game) -> bool {
        let should_close = game.raylib.rl.window_should_close();

        let new_state = match self {
            State::Level1(lvl) => lvl.update(game),
            State::Level4(lvl) => lvl.update(game),
        };

        if let Some(state) = new_state {
            *self = state;
        }

        cfg!(not(target_arch = "wasm32")) && should_close
    }
}
