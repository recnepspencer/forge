use egui::Ui;

pub fn render(ui: &mut Ui) {
    ui.heading("Command projections");
    ui.horizontal(|ui| {
        ui.label("Menu");
        ui.label("Toolbar");
        ui.label("Palette");
        ui.label("Context");
    });
}
