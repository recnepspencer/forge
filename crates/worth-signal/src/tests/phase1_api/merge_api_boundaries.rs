use crate::facade::*;

use super::source_corpus::{
    BRANCH_BASIS_RUNTIME_SOURCE, BRANCH_BASIS_SOURCE, BRANCH_FORK_SOURCE, FACADE_SOURCE,
    LIFECYCLE_SOURCE, MERGE_REQUEST_SOURCE, MERGE_RUNTIME_PLAN_SOURCE,
};

#[test]
fn runtime_builder_uses_expected_defaults() {
    let graph = SignalGraph::new();
    let runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();

    assert_eq!(
        runtime.checkpoint().policy().barrier_for(()),
        CheckpointBarrier::PerOperation
    );
    assert_eq!(
        *runtime.config().fallback_comparator(),
        VersionComparatorPolicy::Exact
    );
}

#[test]
fn merge_api_compile_fail_boundaries_hold() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/ui/runtime_builder_requires_complete_defaults.rs");
    cases.compile_fail("tests/ui/lowered_merge_plan_fields_are_private.rs");
    cases.compile_fail("tests/ui/branch_basis_readmission_authority_is_not_publicly_mintable.rs");
    cases.compile_fail("tests/ui/merge_strategy_witness_cannot_be_deserialized.rs");
}

#[test]
fn branch_basis_api_is_source_visible_through_branching_and_facade_surfaces() {
    assert!(
        BRANCH_BASIS_SOURCE.contains("SignalBranchBasisArtifact"),
        "phase-1 branch basis should define an explicit proof-bearing artifact surface"
    );
    assert!(
        BRANCH_BASIS_SOURCE.contains("UntrackedSnapshot"),
        "phase-1 branch basis should type untracked snapshot posture instead of silently minting basis from arbitrary packets"
    );
    assert!(
        BRANCH_BASIS_RUNTIME_SOURCE.contains("validate_branch_basis_artifact"),
        "phase-1 branch basis should expose typed validation instead of ambient active-branch assumptions"
    );
    assert!(
        FACADE_SOURCE.contains("SignalBranchBasisArtifact"),
        "facade runtime/history surfaces should expose the branch basis artifact family"
    );
    assert!(
        !FACADE_SOURCE.contains("readmit_signal_branch_basis_after_boundary"),
        "facade surfaces should not expose public branch-basis readmission helpers before the dedicated readmission phase exists"
    );
    assert!(
        !FACADE_SOURCE.contains("SignalBranchBasisReadmissionAuthority"),
        "facade surfaces should not export mintable branch-basis readmission authority"
    );
}

#[test]
fn branch_fork_api_is_source_visible_and_cost_honest() {
    assert!(
        BRANCH_FORK_SOURCE.contains("SignalBranchForkRequest"),
        "phase-2 forking should define an explicit fork request surface"
    );
    assert!(
        BRANCH_FORK_SOURCE.contains("SignalBranchForkReceipt"),
        "phase-2 forking should define an explicit fork receipt surface"
    );
    assert!(
        BRANCH_FORK_SOURCE.contains("fork_branch_with_snapshot"),
        "snapshot-seeded forking should cross an explicit snapshot-bearing execution surface instead of hiding bulk snapshot transport inside the request packet"
    );
    assert!(
        BRANCH_FORK_SOURCE.contains("SnapshotPayloadRequiredForFork"),
        "snapshot-seeded forking should fail typed when callers omit the required snapshot payload"
    );
    assert!(
        BRANCH_FORK_SOURCE.contains("SnapshotBasisMismatch"),
        "snapshot-seeded forking should type requested-vs-provided snapshot mismatch instead of collapsing it into a generic failure"
    );
    assert!(
        !BRANCH_FORK_SOURCE.contains("snapshot: SignalSnapshotV1"),
        "fork requests should not embed full snapshot payloads and conceal heavy artifact transport behind a lightweight-looking request type"
    );
    assert!(
        FACADE_SOURCE.contains("SignalBranchForkRequest")
            && FACADE_SOURCE.contains("SignalBranchForkReceipt"),
        "facade runtime surfaces should expose the explicit fork request and receipt families"
    );
    assert!(
        LIFECYCLE_SOURCE
            .contains("self.fork_branch(SignalBranchForkRequest::from_current_branch_head(name))"),
        "compatibility create_branch should lower through the explicit current-parent fork request path instead of retaining a parallel ambient fork semantic lane"
    );
}

#[test]
fn merge_request_api_is_source_visible_and_planner_proof_bearing() {
    assert!(
        MERGE_REQUEST_SOURCE.contains("pub enum BranchMergeRequestScope"),
        "phase-3 merge entry should define an explicit native request scope family"
    );
    assert!(
        MERGE_REQUEST_SOURCE.contains("SelectedNodes(Vec<NodeId>)")
            && MERGE_REQUEST_SOURCE
                .contains("SelectedAspects(Vec<SignalSelectedAspectRequestEntry>)"),
        "phase-3 merge request scope should keep selected-node and selected-aspect semantics distinct"
    );
    assert!(
        MERGE_REQUEST_SOURCE.contains("pub struct NormalizedBranchMergeRequest"),
        "phase-3 merge boundary should produce a proof-bearing normalized request wrapper before planning"
    );
    assert!(
        MERGE_REQUEST_SOURCE.contains("pub fn normalize(&self) -> Result<NormalizedBranchMergeRequest, BranchMergeRequestDenial>"),
        "phase-3 merge request surface should expose one explicit normalization lane"
    );
    assert!(
        MERGE_RUNTIME_PLAN_SOURCE.contains("request: &LoweredFoundationalMergeRequest"),
        "phase-4 merge planning should consume the lowered foundational request proof instead of a raw caller-local scope bag"
    );
    assert!(
        !MERGE_RUNTIME_PLAN_SOURCE.contains("normalized_scope.is_full_branch()"),
        "once phase-4 lowering lands, planner admission should not keep deciding scoped merge eligibility from the native-only scope dialect"
    );
    assert!(
        !MERGE_RUNTIME_PLAN_SOURCE
            .contains("expect(\"merge request normalization already validated above\")"),
        "merge planning should not re-normalize scope after the boundary has already established that proof"
    );
    assert!(
        FACADE_SOURCE.contains("NormalizedBranchMergeRequest")
            && FACADE_SOURCE.contains("BranchMergeRequestScopeFamily"),
        "facade surfaces should expose the explicit scoped merge request family and normalized proof type"
    );
}
