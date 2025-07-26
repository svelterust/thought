use crate::config::Config;
use chrono::{DateTime, Datelike, Local};
use color_eyre::Result;
use eframe::egui;
use egui::{Button, Key, TextEdit, vec2};
use pulldown_cmark::{Parser, html};
use sailfish::TemplateSimple;

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
    timestamp: DateTime<Local>,
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
            timestamp: created_at,
        }
    }
}

#[derive(TemplateSimple)]
#[template(path = "index.stpl")]
struct IndexTemplate<'a> {
    title: &'a str,
    username: &'a str,
    posts: &'a [Post],
}

#[derive(Debug)]
pub struct App {
    post: Post,
    focused: bool,
    config: Config,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self {
            post: Post::default(),
            focused: false,
            config,
        }
    }

    fn load_existing_posts(&self) -> Result<Vec<Post>> {
        let index_path = self.config.folder.join("index.html");
        if !index_path.exists() {
            return Ok(Vec::new());
        }
        Ok(Vec::new())
    }

    fn generate_index(&self, posts: &[Post]) -> Result<()> {
        let template = IndexTemplate {
            title: &self.config.title,
            username: &self.config.username,
            posts,
        };
        let html = template.render_once()?;
        let index_path = self.config.folder.join("index.html");
        Ok(std::fs::write(index_path, html)?)
    }

    fn publish(&mut self) -> Result<()> {
        // Convert current post content to HTML
        let mut posts = self.load_existing_posts()?;
        let html_content = markdown_to_html(&self.post.content);
        let new_post = Post {
            content: html_content,
            date_formatted: self.post.date_formatted.clone(),
            timestamp: self.post.timestamp,
        };

        // Add new post at the beginning (newest first)
        posts.insert(0, new_post);
        Ok(self.generate_index(&posts)?)
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
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
                match self.publish() {
                    Ok(_) => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
                    Err(e) => eprintln!("Failed to publish: {}", e),
                }
            }
        });
    }
}
