use egui::Ui;

pub fn render(ui: &mut Ui) {
    ui.heading("Activity navigation");
    ui.horizontal(|ui| {
        ui.monospace("W");
        ui.monospace("R");
        ui.monospace("E");
    });
}
