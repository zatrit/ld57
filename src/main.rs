#![windows_subsystem = "windows"]

mod controls;
mod update;

use std::{env::current_exe, f32::consts::PI};

use alpacker::{Assets, Pack, pack::TarZstPack};
use anyhow::Result;
use controls::Controls;
use raylib::{color::Color, math::Vector2, prelude::RaylibDraw};
use update::UpdateData;

fn load_content() -> Result<TarZstPack> {
    let mut binary_path = current_exe()?;
    binary_path.pop();

    let assets = Assets::load_from_dir(binary_path)?;
    assets
        .load_pack::<TarZstPack>("content")
        .map_err(anyhow::Error::from)
}

fn rand_color() -> Color {
    Color::new(rand::random(), rand::random(), rand::random(), 255)
}

fn text_size(size: Vector2, rotation: f32) -> Vector2 {
    let radians = rotation / 180. * PI;
    let (sin, cos) = (radians.sin().abs(), radians.cos().abs());

    Vector2::new(size.x * cos + size.y * sin, size.y * cos + size.x * sin)
}

fn main() -> Result<()> {
    println!("Loading assets...");
    let mut content = load_content()?;
    let text = content.get::<String>("hello_world.txt")?.trim().to_owned();

    let (mut rl, thread) = raylib::init()
        .resizable()
        .size(640, 480)
        .title("Hello, World")
        .build();

    let font_size = 40.;
    let font = rl.get_font_default();

    let size = Vector2::new(
        rl.measure_text(&text, font_size as i32) as f32 - font_size,
        font_size,
    );
    let origin = size / 2.;

    let mut position = Vector2::new(200., 40.);
    let mut rotation = 0.;

    let mut velocity = Vector2::new(0., 0.);
    let mut angular_velocity = 0.;

    let controls = Controls::default();

    let (mut fg, mut bg) = (Color::BLACK, Color::WHITE);

    while !rl.window_should_close() {
        let mut update_data = UpdateData::new(&mut rl);
        controls.update(&mut update_data, &mut velocity, &mut angular_velocity);

        position += velocity;
        rotation += angular_velocity;

        let (screen_width, screen_height) =
            (rl.get_screen_width() as f32, rl.get_screen_height() as f32);
        let text_size = text_size(size, rotation);
        let top_left = position - text_size / 2.;
        let down_right = position + text_size / 2.;

        let angular_bounce = angular_velocity.abs() * 0.01;
        if top_left.x < 0. || down_right.x > screen_width {
            velocity.x =
                (velocity.x.abs() + angular_bounce) * (screen_width / 2. - position.x).signum();
        }

        if top_left.y < 0. || down_right.y > screen_height {
            velocity.y =
                (velocity.y.abs() + angular_bounce) * (screen_height / 2. - position.y).signum();
        }

        let limit = 0.1;
        velocity.x = velocity.x.clamp(-limit, limit);
        velocity.y = velocity.y.clamp(-limit, limit);

        velocity *= 0.99995;
        angular_velocity *= 0.99995;

        if rl.is_key_pressed(controls.recolor) {
            fg = rand_color();
            bg = rand_color();
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(bg);
        d.draw_text_pro(&font, &text, &position, origin, rotation, font_size, 1., fg);
    }

    Ok(())
}
