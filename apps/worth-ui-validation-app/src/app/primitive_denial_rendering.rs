use egui::Color32;
use worth_ui::facade::{WorthUiPrimitiveDenialPresentation, WorthUiPrimitiveProofDenial};

pub(crate) fn render_primitive_denial(ui: &mut egui::Ui, denial: &WorthUiPrimitiveProofDenial) {
    ui.centered_and_justified(|ui| {
        ui.vertical_centered(|ui| {
            ui.label(
                egui::RichText::new("Worth primitive projection rejected")
                    .color(Color32::from_rgb(236, 160, 120))
                    .size(16.0),
            );
            ui.add_space(8.0);
            render_denial_detail(ui, denial);
        });
    });
}

fn render_denial_detail(ui: &mut egui::Ui, denial: &WorthUiPrimitiveProofDenial) {
    match denial {
        WorthUiPrimitiveProofDenial::InvalidAuthoredPrimitiveValues { report } => {
            let denial_set = report
                .status()
                .denial_set()
                .expect("invalid primitive values report carries denial set");
            render_report_header(
                ui,
                report.surface_id(),
                report.schema_digest(),
                report.admission_digest(),
                report.counters().schema_count(),
                report.counters().authored_props_seen(),
                report.counters().denials_emitted(),
            );
            for receipt in denial_set.denials() {
                render_denial_presentation(ui, &receipt.presentation());
            }
        }
        WorthUiPrimitiveProofDenial::InvalidFlowLayoutValues { report } => {
            let denial_set = report
                .status()
                .denial_set()
                .expect("invalid flow layout values report carries denial set");
            render_report_header(
                ui,
                report.surface_id(),
                report.schema_digest(),
                report.admission_digest(),
                report.counters().schema_count(),
                report.counters().authored_props_seen(),
                report.counters().denials_emitted(),
            );
            for receipt in denial_set.denials() {
                let presentation = receipt.presentation();
                denial_row(ui, "denial", presentation.title());
                for row in presentation.rows() {
                    denial_row(ui, row.label(), row.value());
                }
            }
        }
        WorthUiPrimitiveProofDenial::InvalidContentValues { report } => {
            let denial_set = report
                .status()
                .denial_set()
                .expect("invalid primitive content values report carries denial set");
            render_report_header(
                ui,
                report.surface_id(),
                report.schema_digest(),
                report.admission_digest(),
                report.counters().schema_count(),
                report.counters().authored_props_seen(),
                report.counters().denials_emitted(),
            );
            for receipt in denial_set.denials() {
                let presentation = receipt.presentation();
                denial_row(ui, "denial", presentation.title());
                for row in presentation.rows() {
                    denial_row(ui, row.label(), row.value());
                }
            }
        }
        WorthUiPrimitiveProofDenial::InvalidEventGeometryValues { report } => {
            let denial_set = report
                .status()
                .denial_set()
                .expect("invalid event geometry values report carries denial set");
            render_report_header(
                ui,
                report.surface_id(),
                report.schema_digest(),
                report.admission_digest(),
                report.counters().schema_count(),
                report.counters().authored_props_seen(),
                report.counters().denials_emitted(),
            );
            for receipt in denial_set.denials() {
                let presentation = receipt.presentation();
                denial_row(ui, "denial", presentation.title());
                for row in presentation.rows() {
                    denial_row(ui, row.label(), row.value());
                }
            }
        }
        WorthUiPrimitiveProofDenial::InvalidAppearanceStateValues { report } => {
            let denial_set = report
                .status()
                .denial_set()
                .expect("invalid appearance state values report carries denial set");
            render_report_header(
                ui,
                report.surface_id(),
                report.schema_digest(),
                report.admission_digest(),
                report.counters().schema_count(),
                report.counters().authored_props_seen(),
                report.counters().denials_emitted(),
            );
            for receipt in denial_set.denials() {
                render_denial_presentation(ui, &receipt.presentation());
            }
        }
        WorthUiPrimitiveProofDenial::InvalidInteractionValues { report } => {
            let denial_set = report
                .status()
                .denial_set()
                .expect("invalid interaction values report carries denial set");
            render_report_header(
                ui,
                report.surface_id(),
                report.schema_digest(),
                report.admission_digest(),
                report.counters().schema_count(),
                report.counters().authored_props_seen(),
                report.counters().denials_emitted(),
            );
            for receipt in denial_set.denials() {
                render_denial_presentation(ui, &receipt.presentation());
            }
        }
        _ => {
            ui.label(
                egui::RichText::new(denial.to_string())
                    .color(Color32::from_rgb(219, 224, 231))
                    .size(13.0),
            );
        }
    }
}

fn render_report_header(
    ui: &mut egui::Ui,
    surface_id: &str,
    schema_digest: u64,
    admission_digest: u64,
    schema_count: usize,
    authored_props_seen: usize,
    denials_emitted: usize,
) {
    denial_row(ui, "surface", surface_id);
    denial_row(ui, "schema_digest", &schema_digest.to_string());
    denial_row(ui, "admission_digest", &admission_digest.to_string());
    denial_row(ui, "schemas", &schema_count.to_string());
    denial_row(ui, "authored_props", &authored_props_seen.to_string());
    denial_row(ui, "denials", &denials_emitted.to_string());
}

fn render_denial_presentation(
    ui: &mut egui::Ui,
    presentation: &WorthUiPrimitiveDenialPresentation,
) {
    denial_row(ui, "denial", presentation.title());
    for row in presentation.rows() {
        denial_row(ui, row.label(), row.value());
    }
}

fn denial_row(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.label(
        egui::RichText::new(format!("{label}: {value}"))
            .color(Color32::from_rgb(219, 224, 231))
            .size(12.0),
    );
}
