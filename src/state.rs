use raylib::{color::Color, prelude::RaylibDraw};

use crate::{
    level::{level1::Level1, level4::Level4, test::TestLevel}, Game
};

pub enum State {
    Test(TestLevel),
    Level1(Level1),
    Level4(Level4),
}

impl State {
    pub fn update(&mut self, game: &mut Game) -> bool {
        let raylib = &mut game.raylib;
        let mut d = raylib.rl.begin_drawing(&raylib.thread);
        d.clear_background(Color::BLACK);
        drop(d);

        let new_state = match self {
            State::Test(level1) => level1.update(game),
            State::Level1(level4) => level4.update(game),
            State::Level4(level4) => level4.update(game),
        };

        if let Some(state) = new_state {
            *self = state;
        }

        let raylib = &mut game.raylib.rl;
        cfg!(not(target_arch = "wasm32")) && raylib.window_should_close()
    }
}
