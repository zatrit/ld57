use std::{cell::RefCell, io};

use alpacker::{Pack, pack::TarZstPack};
use anyhow::Ok;
use controls::Controls;
use level::level4::Level4;
use raylib::{
    RaylibHandle, RaylibThread,
};
use state::State;

#[cfg(target_arch = "wasm32")]
mod wasm;

mod controls;
mod level;
mod player;
mod sprite;
mod state;
mod dialog;
mod interact;

const CONTENT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/content.tar.zst"));

thread_local! {
    static GAME: RefCell<Option<Game>> = RefCell::new(None);
    static STATE: RefCell<State> = RefCell::new(State::Level4(Level4::new()));
}

pub struct Raylib {
    rl: RaylibHandle,
    thread: RaylibThread,
}

pub struct Game {
    pub raylib: Raylib,
    pub controls: Controls,
    pub content: TarZstPack,
}

impl Game {
    fn update(&mut self) -> bool {
        STATE.with_borrow_mut(|state| state.update(self))
    }
}

fn main() -> anyhow::Result<()> {
    let content = TarZstPack::load(io::Cursor::new(CONTENT))?;

    let (mut raylib, thread) = raylib::init()
        .resizable()
        .size(640, 360)
        .title("Yet another Dream")
        .vsync()
        .build();
    raylib.set_exit_key(None);

    let new_game = Game {
        raylib: Raylib { rl: raylib, thread },
        controls: Controls::DEFAULT,
        content,
    };

    GAME.with_borrow_mut(|game| game.replace(new_game));

    #[cfg(not(target_arch = "wasm32"))]
    while !update() {}

    #[cfg(target_arch = "wasm32")]
    unsafe {
        wasm::emscripten_set_main_loop(wasm::_update_wasm, 60, 1)
    };

    Ok(())
}

pub fn update() -> bool {
    GAME.with_borrow_mut(|game| match game {
        Some(game) => game.update(),
        None => false,
    })
}
