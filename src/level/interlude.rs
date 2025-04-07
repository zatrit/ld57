use crate::{
    Game, Raylib,
    dialog::{
        chains_interlude::INTERLUDE_CHAIN,
        handler::{DREAM_PALLETE, DialogHandler, DialogUpdate},
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
    None,
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
    target_dialog_shown: bool,
}

impl Interlude {
    pub fn new(game: &mut Game, plot: Plot) -> anyhow::Result<Self> {
        let dest_color = match &plot {
            Plot::GoTo(plot) => plot.color(),
            _ => Color::BLACK,
        };
        let timer = match &plot {
            Plot::GoTo(plot) => plot.timer(),
            _ => 10.,
        };

        Ok(Self {
            sleeping: game.content.get(&mut game.raylib, "sleeping.png")?,
            dialog: DialogHandler::new(&mut game.raylib.rl, DREAM_PALLETE),
            timer,
            plot,
            camera: PlayerCamera::new(Vector2::new(0., 1.)),
            dest_color,
            target_dialog_shown: false,
        })
    }

    pub fn update(&mut self, game: &mut Game) -> Option<State> {
        let Game {
            raylib, controls, ..
        } = game;
        let Raylib { rl, thread } = raylib;

        let delta = rl.get_frame_time();

        self.camera.update(rl, Vector2::new(0., 90.));

        let delta = Duration::from_secs_f32(delta);
        let dialog_update = self.dialog.update(controls, rl, delta);

        match &self.plot {
            Plot::Choice { .. } => match dialog_update {
                DialogUpdate::Finished(action) => {
                    self.timer = 2.;

                    let plot = replace(&mut self.plot, Plot::None);
                    let Plot::Choice { deeper, awake } = plot else {
                        unreachable!()
                    };

                    let choice = match action {
                        InterludeAction::Deeper => deeper,
                        InterludeAction::Awake => awake,
                        _ => unreachable!(),
                    };

                    self.dest_color = choice.color();
                    self.timer = choice.timer();

                    self.plot = Plot::GoTo(choice);
                }
                DialogUpdate::Hidden => self.dialog.start_dialog(INTERLUDE_CHAIN),
                _ => {}
            },
            Plot::GoTo(state) if !self.target_dialog_shown => match dialog_update {
                DialogUpdate::Finished(_) => self.target_dialog_shown = true,
                DialogUpdate::Hidden => self.dialog.start_dialog(state.interlude_dialog()),
                _ => {}
            },
            _ => {
                self.timer -= delta.as_secs_f32();
            }
        }

        let mut d = rl.begin_drawing(&thread);
        let mut d2 = d.begin_mode2D(*self.camera);
        self.draw(&mut d2);

        if (cfg!(feature = "extra-debug") || self.timer <= 0.) && !self.plot.is_choice() {
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
        d.clear_background(BACKGROUND.lerp(self.dest_color, 1. - self.timer));

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
