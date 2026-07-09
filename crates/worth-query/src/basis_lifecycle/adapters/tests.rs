use crate::historical::HistoricalMaterializationDescriptor;
use crate::query_context::QueryBasisContextRequest;
use crate::runtime::{
    WorthQueryAuthorityLane, WorthQueryBasisAdmissionEvidenceRow, WorthQueryBranchBasisAdmission,
    WorthQueryEffectPolicy, WorthQueryPreviewBasisAdmission, WorthQueryRuntimeEvidenceAuthority,
    WorthQueryRuntimeInspectionEvidence,
};
use crate::session_label::WorthQuerySessionLabel;
use crate::subscription::QuerySubscriptionBasisPosture;

use super::{
    adapt_branch_admission_to_lifecycle, adapt_causal_inspection_evidence_to_lifecycle,
    adapt_historical_materialization_to_lifecycle, adapt_preview_admission_to_lifecycle,
    adapt_query_basis_context_to_lifecycle, adapt_subscription_basis_posture_to_lifecycle,
    BasisLifecycleAdapterOutcome,
};
use crate::basis_lifecycle::BasisLifecycleMigrationSurface;

#[test]
fn branch_and_preview_admissions_lower_into_scoped_lifecycle_proofs() {
    let authority = WorthQueryRuntimeEvidenceAuthority::new();
    let branch = WorthQueryBranchBasisAdmission::new(
        &authority,
        WorthQuerySessionLabel::scoped_strs("basis-lifecycle-tests", ["branch:adapter"])
            .expect("branch label should build"),
        WorthQueryEffectPolicy::AuthoritativeAllowed,
        WorthQueryBasisAdmissionEvidenceRow::rows_from_values(["relational-head"]),
    );
    let preview = WorthQueryPreviewBasisAdmission::new(
        &authority,
        WorthQuerySessionLabel::scoped_strs("basis-lifecycle-tests", ["preview:adapter"])
            .expect("preview label should build"),
        WorthQueryEffectPolicy::DeriveOnly,
        WorthQueryBasisAdmissionEvidenceRow::rows_from_values(["bridge-preview"]),
    );

    let branch_proof = adapt_branch_admission_to_lifecycle(&branch).expect("branch adapts");
    let preview_proof = adapt_preview_admission_to_lifecycle(&preview).expect("preview adapts");

    assert_eq!(
        branch_proof.surface(),
        BasisLifecycleMigrationSurface::BranchPreviewAdmission
    );
    assert_eq!(
        branch_proof.outcome(),
        BasisLifecycleAdapterOutcome::ScopedCapability
    );
    assert_eq!(branch_proof.operation_lane(), "mutation_preparation");
    assert_eq!(
        preview_proof.outcome(),
        BasisLifecycleAdapterOutcome::ScopedCapability
    );
    assert_eq!(preview_proof.operation_lane(), "preview_closeout");
    assert_ne!(
        branch_proof.adapter_proof_digest(),
        preview_proof.adapter_proof_digest()
    );
}

#[test]
fn render_colliding_session_labels_remain_distinct_in_basis_lifecycle_adaptation() {
    let authority = WorthQueryRuntimeEvidenceAuthority::new();
    let left = WorthQueryPreviewBasisAdmission::new(
        &authority,
        WorthQuerySessionLabel::scoped_strs("worth.kernel", ["preview"])
            .expect("left label should build"),
        WorthQueryEffectPolicy::DeriveOnly,
        WorthQueryBasisAdmissionEvidenceRow::rows_from_values(["bridge-preview"]),
    );
    let right = WorthQueryPreviewBasisAdmission::new(
        &authority,
        WorthQuerySessionLabel::scoped_strs("worth", ["kernel", "preview"])
            .expect("right label should build"),
        WorthQueryEffectPolicy::DeriveOnly,
        WorthQueryBasisAdmissionEvidenceRow::rows_from_values(["bridge-preview"]),
    );

    assert_eq!(
        left.label(),
        right.label(),
        "rendered labels should collide"
    );
    assert_ne!(
        left.label_identity(),
        right.label_identity(),
        "typed label identities must remain distinct"
    );

    let left_proof = adapt_preview_admission_to_lifecycle(&left).expect("left preview adapts");
    let right_proof = adapt_preview_admission_to_lifecycle(&right).expect("right preview adapts");

    assert_ne!(
        left_proof.adapter_proof_digest(),
        right_proof.adapter_proof_digest(),
        "lifecycle adaptation must preserve canonical label identity"
    );
}

#[test]
fn read_composition_contexts_lower_or_deny_through_lifecycle() {
    let current =
        adapt_query_basis_context_to_lifecycle(&QueryBasisContextRequest::current_branch_head())
            .expect("current context adapts");
    let branch = adapt_query_basis_context_to_lifecycle(&QueryBasisContextRequest::branch_head(
        "branch:adapter",
    ))
    .expect("branch context adapts");
    let historical = adapt_query_basis_context_to_lifecycle(
        &QueryBasisContextRequest::historical_snapshot("history:adapter"),
    )
    .expect("historical context adapts");
    let preview = adapt_query_basis_context_to_lifecycle(
        &QueryBasisContextRequest::preview_derived_historical("preview:adapter"),
    )
    .expect("preview-derived context adapts");

    assert_eq!(current.operation_lane(), "inspection");
    assert_eq!(branch.operation_lane(), "mutation_preparation");
    assert_eq!(historical.operation_lane(), "replay");
    assert_eq!(
        preview.outcome(),
        BasisLifecycleAdapterOutcome::AdvisoryEligibility
    );
    assert!([current, branch, historical, preview].iter().all(
        |proof| proof.surface() == BasisLifecycleMigrationSurface::ReadCompositionBasisContext
    ));
}

#[test]
fn subscription_postures_do_not_remain_raw_basis_shortcuts() {
    let current =
        adapt_subscription_basis_posture_to_lifecycle(&QuerySubscriptionBasisPosture::CurrentHead)
            .expect("current-head subscription adapts");
    let branch =
        adapt_subscription_basis_posture_to_lifecycle(&QuerySubscriptionBasisPosture::BranchHead)
            .expect("branch subscription adapts");
    let unsupported = adapt_subscription_basis_posture_to_lifecycle(
        &QuerySubscriptionBasisPosture::RuntimeHistoricalSnapshot,
    )
    .expect("unsupported subscription posture still produces typed lifecycle denial");

    assert_eq!(
        current.outcome(),
        BasisLifecycleAdapterOutcome::ScopedCapability
    );
    assert_eq!(
        branch.outcome(),
        BasisLifecycleAdapterOutcome::ScopedCapability
    );
    assert_eq!(
        unsupported.outcome(),
        BasisLifecycleAdapterOutcome::TypedDenial
    );
    assert_eq!(unsupported.operation_lane(), "subscription_declaration");
}

#[test]
fn causal_and_historical_surfaces_have_lifecycle_adapter_evidence() {
    let authority = WorthQueryRuntimeEvidenceAuthority::new();
    let causal = WorthQueryRuntimeInspectionEvidence::new(
        &authority,
        "causal-anchor",
        WorthQueryAuthorityLane::PreviewTruth,
        ["bridge-causal-envelope"],
    );
    let historical = HistoricalMaterializationDescriptor::retained_snapshot("history:retained");

    let causal_proof =
        adapt_causal_inspection_evidence_to_lifecycle(&causal).expect("causal adapts");
    let historical_proof =
        adapt_historical_materialization_to_lifecycle(&historical).expect("historical adapts");

    assert_eq!(
        causal_proof.surface(),
        BasisLifecycleMigrationSurface::CausalInspectionBasisEvidence
    );
    assert_eq!(
        causal_proof.outcome(),
        BasisLifecycleAdapterOutcome::AdvisoryEligibility
    );
    assert_eq!(
        historical_proof.surface(),
        BasisLifecycleMigrationSurface::HistoricalMaterializationBasis
    );
    assert_eq!(
        historical_proof.outcome(),
        BasisLifecycleAdapterOutcome::ScopedCapability
    );
    assert_eq!(historical_proof.operation_lane(), "replay");
}
