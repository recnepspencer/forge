use egui::Ui;

use super::ValidationPageSlotInteractionRenderPlan;

pub fn render_page_slot_interaction(ui: &mut Ui, plan: &ValidationPageSlotInteractionRenderPlan) {
    ui.heading("Page Slot Interaction Proof");
    ui.label(format!("Page host: {}", plan.page_name()));
    ui.label(format!(
        "Appearance dependency: {} ({})",
        plan.shadow_dependency().token_id(),
        plan.shadow_summary()
    ));
    ui.label(format!(
        "Density dependency: {} ({})",
        plan.padding_dependency().token_id(),
        plan.padding_summary()
    ));

    for slot in plan.slots() {
        ui.group(|ui| {
            ui.label(format!("Slot {}", slot.slot_name()));
            ui.monospace(slot.surface_id());
            ui.monospace(slot.component_id());
        });
    }

    if !plan.previous_slots().is_empty() {
        ui.separator();
        ui.label("Previous slot structure");
        for slot in plan.previous_slots() {
            ui.group(|ui| {
                ui.label(format!("Slot {}", slot.slot_name()));
                ui.monospace(slot.surface_id());
                ui.monospace(slot.component_id());
            });
        }
    }

    if !plan.authored_structural_rows().is_empty() {
        ui.separator();
        ui.label("Authored structural changed facts");
        for row in plan.authored_structural_rows() {
            ui.monospace(format!(
                "{:?} {} {:?}",
                row.slice_id(),
                row.subject_label(),
                row.change_posture()
            ));
            ui.label(format!("Families: {:?}", row.changed_fact_families()));
            for fact in row.changed_fact_labels() {
                ui.monospace(fact);
            }
        }
    }

    if let Some(rebind) = plan.latest_rebind() {
        ui.separator();
        ui.label(format!("Page-host rebind status: {:?}", rebind.status()));
        ui.label(format!(
            "Projection intersections: {}",
            rebind.dependency_intersection_count()
        ));
        ui.label(format!(
            "Projection rebuilds: {}",
            rebind.rebuild_attempt_count()
        ));
        for row in rebind.rows() {
            ui.monospace(format!(
                "{:?} {} {:?}",
                row.projection_family(),
                row.projection_identity(),
                row.status()
            ));
        }
    } else {
        ui.separator();
        ui.label("No page-host rebind proof yet.");
    }
}
