use super::host_observation::{
    ValidationHostFrameObservationOutcome, ValidationLiveViewFrameMeasurementProof,
};
use super::proof::ValidationLiveViewProjectionProof;

pub(super) fn render_live_view_evidence(
    ui: &mut egui::Ui,
    runtime: &worth_ui::facade::WorthUiRuntimeHost,
    proof: Result<&ValidationLiveViewProjectionProof, &str>,
    last_edit: Option<&worth_ui::facade::WorthUiLiveViewEditReceipt>,
    last_denial: Option<&worth_ui::facade::WorthUiLiveViewStateEditDenial>,
    last_submission: Option<&worth_ui::facade::WorthUiLiveViewInteractionSubmissionReceipt>,
    last_submission_denial: Option<&worth_ui::facade::WorthUiLiveViewInteractionActivationDenial>,
    last_source_denial: Option<&str>,
    measurement_proof: Option<&ValidationLiveViewFrameMeasurementProof>,
) {
    if let Ok(proof) = proof {
        render_mounted_evidence_rows(ui, proof);
    }
    if let Some(measurement_proof) = measurement_proof {
        render_host_measurement_evidence(ui, measurement_proof);
    }
    let observations = runtime.mount_live_view_observation_evidence(
        last_edit,
        last_denial,
        last_submission,
        last_submission_denial,
        last_source_denial,
    );
    for row in observations.rows() {
        ui.small(format!("{}={}", row.label(), row.value()));
    }
}

fn render_host_measurement_evidence(
    ui: &mut egui::Ui,
    proof: &ValidationLiveViewFrameMeasurementProof,
) {
    match proof.outcome() {
        ValidationHostFrameObservationOutcome::Admitted(receipt) => {
            ui.small(format!(
                "host measurement=admitted digest={} readiness={:?}",
                receipt.receipt_digest(),
                receipt.readiness()
            ));
            ui.small(format!(
                "host measurement counters=bounds:{} viewport:{} scroll:{} text:{} icon:{} dpi:{} elapsed:{}",
                receipt.counters().available_bounds_count(),
                receipt.counters().viewport_count(),
                receipt.counters().scroll_viewport_count(),
                receipt.counters().text_metric_count(),
                receipt.counters().icon_metric_count(),
                receipt.counters().dpi_count(),
                receipt.counters().elapsed_time_count()
            ));
            if let Some(measured) = proof.measured_product_view() {
                ui.small(format!(
                    "measured product view={} consumed_facts={}",
                    measured.receipt_digest(),
                    measured.consumed_facts().len()
                ));
            }
            if let Some(denial) = proof.measurement_denial() {
                ui.small(format!(
                    "measurement denial={:?} subject={}",
                    denial.code(),
                    denial.subject()
                ));
            }
        }
        ValidationHostFrameObservationOutcome::Denied(denials) => {
            ui.small(format!("host measurement=denied count={}", denials.len()));
            for denial in denials {
                ui.small(format!(
                    "host measurement denial={:?} subject={}",
                    denial.code(),
                    denial.subject()
                ));
            }
        }
        ValidationHostFrameObservationOutcome::Unavailable(reason) => {
            ui.small(format!("host measurement=unavailable reason={reason:?}"));
        }
    }
}

fn render_mounted_evidence_rows(ui: &mut egui::Ui, proof: &ValidationLiveViewProjectionProof) {
    let tree = proof.mounted_product_view().composition_tree();
    for root_child in tree.root_children() {
        for child in tree.ordered_children(root_child.node_id()) {
            let worth_ui::facade::WorthUiMountedNodeReceipt::Evidence(evidence) =
                child.mounted_node()
            else {
                continue;
            };
            for row in evidence.rows() {
                ui.small(format!("{}={}", row.label(), row.value()));
            }
        }
    }
}
