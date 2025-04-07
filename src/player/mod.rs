use std::time::Duration;

use alpacker::data::raylib::PackRaylibExt;
use raylib::{
    math::{Rectangle, Vector2}, prelude::{RaylibDraw, RaylibDrawHandle, RaylibMode2D, RaylibShaderModeExt}, RaylibHandle
};

use crate::{
    Game,
    controls::Controls,
    sprite::{Sprite, data::SpriteData},
};

const PLAYER_SIZE: Vector2 = Vector2::new(8., 22.);
const PIXELS_PER_SECOND: f32 = 24.;

pub struct Player {
    pub sprite: Sprite,
    pub pos: Vector2,
    pub back: bool,
}

impl Player {
    pub fn new(game: &mut Game, pos: Vector2) -> anyhow::Result<Self> {
        let Game {
            raylib, content, ..
        } = game;

        let sprite = Sprite::new(
            content.get::<SpriteData>(raylib, "player.json")?,
            Some("idle_front"),
        );

        Ok(Self {
            sprite,
            pos,
            back: false,
        })
    }

    pub fn draw(&self, draw: &mut RaylibMode2D<impl RaylibDraw>) {
        self.sprite.draw(draw, self.pos);
    }

    pub fn update(
        &mut self,
        rl: &mut RaylibHandle,
        delta: Duration,
        controls: &Controls,
        walls: &[Rectangle],
    ) {
        self.sprite.update(delta);

        let mut dir = Vector2::zero();

        if controls.down.is_down(rl) {
            dir.y += 1.;
        }
        if controls.up.is_down(rl) {
            dir.y -= 1.;
        }
        if controls.left.is_down(rl) {
            dir.x -= 1.;
        }
        if controls.right.is_down(rl) {
            dir.x += 1.;
        }

        if dir.y != 0. {
            self.back = dir.y < 0.;
        }
        if dir.x != 0. {
            self.sprite.flip_x = dir.x < 0.;
        }

        let moved = calc_move(
            self.pos,
            PIXELS_PER_SECOND * delta.as_secs_f32(),
            dir,
            walls,
        );

        if moved.x != 0. || moved.y != 0. {
            self.sprite.play_tag(
                Some(if self.back { "walk_back" } else { "walk_front" }),
                None,
            );
            self.pos += moved;
        } else {
            self.sprite.play_tag(
                Some(if self.back { "idle_back" } else { "idle_front" }),
                None,
            );
            self.pos = Vector2::new(self.pos.x.round(), self.pos.y.round());
        }
    }
}

fn calc_move(pos: Vector2, speed: f32, dir: Vector2, walls: &[Rectangle]) -> Vector2 {
    let mut x = dir.x * speed;
    let rect = Rectangle::new(pos.x + x, pos.y, PLAYER_SIZE.x, PLAYER_SIZE.y);
    if check_collision(rect, walls) {
        x = 0.;
    }

    let mut y = dir.y * speed;
    let rect = Rectangle::new(pos.x, pos.y + y, PLAYER_SIZE.x, PLAYER_SIZE.y);
    if check_collision(rect, walls) {
        y = 0.;
    }

    Vector2 { x, y }
}

pub fn check_collision(rect: Rectangle, colliders: &[Rectangle]) -> bool {
    colliders.iter().any(|c| c.check_collision_recs(&rect))
}
