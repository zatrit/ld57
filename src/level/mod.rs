use raylib::{color::Color, prelude::{RaylibDraw, RaylibDrawHandle}};

pub mod interlude;
pub mod rules;

pub mod level1;
pub mod level21;
pub mod level22;
pub mod level31;
pub mod level32;
pub mod level33;

pub const FONT_SIZE: i32 = 16;

pub fn draw_line(d: &mut RaylibDrawHandle, y: i32, line: &str, color: Color) {
    let width = d.measure_text(line, FONT_SIZE);
    let x = (d.get_screen_width() - width) / 2;

    d.draw_text(line, x, y, FONT_SIZE, color);
}
