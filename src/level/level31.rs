use raylib::{color::Color, prelude::RaylibDraw};

use crate::{Game, Raylib, state::State};

use super::rules::Rules;

pub struct Level31 {
    radius: f32,
    center_x: f32,
    center_y: f32,
    speed_x: f32,
    speed_y: f32,
}

const SPEED: f32 = 2.;

impl Level31 {
    pub const fn new() -> Self {
        Self {
            radius: 10.,
            center_x: 100.,
            center_y: 100.,
            speed_x: SPEED,
            speed_y: SPEED,
        }
    }

    pub fn update(&mut self, game: &mut Game) -> Option<State> {
        let Raylib { rl, thread } = &mut game.raylib;

        if game.controls.interact.is_pressed(rl) {
            return Some(State::Rules(Rules));
        }

        let delta = rl.get_frame_time();
        self.radius += delta;
        self.center_x += delta * self.speed_x * self.radius;
        self.center_y += delta * self.speed_y * self.radius;

        let (width, height) = (rl.get_screen_width() as f32, rl.get_screen_height() as f32);

        if self.center_x - self.radius < 0. {
            self.speed_x = SPEED;
        } else if self.center_x + self.radius > width {
            self.speed_x = -SPEED;
        }

        if self.center_y - self.radius < 0. {
            self.speed_y = SPEED;
        } else if self.center_y + self.radius > height {
            self.speed_y = -SPEED;
        }

        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::BLACK);
        d.draw_circle(
            self.center_x as i32,
            self.center_y as i32,
            self.radius,
            Color::BLUE,
        );

        None
    }
}
