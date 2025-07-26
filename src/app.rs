use eframe::egui;

#[derive(Default)]
pub struct App {
    thought: String,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Thought");
            ui.add_space(5.0);
            ui.text_edit_multiline(&mut self.thought);
            if !self.thought.is_empty() {
                ui.add_space(5.0);
                if ui.button("Publish").clicked() {
                    println!("Thought published: {}", self.thought);
                }
            }
        });
    }
}
