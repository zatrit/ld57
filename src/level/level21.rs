// Vibe coded a maze. I'll refactor this later

use crate::{
    Game,
    dialog::{
        chains_level1::BLUE_BED_CHAIN,
        chains_level21::{FAVORITE_BED_CHAIN, MIRROR_CHAIN, WHITE_CAT_CHAIN},
        handler::{DREAM_PALLETE, DialogHandler, DialogUpdate},
    },
    interact::Interact,
    player::{Player, camera::PlayerCamera},
    sprite::simple::SimpleSprite,
    state::{State, level3_deep},
};
use alpacker::data::raylib::PackRaylibExt;
use maze_generator::{
    growing_tree::GrowingTreeGenerator, prelude::*, recursive_backtracking::RbGenerator,
};
use raylib::prelude::*;
use std::time::Duration;

use super::{interlude::Interlude, level1::InteractAction};

// Constants for cell dimensions
const CELL_WIDTH: i32 = 32;
const CELL_HEIGHT: i32 = 32;
const CELL_SIZE: Vector2 = Vector2::new(CELL_WIDTH as f32, CELL_HEIGHT as f32);

const WHITE_CAT_RECT: Rectangle = Rectangle::new(174., 80., 8., 14.);

const BED_RECT: Rectangle = Rectangle::new(0., CELL_HEIGHT as f32 - 34., 16., 35.);
const BED_INTERACT_RECT: Rectangle = Rectangle::new(0., CELL_HEIGHT as f32 - 34., 20., 16.);

pub struct Level21 {
    maze: Maze,
    player: Player,
    touched_mirror: bool,
    camera: PlayerCamera,
    walls: Vec<Rectangle>,
    maze_timer: Duration,

    dialog: DialogHandler<InteractAction>,

    white_cat: SimpleSprite,
    mirror: SimpleSprite,
    bed: SimpleSprite,
    blue_bed: SimpleSprite,

    mirror_interact: Interact,
    white_cat_interact: Interact,
    bed_interact: Interact,
}

impl Level21 {
    pub fn new(game: &mut Game) -> anyhow::Result<Self> {
        let mut generator = RbGenerator::new(Some([69; 32]));
        let maze = generator.generate(16, 16).unwrap();

        let start_x = maze.start.x * CELL_WIDTH + 17;
        let start_y = maze.start.y * CELL_HEIGHT + 1;
        let player = Player::new(game, Vector2::new(start_x as f32, start_y as f32))?;

        let Game {
            raylib, content, ..
        } = game;

        let goal_x = maze.goal.x * CELL_WIDTH;
        let goal_y = maze.goal.y * CELL_HEIGHT;
        let mirror_rect = Rectangle::new(goal_x as f32 + 8., goal_y as f32 + 2., 14., 29.);
        let camera = PlayerCamera::new(Vector2::new(1., 1.));
        let walls = Self::get_wall_rectangles(&maze);

        Ok(Self {
            maze,
            player,
            touched_mirror: false,
            camera,
            walls,
            maze_timer: Duration::ZERO,

            dialog: DialogHandler::new(&mut raylib.rl, DREAM_PALLETE),

            white_cat: content.get::<SimpleSprite>(raylib, "white_cat.png")?,
            mirror: content.get::<SimpleSprite>(raylib, "mirror.png")?,
            bed: content.get::<SimpleSprite>(raylib, "bed.png")?,
            blue_bed: content.get::<SimpleSprite>(raylib, "blue_bed.png")?,

            white_cat_interact: Interact::new(WHITE_CAT_RECT, WHITE_CAT_RECT),
            mirror_interact: Interact::new(mirror_rect, mirror_rect),
            bed_interact: Interact::new(BED_RECT, BED_INTERACT_RECT),
        })
    }

    // Generates wall rectangles for a given maze.
    fn get_wall_rectangles(maze: &Maze) -> Vec<Rectangle> {
        let mut walls = vec![Rectangle::new(0., CELL_HEIGHT as f32 - 34., 16., 12.)];
        let thickness = 1.0;
        for y in 0..maze.size.1 {
            for x in 0..maze.size.0 {
                if let Some(field) = maze.get_field(&Coordinates { x, y }) {
                    let cell_x = x as f32 * CELL_SIZE.x;
                    let cell_y = y as f32 * CELL_SIZE.y;
                    let size_x = CELL_SIZE.x;
                    let size_y = CELL_SIZE.y;

                    if !field.has_passage(&Direction::North) {
                        walls.push(Rectangle::new(
                            cell_x,
                            cell_y - thickness / 2.0,
                            size_x,
                            thickness,
                        ));
                    }
                    if !field.has_passage(&Direction::West) {
                        walls.push(Rectangle::new(
                            cell_x - thickness / 2.0,
                            cell_y,
                            thickness,
                            size_y,
                        ));
                    }
                    if !field.has_passage(&Direction::South) {
                        walls.push(Rectangle::new(
                            cell_x,
                            cell_y + size_y - thickness / 2.0,
                            size_x,
                            thickness,
                        ));
                    }
                    if !field.has_passage(&Direction::East) {
                        walls.push(Rectangle::new(
                            cell_x + size_x - thickness / 2.0,
                            cell_y,
                            thickness,
                            size_y,
                        ));
                    }
                }
            }
        }
        walls
    }

    // Returns a new random seed as a [u8; 32]. Here we use current time in nanos for simplicity.
    fn generate_seed() -> [u8; 32] {
        use std::time::SystemTime;
        let nanos = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        [(nanos % 256) as u8; 32]
    }

    // Regenerates the maze if conditions are met.
    fn regenerate_maze(&mut self) {
        let max_attempts = 10;
        for _ in 0..max_attempts {
            let seed = Self::generate_seed();
            let mut generator = GrowingTreeGenerator::new(Some(seed));
            if let Ok(new_maze) = generator.generate(16, 16) {
                // Check that the distance between start and goal is large enough (threshold = 6 cells)
                let dx = (new_maze.goal.x - new_maze.start.x).abs();
                let dy = (new_maze.goal.y - new_maze.start.y).abs();
                if dx + dy < 6 {
                    continue;
                }
                let new_walls = Self::get_wall_rectangles(&new_maze);
                let player_rect = self.player.rect();
                // Check that player is not colliding with any wall in the new maze
                if new_walls
                    .iter()
                    .any(|w| player_rect.check_collision_recs(w))
                {
                    // Reposition player to new maze start if needed.
                    let new_start_x = new_maze.start.x * CELL_WIDTH + 17;
                    let new_start_y = new_maze.start.y * CELL_HEIGHT + 1;
                    self.player.pos = Vector2::new(new_start_x as f32, new_start_y as f32);
                }
                // Update maze, walls, and target.
                self.maze = new_maze;
                self.walls = new_walls;
                let goal_x = self.maze.goal.x * CELL_WIDTH;
                let goal_y = self.maze.goal.y * CELL_HEIGHT;
                let mirror_rect = Rectangle::new(goal_x as f32 + 8., goal_y as f32 + 2., 14., 29.);
                self.mirror_interact = Interact::new(mirror_rect, mirror_rect);
                break;
            }
        }
    }

    pub fn update(&mut self, game: &mut Game) -> Option<State> {
        let Game {
            raylib, controls, ..
        } = game;
        let delta_sec = raylib.rl.get_frame_time();
        let delta = Duration::from_secs_f32(delta_sec);

        // Increment maze timer and regenerate maze every 10 seconds.
        if self.maze_timer >= Duration::from_secs(20) {
            self.regenerate_maze();
            self.maze_timer = Duration::ZERO;
        }

        match self.dialog.update(controls, &mut raylib.rl, delta) {
            DialogUpdate::Visible => {}
            DialogUpdate::Finished(action) => match action {
                InteractAction::Touch => {
                    self.touched_mirror = true;
                    self.mirror_interact.touching = false;
                }
                InteractAction::Sleep => {
                    let plot = level3_deep(game, self.touched_mirror);
                    return Some(State::Interlude(Interlude::new(game, plot).unwrap()));
                }
                _ => {}
            },
            DialogUpdate::Hidden => {
                // Update player with collision walls.
                self.player
                    .update(&mut raylib.rl, delta, controls, &self.walls);
                self.maze_timer += delta;

                if self
                    .white_cat_interact
                    .update(&self.player.rect(), controls, &mut raylib.rl)
                {
                    self.dialog.start_dialog(WHITE_CAT_CHAIN);
                }

                if self
                    .bed_interact
                    .update(&self.player.rect(), controls, &mut raylib.rl)
                {
                    self.dialog.start_dialog(if self.touched_mirror {
                        FAVORITE_BED_CHAIN
                    } else {
                        BLUE_BED_CHAIN
                    });
                }

                if !self.touched_mirror
                    && self
                        .mirror_interact
                        .update(&self.player.rect(), controls, &mut raylib.rl)
                {
                    self.dialog.start_dialog(MIRROR_CHAIN);
                }
            }
        }

        self.camera.update(&mut raylib.rl, self.player.pos);

        let mut d = raylib.rl.begin_drawing(&raylib.thread);
        let mut d2 = d.begin_mode2D(*self.camera);
        self.draw(&mut d2);

        None
    }

    pub fn draw(&self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        d.clear_background(Color::BLACK);
        self.draw_maze(d);

        let goal_x = self.maze.goal.x * CELL_WIDTH + CELL_WIDTH / 2 - 8;
        let goal_y = self.maze.goal.y * CELL_HEIGHT;
        // d.draw_rectangle(goal_x, goal_y + 2, CELL_WIDTH, CELL_HEIGHT, Color::GREEN);
        self.mirror
            .draw(d, Vector2::new(goal_x as f32, goal_y as f32 + 2.));

        d.draw_rectangle(
            0,
            0,
            self.maze.size.0 * CELL_WIDTH,
            self.maze.size.0 * CELL_WIDTH,
            Color::WHITE.alpha((self.maze_timer.as_secs_f32() - 15.) / 5.),
        );

        d.draw_texture(&self.bed.0, 1, CELL_HEIGHT - 33, Color::WHITE);
        if !self.touched_mirror {
            d.draw_texture(&self.blue_bed.0, 1, CELL_HEIGHT - 33 + 5, Color::WHITE);
        }

        d.draw_texture(
            &self.white_cat.0,
            WHITE_CAT_RECT.x as i32 + 1,
            WHITE_CAT_RECT.y as i32 + 1,
            Color::WHITE,
        );

        self.white_cat_interact.draw(d);
        self.mirror_interact.draw(d);
        self.bed_interact.draw(d);

        self.player.draw(d);
        let screen = self
            .camera
            .screen_rect(d.get_screen_width(), d.get_screen_height());
        self.dialog.draw(screen, d);
    }

    fn draw_maze(&self, d: &mut impl RaylibDraw) {
        for y in 0..self.maze.size.1 {
            for x in 0..self.maze.size.0 {
                if let Some(field) = self.maze.get_field(&Coordinates { x, y }) {
                    let cell_x = x * CELL_WIDTH;
                    // Slight adjustment on Y for visual alignment
                    let cell_y = y * CELL_HEIGHT + 2;
                    let wall_color =
                        Color::WHITE.lerp(Color::GREEN, (self.maze_timer.as_secs_f32() - 10.) / 5.);

                    let alert = wall_color == Color::GREEN;

                    if alert || !field.has_passage(&Direction::North) {
                        d.draw_line(cell_x, cell_y, cell_x + CELL_WIDTH, cell_y, wall_color);
                    }
                    if alert || !field.has_passage(&Direction::West) {
                        d.draw_line(cell_x, cell_y, cell_x, cell_y + CELL_HEIGHT, wall_color);
                    }

                    if alert || !field.has_passage(&Direction::South) {
                        d.draw_line(
                            cell_x,
                            cell_y + CELL_HEIGHT,
                            cell_x + CELL_WIDTH,
                            cell_y + CELL_HEIGHT,
                            wall_color,
                        );
                    }

                    if alert || !field.has_passage(&Direction::East) {
                        d.draw_line(
                            cell_x + CELL_WIDTH,
                            cell_y,
                            cell_x + CELL_WIDTH,
                            cell_y + CELL_HEIGHT,
                            wall_color,
                        );
                    }
                }
            }
        }
    }
}
