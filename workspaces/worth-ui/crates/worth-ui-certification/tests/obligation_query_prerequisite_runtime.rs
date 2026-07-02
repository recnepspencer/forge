#[path = "fixtures/obligation_dispatch_prerequisite_support/mod.rs"]
mod obligation_dispatch_prerequisite_support;

use worth_ui::facade::obligations::UiObligationFamily;
use worth_ui_query_binding::{
    WorthUiQueryBasisPosture, WorthUiQueryCausalExplanationLane, WorthUiQueryInspectionLane,
    WorthUiQueryProjectionConsumptionLane,
};

use self::obligation_dispatch_prerequisite_support::{
    execute_for_target, graph_aligned_query_target, query_touch, query_touch_app,
};

#[test]
fn query_requirement_selection_and_verdicts_retain_query_owned_prerequisite_evidence() {
    let app = query_touch_app();
    let touch = query_touch(&app);
    let bundle = execute_for_target(&app, &touch, graph_aligned_query_target(&touch));

    let selected = bundle
        .selected
        .obligations()
        .iter()
        .find(|obligation| obligation.family() == UiObligationFamily::QueryBindingRequirement)
        .expect("query binding requirement should be selected");
    let query_ref = selected
        .prerequisite_evidence_refs()
        .iter()
        .find_map(|evidence_ref| evidence_ref.query())
        .expect("selected query requirement should retain query prerequisite evidence");

    assert_eq!(
        query_ref.basis_posture(),
        WorthUiQueryBasisPosture::GraphAligned
    );
    assert_eq!(
        query_ref.projection_consumption_lane(),
        WorthUiQueryProjectionConsumptionLane::ConsumeProjectionFacts
    );
    assert_eq!(
        query_ref.inspection_lane(),
        WorthUiQueryInspectionLane::WorkspaceInspect
    );
    assert_eq!(
        query_ref.causal_explanation_lane(),
        WorthUiQueryCausalExplanationLane::AdmitAndRequestCausalInspection
    );

    let verdict = bundle
        .verdicts
        .iter()
        .find(|verdict| verdict.family() == Some(UiObligationFamily::QueryBindingRequirement))
        .expect("query binding verdict should exist");
    let verdict_ref = verdict
        .prerequisite_evidence_refs()
        .iter()
        .find_map(|evidence_ref| evidence_ref.query())
        .expect("query verdict should retain query prerequisite evidence");

    assert_eq!(verdict_ref, query_ref);
}
