// Modules
mod app;

// Imports
use app::App;
use eframe::egui;

fn main() -> eframe::Result {
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
            cc.egui_ctx.set_pixels_per_point(2.0);

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
