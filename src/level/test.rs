use std::time::Duration;

use alpacker::data::raylib::PackRaylibExt;
use raylib::{camera::Camera2D, ffi::CameraMode, math::Vector2, prelude::RaylibMode2DExt};

use crate::{
    Game, Raylib, calc_camera_zoom,
    sprite::{Sprite, data::SpriteData},
    state::State,
};

pub struct TestLevel {
    sprite: Sprite,
    camera: Camera2D,
}

impl TestLevel {
    pub fn new(game: &mut Game) -> anyhow::Result<Self> {
        let Game {
            raylib, content, ..
        } = game;
        let sprite = content.get::<SpriteData>(raylib, "player.json")?;
        eprintln!("{sprite:?}");
        let sprite = Sprite::new(sprite, None);

        Ok(Self {
            sprite,
            camera: Camera2D::default(),
        })
    }

    pub fn update(&mut self, game: &mut Game) -> Option<State> {
        let Raylib { rl, thread } = &mut game.raylib;
        let delta = Duration::from_secs_f32(rl.get_frame_time());

        let (width, height) = (rl.get_screen_width(), rl.get_screen_height());
        self.camera.zoom = calc_camera_zoom(width, height);

        let mut d = rl.begin_drawing(&thread);
        let mut d2 = d.begin_mode2D(&self.camera);

        self.sprite.update(delta);
        self.sprite.draw(&mut d2, Vector2::one());

        None
    }
}
