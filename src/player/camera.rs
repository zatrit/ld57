use std::ops::{Deref, DerefMut};

use raylib::{RaylibHandle, camera::Camera2D, math::Vector2, prelude::Rectangle};

#[derive(Debug)]
pub struct PlayerCamera {
    pub camera: Camera2D,
    pub player_pos_mult: Vector2,
}

pub const fn calc_camera_zoom(width: i32, height: i32) -> f32 {
    (width as f32 / 320.0).min(height as f32 / 180.0)
}

impl PlayerCamera {
    pub fn new(player_pos_mult: Vector2) -> Self {
        Self {
            player_pos_mult,
            camera: Camera2D::default(),
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, player: Vector2) {
        let (width, height) = (rl.get_screen_width(), rl.get_screen_height());

        self.zoom = calc_camera_zoom(width, height);
        self.target = player * self.player_pos_mult;
        self.offset.y = 90. * self.zoom;
    }

    pub fn screen_rect(&self, width: i32, height: i32) -> Rectangle {
        let screen_width = width as f32 / self.zoom;
        let screen_height = height as f32 / self.zoom;

        let screen = self.target - self.offset / self.zoom;
        Rectangle::new(screen.x, screen.y, screen_width, screen_height)
    }
}

impl Deref for PlayerCamera {
    type Target = Camera2D;

    fn deref(&self) -> &Self::Target {
        &self.camera
    }
}

impl DerefMut for PlayerCamera {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.camera
    }
}
