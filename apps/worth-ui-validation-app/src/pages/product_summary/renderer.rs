use eframe::egui::{self, RichText};

use super::{
    ValidationProductSummaryEvidenceKind, ValidationProductSummaryEvidenceStatus,
    ValidationProductSummaryRenderPlan,
};

pub fn render_product_summary_page(ui: &mut egui::Ui, plan: &ValidationProductSummaryRenderPlan) {
    ui.heading("Product Summary");
    ui.label("Runtime-backed product slice for manual hot reload validation.");
    ui.separator();

    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.label(RichText::new("Runtime receipts").strong());
            ui.monospace(format!("page: {}", plan.page_name()));
            ui.monospace(format!(
                "page host frame: {}",
                plan.page_host_frame_digest()
            ));
            ui.monospace(format!("artifact: {}", plan.active_artifact_digest()));
            ui.monospace(format!("plan: {}", plan.active_plan_digest()));
            ui.monospace(format!("snapshot: {}", plan.capability_snapshot_digest()));
            ui.monospace(format!("frame epoch: {}", plan.frame_epoch()));
        });

        ui.separator();

        ui.vertical(|ui| {
            ui.label(RichText::new("Page slots").strong());
            for slot in plan.slots() {
                ui.horizontal(|ui| {
                    ui.monospace(slot.slot_name());
                    ui.label("->");
                    ui.monospace(slot.surface_id());
                });
            }
        });
    });

    ui.separator();
    render_evidence(ui, plan);
}

fn render_evidence(ui: &mut egui::Ui, plan: &ValidationProductSummaryRenderPlan) {
    let evidence = plan.evidence();
    let heading = match evidence.kind() {
        ValidationProductSummaryEvidenceKind::LaunchReceipt => "Launch evidence",
        ValidationProductSummaryEvidenceKind::RuntimeReload => "Runtime reload evidence",
        ValidationProductSummaryEvidenceKind::AuthoredBatchReload => {
            "Authoring-truth final boss evidence"
        }
        ValidationProductSummaryEvidenceKind::ThemeReload => "Theme reload evidence",
        ValidationProductSummaryEvidenceKind::CommandReload => "Command reload evidence",
        ValidationProductSummaryEvidenceKind::ComponentReload => "Component reload evidence",
        ValidationProductSummaryEvidenceKind::CommandProjectionReload => {
            "Command projection reload evidence"
        }
        ValidationProductSummaryEvidenceKind::AppearanceReload => "Appearance reload evidence",
        ValidationProductSummaryEvidenceKind::DensityReload => "Density reload evidence",
        ValidationProductSummaryEvidenceKind::Denial => "Denied reload evidence",
    };

    ui.label(RichText::new(heading).strong());
    ui.monospace(format!(
        "status: {}",
        evidence_status_label(evidence.status())
    ));
    ui.monospace(format!("primary digest: {}", evidence.primary_digest()));
    if let Some(digest) = evidence.secondary_digest() {
        ui.monospace(format!("secondary digest: {digest}"));
    }
    if let Some(status) = evidence.header_rebind_status() {
        ui.monospace(format!("header rebind: {status}"));
    }
    if let Some(count) = evidence.touched_count() {
        ui.monospace(format!("touched capability entries: {count}"));
    }
    ui.monospace(format!(
        "query bindings compared: {}",
        evidence.query_bindings_compared()
    ));
    ui.monospace(format!(
        "query rebind entries: {}",
        evidence.query_rebind_entries()
    ));
    ui.monospace(format!(
        "changed runtime facts: {}",
        evidence.changed_fact_count()
    ));
    if let Some(detail) = evidence.denial_detail() {
        ui.colored_label(ui.visuals().error_fg_color, detail);
    }
}

fn evidence_status_label(status: &ValidationProductSummaryEvidenceStatus) -> String {
    match status {
        ValidationProductSummaryEvidenceStatus::LaunchReceipt => "LaunchReceipt".to_owned(),
        ValidationProductSummaryEvidenceStatus::RuntimeReload(status) => format!("{status:?}"),
        ValidationProductSummaryEvidenceStatus::CapabilityReload(status) => format!("{status:?}"),
        ValidationProductSummaryEvidenceStatus::Denial(status) => {
            format!("{status:?}")
        }
    }
}
