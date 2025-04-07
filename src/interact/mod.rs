use raylib::math::Rectangle;

use crate::controls::Controls;
use raylib::prelude::*;

pub const OUTLINE: Color = Color::new(255, 255, 224, 255);

#[derive(Debug, Clone, Copy)]
pub struct Interact {
    pub outline_rect: Rectangle,
    pub interact_rect: Rectangle,
    pub touching: bool,
}

impl Interact {
    pub const fn new(outline_rect: Rectangle, interact_rect: Rectangle) -> Self {
        Self {
            outline_rect,
            interact_rect,
            touching: false,
        }
    }

    pub fn update(
        &mut self,
        player: &Rectangle,
        controls: &Controls,
        rl: &mut RaylibHandle,
    ) -> bool {
        self.touching = self.interact_rect.check_collision_recs(&player);
        self.touching && controls.interact.is_pressed(rl)
    }

    pub fn draw(&self, d: &mut impl RaylibDraw) {
        if self.touching {
            d.draw_rectangle_lines_ex(self.outline_rect, 1.0, OUTLINE);
        }
    }
}
