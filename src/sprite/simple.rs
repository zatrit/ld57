use std::path::Path;

use alpacker::{
    Pack,
    data::raylib::RaylibAsset,
};
use raylib::{
    color::Color, math::Vector2, prelude::RaylibDraw, texture::{Image, Texture2D}
};

use crate::Raylib;

#[derive(Debug)]
pub struct SimpleSprite(pub Texture2D);

impl<'r> RaylibAsset<'r> for SimpleSprite {
    type System = Raylib;
    type Error = anyhow::Error;

    fn load(
        pack: &mut impl Pack,
        Raylib { rl, thread }: &'r mut Self::System,
        path: impl AsRef<Path>,
    ) -> Result<Self, Self::Error> {
        let image = pack.get::<Image>(path)?;
        let texture = rl.load_texture_from_image(thread, &image)?;
        Ok(Self(texture))
    }
}

impl SimpleSprite {
    #[inline]
    pub fn draw(&self, draw: &mut impl RaylibDraw, position: Vector2) {
        draw.draw_texture(&self.0, position.x as i32, position.y as i32, Color::WHITE);
    }
}
