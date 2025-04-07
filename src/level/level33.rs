use std::time::Duration;

use alpacker::data::raylib::PackRaylibExt;
use raylib::{
    camera::Camera2D,
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibMode2DExt},
};

use crate::{
    Game, Raylib,
    player::camera::calc_camera_zoom,
    sprite::{Sprite, data::SpriteData, simple::SimpleSprite},
    state::State,
};

use super::FONT_SIZE;

pub struct Level33 {
    credits: Vec<String>,
    dev: SimpleSprite,
    vitalic: Sprite,

    timer: f64,
}

impl Level33 {
    pub fn new(game: &mut Game) -> anyhow::Result<Self> {
        let Game {
            raylib, content, ..
        } = game;

        Ok(Self {
            credits: alpacker::Pack::get::<String>(content, "credits.txt")?
                .split("\n")
                .map(str::to_string)
                .collect(),
            dev: content.get::<SimpleSprite>(raylib, "dev.png")?,
            vitalic: Sprite::new(content.get::<SpriteData>(raylib, "vitalic.json")?, None),
            timer: 0.,
        })
    }

    pub fn update(&mut self, game: &mut Game) -> Option<State> {
        let Raylib { rl, thread } = &mut game.raylib;

        let delta = Duration::from_secs_f32(rl.get_frame_time());
        self.timer += delta.as_secs_f64();
        self.vitalic.update(delta);

        let mut d = rl.begin_drawing(thread);

        let (screen_width, screen_height) = (d.get_screen_width(), d.get_screen_height());
        let zoom = calc_camera_zoom(screen_width, screen_height);

        let mut d2 = d.begin_mode2D(Camera2D {
            zoom,
            ..Default::default()
        });

        d2.clear_background(Color::BLACK);
        self.dev.draw(&mut d2, Vector2::zero());
        self.vitalic.draw(&mut d2, Vector2::new(121., 55.));

        for (i, line) in self.credits.iter().enumerate() {
            let y = (screen_height as f32 / zoom) as i32 - (10. * self.timer) as i32
                + i as i32 * (FONT_SIZE + 2);

            let width = d2.measure_text(line, FONT_SIZE);
            let x = ((screen_width as f32 / zoom) as i32 - width) / 2;

            d2.draw_text(line, x, y, FONT_SIZE, Color::WHITE);
        }

        None
    }
}
