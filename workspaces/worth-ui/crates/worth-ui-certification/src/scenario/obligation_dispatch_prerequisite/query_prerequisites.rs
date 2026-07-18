//! Query prerequisite evidence used by obligation admission targets.

use worth_ui::facade::admission::UiAdmissionQueryBasis;
use worth_ui::facade::graph::{UiGraphTouchDescriptor, UiGraphWorldProfile};
use worth_ui_query_binding::certification::worth_ui_query_snapshot_prerequisites;
use worth_ui_query_binding::{
    WorthUiQueryBasisPosture, WorthUiQueryCausalExplanationLane, WorthUiQueryInspectionLane,
    WorthUiQueryPrerequisiteBoundary, WorthUiQueryPrerequisiteEvidence,
    WorthUiQueryProjectionConsumptionLane,
};

pub fn query_snapshot_world_profile(
    snapshot_label: &str,
    schema_basis_parts: [&str; 3],
) -> UiGraphWorldProfile {
    let prerequisites = worth_ui_query_snapshot_prerequisites(snapshot_label, schema_basis_parts);
    UiGraphWorldProfile::query_snapshot_basis(prerequisites)
}

pub fn query_prerequisites(
    touch: &UiGraphTouchDescriptor,
    query_basis: UiAdmissionQueryBasis,
) -> WorthUiQueryPrerequisiteEvidence {
    let UiGraphWorldProfile::QuerySnapshotBasis { prerequisites } = touch.world().world_profile()
    else {
        panic!("query prerequisite tests require query snapshot worlds");
    };

    let basis_posture = match query_basis {
        UiAdmissionQueryBasis::GraphAligned => WorthUiQueryBasisPosture::GraphAligned,
        UiAdmissionQueryBasis::WrongWorldProjection => {
            WorthUiQueryBasisPosture::WrongWorldProjection
        }
        UiAdmissionQueryBasis::RebindRequired => WorthUiQueryBasisPosture::RebindRequired,
        UiAdmissionQueryBasis::StaleReceipt => WorthUiQueryBasisPosture::StaleReceipt,
        UiAdmissionQueryBasis::AmbiguousSources => WorthUiQueryBasisPosture::AmbiguousSources,
    };
    let inspection_lane = match query_basis {
        UiAdmissionQueryBasis::GraphAligned => WorthUiQueryInspectionLane::WorkspaceInspect,
        UiAdmissionQueryBasis::WrongWorldProjection
        | UiAdmissionQueryBasis::RebindRequired
        | UiAdmissionQueryBasis::StaleReceipt
        | UiAdmissionQueryBasis::AmbiguousSources => WorthUiQueryInspectionLane::NotRequested,
    };
    let causal_explanation_lane = match query_basis {
        UiAdmissionQueryBasis::GraphAligned => {
            WorthUiQueryCausalExplanationLane::AdmitAndRequestCausalInspection
        }
        UiAdmissionQueryBasis::WrongWorldProjection
        | UiAdmissionQueryBasis::RebindRequired
        | UiAdmissionQueryBasis::StaleReceipt
        | UiAdmissionQueryBasis::AmbiguousSources => {
            WorthUiQueryCausalExplanationLane::NotRequested
        }
    };

    WorthUiQueryPrerequisiteBoundary::new()
        .assemble(
            prerequisites.basis().clone(),
            prerequisites.resolution_report().clone(),
            basis_posture,
            WorthUiQueryProjectionConsumptionLane::ConsumeProjectionFacts,
            inspection_lane,
            causal_explanation_lane,
        )
        .expect("query prerequisite assembly should admit")
}
