use std::{cell::RefCell, io};

use alpacker::{Pack, pack::TarZstPack};
use anyhow::Ok;
use raylib::{RaylibHandle, RaylibThread};
use state::GameState;

mod state;
#[cfg(target_arch = "wasm32")]
mod wasm;

const CONTENT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/content.tar.zst"));

thread_local! {
    static GAME: RefCell<Option<Game>> = RefCell::new(None);
    static STATE: RefCell<GameState> = RefCell::new(GameState::Loading);
}

pub struct Game {
    pub raylib: RaylibHandle,
    pub thread: RaylibThread,
    pub content: TarZstPack,
}

impl Game {
    fn update(&mut self) -> bool {
        STATE.with_borrow_mut(|state| state.update(self))
    }
}

fn main() -> anyhow::Result<()> {
    let content = TarZstPack::load(io::Cursor::new(CONTENT))?;

    let (raylib, thread) = raylib::init()
        .resizable()
        .size(640, 480)
        .title("Hello, World")
        .vsync()
        .build();

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
