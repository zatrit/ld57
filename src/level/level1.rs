use alpacker::data::raylib::PackRaylibExt;
use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibMode2DExt},
};
use std::{f64::consts::PI, time::Duration};

use crate::{
    Game, Raylib,
    dialog::{
        DialogChain,
        chains_level1::{
            BLUE_BED_CHAIN, CARPET_CHAIN, FLOWERS_CHAIN, FRIDGE_CHAIN, GRASS_CHAIN, RED_BED_CHAIN,
        },
        handler::{DREAM_PALLETE, DialogHandler, DialogUpdate},
    },
    interact::Interact,
    player::{Player, camera::PlayerCamera},
    sprite::simple::SimpleSprite,
    state::{State, level2},
};

use super::{
    interlude::{Interlude, Plot},
    level21::Level21,
};

const WALLS: &[Rectangle] = &[
    // Front side
    Rectangle::new(78., 142., 81., 12.),
    Rectangle::new(176., 142., 65., 12.),
    // Kitchen
    Rectangle::new(94., 80., 65., 1.),
    Rectangle::new(95., 81., 16., 7.), // Fridge
    Rectangle::new(94., 81., 1., 61.),
    // Bedroom
    Rectangle::new(158., 33., 1., 47.),
    Rectangle::new(158., 32., 69., 1.),
    Rectangle::new(176., 33., 14., 6.),
    Rectangle::new(224., 33., 1., 109.),
    Rectangle::new(159., 33., 15., 28.), // Bed
    // Outside
    Rectangle::new(78., 154., 1., 83.),
    Rectangle::new(240., 154., 1., 83.),
    Rectangle::new(78., 237., 163., 1.),
];

// Carpet
const CARPET: Rectangle = Rectangle::new(161., 96., 62., 47.);
// Flowers
const FLOWER_BED_1: Rectangle = Rectangle::new(93., 158., 66., 16.);
const FLOWER_BED_2: Rectangle = Rectangle::new(177., 158., 51., 16.);
// Grass
const GRASS_PATCH_1: Rectangle = Rectangle::new(80., 193., 80., 47.);
const GRASS_PATCH_2: Rectangle = Rectangle::new(176., 193., 64., 47.);

pub const INTERACTS: [(Interact, DialogChain<InteractAction>); 6] = [
    // Fridge
    (
        Interact::new(
            Rectangle::new(96., 83., 16., 27.),
            Rectangle::new(96., 96., 16., 16.),
        ),
        FRIDGE_CHAIN,
    ),
    // Capret
    (Interact::new(CARPET, CARPET), CARPET_CHAIN),
    // Flowers)
    (
        Interact::new(FLOWER_BED_1, Rectangle::new(93., 158., 66., 8.)),
        FLOWERS_CHAIN,
    ),
    (
        Interact::new(FLOWER_BED_2, Rectangle::new(177., 158., 51., 8.)),
        FLOWERS_CHAIN,
    ),
    // Grass
    (Interact::new(GRASS_PATCH_1, GRASS_PATCH_1), GRASS_CHAIN),
    (Interact::new(GRASS_PATCH_2, GRASS_PATCH_2), GRASS_CHAIN),
];

const BED: Rectangle = Rectangle::new(161., 51., 14., 33.);
pub const BED_INTERACT: Interact = Interact::new(BED, Rectangle::new(160., 50., 16., 24.));

pub const BACKGROUND: Color = Color::new(65, 32, 81, 255);

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy)]
pub enum InteractAction {
    Touch,
    Sleep,
    #[default]
    None,
}

pub struct Level1 {
    house: SimpleSprite,
    roof: SimpleSprite,
    things: SimpleSprite,
    outside: SimpleSprite,
    blue_bed: SimpleSprite,

    player: Player,
    camera: PlayerCamera,

    dialog: DialogHandler<InteractAction>,
    interacts: [(Interact, DialogChain<InteractAction>); 6],
    bed_interact: Interact,

    touched_grass: bool,
    time: f64,
}

impl Level1 {
    pub fn new(game: &mut Game) -> anyhow::Result<Self> {
        let Game {
            raylib, content, ..
        } = game;

        Ok(Self {
            house: content.get::<SimpleSprite>(raylib, "house.png")?,
            roof: content.get::<SimpleSprite>(raylib, "roof.png")?,
            things: content.get::<SimpleSprite>(raylib, "things.png")?,
            outside: content.get::<SimpleSprite>(raylib, "outside.png")?,
            blue_bed: content.get::<SimpleSprite>(raylib, "blue_bed.png")?,

            interacts: INTERACTS,
            bed_interact: BED_INTERACT,

            dialog: DialogHandler::new(&mut raylib.rl, DREAM_PALLETE),
            player: Player::new(game, Vector2::new(176., 56.))?,
            camera: PlayerCamera::new(Vector2::new(0.75, 0.75)),
            touched_grass: false,
            time: 0.0,
        })
    }

    pub fn update(&mut self, game: &mut Game) -> Option<State> {
        let Game {
            raylib, controls, ..
        } = game;
        let Raylib { rl, thread } = raylib;

        let delta = rl.get_frame_time();
        self.time += delta as f64;
        let delta = Duration::from_secs_f32(delta);

        match self.dialog.update(controls, rl, delta) {
            DialogUpdate::Visible => {}
            DialogUpdate::Hidden => {
                self.player.update(rl, delta, controls, &WALLS);

                for (interact, dialog) in &mut self.interacts {
                    if interact.update(&self.player.rect(), controls, rl) {
                        self.dialog.start_dialog(*dialog);
                    }
                }

                if self.bed_interact.update(&self.player.rect(), controls, rl) {
                    self.dialog.start_dialog(if self.touched_grass {
                        RED_BED_CHAIN
                    } else {
                        BLUE_BED_CHAIN
                    });
                }
            }
            DialogUpdate::Finished(action) => match action {
                InteractAction::Touch => self.touched_grass = true,
                InteractAction::Sleep => {
                    let plot = level2(game, self.touched_grass);
                    return Some(State::Interlude(Interlude::new(game, plot).unwrap()));
                }
                InteractAction::None => {}
            },
        };

        self.camera.update(rl, self.player.pos);

        let mut d = rl.begin_drawing(thread);

        d.clear_background(BACKGROUND);
        let mut d2 = d.begin_mode2D(*self.camera);

        let offset_y = (self.time * (2.0 * PI / 10.0)).sin() * 4.0;
        d2.draw_texture(&self.things.0, 45, (147. + offset_y) as i32, Color::WHITE);

        d2.draw_texture(&self.house.0, 76, 32, Color::WHITE);

        if !self.touched_grass {
            d2.draw_texture(&self.blue_bed.0, 161, 58, Color::WHITE);
        }

        for (interact, _) in &mut self.interacts {
            interact.draw(&mut d2);
        }
        self.bed_interact.draw(&mut d2);

        self.player.draw(&mut d2);

        let alpha = ((self.player.pos.y - 146.) / 8.).clamp(0., 1.);
        d2.draw_texture(&self.roof.0, 80, 25, Color::WHITE.alpha(alpha));
        d2.draw_texture(&self.outside.0, 20, 127, Color::WHITE.alpha(1. - alpha));

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
