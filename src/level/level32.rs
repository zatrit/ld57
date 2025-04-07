use std::time::Duration;

use alpacker::data::raylib::PackRaylibExt;
use raylib::{
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibMode2DExt},
};

use crate::{
    Game, Raylib,
    dialog::{
        chains_level32::WHEAT_ENDING_CHAIN,
        handler::{DREAM_PALLETE, DialogHandler, DialogUpdate},
    },
    player::{Player, camera::PlayerCamera},
    sprite::simple::SimpleSprite,
    state::State,
};

use super::rules::Rules;

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
    dialog_shown: bool,
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
            camera: PlayerCamera::new(Vector2::one()),
            dialog: DialogHandler::new(&mut raylib.rl, DREAM_PALLETE),
            dialog_shown: false,
        })
    }

    pub fn update(&mut self, game: &mut Game) -> Option<crate::state::State> {
        let Game {
            raylib, controls, ..
        } = game;
        let Raylib { rl, thread } = raylib;

        let delta = Duration::from_secs_f32(rl.get_frame_time());
        self.camera.update(rl, self.player.pos);

        match self.dialog.update(controls, rl, delta) {
            DialogUpdate::Visible => {}
            DialogUpdate::Finished(choice) => match choice {
                EndingChoice::Restart => return Some(State::Rules(Rules)),
                EndingChoice::Finish => {}
            },
            DialogUpdate::Hidden => {
                if !self.dialog_shown {
                    self.dialog.start_dialog(WHEAT_ENDING_CHAIN);
                    self.dialog_shown = true;
                }
            }
        };

        let mut d = rl.begin_drawing(&thread);
        let mut d2 = d.begin_mode2D(*self.camera);

        d2.draw_rectangle(0, 0, 320, 180, Color::new(92, 139, 168, 255));
        d2.draw_rectangle(
            0,
            180,
            320,
            d2.get_screen_height(),
            Color::new(224, 107, 81, 255),
        );
        for i in 0..=3 {
            self.field.draw(&mut d2, Vector2::new(i as f32 * 128., 16.));
        }

        self.player.draw(&mut d2);

        self.dialog.draw(
            self.camera
                .screen_rect(d2.get_screen_width(), d2.get_screen_height()),
            &mut d2,
        );

        None
    }
}
