use super::temporal_async_fixtures::{
    exact_policy_basis_reuse_descriptor, freeze_future_preserving_grouped_saved_query,
    saved_query_reuse_descriptor_for_saved,
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

fn policy_mode_for_basis_family(family: &QueryContextFamily) -> PolicyExecutionModeRequest {
    match family {
        QueryContextFamily::CurrentBranchHead => PolicyExecutionModeRequest::CurrentRead,
        QueryContextFamily::BranchHead => PolicyExecutionModeRequest::BranchRead,
        QueryContextFamily::HistoricalSnapshot | QueryContextFamily::HistoricalCommit => {
            PolicyExecutionModeRequest::HistoricalRead
        }
        QueryContextFamily::PreviewDerivedHistorical | QueryContextFamily::DiffComparison => {
            panic!("grouped preserved reuse should not admit preview or diff policy basis")
        }
    }
}

#[test]
fn grouped_runtime_backed_saved_query_freeze_retains_future_preserving_surface_posture() {
    let saved = freeze_future_preserving_grouped_saved_query();

    assert_eq!(
        saved.metadata().view_shape_family().as_str(),
        "kanban_grouped"
    );
    assert_eq!(
        saved.metadata().temporal_async_surface_posture(),
        SavedQueryTemporalAsyncSurfacePosture::FuturePreservingRuntimeBacked
    );
}

#[test]
fn grouped_future_preserving_saved_query_reuse_denies_without_policy_basis_evidence() {
    let saved = freeze_future_preserving_grouped_saved_query();

    let SavedQueryReuseOutcome::Denied(denial) =
        evaluate_saved_query_reuse(&saved, &saved_query_reuse_descriptor_for_saved(&saved))
    else {
        panic!("grouped future-preserving reuse without policy-basis evidence must deny");
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
fn grouped_future_preserving_saved_query_reuse_admits_with_explicit_preserved_basis_evidence() {
    let saved = freeze_future_preserving_grouped_saved_query();
    let basis_family = saved
        .metadata()
        .basis_family()
        .cloned()
        .expect("future-preserving grouped saved query should carry a basis family");
    let descriptor =
        saved_query_reuse_descriptor_for_saved(&saved).with_policy_basis_reuse_descriptor(
            exact_policy_basis_reuse_descriptor(saved.digest().as_str(), &basis_family),
        );

    let SavedQueryReuseOutcome::Admitted(decision) =
        evaluate_saved_query_reuse(&saved, &descriptor)
    else {
        panic!("grouped future-preserving reuse with explicit policy-basis evidence must admit");
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
fn grouped_future_preserving_saved_query_reuse_can_require_fresh_freeze_without_drift() {
    let saved = freeze_future_preserving_grouped_saved_query();
    let basis_family = saved
        .metadata()
        .basis_family()
        .cloned()
        .expect("future-preserving grouped saved query should carry a basis family");
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
        panic!("compatible grouped preserved reuse should require a fresh freeze");
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
fn grouped_future_preserving_saved_query_reuse_rejects_policy_basis_evidence_minted_for_ordinary_surface(
) {
    let saved = freeze_future_preserving_grouped_saved_query();
    let basis_family = saved
        .metadata()
        .basis_family()
        .cloned()
        .expect("future-preserving grouped saved query should carry a basis family");
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
        panic!("ordinary-surface policy evidence must not admit grouped preserved reuse");
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
