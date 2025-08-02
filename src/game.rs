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

type MapSize = u16;

struct GameState {
    player_direction: Direction,
    player_draw_direction: Direction,
    player_position: (MapSize, MapSize),
    body_parts: VecDeque<(MapSize, MapSize)>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
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
    map_size: (MapSize, MapSize),
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
    end: bool,
}

impl Default for MyRustSnakeGame {
    fn default() -> Self {
        Self {
            map_size: (10, 10),
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
            end: true,
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
        if self.end {
            return;
        }
        let now = std::time::Instant::now();

        // move timer
        let mut dt = (now - self.move_timer.last_update).as_secs_f32();
        self.move_timer.last_update = now;
        self.move_timer.elapsed_time += dt;

        let mut update_interval = 0.45; // seconds
        if self.move_timer.elapsed_time >= update_interval {
            self.game_update();
            self.move_timer.elapsed_time -= update_interval;
        }

        // food timer
        dt = (now - self.food_timer.last_update).as_secs_f32();
        self.food_timer.last_update = now;
        self.food_timer.elapsed_time += dt;

        update_interval = 3.0; // seconds
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
        let mut next_food_location: (MapSize, MapSize);

        loop {
            next_food_location = (
                rng.random::<MapSize>() % self.map_size.0,
                rng.random::<MapSize>() % self.map_size.1,
            );
            let mut collision = false;

            if next_food_location == self.game_state.player_position {
                collision = true;
            }

            for part in self.game_state.body_parts.clone() {
                if part == next_food_location {
                    collision = true;
                    break;
                }
            }

            if collision == false {
                break;
            }
        }

        self.food_location = Some(next_food_location);
    }

    fn draw_game(&mut self, ui: &mut egui::Ui) {
        self.draw_background(ui, self.map_size);
        self.draw_player(ui);
        if self.end{
            self.draw_menu(ui)
        }
    }

    fn draw_menu(&mut self, ui: &mut egui::Ui) {
        let panel_rect = ui.max_rect();
        let frame_width = panel_rect.width() * 0.5;
        let frame_height = panel_rect.height() * 0.4;
        let frame_x = panel_rect.center().x - frame_width / 2.0;
        let frame_y = panel_rect.center().y - frame_height / 2.0;
        let frame_rect = egui::Rect::from_min_size(
            egui::pos2(frame_x, frame_y),
            egui::vec2(frame_width, frame_height),
        );

        ui.painter().rect_filled(frame_rect, 10.0, Color32::DARK_GRAY);
        ui.painter().rect_stroke(frame_rect, 10.0, (2.0, Color32::WHITE));

        let mut menu_ui = ui.child_ui(frame_rect, ui.layout().clone());

        menu_ui.vertical_centered(|ui| {
            ui.heading("RustSnake");
            ui.label("Press Start to play!");
            ui.add_space(16.0);

            if self.end {
                let snake_length = self.game_state.body_parts.len() + 1;
                ui.label(format!("Final snake length: {}", snake_length));
            }

            ui.add_space(16.0);

            let start_btn = egui::Button::new("Start")
                .min_size(egui::vec2(120.0, 40.0))
                .wrap(true);
            if ui.add(start_btn).clicked() {
                self.end = false;
                self.game_state = GameState::default();
                self.food_location = None;
                self.move_timer = Timer::default();
                self.food_timer = Timer::default();
            }
        });
    }

    fn draw_background(&self, ui: &egui::Ui, map_size: (MapSize, MapSize)) {
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

        let rows = map_size.0;
        let cols = map_size.1;
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
            let current_dir = self.game_state.player_draw_direction;
            if inp.keys_down.len() > 0 {
                if (inp.key_down(Key::ArrowDown) || inp.key_down(Key::S))
                    && current_dir != Direction::BottomUp
                {
                    self.game_state.player_direction = Direction::TopDown;
                } else if (inp.key_down(Key::ArrowUp) || inp.key_down(Key::W))
                    && current_dir != Direction::TopDown
                {
                    self.game_state.player_direction = Direction::BottomUp;
                } else if (inp.key_down(Key::ArrowRight) || inp.key_down(Key::D))
                    && current_dir != Direction::RightToLeft
                {
                    self.game_state.player_direction = Direction::LeftToRight;
                } else if (inp.key_down(Key::ArrowLeft) || inp.key_down(Key::A))
                    && current_dir != Direction::LeftToRight
                {
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

    fn stop_game(&mut self) {
        println!("GAME OVER!");
        self.end = true;
    }

    fn game_update(&mut self) {
        let prev_player_pos = self.game_state.player_position;
        let mut next_player_pos = self.game_state.player_position;

        // where we going
        match self.game_state.player_direction {
            Direction::TopDown => {
                if next_player_pos.0 + 1 < self.map_size.0 {
                    next_player_pos.0 += 1;
                }
            }
            Direction::BottomUp => {
                if next_player_pos.0 > 0 {
                    next_player_pos.0 -= 1;
                }
            }
            Direction::LeftToRight => {
                if next_player_pos.1 + 1 < self.map_size.1 {
                    next_player_pos.1 += 1;
                }
            }
            Direction::RightToLeft => {
                if next_player_pos.1 > 0 {
                    next_player_pos.1 -= 1;
                }
            }
        }

        let mut collision = false;
        
        for part in self.game_state.body_parts.clone() {
            if part == next_player_pos {
                collision = true;
            }
        }

        if next_player_pos == prev_player_pos || collision {
            self.stop_game();
            return;
        }

        // here we test collision and end game perhaps

        self.game_state.player_position = next_player_pos;

        self.game_state.body_parts.push_front(prev_player_pos);

        if self.game_state.player_position
            == self
                .food_location
                .unwrap_or((self.map_size.0, self.map_size.1))
        {
            self.food_location = None;
        } else {
            self.game_state.body_parts.pop_back();
        }

        self.game_state.player_draw_direction = self.game_state.player_direction;

        // you won man
        let snake_length = self.game_state.body_parts.len() + 1;
        let max_length = (self.map_size.0 * self.map_size.1) as usize;
        if snake_length >= max_length {
            self.stop_game();
        }
    }

    fn draw_at_cell(
        &self,
        ui: &egui::Ui,
        image: egui::ImageSource<'static>,
        grid_pos: (MapSize, MapSize),
        map_size: (MapSize, MapSize),
    ) {
        let rows = map_size.0;
        let cols = map_size.1;
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
