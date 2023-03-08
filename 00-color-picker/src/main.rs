#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::{ColorImage, TextureHandle, TextureOptions};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        "Color Picker",
        options,
        Box::new(|_cc| Box::<ColorPickerApp>::default()),
    )
}

struct ColorPickerApp {
    r: u8,
    g: u8,
    b: u8,
    hex: String,
}

impl Default for ColorPickerApp {
    fn default() -> Self {
        let r = 200;
        let g = 50;
        let b = 90;
        let hex = format!("{r:2x}{g:2x}{b:2x}");
        Self { r, g, b, hex }
    }
}

impl ColorPickerApp {
    fn valid_hex(&self) -> bool {
        let hex_chars: Vec<char> = ('A'..='F').chain('a'..='f').chain('0'..='9').collect();
        self.hex.chars().all(|c| hex_chars.contains(&c)) && self.hex.len() == 6
    }

    fn hex_to_rgb(&mut self) {
        self.r = u8::from_str_radix(&self.hex[0..2], 16).expect("already validated by valid_hex");
        self.g = u8::from_str_radix(&self.hex[2..4], 16).expect("already validated by valid_hex");
        self.b = u8::from_str_radix(&self.hex[4..6], 16).expect("already validated by valid_hex");
    }

    fn rgb_to_hex(&mut self) {
        self.hex = format!("{:2x}{:2x}{:2x}", self.r, self.g, self.b);
    }
}

impl eframe::App for ColorPickerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_pixels_per_point(1.5);
            let texture: &TextureHandle = {
                let color_image = ColorImage::from_rgb([1, 1], &[self.r, self.g, self.b]);
                let texture_options = TextureOptions::default();
                &ui.ctx().load_texture("color", color_image, texture_options)
            };
            ui.image(texture, egui::vec2(100.0, 100.0));
            ui.add_space(10.0);
            ui.add(egui::Slider::new(&mut self.r, 0..=255).text("red"));
            ui.add(egui::Slider::new(&mut self.g, 0..=255).text("green"));
            ui.add(egui::Slider::new(&mut self.b, 0..=255).text("blue"));
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.style_mut().spacing.text_edit_width = 55.0;
                let hex_label = ui.label("Hex: ");
                ui.text_edit_singleline(&mut self.hex)
                    .labelled_by(hex_label.id);
                if ui
                    .add_enabled(self.valid_hex(), egui::Button::new("convert hex to rgb"))
                    .clicked()
                {
                    self.hex_to_rgb();
                }
                if ui.button("convert rgb to hex").clicked() {
                    self.rgb_to_hex();
                }
            });
        });
    }
}
