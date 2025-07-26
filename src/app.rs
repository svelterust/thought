use eframe::egui;
use egui::{Button, Key, TextEdit, vec2};

#[derive(Default)]
pub struct App {
    thought: String,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Header
            ui.heading("Thought");
            ui.add_space(5.0);

            // Text input area
            let available_height = ui.available_height() - 40.0;
            let text_area = ui.add_sized(
                vec2(ui.available_width(), available_height),
                TextEdit::multiline(&mut self.thought),
            );
            text_area.request_focus();
            ui.add_space(5.0);

            // Button with disabled/enabled state
            let button = Button::new("Publish");
            let button_response = if self.thought.is_empty() {
                ui.add_enabled_ui(false, |ui| {
                    ui.add_sized(vec2(ui.available_width(), 30.0), button)
                })
                .inner
            } else {
                ui.add_sized(vec2(ui.available_width(), 30.0), button)
            };

            // Publish to the world!
            let ctrl_enter_pressed = ui.input(|i| {
                i.modifiers.ctrl && i.key_pressed(Key::Enter) && !self.thought.is_empty()
            });
            if button_response.clicked() || ctrl_enter_pressed {
                println!("Thought published: {}", self.thought);
            }
        });
    }
}
