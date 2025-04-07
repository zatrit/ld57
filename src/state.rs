use std::cell::RefCell;

use rand::{Rng, rngs::ThreadRng};
use raylib::color::Color;

use crate::{
    Game,
    dialog::{
        DialogChain,
        chains_interlude::{HOME_CHAIN, MAZE_CHAIN, SKY_ISLES_CHAIN, TOO_DEEP_CHAIN, WHEAT_CHAIN},
    },
    level::{
        interlude::{Interlude, InterludeAction, Plot},
        level1::{self, Level1},
        level21::Level21,
        level22::{self, Level22},
        level31::Level31,
        level32::Level32,
        rules::Rules,
    },
};

thread_local! {
    pub static RNG: RefCell<ThreadRng> = RefCell::new({
        let mut rng =rand::rng();
        rng.reseed().unwrap();
        rng
    });
}

pub enum State {
    Rules(Rules),
    Interlude(Interlude),
    Level1(Level1),
    Level21(Level21),
    Level22(Level22),
    Level31(Level31),
    Level32(Level32),
}

impl State {
    pub fn update(&mut self, game: &mut Game) -> bool {
        use State::*;
        let should_close = game.raylib.rl.window_should_close();

        let new_state = match self {
            Interlude(interlude) => interlude.update(game),
            Rules(rules) => rules.update(game),
            Level1(lvl) => lvl.update(game),
            Level21(lvl) => lvl.update(game),
            Level22(lvl) => lvl.update(game),
            Level31(lvl) => lvl.update(game),
            Level32(lvl) => lvl.update(game),
        };

        if let Some(state) = new_state {
            *self = state;
        }

        cfg!(not(target_arch = "wasm32")) && should_close
    }

    pub const fn color(&self) -> Color {
        use State::*;
        match self {
            Level1(_) => level1::BACKGROUND,
            Level22(_) => level22::BACKGROUND,
            Level31(_) => Color::BLUE,
            Level32(_) => Color::new(242, 165, 97, 255),
            _ => Color::BLACK,
        }
    }

    pub const fn interlude_dialog(&self) -> DialogChain<InterludeAction> {
        use State::*;
        match self {
            Level1(_) => HOME_CHAIN,
            Level22(_) => SKY_ISLES_CHAIN,
            Level31(_) => TOO_DEEP_CHAIN,
            Level32(_) => WHEAT_CHAIN,
            _ => MAZE_CHAIN,
        }
    }

    pub const fn timer(&self) -> f32 {
        use State::*;
        match self {
            Level1(_) => 3.,
            Level21(_) | Level22(_) => 2.,
            Level31(_) | Level32(_) => 1.,
            _ => 0.,
        }
    }
}

pub fn level2(game: &mut Game, quest_done: bool) -> Plot {
    if quest_done {
        Plot::Choice {
            deeper: Box::new(State::Level21(Level21::new(game).unwrap())),
            awake: Box::new(State::Level22(Level22::new(game).unwrap())),
        }
    } else {
        Plot::GoTo(Box::new(if rand::random_bool(0.5) {
            State::Level21(Level21::new(game).unwrap())
        } else {
            State::Level22(Level22::new(game).unwrap())
        }))
    }
}

pub fn level3_deep(game: &mut Game, quest_done: bool) -> Plot {
    if quest_done {
        Plot::Choice {
            deeper: Box::new(State::Level31(Level31::new())),
            awake: Box::new(State::Level32(Level32::new(game).unwrap())),
        }
    } else if RNG.with_borrow_mut(|rng| rng.random_bool(0.5)) {
        Plot::GoTo(Box::new(State::Level31(Level31::new())))
    } else {
        Plot::GoTo(Box::new(State::Level32(Level32::new(game).unwrap())))
    }
}

pub fn level3_awake(game: &mut Game, quest_done: bool) -> Plot {
    if quest_done {
        Plot::Choice {
            deeper: todo!(),
            awake: todo!(),
        }
    } else if RNG.with_borrow_mut(|rng| rng.random_bool(0.5)) {
        todo!()
    } else {
        todo!()
    }
}
