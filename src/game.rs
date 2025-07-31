#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{collections::VecDeque, time::Instant};

use eframe::egui;
use egui::{Color32, Direction, Key};
use rand::prelude::*;

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),

        ..Default::default()
    };
    eframe::run_native(
        "Rust smake",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyRustSnakeGame>::default()
        }),
    )
}

struct GameState {
    snake_length: u16,
    player_direction: Direction,
    player_draw_direction: Direction,
    player_position: (u16, u16),
    body_parts: VecDeque<(u16, u16)>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            snake_length: 0,
            player_direction: Direction::TopDown,
            player_draw_direction: Direction::TopDown,
            player_position: (5, 5),
            body_parts: VecDeque::new(),
        }
    }
}

struct Timer {
    elapsed_time: f32,
    last_update: Instant,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            elapsed_time: 0.0,
            last_update: Instant::now(),
        }
    }
}

struct MyRustSnakeGame {
    map_size: (u8, u8),
    head_img: egui::ImageSource<'static>,
    head_img_left: egui::ImageSource<'static>,
    head_img_up: egui::ImageSource<'static>,
    head_img_right: egui::ImageSource<'static>,
    body_img: egui::ImageSource<'static>,
    food_img: egui::ImageSource<'static>,
    move_timer: Timer,
    food_timer: Timer,
    game_state: GameState,
    food_location: Option<(u16, u16)>,
}

impl Default for MyRustSnakeGame {
    fn default() -> Self {
        Self {
            map_size: (15, 17),
            head_img: egui::include_image!("assets/snake_head.png"),
            head_img_left: egui::include_image!("assets/snake_head_left.png"),
            head_img_up: egui::include_image!("assets/snake_head_up.png"),
            head_img_right: egui::include_image!("assets/snake_head_right.png"),
            body_img: egui::include_image!("assets/snake_body.png"),
            food_img: egui::include_image!("assets/apple.png"),
            move_timer: Timer::default(),
            food_timer: Timer::default(),
            game_state: GameState::default(),
            food_location: None,
        }
    }
}

impl eframe::App for MyRustSnakeGame {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(Color32::BLACK))
            .show(ctx, |ui| {
                self.handle_input(ui);
                self.game_logic();
                self.draw_game(ui);
            });
        ctx.request_repaint();
    }
}

impl MyRustSnakeGame {
    fn game_logic(&mut self) {
        let now = std::time::Instant::now();

        // move timer
        let mut dt = (now - self.move_timer.last_update).as_secs_f32();
        self.move_timer.last_update = now;
        self.move_timer.elapsed_time += dt;

        let mut update_interval = 0.5; // seconds
        if self.move_timer.elapsed_time >= update_interval {
            self.game_update();
            self.move_timer.elapsed_time -= update_interval;
        }

        // food timer
        dt = (now - self.food_timer.last_update).as_secs_f32();
        self.food_timer.last_update = now;
        self.food_timer.elapsed_time += dt;

        update_interval = 5.0; // seconds
        if self.food_timer.elapsed_time >= update_interval {
            self.spawn_food();
            self.food_timer.elapsed_time -= update_interval;
        }
    }

    fn spawn_food(&mut self) {
        if self.food_location.is_some() {
            return;
        }
        let mut rng = rand::rng();
        self.food_location = Some((
            rng.random::<u16>() % (self.map_size.0 as u16),
            rng.random::<u16>() % (self.map_size.1 as u16),
        ));
    }

    fn draw_game(&mut self, ui: &egui::Ui) {
        self.draw_background(ui, self.map_size);
        self.draw_player(ui);
    }

    fn draw_background(&self, ui: &egui::Ui, map_size: (u8, u8)) {
        let painter = ui.painter();
        let mut rect = ui.max_rect(); // The whole panel

        // Make it a square and center that shit
        if rect.width() < rect.height() {
            rect.min.y += (rect.height() - rect.width()) / 2.0;
            rect.max.y += (rect.height() - rect.width()) / 2.0;
            rect.set_height(rect.width());
        } else {
            rect.min.x += (rect.width() - rect.height()) / 2.0;
            rect.max.x += (rect.width() - rect.height()) / 2.0;
            rect.set_width(rect.height());
        }

        painter.rect_filled(rect, 0.0, Color32::GRAY); // Fill background
        painter.rect_stroke(rect, 0.0, (2.0, Color32::BLUE)); // Red border

        let rows = map_size.0 as usize;
        let cols = map_size.1 as usize;
        let cell_width = rect.width() / cols as f32;
        let cell_height = rect.height() / rows as f32;

        for col in 0..=cols {
            let x = rect.left() + col as f32 * cell_width;
            painter.line_segment(
                [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
                (3.0, Color32::BLACK),
            );
        }

        for row in 0..=rows {
            let y = rect.top() + row as f32 * cell_height;
            painter.line_segment(
                [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
                (3.0, Color32::BLACK),
            );
        }
    }

    fn handle_input(&mut self, ui: &egui::Ui) {
        ui.input(|inp| {
            if inp.keys_down.len() > 0 {
                if inp.key_down(Key::ArrowDown) || inp.key_down(Key::S) {
                    self.game_state.player_direction = Direction::TopDown;
                } else if inp.key_down(Key::ArrowUp) || inp.key_down(Key::W) {
                    self.game_state.player_direction = Direction::BottomUp;
                } else if inp.key_down(Key::ArrowRight) || inp.key_down(Key::D) {
                    self.game_state.player_direction = Direction::LeftToRight;
                } else if inp.key_down(Key::ArrowLeft) || inp.key_down(Key::A) {
                    self.game_state.player_direction = Direction::RightToLeft;
                }
            }
        });
    }

    fn draw_player(&mut self, ui: &egui::Ui) {
        for part in self.game_state.body_parts.clone() {
            self.draw_at_cell(ui, self.body_img.clone(), part, self.map_size);
        }

        let head_img = match self.game_state.player_draw_direction {
            Direction::TopDown => &self.head_img,
            Direction::BottomUp => &self.head_img_up,
            Direction::LeftToRight => &self.head_img_right,
            Direction::RightToLeft => &self.head_img_left,
        };
        self.draw_at_cell(
            ui,
            head_img.clone(),
            self.game_state.player_position,
            self.map_size,
        );

        if self.food_location.is_some() {
            self.draw_at_cell(
                ui,
                self.food_img.clone(),
                self.food_location.unwrap(),
                self.map_size,
            );
        }
    }

    fn game_update(&mut self) {
        let prev_player_pos = self.game_state.player_position;

        // move the head
        match self.game_state.player_direction {
            Direction::TopDown => {
                self.game_state.player_position.0 =
                    (self.game_state.player_position.0 + 1) % self.map_size.0 as u16;
            }
            Direction::BottomUp => {
                self.game_state.player_position.0 =
                    (self.game_state.player_position.0 + self.map_size.0 as u16 - 1)
                        % self.map_size.0 as u16;
            }
            Direction::LeftToRight => {
                self.game_state.player_position.1 =
                    (self.game_state.player_position.1 + 1) % self.map_size.1 as u16;
            }
            Direction::RightToLeft => {
                self.game_state.player_position.1 =
                    (self.game_state.player_position.1 + self.map_size.1 as u16 - 1)
                        % self.map_size.1 as u16;
            }
        }

        self.game_state.body_parts.push_front(prev_player_pos);

        if self.game_state.player_position
            == self
                .food_location
                .unwrap_or((self.map_size.0 as u16, self.map_size.1 as u16))
        {
            self.food_location = None;
        } else {
            self.game_state.body_parts.pop_back();
        }

        self.game_state.player_draw_direction = self.game_state.player_direction;
    }

    fn draw_at_cell(
        &self,
        ui: &egui::Ui,
        image: egui::ImageSource<'static>,
        grid_pos: (u16, u16),
        map_size: (u8, u8),
    ) {
        let rows = map_size.0 as usize;
        let cols = map_size.1 as usize;
        let mut rect = ui.max_rect();

        // Make it a square and center it
        if rect.width() < rect.height() {
            rect.min.y += (rect.height() - rect.width()) / 2.0;
            rect.set_height(rect.width());
        } else {
            rect.min.x += (rect.width() - rect.height()) / 2.0;
            rect.set_width(rect.height());
        }

        let cell_width = rect.width() / cols as f32;
        let cell_height = rect.height() / rows as f32;

        let tile_rect = egui::Rect::from_min_size(
            egui::pos2(
                rect.left() + grid_pos.1 as f32 * cell_width,
                rect.top() + grid_pos.0 as f32 * cell_height,
            ),
            egui::vec2(cell_width, cell_height),
        );

        egui::Image::new(image).paint_at(ui, tile_rect);
    }
}
