use crate::capability::ViewBindingId;
use crate::runtime::{
    WorthUiQueryBindingComparison, WorthUiQueryBindingComparisonOutcome,
    WorthUiQueryBindingPostureDriftFamily, WorthUiQueryRuntimeFactLoweringInput,
    WorthUiRuntimeFactId, WorthUiRuntimeFactSet,
};

pub(super) fn lower_admitted_query_runtime_facts(
    input: &WorthUiQueryRuntimeFactLoweringInput,
) -> WorthUiRuntimeFactSet {
    let mut changed_facts = WorthUiRuntimeFactSet::empty();
    lower_comparison_facts(input.binding_comparison(), &mut changed_facts);
    lower_projection_fact_receipts(input, &mut changed_facts);
    lower_state_snapshot_receipts(input, &mut changed_facts);
    lower_effect_posture_receipts(input, &mut changed_facts);
    lower_virtualized_frame_targets(input, &mut changed_facts);
    changed_facts
}

fn lower_comparison_facts(
    comparison: &WorthUiQueryBindingComparison,
    changed_facts: &mut WorthUiRuntimeFactSet,
) {
    for entry in comparison.entries() {
        if entry.outcome() == WorthUiQueryBindingComparisonOutcome::PreserveMeaning {
            continue;
        }
        let view_binding_id = ViewBindingId::new(entry.identity().view_binding_id())
            .expect("query binding identities preserve validated view binding ids");
        changed_facts.insert(WorthUiRuntimeFactId::query_binding(&view_binding_id));
        for drift in entry.posture_drifts() {
            lower_drift_fact(*drift, &view_binding_id, changed_facts);
        }
        if entry.posture_drifts().is_empty() {
            changed_facts.insert(WorthUiRuntimeFactId::query_live_view(&view_binding_id));
        }
    }
}

fn lower_drift_fact(
    drift: WorthUiQueryBindingPostureDriftFamily,
    view_binding_id: &ViewBindingId,
    changed_facts: &mut WorthUiRuntimeFactSet,
) {
    match drift {
        WorthUiQueryBindingPostureDriftFamily::SupportAdmission
        | WorthUiQueryBindingPostureDriftFamily::BasisCapability => {
            changed_facts.insert(WorthUiRuntimeFactId::query_computed_view(view_binding_id));
            changed_facts.insert(WorthUiRuntimeFactId::query_result_posture(view_binding_id));
        }
        WorthUiQueryBindingPostureDriftFamily::LiveCompatibility => {
            changed_facts.insert(WorthUiRuntimeFactId::query_live_view(view_binding_id));
        }
        WorthUiQueryBindingPostureDriftFamily::AsyncResultState => {
            changed_facts.insert(WorthUiRuntimeFactId::query_result_posture(view_binding_id));
        }
        WorthUiQueryBindingPostureDriftFamily::Recovery => {
            changed_facts.insert(WorthUiRuntimeFactId::query_recovery_posture(
                view_binding_id.as_str(),
            ));
        }
        WorthUiQueryBindingPostureDriftFamily::Inspection => {
            changed_facts.insert(WorthUiRuntimeFactId::query_inspection_target(
                view_binding_id.as_str(),
            ));
        }
        WorthUiQueryBindingPostureDriftFamily::ProjectionConsumption => {
            changed_facts.insert(WorthUiRuntimeFactId::query_projection_fact(
                view_binding_id.as_str(),
            ));
        }
        WorthUiQueryBindingPostureDriftFamily::DenialPresentation => {}
    }
}

fn lower_projection_fact_receipts(
    input: &WorthUiQueryRuntimeFactLoweringInput,
    changed_facts: &mut WorthUiRuntimeFactSet,
) {
    for receipt in input.projection_fact_receipts() {
        changed_facts.insert(WorthUiRuntimeFactId::query_projection_fact(
            receipt.receipt_identity().to_owned(),
        ));
    }
}

fn lower_state_snapshot_receipts(
    input: &WorthUiQueryRuntimeFactLoweringInput,
    changed_facts: &mut WorthUiRuntimeFactSet,
) {
    for receipt in input.state_snapshot_receipts() {
        changed_facts.insert(WorthUiRuntimeFactId::query_state_snapshot(
            receipt.receipt_identity().to_owned(),
        ));
    }
}

fn lower_effect_posture_receipts(
    input: &WorthUiQueryRuntimeFactLoweringInput,
    changed_facts: &mut WorthUiRuntimeFactSet,
) {
    for receipt in input.effect_posture_receipts() {
        changed_facts.insert(WorthUiRuntimeFactId::query_effect_posture(
            receipt.receipt_identity().to_owned(),
        ));
    }
}

fn lower_virtualized_frame_targets(
    input: &WorthUiQueryRuntimeFactLoweringInput,
    changed_facts: &mut WorthUiRuntimeFactSet,
) {
    for target in input.virtualized_frame_targets() {
        changed_facts.insert(WorthUiRuntimeFactId::virtualized_data_frame(*target));
    }
}
