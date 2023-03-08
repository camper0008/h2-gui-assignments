#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::{vec2, Grid, Ui};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        centered: true,
        initial_window_size: Some(vec2(425.0, 500.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Calculator",
        options,
        Box::new(|_cc| Box::<CalculatorApp>::default()),
    )
}

enum Mode {
    Plus,
    Minus,
    Mult,
    Div,
}

struct CalculatorApp {
    sum: i64,
    input: String,
    mode: Mode,
}

impl Default for CalculatorApp {
    fn default() -> Self {
        Self {
            sum: 0,
            input: String::new(),
            mode: Mode::Plus,
        }
    }
}

impl CalculatorApp {
    fn add_input(&mut self, input: &str) {
        self.input += input;
    }
    fn evaluate(&mut self) {
        if self.input.is_empty() {
            self.input = "0".to_string();
        }
        let input: i64 = self.input.parse().expect("should be valid number");
        match self.mode {
            Mode::Plus => self.sum += input,
            Mode::Minus => self.sum -= input,
            Mode::Mult => self.sum *= input,
            Mode::Div => self.sum /= input,
        }
        self.input = String::new();
    }
}

fn add_number_buttons(app: &mut CalculatorApp, ui: &mut Ui, numbers: &[&str]) {
    for number in numbers.iter() {
        if ui.button(*number).clicked() {
            app.add_input(number);
        }
    }
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_pixels_per_point(2.0);
            ui.horizontal(|ui| {
                ui.label(&self.sum.to_string());
                ui.label(match self.mode {
                    Mode::Plus => "+",
                    Mode::Div => "÷",
                    Mode::Mult => "×",
                    Mode::Minus => "−",
                });
                ui.label(&self.input);
            });
            Grid::new("main_grid").show(ui, |ui| {
                ui.spacing_mut().button_padding = vec2(17.5, 15.0);
                ui.end_row();
                add_number_buttons(self, ui, &["1", "2", "3"]);
                if ui.button("−").clicked() {
                    self.evaluate();
                    self.mode = Mode::Minus;
                }
                ui.end_row();
                add_number_buttons(self, ui, &["4", "5", "6"]);
                if ui.button("÷").clicked() {
                    self.evaluate();
                    self.mode = Mode::Div;
                }
                ui.end_row();
                add_number_buttons(self, ui, &["7", "8", "9"]);
                if ui.button("×").clicked() {
                    self.evaluate();
                    self.mode = Mode::Mult;
                }
                ui.end_row();
                if ui.button("C").clicked() && !self.input.is_empty() {
                    self.input = self.input[..self.input.len() - 1].to_string();
                }
                add_number_buttons(self, ui, &["0"]);
                if ui.button("+").clicked() {
                    self.evaluate();
                    self.mode = Mode::Plus;
                }
                if ui.button("=").clicked() {
                    self.evaluate();
                    self.mode = Mode::Plus;
                }
                ui.end_row();
            })
        });
    }
}
