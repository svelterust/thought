use crate::config::Config;
use chrono::{Datelike, Local};
use anyhow::Result;
use eframe::egui::*;
use pulldown_cmark::{Parser, html};
use sailfish::TemplateSimple;
use scraper::{Html, Selector};

const STYLES_CSS: &str = include_str!("styles.css");

#[derive(Debug, Default)]
pub struct Post {
    content: String,
    date_formatted: String,
}

impl Post {
    fn new() -> Self {
        let now = Local::now();
        let day = now.day();
        let suffix = match day % 10 {
            1 if day != 11 => "st",
            2 if day != 12 => "nd",
            3 if day != 13 => "rd",
            _ => "th",
        };
        Self {
            content: String::new(),
            date_formatted: format!("{} {}{}, {}", now.format("%B"), day, suffix, now.year()),
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
            post: Post::new(),
            focused: false,
            config,
        }
    }

    fn load_existing_posts(&self) -> Result<Vec<Post>> {
        let index_path = self.config.folder.join("index.html");
        if !index_path.exists() {
            return Ok(Vec::new());
        }

        let document = Html::parse_document(&std::fs::read_to_string(&index_path)?);
        let post_selector = Selector::parse("article").unwrap();
        let date_selector = Selector::parse("time").unwrap();

        Ok(document
            .select(&post_selector)
            .filter_map(|post| {
                post.select(&date_selector).next().map(|date| {
                    let mut content = post.inner_html();
                    if let Some(pos) = content.find("</header>") {
                        content = content[pos + 9..].trim().to_string();
                    }
                    Post {
                        content,
                        date_formatted: date.text().collect(),
                    }
                })
            })
            .collect())
    }

    fn generate_index(&self, posts: &[Post]) -> Result<()> {
        std::fs::write(
            self.config.folder.join("index.html"),
            IndexTemplate {
                title: &self.config.title,
                username: &self.config.username,
                posts,
            }
            .render_once()?,
        )?;

        let styles_path = self.config.folder.join("styles.css");
        if !styles_path.exists() {
            std::fs::write(styles_path, STYLES_CSS)?;
        }
        Ok(())
    }

    fn publish(&mut self) -> Result<()> {
        let mut posts = self.load_existing_posts()?;
        let mut html_content = String::new();
        html::push_html(&mut html_content, Parser::new(&self.post.content));

        posts.insert(
            0,
            Post {
                content: html_content,
                date_formatted: self.post.date_formatted.clone(),
            },
        );
        self.generate_index(&posts)
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let text_area = ui.add_sized(
                vec2(ui.available_width(), ui.available_height() - 40.0),
                TextEdit::multiline(&mut self.post.content).lock_focus(true),
            );
            if !self.focused {
                text_area.request_focus();
                self.focused = true;
            }

            ui.add_space(5.0);
            let button_response = ui.add_enabled(
                !self.post.content.is_empty(),
                Button::new("Publish").min_size(vec2(ui.available_width(), 30.0)),
            );

            if (button_response.clicked()
                || ui.input(|i| i.modifiers.ctrl && i.key_pressed(Key::Enter)))
                && !self.post.content.is_empty()
            {
                match self.publish() {
                    Ok(_) => ctx.send_viewport_cmd(ViewportCommand::Close),
                    Err(e) => eprintln!("Failed to publish: {e}"),
                }
            }
        });
    }
}
