use std::ops::{Deref, DerefMut};

use raylib::{RaylibHandle, camera::Camera2D};

use super::Player;

#[derive(Default, Debug)]
pub struct PlayerCamera(pub Camera2D);

pub const fn calc_camera_zoom(width: i32, height: i32) -> f32 {
    (width as f32 / 320.0).min(height as f32 / 180.0)
}

impl PlayerCamera {
    pub fn update(&mut self, rl: &mut RaylibHandle, player: &Player) {
        let (width, height) = (rl.get_screen_width(), rl.get_screen_height());

        self.0.zoom = calc_camera_zoom(width, height);
        self.0.target.y = player.pos.y - 90.;
    }
}

impl Deref for PlayerCamera {
    type Target = Camera2D;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PlayerCamera {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
