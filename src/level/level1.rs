use alpacker::data::raylib::PackRaylibExt;
use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibMode2DExt},
};
use std::{f64::consts::PI, time::Duration};

use crate::{
    Game, Raylib,
    player::{Player, camera::PlayerCamera},
    sprite::simple::SimpleSprite,
    state::State,
};

const WALLS: [Rectangle; 13] = [
    // Front side
    Rectangle::new(78., 142., 81., 12.),
    Rectangle::new(176., 142., 65., 12.),
    // Kitchen
    Rectangle::new(94., 80., 65., 1.),
    Rectangle::new(95., 81., 16., 7.), // Fridge
    Rectangle::new(94., 81., 1., 61.),
    // Bedroom
    Rectangle::new(158., 33., 1., 47.),
    Rectangle::new(158., 32., 69., 1.),
    Rectangle::new(176., 33., 14., 6.),
    Rectangle::new(224., 33., 1., 109.),
    Rectangle::new(159., 33., 15., 28.), // Bed
    // Outside
    Rectangle::new(78., 154., 1., 83.),
    Rectangle::new(240., 154., 1., 83.),
    Rectangle::new(78., 237., 163., 1.),

];

const BACKGROUND: Color = Color::new(65, 32, 81, 255);

pub struct Level1 {
    house: SimpleSprite,
    roof: SimpleSprite,
    things: SimpleSprite,
    outside: SimpleSprite,

    player: Player,
    camera: PlayerCamera,

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
            camera: PlayerCamera::default(),
            time: 0.0,
        })
    }

    pub fn update(&mut self, game: &mut Game) -> Option<State> {
        let Game {
            raylib, controls, ..
        } = game;
        let Raylib { rl, thread } = raylib;

        let delta = rl.get_frame_time();
        self.time += delta as f64;
        let delta = Duration::from_secs_f32(delta);

        self.player.update(rl, delta, controls, &WALLS);
        self.camera.update(rl, &self.player);

        let mut d = rl.begin_drawing(thread);

        d.clear_background(BACKGROUND);
        let mut d2 = d.begin_mode2D(*self.camera);

        let offset_y = (self.time * (2.0 * PI / 10.0)).sin() * 4.0;
        d2.draw_texture(&self.things.0, 45, (147. + offset_y) as i32, Color::WHITE);

        d2.draw_texture(&self.house.0, 76, 32, Color::WHITE);
        self.player.draw(&mut d2);

        let alpha = ((self.player.pos.y - 146.) / 8.).clamp(0., 1.);
        d2.draw_texture(&self.roof.0, 80, 25, Color::WHITE.alpha(alpha));
        d2.draw_texture(&self.outside.0, 20, 127, Color::WHITE.alpha(1. - alpha));

        if cfg!(debug_assertions) {
            for wall in &WALLS {
                d2.draw_rectangle_rec(wall, Color::GRAY.alpha(0.5));
            }
        }

        None
    }
}
