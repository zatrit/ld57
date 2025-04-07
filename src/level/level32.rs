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

use super::{FONT_SIZE, rules::Rules};

#[derive(Debug, Clone, Copy)]
pub enum EndingChoice {
    Restart,
    Finish,
}

pub struct Level32 {
    field: SimpleSprite,
    credits: Vec<String>,

    player: Player,
    camera: PlayerCamera,

    dialog: DialogHandler<EndingChoice>,
    dialog_shown: bool,

    timer: f64,
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
            credits: alpacker::Pack::get::<String>(content, "credits.txt")?
                .split("\n")
                .map(str::to_string)
                .collect(),
            field: content.get::<SimpleSprite>(raylib, "field.png")?,
            camera: PlayerCamera::new(Vector2::one()),
            dialog: DialogHandler::new(&mut raylib.rl, DREAM_PALLETE),
            dialog_shown: false,
            timer: 0.,
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
                } else {
                    self.timer += delta.as_secs_f64();
                }
            }
        };

        let mut d = rl.begin_drawing(thread);
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

        let screen_rect = self
            .camera
            .screen_rect(d2.get_screen_width(), d2.get_screen_height());

        self.dialog.draw(screen_rect, &mut d2);

        for (i, line) in self.credits.iter().enumerate() {
            let y =
                screen_rect.height as i32 - (10. * self.timer) as i32 + i as i32 * (FONT_SIZE + 2);
            let width = d2.measure_text(line, FONT_SIZE);
            let x = (screen_rect.width as i32 - width) / 2;

            d2.draw_text(line, x, y, FONT_SIZE, Color::BLACK);
        }

        None
    }
}
