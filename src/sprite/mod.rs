pub mod data;
pub mod simple;

use std::time::Duration;

use data::{Frame, FrameTag, SpriteData};
use raylib::{
    color::Color,
    math::Rectangle,
    prelude::{RaylibDraw, Vector2},
};

#[derive(Debug)]
pub struct Sprite {
    data: SpriteData,

    current_frame: usize,
    current_loop: FrameTag,

    pub flip_x: bool,
    pub flip_y: bool,

    timer: Duration,
}

impl Sprite {
    pub fn new(data: SpriteData, tag: Option<&str>) -> Self {
        Self {
            current_frame: 0,
            current_loop: match tag {
                Some(tag) => data.tags[tag],
                None => FrameTag::from_start(data.frames.len()),
            },
            data,
            flip_x: false,
            flip_y: false,
            timer: Duration::ZERO,
        }
    }

    fn tag(&self, tag: Option<&str>) -> FrameTag {
        match tag {
            Some(tag) => self.data.tags[tag],
            None => FrameTag::from_start(self.data.frames.len()),
        }
    }

    fn current_loop(&self) -> &[Frame] {
        let FrameTag { from, to } = self.current_loop;
        &self.data.frames[from..to]
    }

    fn current_frame(&self) -> &Frame {
        &self.current_loop()[self.current_frame]
    }

    pub fn draw(&self, draw: &mut impl RaylibDraw, position: Vector2) {
        let Frame { rect, offset, .. } = self.current_loop()[self.current_frame];
        let rect = flip_rect(rect, self.flip_x, self.flip_y);

        draw.draw_texture_rec(&self.data.texture, rect, position + offset, Color::WHITE);
    }

    pub fn update(&mut self, delta: Duration) {
        let duration = self.current_frame().duration;
        self.timer += delta;

        if duration <= self.timer {
            self.timer -= duration;
            self.current_frame = (self.current_frame + 1) % self.current_loop().len()
        }
    }

    pub fn play_tag(&mut self, tag: Option<&str>, frame: usize) {
        let tag = self.tag(tag);
        self.current_loop = tag;
        self.current_frame = frame % self.current_loop().len();
    }
}

const fn flip_rect(mut rect: Rectangle, flip_x: bool, flip_y: bool) -> Rectangle {
    if flip_x {
        rect.x += rect.width;
        rect.width = -rect.width;
    }

    if flip_y {
        rect.y += rect.height;
        rect.height = -rect.height;
    }

    rect
}
