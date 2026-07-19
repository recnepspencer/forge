use crate::policy_basis::{
    classify_saved_query_policy_tenant_reuse, PolicyExecutionModeRequest,
    PolicyReuseEquivalenceContract, SavedQueryPolicyReuseDescriptor,
    SavedQueryPolicyReuseDisposition,
};
use crate::query_context::QueryContextFamily;
use crate::saved_query::SavedQueryTemporalAsyncSurfacePosture;

#[test]
fn future_preserving_policy_basis_reuse_requires_basis_family_execution_mode_alignment() {
    let descriptor = SavedQueryPolicyReuseDescriptor::new(
        "saved-a",
        "policy-a",
        "tenant-truth-a",
        "tenant-schema-a",
        "branch-a",
        PolicyExecutionModeRequest::CurrentRead,
        "policy-a",
        "tenant-truth-a",
        "tenant-schema-a",
        "branch-a",
        PolicyExecutionModeRequest::CurrentRead,
    )
    .with_temporal_async_surface(
        SavedQueryTemporalAsyncSurfacePosture::FuturePreservingRuntimeBacked,
        Some(QueryContextFamily::CurrentBranchHead),
        Some(QueryContextFamily::CurrentBranchHead),
    );

    assert_eq!(
        classify_saved_query_policy_tenant_reuse(&descriptor),
        SavedQueryPolicyReuseDisposition::LegalNoSemanticChange
    );
}

#[test]
fn future_preserving_policy_basis_reuse_admits_branch_and_historical_lanes() {
    let branch_descriptor = SavedQueryPolicyReuseDescriptor::new(
        "saved-a",
        "policy-a",
        "tenant-truth-a",
        "tenant-schema-a",
        "branch-a",
        PolicyExecutionModeRequest::BranchRead,
        "policy-a",
        "tenant-truth-a",
        "tenant-schema-a",
        "branch-a",
        PolicyExecutionModeRequest::BranchRead,
    )
    .with_temporal_async_surface(
        SavedQueryTemporalAsyncSurfacePosture::FuturePreservingRuntimeBacked,
        Some(QueryContextFamily::BranchHead),
        Some(QueryContextFamily::BranchHead),
    );
    let historical_descriptor = SavedQueryPolicyReuseDescriptor::new(
        "saved-a",
        "policy-a",
        "tenant-truth-a",
        "tenant-schema-a",
        "branch-a",
        PolicyExecutionModeRequest::HistoricalRead,
        "policy-a",
        "tenant-truth-a",
        "tenant-schema-a",
        "branch-a",
        PolicyExecutionModeRequest::HistoricalRead,
    )
    .with_temporal_async_surface(
        SavedQueryTemporalAsyncSurfacePosture::FuturePreservingRuntimeBacked,
        Some(QueryContextFamily::HistoricalSnapshot),
        Some(QueryContextFamily::HistoricalCommit),
    );

    assert_eq!(
        classify_saved_query_policy_tenant_reuse(&branch_descriptor),
        SavedQueryPolicyReuseDisposition::LegalNoSemanticChange
    );
    assert_eq!(
        classify_saved_query_policy_tenant_reuse(&historical_descriptor),
        SavedQueryPolicyReuseDisposition::LegalNoSemanticChange
    );
}

#[test]
fn future_preserving_policy_basis_reuse_rejects_historical_mode_drift() {
    let descriptor = SavedQueryPolicyReuseDescriptor::new(
        "saved-a",
        "policy-a",
        "tenant-truth-a",
        "tenant-schema-a",
        "branch-a",
        PolicyExecutionModeRequest::CurrentRead,
        "policy-a",
        "tenant-truth-a",
        "tenant-schema-a",
        "branch-a",
        PolicyExecutionModeRequest::HistoricalRead,
    )
    .with_temporal_async_surface(
        SavedQueryTemporalAsyncSurfacePosture::FuturePreservingRuntimeBacked,
        Some(QueryContextFamily::CurrentBranchHead),
        Some(QueryContextFamily::CurrentBranchHead),
    );

    assert_eq!(
        classify_saved_query_policy_tenant_reuse(&descriptor),
        SavedQueryPolicyReuseDisposition::IllegalSemanticDrift
    );
}

#[test]
fn future_preserving_policy_basis_reuse_rejects_missing_surface_metadata_even_when_basis_matches() {
    let descriptor = SavedQueryPolicyReuseDescriptor::new(
        "saved-a",
        "policy-a",
        "tenant-truth-a",
        "tenant-schema-a",
        "branch-a",
        PolicyExecutionModeRequest::CurrentRead,
        "policy-a",
        "tenant-truth-a",
        "tenant-schema-a",
        "branch-a",
        PolicyExecutionModeRequest::CurrentRead,
    )
    .with_temporal_async_surface(
        SavedQueryTemporalAsyncSurfacePosture::FuturePreservingRuntimeBacked,
        None,
        Some(QueryContextFamily::CurrentBranchHead),
    );

    assert_eq!(
        classify_saved_query_policy_tenant_reuse(&descriptor),
        SavedQueryPolicyReuseDisposition::IllegalSemanticDrift
    );
}

#[test]
fn visible_but_deferred_temporal_async_policy_reuse_stays_illegal_even_with_equivalence() {
    let descriptor = SavedQueryPolicyReuseDescriptor::new(
        "saved-a",
        "policy-a",
        "tenant-truth-a",
        "tenant-schema-a",
        "branch-a",
        PolicyExecutionModeRequest::CurrentRead,
        "policy-b",
        "tenant-truth-b",
        "tenant-schema-b",
        "branch-a",
        PolicyExecutionModeRequest::CurrentRead,
    )
    .with_equivalence(PolicyReuseEquivalenceContract::fresh_freeze_required())
    .with_temporal_async_surface(
        SavedQueryTemporalAsyncSurfacePosture::VisibleButDeferred,
        Some(QueryContextFamily::CurrentBranchHead),
        Some(QueryContextFamily::CurrentBranchHead),
    );

    assert_eq!(
        classify_saved_query_policy_tenant_reuse(&descriptor),
        SavedQueryPolicyReuseDisposition::IllegalSemanticDrift
    );
}
