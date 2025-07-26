// Modules
mod app;
mod config;

// Imports
use app::App;
use color_eyre::{Result, eyre::eyre};
use config::Config;
use eframe::egui;
use std::io::{self, Write};
use std::path::PathBuf;

fn run_app() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(false)
            .with_decorations(false),
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        "Thought",
        options,
        Box::new(|cc| {
            // Setup App
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            cc.egui_ctx.set_pixels_per_point(1.5);

            // Set style
            let mut style = (*cc.egui_ctx.style()).clone();
            style.text_styles = [
                (
                    egui::TextStyle::Heading,
                    egui::FontId::new(24.0, egui::FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Body,
                    egui::FontId::new(18.0, egui::FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Button,
                    egui::FontId::new(16.0, egui::FontFamily::Proportional),
                ),
            ]
            .iter()
            .cloned()
            .collect();
            cc.egui_ctx.set_style(style);
            Ok(Box::new(App::default()))
        }),
    )
}

fn main() -> Result<()> {
    // Check if config exists
    color_eyre::install()?;
    if Config::load().is_none() {
        // Prompt for title
        print!("Enter blog title: ");
        io::stdout().flush()?;
        let mut title = String::new();
        io::stdin().read_line(&mut title)?;
        let title = title.trim().to_string();
        if title.is_empty() {
            eprintln!("Title cannot be empty.");
            return Ok(());
        }

        // Prompt for folder
        let default_folder = dirs::home_dir()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        print!("Enter blog folder: {default_folder}");
        io::stdout().flush()?;
        let mut folder_input = String::new();
        io::stdin().read_line(&mut folder_input)?;
        let folder_input = folder_input.trim();
        let folder = if folder_input.is_empty() {
            PathBuf::from(default_folder)
        } else {
            PathBuf::from(folder_input)
        };

        // Create config
        let config = Config { title, folder };
        config.save()?;
        config.ensure_folder_exists()?;
    }

    // Start app
    run_app().map_err(|err| eyre!("Failed to start app: {err}"))
}
