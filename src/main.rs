// Modules
mod app;

// Imports
use app::App;
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Thought",
        options,
        Box::new(|cc| {
            // Setup App
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            cc.egui_ctx.set_pixels_per_point(2.0);
            Ok(Box::new(App::default()))
        }),
    )
}
