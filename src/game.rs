#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use egui::Color32;

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

            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    map_size: (u8, u8),
    head_img: egui::ImageSource<'static>,
    body_img: egui::ImageSource<'static>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { 
            map_size: (15, 17),
            head_img: egui::include_image!("assets/snake_head.png"),
            body_img: egui::include_image!("assets/snake_body.png"),
         }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(Color32::BLACK))
            .show(ctx, |ui| {
                self.draw_background(ui, self.map_size);
                self.draw_player(ui, (0, 0), 0.0, self.map_size);
            });
    }
}

impl MyApp {
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

        painter.rect_filled(rect, 0.0, Color32::WHITE); // Fill background
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

    fn draw_player(
        &self,
        ui: &egui::Ui,
        grid_pos: (usize, usize),
        _direction: f32,
        map_size: (u8, u8),
    ) {
        let rows = map_size.0 as usize;
        let cols = map_size.1 as usize;
        let mut rect = ui.max_rect();
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
        
        let cell_width = rect.width() / cols as f32;
        let cell_height = rect.height() / rows as f32;

        let tile_rect = egui::Rect::from_min_size(
            egui::pos2(
                rect.left() + grid_pos.1 as f32 * cell_width,
                rect.top() + (grid_pos.0 + 1) as f32 * cell_height,
            ),
            egui::vec2(cell_width, cell_height),
        );

         let tile_rect_body = egui::Rect::from_min_size(
            egui::pos2(
                rect.left() + grid_pos.1 as f32 * cell_width,
                rect.top() + grid_pos.0 as f32 * cell_height,
            ),
            egui::vec2(cell_width, cell_height),
        );

        // should clone just the reference not the image making it cheap ig
        egui::Image::new(self.head_img.clone()).paint_at(ui, tile_rect);

        egui::Image::new(self.body_img.clone()).paint_at(ui, tile_rect_body);
    }
}
