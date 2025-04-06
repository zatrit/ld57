use alpacker::data::raylib::PackRaylibExt;
use raylib::{
    camera::Camera2D,
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibMode2DExt},
};
use std::time::Duration;

use crate::{
    Game, Raylib, calc_camera_zoom,
    sprite::{Sprite, data::SpriteData, simple::SimpleSprite},
    state::State,
};

pub struct Level1 {
    house: SimpleSprite,
    roof: SimpleSprite,
    things: SimpleSprite,
    outside: SimpleSprite,
    camera: Camera2D,
    time: f32,
}

impl Level1 {
    pub fn new(game: &mut Game) -> anyhow::Result<Self> {
        let Game {
            raylib, content, ..
        } = game;

        Ok(Self {
            house: content.get::<SimpleSprite>(raylib, "house.png")?,
            roof: content.get::<SimpleSprite>(raylib, "roof.png")?,
            things: content.get::<SimpleSprite>(raylib, "things.png")?,
            outside: content.get::<SimpleSprite>(raylib, "outside.png")?,
            time: 0.0,
            camera: Camera2D::default(),
        })
    }

    pub fn update(&mut self, game: &mut Game) -> Option<State> {
        let Raylib { rl, thread } = &mut game.raylib;

        let (width, height) = (rl.get_screen_width(), rl.get_screen_height());
        self.camera.zoom = calc_camera_zoom(width, height);

        let delta = Duration::from_secs_f32(rl.get_frame_time());
        self.time += delta.as_secs_f32();
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color { r: 65, g: 32, b: 81, a: 255 });
        let mut d2 = d.begin_mode2D(&self.camera);

        let offset_y = (self.time * (2.0 * std::f32::consts::PI / 10.0)).sin() * 4.0;
        d2.draw_texture(&self.things.0, 45, (147.0 + offset_y) as i32, Color::WHITE);

        d2.draw_texture(&self.house.0, 76, 32, Color::WHITE);
        // d2.draw_texture(&self.roof.0, 80, 25, Color::WHITE);
        // d2.draw_texture(&self.outside.0, 20, 127, Color::WHITE);

        None
    }
}
