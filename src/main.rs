#![windows_subsystem = "windows"]

mod controls;
mod update;

use std::{env::current_exe, time::Instant};

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
    let origin = Vector2::new(
        rl.measure_text(&text, font_size as i32) as f32 / 2.,
        font_size / 2.,
    );

    let mut position = Vector2::new(40., 40.);
    let mut rotation = 0.;

    let mut last_update = Instant::now();
    let controls = Controls::default();

    let (mut fg, mut bg) = (Color::BLACK, Color::WHITE);

    while !rl.window_should_close() {
        let mut update_data = UpdateData::new(&mut rl, &mut last_update);
        controls.update(&mut update_data, &mut position, &mut rotation);

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
