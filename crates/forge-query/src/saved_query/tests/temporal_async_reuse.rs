use super::temporal_async_fixtures::{
    basis_aware_composed_detail, exact_policy_basis_reuse_descriptor,
    freeze_future_preserving_detail_saved_query, saved_query_reuse_descriptor_for_saved,
};
use crate::policy_basis::{
    PolicyExecutionModeRequest, PolicyReuseEquivalenceContract, SavedQueryPolicyReuseDescriptor,
    SavedQueryPolicyReuseDisposition,
};
use crate::query_context::QueryContextFamily;
use crate::saved_query::{
    evaluate_saved_query_reuse, SavedQueryRebindingDimension, SavedQueryRebindingLegality,
    SavedQueryReuseOutcome, SavedQueryTemporalAsyncSurfacePosture,
};

fn freeze_future_preserving_saved_query() -> crate::saved_query::SavedQueryArtifact {
    let composed = basis_aware_composed_detail();
    freeze_future_preserving_detail_saved_query(&composed)
}

fn policy_mode_for_basis_family(family: &QueryContextFamily) -> PolicyExecutionModeRequest {
    match family {
        QueryContextFamily::CurrentBranchHead => PolicyExecutionModeRequest::CurrentRead,
        QueryContextFamily::BranchHead => PolicyExecutionModeRequest::BranchRead,
        QueryContextFamily::HistoricalSnapshot | QueryContextFamily::HistoricalCommit => {
            PolicyExecutionModeRequest::HistoricalRead
        }
        QueryContextFamily::PreviewDerivedHistorical | QueryContextFamily::DiffComparison => {
            panic!("phase 13 preserved reuse should not admit preview or diff policy basis")
        }
    }
}

#[test]
fn future_preserving_saved_query_reuse_denies_when_policy_basis_evidence_is_omitted() {
    let saved = freeze_future_preserving_saved_query();
    let descriptor = saved_query_reuse_descriptor_for_saved(&saved);

    let SavedQueryReuseOutcome::Denied(denial) = evaluate_saved_query_reuse(&saved, &descriptor)
    else {
        panic!("future-preserving reuse without policy-basis evidence must deny");
    };
    assert_eq!(
        denial.temporal_async_surface_posture(),
        SavedQueryTemporalAsyncSurfacePosture::FuturePreservingRuntimeBacked
    );
    assert_eq!(
        denial.temporal_async_drift_dimension(),
        Some(SavedQueryRebindingDimension::PolicyBasisReuse)
    );
    assert_eq!(
        denial.policy_basis_reuse_disposition(),
        SavedQueryPolicyReuseDisposition::IllegalSemanticDrift
    );
}

#[test]
fn future_preserving_saved_query_reuse_admits_once_policy_basis_preservation_is_explicit() {
    let saved = freeze_future_preserving_saved_query();
    let basis_family = saved
        .metadata()
        .basis_family()
        .cloned()
        .expect("future-preserving saved query should carry a basis family");
    let descriptor =
        saved_query_reuse_descriptor_for_saved(&saved).with_policy_basis_reuse_descriptor(
            exact_policy_basis_reuse_descriptor(saved.digest().as_str(), &basis_family),
        );

    let SavedQueryReuseOutcome::Admitted(decision) =
        evaluate_saved_query_reuse(&saved, &descriptor)
    else {
        panic!("future-preserving reuse with explicit policy-basis preservation must admit");
    };
    assert_eq!(
        decision.overall(),
        SavedQueryRebindingLegality::LegalNoSemanticChange
    );
    assert_eq!(
        decision.temporal_async_surface_posture(),
        SavedQueryTemporalAsyncSurfacePosture::FuturePreservingRuntimeBacked
    );
    assert_eq!(
        decision.policy_basis_reuse_disposition(),
        SavedQueryPolicyReuseDisposition::LegalNoSemanticChange
    );
}

#[test]
fn future_preserving_saved_query_reuse_can_require_fresh_freeze_without_becoming_drift() {
    let saved = freeze_future_preserving_saved_query();
    let basis_family = saved
        .metadata()
        .basis_family()
        .cloned()
        .expect("future-preserving saved query should carry a basis family");
    let execution_mode = policy_mode_for_basis_family(&basis_family);
    let descriptor = saved_query_reuse_descriptor_for_saved(&saved)
        .with_policy_basis_reuse_descriptor(
            SavedQueryPolicyReuseDescriptor::new(
                saved.digest().as_str(),
                "policy:a",
                "tenant-truth:a",
                "tenant-schema:a",
                "branch:a",
                execution_mode,
                "policy:b",
                "tenant-truth:b",
                "tenant-schema:b",
                "branch:a",
                execution_mode,
            )
            .with_equivalence(PolicyReuseEquivalenceContract::fresh_freeze_required())
            .with_temporal_async_surface(
                SavedQueryTemporalAsyncSurfacePosture::FuturePreservingRuntimeBacked,
                Some(basis_family.clone()),
                Some(basis_family),
            ),
        );

    let SavedQueryReuseOutcome::Admitted(decision) =
        evaluate_saved_query_reuse(&saved, &descriptor)
    else {
        panic!("compatible future-preserving policy-basis rebind should require fresh freeze");
    };
    assert_eq!(
        decision.overall(),
        SavedQueryRebindingLegality::LegalRequiresFreshFreeze
    );
    assert_eq!(
        decision.policy_basis_reuse_disposition(),
        SavedQueryPolicyReuseDisposition::LegalRequiresFreshFreeze
    );
}

#[test]
fn future_preserving_saved_query_reuse_rejects_policy_basis_evidence_from_another_saved_query() {
    let saved = freeze_future_preserving_saved_query();
    let basis_family = saved
        .metadata()
        .basis_family()
        .cloned()
        .expect("future-preserving saved query should carry a basis family");
    let execution_mode = policy_mode_for_basis_family(&basis_family);
    let descriptor = saved_query_reuse_descriptor_for_saved(&saved)
        .with_policy_basis_reuse_descriptor(
            SavedQueryPolicyReuseDescriptor::new(
                "forged-other-saved-query",
                "policy:a",
                "tenant-truth:a",
                "tenant-schema:a",
                "branch:a",
                execution_mode,
                "policy:a",
                "tenant-truth:a",
                "tenant-schema:a",
                "branch:a",
                execution_mode,
            )
            .with_temporal_async_surface(
                SavedQueryTemporalAsyncSurfacePosture::FuturePreservingRuntimeBacked,
                Some(basis_family.clone()),
                Some(basis_family),
            ),
        );

    let SavedQueryReuseOutcome::Denied(denial) = evaluate_saved_query_reuse(&saved, &descriptor)
    else {
        panic!("policy-basis evidence from another saved query must deny");
    };
    assert_eq!(
        denial.temporal_async_drift_dimension(),
        Some(SavedQueryRebindingDimension::PolicyBasisReuse)
    );
    assert_eq!(
        denial.policy_basis_reuse_disposition(),
        SavedQueryPolicyReuseDisposition::LegalNoSemanticChange
    );
}

#[test]
fn future_preserving_saved_query_reuse_rejects_policy_basis_evidence_minted_for_ordinary_surface() {
    let saved = freeze_future_preserving_saved_query();
    let basis_family = saved
        .metadata()
        .basis_family()
        .cloned()
        .expect("future-preserving saved query should carry a basis family");
    let execution_mode = policy_mode_for_basis_family(&basis_family);
    let descriptor = saved_query_reuse_descriptor_for_saved(&saved)
        .with_policy_basis_reuse_descriptor(SavedQueryPolicyReuseDescriptor::new(
            saved.digest().as_str(),
            "policy:a",
            "tenant-truth:a",
            "tenant-schema:a",
            "branch:a",
            execution_mode,
            "policy:a",
            "tenant-truth:a",
            "tenant-schema:a",
            "branch:a",
            execution_mode,
        ));

    let SavedQueryReuseOutcome::Denied(denial) = evaluate_saved_query_reuse(&saved, &descriptor)
    else {
        panic!("ordinary-surface policy evidence must not admit future-preserving reuse");
    };
    assert_eq!(
        denial.temporal_async_drift_dimension(),
        Some(SavedQueryRebindingDimension::PolicyBasisReuse)
    );
    assert_eq!(
        denial.policy_basis_reuse_disposition(),
        SavedQueryPolicyReuseDisposition::LegalNoSemanticChange
    );
}
