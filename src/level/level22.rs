use std::time::Duration;

use alpacker::data::raylib::PackRaylibExt;
use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibMode2DExt},
};

use crate::{
    Game, Raylib,
    dialog::{
        DialogChain,
        chains_level22::{
            BLACK_CAT_CHAIN, BLUE_BED1_CHAIN, BLUE_BED2_CHAIN, FAVORITE_BED_CHAIN, FLAG_CHAIN,
            FRIDGE_CHAIN, STAIRS_CHAIN, TENT_CHAIN,
        },
        handler::{DialogHandler, DialogUpdate, REAL_PALLETE},
    },
    interact::Interact,
    player::{Player, camera::PlayerCamera},
    sprite::simple::SimpleSprite,
    state::{State, level3_awake},
};

use super::interlude::Interlude;

pub const BACKGROUND: Color = Color::new(114, 182, 207, 255);

const INTERACTS: [(Interact, DialogChain<InteractAction>); 11] = [
    (
        Interact::new(
            Rectangle::new(144.0, 128.0, 16.0, 11.0),
            Rectangle::new(143.0, 128.0, 15.0, 10.0),
        ),
        STAIRS_CHAIN,
    ),
    (
        Interact::new(
            Rectangle::new(208.0, 176.0, 16.0, 11.0),
            Rectangle::new(209.0, 176.0, 14.0, 7.0),
        ),
        STAIRS_CHAIN,
    ),
    (
        Interact::new(
            Rectangle::new(80.0, 240.0, 16.0, 11.0),
            Rectangle::new(75.0, 241.0, 18.0, 9.0),
        ),
        STAIRS_CHAIN,
    ),
    (
        Interact::new(
            Rectangle::new(112.0, 368.0, 16.0, 11.0),
            Rectangle::new(104.0, 370.0, 22.0, 22.0),
        ),
        STAIRS_CHAIN,
    ),
    (
        Interact::new(
            Rectangle::new(161.0, 76.0, 14.0, 33.0),
            Rectangle::new(160.0, 75.0, 19.0, 22.0),
        ),
        BLUE_BED2_CHAIN,
    ),
    (
        Interact::new(
            Rectangle::new(48.0, 148.0, 16.0, 27.0),
            Rectangle::new(46.0, 140.0, 19.0, 27.0),
        ),
        FRIDGE_CHAIN,
    ),
    (
        Interact::new(
            Rectangle::new(225.0, 192.0, 14.0, 20.0),
            Rectangle::new(223.0, 204.0, 17.0, 11.0),
        ),
        BLUE_BED1_CHAIN,
    ),
    (
        Interact::new(
            Rectangle::new(30.0, 253.0, 10.0, 17.0),
            Rectangle::new(30.0, 251.0, 17.0, 14.0),
        ),
        FLAG_CHAIN,
    ),
    (
        Interact::new(
            Rectangle::new(192.0, 316.0, 49.0, 18.0),
            Rectangle::new(193.0, 312.0, 46.0, 22.0),
        ),
        TENT_CHAIN,
    ),
    (
        Interact::new(
            Rectangle::new(135.0, 327.0, 8.0, 15.0),
            Rectangle::new(129.0, 321.0, 14.0, 16.0),
        ),
        BLACK_CAT_CHAIN,
    ),
    (
        Interact::new(
            Rectangle::new(65.0, 379.0, 14.0, 33.0),
            Rectangle::new(62.0, 375.0, 19.0, 29.0),
        ),
        FAVORITE_BED_CHAIN,
    ),
];

const WALLS: &[Rectangle] = &[
    Rectangle::new(158.0, 60.0, 51.0, 1.0),
    Rectangle::new(158.0, 61.0, 1.0, 63.0),
    Rectangle::new(208.0, 61.0, 1.0, 112.0),
    Rectangle::new(160.0, 75.0, 14.0, 11.0),
    Rectangle::new(164.0, 86.0, 8.0, 1.0),
    Rectangle::new(126.0, 123.0, 32.0, 1.0),
    Rectangle::new(126.0, 124.0, 1.0, 17.0),
    Rectangle::new(176.0, 125.0, 15.0, 1.0),
    Rectangle::new(176.0, 126.0, 1.0, 64.0),
    Rectangle::new(190.0, 126.0, 1.0, 112.0),
    Rectangle::new(46.0, 140.0, 80.0, 1.0),
    Rectangle::new(46.0, 141.0, 17.0, 14.0),
    Rectangle::new(46.0, 155.0, 1.0, 82.0),
    Rectangle::new(209.0, 172.0, 32.0, 1.0),
    Rectangle::new(240.0, 173.0, 1.0, 65.0),
    Rectangle::new(96.0, 175.0, 31.0, 1.0),
    Rectangle::new(96.0, 176.0, 1.0, 64.0),
    Rectangle::new(126.0, 176.0, 1.0, 14.0),
    Rectangle::new(127.0, 189.0, 49.0, 1.0),
    Rectangle::new(228.0, 195.0, 8.0, 4.0),
    Rectangle::new(30.0, 236.0, 16.0, 1.0),
    Rectangle::new(30.0, 237.0, 1.0, 65.0),
    Rectangle::new(191.0, 237.0, 16.0, 1.0),
    Rectangle::new(224.0, 237.0, 16.0, 1.0),
    Rectangle::new(206.0, 238.0, 1.0, 31.0),
    Rectangle::new(224.0, 238.0, 1.0, 31.0),
    Rectangle::new(80.0, 239.0, 16.0, 1.0),
    Rectangle::new(80.0, 240.0, 1.0, 29.0),
    Rectangle::new(81.0, 268.0, 64.0, 1.0),
    Rectangle::new(190.0, 268.0, 16.0, 1.0),
    Rectangle::new(225.0, 268.0, 16.0, 1.0),
    Rectangle::new(144.0, 269.0, 1.0, 16.0),
    Rectangle::new(190.0, 269.0, 1.0, 16.0),
    Rectangle::new(240.0, 269.0, 1.0, 45.0),
    Rectangle::new(145.0, 284.0, 45.0, 1.0),
    Rectangle::new(31.0, 301.0, 64.0, 1.0),
    Rectangle::new(94.0, 302.0, 1.0, 63.0),
    Rectangle::new(208.0, 313.0, 32.0, 1.0),
    Rectangle::new(208.0, 314.0, 1.0, 20.0),
    Rectangle::new(144.0, 319.0, 47.0, 1.0),
    Rectangle::new(144.0, 320.0, 1.0, 14.0),
    Rectangle::new(190.0, 320.0, 1.0, 14.0),
    Rectangle::new(135.0, 327.0, 9.0, 7.0),
    Rectangle::new(166.0, 331.0, 2.0, 2.0),
    Rectangle::new(162.0, 332.0, 1.0, 4.0),
    Rectangle::new(169.0, 332.0, 1.0, 1.0),
    Rectangle::new(112.0, 333.0, 23.0, 1.0),
    Rectangle::new(191.0, 333.0, 17.0, 1.0),
    Rectangle::new(112.0, 334.0, 1.0, 96.0),
    Rectangle::new(163.0, 334.0, 1.0, 2.0),
    Rectangle::new(171.0, 334.0, 2.0, 2.0),
    Rectangle::new(164.0, 335.0, 7.0, 1.0),
    Rectangle::new(62.0, 364.0, 32.0, 1.0),
    Rectangle::new(62.0, 365.0, 16.0, 25.0),
    Rectangle::new(62.0, 390.0, 1.0, 40.0),
    Rectangle::new(63.0, 429.0, 49.0, 1.0),
];

const STAIRCASES: &[(Rectangle, Rectangle)] = &[];

#[derive(Debug, Clone, Copy)]
pub enum InteractAction {
    SleepBlue,
    SleepRed,
    None,
}

pub struct Level22 {
    islands: SimpleSprite,

    player: Player,
    camera: PlayerCamera,

    dialog: DialogHandler<InteractAction>,

    interacts: [(Interact, DialogChain<InteractAction>); 11],

    walls: Vec<Rectangle>,
}

impl Level22 {
    pub fn new(game: &mut Game) -> anyhow::Result<Self> {
        let Game {
            raylib, content, ..
        } = game;

        Ok(Self {
            islands: content.get::<SimpleSprite>(raylib, "islands.png")?,

            dialog: DialogHandler::new(&mut raylib.rl, REAL_PALLETE),

            player: Player::new(game, Vector2::new(182., 68.))?,
            camera: PlayerCamera::new(Vector2::one() * 0.95),

            interacts: INTERACTS,
            walls: WALLS.to_vec(),
        })
    }

    pub fn update(&mut self, game: &mut Game) -> Option<State> {
        let Game {
            raylib, controls, ..
        } = game;
        let Raylib { rl, thread } = raylib;

        let delta = rl.get_frame_time();
        let delta = Duration::from_secs_f32(delta);

        match self.dialog.update(controls, rl, delta) {
            DialogUpdate::Visible => {}
            DialogUpdate::Hidden => {
                self.player.update(rl, delta, controls, &self.walls);

                for (interact, dialog) in &mut self.interacts {
                    if interact.update(&self.player.rect(), controls, rl) {
                        self.dialog.start_dialog(*dialog);
                    }
                }
            }
            DialogUpdate::Finished(action) => {
                let quest = match action {
                    InteractAction::SleepRed => Some(true),
                    InteractAction::SleepBlue => Some(false),
                    InteractAction::None => None,
                };

                if let Some(quest) = quest {
                    let plot = level3_awake(game, quest);
                    return Some(State::Interlude(Interlude::new(game, plot).unwrap()));
                };
            }
        };

        self.camera.update(rl, self.player.pos);

        let mut d = rl.begin_drawing(thread);
        let mut d2 = d.begin_mode2D(*self.camera);

        d2.clear_background(BACKGROUND);

        self.islands.draw(&mut d2, Vector2::new(31., 76.));

        for (interact, _) in &mut self.interacts {
            interact.draw(&mut d2);
        }

        self.player.draw(&mut d2);

        if cfg!(feature = "extra-debug") {
            for wall in WALLS {
                d2.draw_rectangle_rec(wall, Color::GRAY.alpha(0.5));
            }
        }

        self.dialog.draw(
            self.camera
                .screen_rect(d2.get_screen_width(), d2.get_screen_height()),
            &mut d2,
        );

        None
    }
}
