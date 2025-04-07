use raylib::{
    color::Color,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::{Game, Raylib, state::State};

use super::{
    interlude::{Interlude, Plot},
    level1::{self, Level1},
};

pub struct Rules;

const FONT_SIZE: i32 = 24;

const RULES: &[&str] = &[
    "Yet another Dream",
    "A game about sleeping VERY deeply.",
    "",
    "Controls:",
    "WASD / Arrow Keys - Move",
    "E / Z - Interact or advance dialogue",
    "R / C - Inventory or skip dialogue typing",
    "",
    "Press E or Z to continue...",
];


impl Rules {
    pub fn update(&self, game: &mut Game) -> Option<State> {
        let Raylib { rl, thread } = &mut game.raylib;
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        for (i, line) in RULES.iter().enumerate() {
            self.draw_line(&mut d, 100 + i as i32 * (FONT_SIZE + 2), line);
        }

        drop(d);

        if game.controls.interact.is_pressed(rl) {
            let plot = Plot::GoTo(Box::new(State::Level1(Level1::new(game).unwrap())));
            return Some(State::Interlude(
                Interlude::new(game, plot, level1::BACKGROUND, 4.).unwrap(),
            ));
        }

        None
    }

    pub fn draw_line(&self, d: &mut RaylibDrawHandle, y: i32, line: &str) {
        let width = d.measure_text(line, FONT_SIZE);
        let x = (d.get_screen_width() - width) / 2;

        d.draw_text(line, x, y, FONT_SIZE, Color::WHITE);
    }
}
