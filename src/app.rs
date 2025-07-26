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
            let text_area = ui.add_sized(
                ui.available_size(),
                egui::TextEdit::multiline(&mut self.thought),
            );
            text_area.request_focus();
            if !self.thought.is_empty() {
                ui.add_space(5.0);
                if ui.button("Publish").clicked() {
                    println!("Thought published: {}", self.thought);
                }
            }
        });
    }
}
