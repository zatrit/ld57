use crate::{
    Game, Raylib,
    dialog::{
        chains::INTERLUDE_CHAIN,
        handler::{DialogHandler, DialogUpdate},
    },
    player::camera::PlayerCamera,
    sprite::simple::SimpleSprite,
    state::State,
};
use alpacker::data::raylib::PackRaylibExt;
use raylib::prelude::*;
use std::{mem::replace, time::Duration};

const BACKGROUND: Color = Color::new(61, 0, 61, 255);

#[derive(Debug, Clone, Copy)]
pub enum InterludeAction {
    Deeper,
    Awake,
}

pub enum Plot {
    GoTo(Box<State>),
    Choice {
        deeper: Box<State>,
        awake: Box<State>,
    },
    None,
}

impl Plot {
    fn is_choice(&self) -> bool {
        match self {
            Self::Choice { .. } => true,
            _ => false,
        }
    }
}

pub struct Interlude {
    sleeping: SimpleSprite,
    timer: f32,
    dialog: DialogHandler<InterludeAction>,
    plot: Plot,
    camera: PlayerCamera,
    dest_color: Color,
}

impl Interlude {
    pub fn new(game: &mut Game, plot: Plot, dest_color: Color, timer: f32) -> anyhow::Result<Self> {
        Ok(Self {
            sleeping: game.content.get(&mut game.raylib, "sleeping.png")?,
            dialog: DialogHandler::new(&mut game.raylib.rl),
            timer,
            plot,
            camera: PlayerCamera::new(Vector2::one()),
            dest_color,
        })
    }

    pub fn update(&mut self, game: &mut Game) -> Option<State> {
        let Game {
            raylib, controls, ..
        } = game;
        let Raylib { rl, thread } = raylib;

        let delta = rl.get_frame_time();

        self.camera.update(rl, Vector2::new(0., 90.));

        if let Plot::Choice { .. } = &self.plot {
            let delta = Duration::from_secs_f32(delta);
            match self.dialog.update(controls, rl, delta) {
                DialogUpdate::Finished(action) => {
                    self.timer = 2.;

                    let plot = replace(&mut self.plot, Plot::None);
                    let Plot::Choice { deeper, awake } = plot else {
                        unreachable!()
                    };

                    let choice = match action {
                        InterludeAction::Deeper => deeper,
                        InterludeAction::Awake => awake,
                    };

                    self.plot = Plot::GoTo(choice);
                }
                DialogUpdate::Hidden => self.dialog.start_dialog(INTERLUDE_CHAIN),
                _ => {}
            }
        } else {
            self.timer -= delta;
        }

        let mut d = rl.begin_drawing(&thread);
        let mut d2 = d.begin_mode2D(*self.camera);
        self.draw(&mut d2);

        if self.timer <= 0. && !self.plot.is_choice() {
            let plot = replace(&mut self.plot, Plot::None);

            Some(match plot {
                Plot::GoTo(state) => *state,
                _ => unreachable!(),
            })
        } else {
            None
        }
    }

    pub fn draw(&self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        d.clear_background(BACKGROUND.lerp(self.dest_color, 1. - self.timer / 2.));

        let sprite_w = self.sleeping.0.width;
        let sprite_h = self.sleeping.0.height;

        d.draw_texture(
            &self.sleeping.0,
            160 - sprite_w / 2,
            90 - sprite_h / 2,
            Color::WHITE.alpha(self.timer / 2.),
        );

        self.dialog.draw(
            self.camera
                .screen_rect(d.get_screen_width(), d.get_screen_height()),
            d,
        );
    }
}
