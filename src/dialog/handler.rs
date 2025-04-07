use raylib::text::WeakFont;

use super::{Action, DialogAction, DialogChain, DialogNode};
use crate::controls::Controls;
use raylib::prelude::*;
use std::time::Duration;

const LETTERS_PER_SECOND: f32 = 15.;
const FONT_SIZE: f32 = 8.;
const PADDING: i32 = 4;
const SPACING: i32 = 2;

pub struct DisplayDialog<A: Action> {
    pub node: usize,
    pub chain: DialogChain<A>,
    pub current_letter: usize,
    pub current_selection: usize,
    pub elapsed: Duration,
}

pub struct DialogHandler<A: Action> {
    pub font: WeakFont,
    pub dialog: Option<DisplayDialog<A>>,
}

pub enum DialogUpdate<A: Action> {
    Visible,
    Finished(A),
    Hidden,
}

impl<A: Action> DialogHandler<A> {
    pub fn new(rl: &mut RaylibHandle) -> Self {
        DialogHandler {
            font: rl.get_font_default(),
            dialog: None,
        }
    }

    pub fn start_dialog(&mut self, chain: DialogChain<A>) {
        self.dialog = Some(DisplayDialog {
            node: chain.start,
            chain,
            current_letter: 0,
            current_selection: 0,
            elapsed: Duration::ZERO,
        });
    }

    pub fn update(
        &mut self,
        controls: &Controls,
        rl: &mut RaylibHandle,
        delta: Duration,
    ) -> DialogUpdate<A> {
        let dialog = match self.dialog.as_mut() {
            Some(d) => d,
            None => return DialogUpdate::Hidden,
        };

        let node: &DialogNode<A> = &dialog.chain.nodes[dialog.node];
        let full_text = node.text;
        let text_len = full_text.len();

        dialog.elapsed += delta;

        if dialog.current_letter < text_len {
            if controls.inventory.is_pressed(rl) {
                dialog.current_letter = text_len;
            } else {
                dialog.current_letter =
                    (dialog.elapsed.as_secs_f32() * LETTERS_PER_SECOND) as usize;
            }
            return DialogUpdate::Visible;
        }

        if !node.options.is_empty() {
            if controls.up.is_pressed(rl) {
                if dialog.current_selection > 0 {
                    dialog.current_selection -= 1;
                }
            }

            if controls.down.is_pressed(rl) {
                if dialog.current_selection + 1 < node.options.len() {
                    dialog.current_selection += 1;
                }
            }

            if controls.interact.is_pressed(rl) {
                let option = &node.options[dialog.current_selection];
                match option.action {
                    DialogAction::Next(next_index) => {
                        dialog.node = next_index;
                        dialog.current_letter = 0;
                        dialog.elapsed = Duration::ZERO;
                        dialog.current_selection = 0;
                    }
                    DialogAction::Finish(finish_val) => {
                        self.dialog = None;
                        return DialogUpdate::Finished(finish_val);
                    }
                }
            }
        } else {
            if controls.interact.is_pressed(rl) {
                let ret = match node.options.first() {
                    Some(opt) => match opt.action {
                        DialogAction::Finish(val) => val,
                        _ => panic!("Expected Finish action in terminal node"),
                    },
                    None => {
                        panic!("Terminal node without option not allowed")
                    }
                };
                self.dialog = None;
                return DialogUpdate::Finished(ret);
            }
        }

        DialogUpdate::Visible
    }

    pub fn draw(&self, screen: Rectangle, d: &mut impl RaylibDraw) {
        if self.dialog.is_none() {
            return;
        }

        let margin = 8;
        let win_width = screen.width as i32 - margin * 2;
        let win_height = 50;
        let win_x = screen.x as i32 + margin;
        let win_y = (screen.y + screen.height) as i32 - win_height - margin;

        d.draw_rectangle(
            win_x - 2,
            win_y - 2,
            win_width + 4,
            win_height + 4,
            Color::WHITE,
        );
        d.draw_rectangle(win_x, win_y, win_width, win_height, Color::BLACK);

        let dialog = match &self.dialog {
            Some(d) => d,
            None => return,
        };

        let node = &dialog.chain.nodes[dialog.node];
        let text_to_draw = &node.text[0..dialog.current_letter];
        d.draw_text_ex(
            &self.font,
            text_to_draw,
            raylib::math::Vector2::new((win_x + PADDING) as f32, (win_y + PADDING) as f32),
            FONT_SIZE,
            SPACING as f32,
            Color::WHITE,
        );

        if dialog.current_letter >= node.text.chars().count() && !node.options.is_empty() {
            let mut opt_y = win_y + FONT_SIZE as i32 + SPACING * 2 + PADDING;
            for (i, option) in node.options.iter().enumerate() {
                let prefix = if i == dialog.current_selection {
                    "> "
                } else {
                    "  "
                };
                let label = format!("{}{}", prefix, option.label);
                d.draw_text_ex(
                    &self.font,
                    &label,
                    raylib::math::Vector2::new((win_x + PADDING) as f32, opt_y as f32),
                    FONT_SIZE,
                    SPACING as f32,
                    Color::WHITE,
                );
                opt_y += FONT_SIZE as i32 + SPACING;
            }
        }
    }
}
