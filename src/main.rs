use std::{cell::RefCell, io};

use alpacker::{Pack, pack::TarZstPack};
use anyhow::Ok;
use raylib::{RaylibHandle, RaylibThread, color::Color, prelude::RaylibDraw};

#[cfg(target_arch = "wasm32")]
mod wasm;

const CONTENT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/content.tar.zst"));
const FPS: i32 = 60;

thread_local! {
    static GAME: RefCell<Option<Game>> = RefCell::new(None);
}

pub struct Game {
    pub raylib: RaylibHandle,
    pub thread: RaylibThread,
    pub content: TarZstPack,
}

fn rand_color() -> Color {
    Color::new(rand::random(), rand::random(), rand::random(), 255)
}

impl Game {
    fn update(&mut self) -> bool {
        let Self { raylib, thread, .. } = self;

        let mut d = raylib.begin_drawing(&thread);
        d.clear_background(rand_color());
        drop(d);

        cfg!(not(target_arch = "wasm32")) && raylib.window_should_close()
    }
}

fn main() -> anyhow::Result<()> {
    let content = TarZstPack::load(io::Cursor::new(CONTENT))?;

    let (mut raylib, thread) = raylib::init()
        .resizable()
        .size(640, 480)
        .title("Hello, World")
        .build();

    raylib.set_target_fps(FPS as u32);

    let new_game = Game {
        content,
        raylib,
        thread,
    };

    GAME.with_borrow_mut(|game| game.replace(new_game));

    #[cfg(not(target_arch = "wasm32"))]
    while !update() {}

    #[cfg(target_arch = "wasm32")]
    unsafe {
        wasm::emscripten_set_main_loop(wasm::_update_wasm, 0, 1);
    }

    Ok(())
}

pub fn update() -> bool {
    GAME.with_borrow_mut(|game| match game {
        Some(game) => game.update(),
        None => false,
    })
}
