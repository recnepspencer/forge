use std::sync::Arc;

use worth_query::facade::certification::admit_runtime_current_snapshot_basis_for_certification;
use worth_ui::facade::admission::UiAdmissionQueryBasis;
use worth_ui::facade::graph::{
    snapshot_resolution_report, QueryExternalIdentityToken, QueryExternalSchemaBasisToken,
    UiGraphTouchDescriptor, UiGraphWorldProfile, WorthQuerySnapshotIdentity,
};
use worth_ui_query_binding::{
    WorthUiQueryBasisPosture, WorthUiQueryBindingSubsystem, WorthUiQueryCausalExplanationLane,
    WorthUiQueryInspectionLane, WorthUiQueryPrerequisiteEvidence,
    WorthUiQueryProjectionConsumptionLane,
};

pub fn query_snapshot_world_profile(
    snapshot_label: &str,
    schema_basis_parts: [&str; 3],
) -> UiGraphWorldProfile {
    let snapshot_identity = WorthQuerySnapshotIdentity::admit_external_token(
        QueryExternalIdentityToken::new(Arc::<str>::from(snapshot_label)),
    );
    let basis = admit_runtime_current_snapshot_basis_for_certification(
        snapshot_identity.evidence_identity(),
        QueryExternalSchemaBasisToken::from_domain_parts(
            &schema_basis_parts
                .into_iter()
                .map(str::to_owned)
                .collect::<Vec<_>>(),
        ),
    )
    .expect("runtime current snapshot basis should resolve");

    UiGraphWorldProfile::query_snapshot_basis(basis.clone(), snapshot_resolution_report(&basis))
        .expect("query snapshot basis world should admit")
}

pub fn query_prerequisites(
    touch: &UiGraphTouchDescriptor,
    query_basis: UiAdmissionQueryBasis,
) -> WorthUiQueryPrerequisiteEvidence {
    let UiGraphWorldProfile::QuerySnapshotBasis {
        basis,
        resolution_report,
    } = touch.world().world_profile()
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

    WorthUiQueryBindingSubsystem::bootstrap()
        .prerequisites()
        .assemble(
            basis.clone(),
            resolution_report.clone(),
            basis_posture,
            WorthUiQueryProjectionConsumptionLane::ConsumeProjectionFacts,
            inspection_lane,
            causal_explanation_lane,
        )
        .expect("query prerequisite assembly should admit")
}
