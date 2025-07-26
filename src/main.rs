// Modules
mod app;
mod config;

// Imports
use app::App;
use config::Config;
use eframe::egui;
use std::io::{self, Write};
use std::path::PathBuf;

fn main() -> eframe::Result {
    // Check if config exists
    if Config::load().is_none() {
        println!("Welcome to Thought!");
        println!("Let's set up your thought collection.\n");
        
        // Prompt for title
        print!("Enter blog title: ");
        io::stdout().flush().unwrap();
        let mut title = String::new();
        io::stdin().read_line(&mut title).unwrap();
        let title = title.trim().to_string();
        
        if title.is_empty() {
            eprintln!("Title cannot be empty. Exiting.");
            return Ok(());
        }
        
        // Prompt for folder with default
        let default_folder = dirs::home_dir()
            .unwrap_or_default()
            .join("thoughts")
            .to_string_lossy()
            .to_string();
        
        print!("Enter save folder [{}]: ", default_folder);
        io::stdout().flush().unwrap();
        let mut folder_input = String::new();
        io::stdin().read_line(&mut folder_input).unwrap();
        let folder_input = folder_input.trim();
        
        let folder = if folder_input.is_empty() {
            PathBuf::from(default_folder)
        } else {
            PathBuf::from(folder_input)
        };
        
        // Create config
        let config = Config { title, folder };
        
        // Save config
        if let Err(e) = config.save() {
            eprintln!("Failed to save config: {}", e);
            return Ok(());
        }
        
        // Create folder
        if let Err(e) = config.ensure_folder_exists() {
            eprintln!("Failed to create folder: {}", e);
            return Ok(());
        }
        
        println!("Setup complete! Starting Thought...\n");
    }
    
    // Run main app
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
