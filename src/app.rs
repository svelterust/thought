use crate::config::Config;
use chrono::{DateTime, Datelike, Local};
use color_eyre::Result;
use eframe::egui;
use egui::{Button, Key, TextEdit, vec2};
use pulldown_cmark::{Parser, html};
use sailfish::TemplateSimple;

// Embed default styles here
const STYLES_CSS: &str = include_str!("styles.css");

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
        use scraper::{Html, Selector};

        let index_path = self.config.folder.join("index.html");
        if !index_path.exists() {
            return Ok(Vec::new());
        }

        let html_content = std::fs::read_to_string(&index_path)?;
        let document = Html::parse_document(&html_content);

        let post_selector = Selector::parse("article").unwrap();
        let date_selector = Selector::parse("time").unwrap();

        let mut posts = Vec::new();

        for post_element in document.select(&post_selector) {
            if let Some(date_element) = post_element.select(&date_selector).next() {
                let date_text = date_element.text().collect::<String>();
                let date_formatted = date_text.to_string();

                // Get content by removing the header
                let mut content_html = post_element.inner_html();
                if let Some(header_end) = content_html.find("</header>") {
                    content_html = content_html[header_end + 9..].trim().to_string();
                }

                posts.push(Post {
                    content: content_html,
                    date_formatted,
                });
            }
        }

        Ok(posts)
    }

    fn generate_index(&self, posts: &[Post]) -> Result<()> {
        let template = IndexTemplate {
            title: &self.config.title,
            username: &self.config.username,
            posts,
        };
        let html = template.render_once()?;
        let index_path = self.config.folder.join("index.html");
        std::fs::write(index_path, html)?;

        // Generate styles.css if it doesn't exist
        let styles_path = self.config.folder.join("styles.css");
        if !styles_path.exists() {
            std::fs::write(styles_path, STYLES_CSS)?;
        }
        Ok(())
    }

    fn publish(&mut self) -> Result<()> {
        // Convert current post content to HTML
        let mut posts = self.load_existing_posts()?;
        let html_content = markdown_to_html(&self.post.content);
        let new_post = Post {
            content: html_content,
            date_formatted: self.post.date_formatted.clone(),
        };

        // Add new post at the beginning (newest first)
        posts.insert(0, new_post);
        self.generate_index(&posts)
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
                    Err(e) => eprintln!("Failed to publish: {e}"),
                }
            }
        });
    }
}
