
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
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            map_size: (15,17)
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::CentralPanel::default()
       .frame(egui::Frame::none().fill(Color32::BLACK))
        .show(ctx, |ui| {
                let painter = ui.painter();

                let mut rect = ui.max_rect(); // The whole panel
                
                // Make it a square and center that shit
                if rect.width() < rect.height() {
                    rect.min.y += (rect.height() - rect.width()) / 2.0;
                    rect.max.y += (rect.height() - rect.width()) / 2.0;
                    rect.set_height(rect.width());
                }else{
                    rect.min.x += (rect.width() - rect.height()) / 2.0;
                    rect.max.x += (rect.width() - rect.height()) / 2.0;
                    rect.set_width(rect.height());
                }

                painter.rect_filled(rect, 0.0, Color32::WHITE); // Fill background
                painter.rect_stroke(rect, 0.0, (2.0, Color32::BLUE)); // Red border

                let rows = self.map_size.0 as usize;
                let cols = self.map_size.1 as usize;
                let cell_width = rect.width() / cols as f32;
                let cell_height = rect.height() / rows as f32;

                for col in 0..=cols {
                    let x = rect.left() + col as f32 * cell_width;
                    painter.line_segment(
                        [
                            egui::pos2(x, rect.top()),
                            egui::pos2(x, rect.bottom()),
                        ],
                        (3.0, Color32::BLACK),
                    );
                }

                for row in 0..=rows {
                    let y = rect.top() + row as f32 * cell_height;
                    painter.line_segment(
                        [
                            egui::pos2(rect.left(), y),
                            egui::pos2(rect.right(), y),
                        ],
                        (3.0, Color32::GRAY),
                    );
                }



                // Example: Draw a green circle in the center
                let center = rect.center();
                painter.circle_filled(center, 40.0, Color32::GREEN);
            });
    }
}