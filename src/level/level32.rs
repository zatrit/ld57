use alpacker::data::raylib::PackRaylibExt;
use raylib::math::Vector2;

use crate::{
    Game,
    dialog::handler::{DREAM_PALLETE, DialogHandler},
    player::{Player, camera::PlayerCamera},
    sprite::simple::SimpleSprite,
};

#[derive(Debug, Clone, Copy)]
pub enum EndingChoice {
    Restart,
    Finish,
}

pub struct Level32 {
    field: SimpleSprite,

    player: Player,
    camera: PlayerCamera,

    dialog: DialogHandler<EndingChoice>,
}

impl Level32 {
    pub fn new(game: &mut Game) -> anyhow::Result<Self> {
        let mut player = Player::new(game, Vector2::new(160., 90.))?;
        player.sprite.strip_down = 12.;

        let Game {
            raylib, content, ..
        } = game;

        Ok(Self {
            player,
            field: content.get::<SimpleSprite>(raylib, "field.png")?,
            camera: PlayerCamera::new(Vector2::zero()),
            dialog: DialogHandler::new(&mut raylib.rl, DREAM_PALLETE),
        })
    }

    pub fn update(&self, game: &mut Game) -> Option<crate::state::State> {
        todo!()
    }
}
