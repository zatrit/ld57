use alpacker::data::raylib::PackRaylibExt;
use raylib::{
    camera::Camera2D,
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibMode2DExt},
};
use std::{f64::consts::PI, time::Duration};

use crate::{
    Game, Raylib, calc_camera_zoom, player::Player, sprite::simple::SimpleSprite, state::State,
};

const WALLS: [Rectangle; 1] = [Rectangle::new(91., 142., 68., 12.)];

const BACKGROUND: Color = Color::new(65, 32, 81, 255);

pub struct Level1 {
    house: SimpleSprite,
    roof: SimpleSprite,
    things: SimpleSprite,
    outside: SimpleSprite,

    player: Player,

    camera: Camera2D,
    time: f64,
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
            player: Player::new(game, Vector2::new(180., 72.))?,
            time: 0.0,
            camera: Camera2D::default(),
        })
    }

    pub fn update(&mut self, game: &mut Game) -> Option<State> {
        let Game {
            raylib, controls, ..
        } = game;
        let Raylib { rl, thread } = raylib;

        let (width, height) = (rl.get_screen_width(), rl.get_screen_height());
        self.camera.zoom = calc_camera_zoom(width, height);

        let delta = rl.get_frame_time();
        self.time += delta as f64;
        let delta = Duration::from_secs_f32(delta);

        self.player.update(rl, delta, controls, &WALLS);

        let mut d = rl.begin_drawing(thread);

        d.clear_background(BACKGROUND);
        let mut d2 = d.begin_mode2D(&self.camera);

        let offset_y = (self.time * (2.0 * PI / 10.0)).sin() * 4.0;
        d2.draw_texture(&self.things.0, 45, (147. + offset_y) as i32, Color::WHITE);

        d2.draw_texture(&self.house.0, 76, 32, Color::WHITE);
        self.player.draw(&mut d2);

        for wall in &WALLS {
            d2.draw_rectangle_rec(wall, Color::GRAY.alpha(0.5));
        }

        // d2.draw_texture(&self.roof.0, 80, 25, Color::WHITE);
        // d2.draw_texture(&self.outside.0, 20, 127, Color::WHITE);

        None
    }
}
