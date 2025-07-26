use chrono::{DateTime, Datelike, Local};
use eframe::egui;
use egui::{Button, Key, TextEdit, vec2};
use pulldown_cmark::{Parser, html};

fn markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}

#[derive(Debug)]
pub struct Post {
    content: String,
    date_formatted: String,
}

fn format_date(date: &DateTime<Local>) -> String {
    let day = date.day();
    let suffix = match day {
        1 | 21 | 31 => "st",
        2 | 22 => "nd",
        3 | 23 => "rd",
        _ => "th",
    };
    format!(
        "{} {}{}, {}",
        date.format("%B"),
        day,
        suffix,
        date.format("%Y"),
    )
}

impl Default for Post {
    fn default() -> Self {
        // Setup created_at and format it
        let content = String::new();
        let created_at = Local::now();
        let date_formatted = format_date(&created_at);
        Self {
            content,
            date_formatted,
        }
    }
}

#[derive(Debug, Default)]
pub struct App {
    focused: bool,
    post: Post,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Header
            ui.heading(&self.post.date_formatted);
            ui.add_space(5.0);

            // Text input area
            let available_height = ui.available_height() - 40.0;
            let text_area = ui.add_sized(
                vec2(ui.available_width(), available_height),
                TextEdit::multiline(&mut self.post.content).lock_focus(true),
            );
            if !self.focused {
                text_area.request_focus();
                self.focused = true;
            }
            ui.add_space(5.0);

            // Button with disabled/enabled state
            let button = Button::new("Publish");
            let button_response = if self.post.content.is_empty() {
                ui.add_enabled_ui(false, |ui| {
                    ui.add_sized(vec2(ui.available_width(), 30.0), button)
                })
                .inner
            } else {
                ui.add_sized(vec2(ui.available_width(), 30.0), button)
            };

            // Publish to the world!
            let ctrl_enter_pressed = ui.input(|i| {
                i.modifiers.ctrl && i.key_pressed(Key::Enter) && !self.post.content.is_empty()
            });
            if button_response.clicked() || ctrl_enter_pressed {
                let html = markdown_to_html(&self.post.content);
                println!("{}", html.trim());
            }
        });
    }
}
