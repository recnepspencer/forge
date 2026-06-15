use egui::Ui;

pub fn render(ui: &mut Ui) {
    ui.heading("Tabbed editor");
    ui.horizontal(|ui| {
        ui.label("Atlas");
        ui.separator();
        ui.label("Runtime");
        ui.separator();
        ui.label("Evidence");
    });
}
