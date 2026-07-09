use crate::data::telemetry::{
    CheckpointTelemetry, EvaluationTelemetry, ExecutionTelemetry, InvalidationTelemetry,
    PlannerTelemetry, StorageTelemetry, TemporalTelemetry, TransactionTelemetry,
};
use crate::easy::ReactiveGraph;
use crate::facade::*;
use crate::tests::support::*;

const HOT_APPLY_SOURCE: &str = include_str!("../logic/evaluation/engine/apply.rs");
const HOT_PREPARED_APPLY_SOURCE: &str =
    include_str!("../logic/evaluation/engine/prepared_apply.rs");
const HOT_SEMANTIC_FINALIZE_SOURCE: &str = include_str!("../logic/planner/semantic/mod.rs");
const HOT_EFFECT_SOURCE: &str = include_str!("../data/graph/runtime/effect.rs");
const HOT_SERIAL_BATCH_SOURCE: &str = include_str!("../logic/planner/apply/serial_batch.rs");
const HOT_STAGE_SOURCE: &str = include_str!("../logic/planner/apply/stage.rs");
const HOT_PLANNING_SOURCE: &str = include_str!("../logic/planner/planning/mod.rs");
const HOT_VALIDATION_SOURCE: &str = include_str!("../logic/planner/planning/validation.rs");
const HOT_PRECOMPUTE_SOURCE: &str = include_str!("../logic/planner/precompute/mod.rs");
const HOT_CONTEXT_SOURCE: &str = include_str!("../logic/context.rs");
const HOT_REUSE_CONTEXT_SOURCE: &str =
    include_str!("../logic/evaluation/reuse/context_resolution.rs");
const HOT_INVALIDATION_ROUTING_SOURCE: &str = include_str!("../logic/invalidation/routing.rs");
const HOT_INVALIDATION_SUBSCRIPTION_SOURCE: &str =
    include_str!("../logic/invalidation/subscription.rs");
const HOT_TRANSACTION_OBSERVATION_MUTATION_SOURCE: &str =
    include_str!("../logic/transaction/runtime/transaction/transaction_mutation.rs");
const HOT_EASY_OBSERVATION_SOURCE: &str = include_str!("../easy/observation.rs");
const HOT_RUNTIME_OBSERVATION_SOURCE: &str =
    include_str!("../logic/transaction/runtime/state/runtime_observation.rs");
const PROOF_SOURCE: &str = include_str!("../data/proof.rs");
const PLANNER_MODEL_SOURCE: &str = include_str!("../logic/planner/model/mod.rs");
const SEMANTIC_SOURCE: &str = include_str!("../logic/planner/semantic/mod.rs");
const WORKSPACE_SOURCE: &str = include_str!("../logic/planner/apply/workspace.rs");
const PATCH_BUFFER_SOURCE: &str = include_str!("../logic/transaction/patch_buffer.rs");
const MERGE_EXECUTE_SOURCE: &str =
    include_str!("../logic/transaction/runtime/state/merge/execute.rs");
const MERGE_FOUNDATIONAL_SCOPE_SOURCE: &str =
    include_str!("../logic/transaction/runtime/state/merge/foundational_scope.rs");
const MERGE_CANDIDATE_SCOPE_SOURCE: &str =
    include_str!("../logic/transaction/runtime/state/merge/candidate_scope.rs");
const MERGE_SCOPED_PROOF_SOURCE: &str =
    include_str!("../logic/transaction/runtime/state/merge/scoped_proof.rs");
const MERGE_COMPATIBILITY_WITNESS_SOURCE: &str =
    include_str!("../logic/transaction/runtime/state/merge/compatibility/witness.rs");
const MERGE_COMPATIBILITY_FACTS_SOURCE: &str =
    include_str!("../logic/transaction/runtime/state/merge/compatibility/facts.rs");
const MERGE_COMPATIBILITY_DENIAL_SOURCE: &str =
    include_str!("../logic/transaction/runtime/state/merge/compatibility/denial.rs");
const MERGE_COMPATIBILITY_READMISSION_SOURCE: &str =
    include_str!("../logic/transaction/runtime/state/merge/compatibility/readmission.rs");
const MERGE_INSPECTION_SUPPORT_WITNESS_SOURCE: &str =
    include_str!("../logic/transaction/runtime/state/inspection/support_witness.rs");
const MERGE_INSPECTION_SUPPORT_ROWS_SOURCE: &str =
    include_str!("../logic/transaction/runtime/state/inspection/support_rows.rs");
const MERGE_INSPECTION_ABSENCE_SOURCE: &str =
    include_str!("../logic/transaction/runtime/state/inspection/absence.rs");
const MERGE_STRATEGY_IDENTITY_SOURCE: &str =
    include_str!("../logic/transaction/runtime/state/merge/strategy_identity.rs");
const MERGE_STRATEGY_WITNESS_SOURCE: &str =
    include_str!("../logic/transaction/runtime/state/merge/strategy_witness.rs");
const MERGE_PLAN_SOURCE: &str = include_str!("../logic/transaction/runtime/state/merge/plan.rs");
const MERGE_REQUEST_SOURCE: &str =
    include_str!("../logic/transaction/runtime/state/merge/request.rs");
const GUIDED_SOURCE: &str = include_str!("../logic/transaction/runtime/state/guided.rs");
const MERGE_RUNTIME_SOURCE: &str =
    include_str!("../logic/transaction/runtime/state/branching/merge_runtime.rs");
const BRANCH_BASIS_SOURCE: &str =
    include_str!("../logic/transaction/runtime/state/branching/basis.rs");
const BRANCH_FORK_SOURCE: &str =
    include_str!("../logic/transaction/runtime/state/branching/fork.rs");
const BRANCHES_SOURCE: &str =
    include_str!("../logic/transaction/runtime/state/branching/branches.rs");
const RUNTIME_STATE_SOURCE: &str =
    include_str!("../logic/transaction/runtime/state/runtime_state.rs");
const SNAPSHOT_RESTORE_SOURCE: &str = include_str!("../data/graph/diagnostics_access/artifacts.rs");
const RUNTIME_SNAPSHOTTING_SOURCE: &str =
    include_str!("../logic/transaction/runtime/state/branching/snapshotting.rs");
const CHECKPOINT_IMAGE_SOURCE: &str = include_str!("../data/node/checkpoint_image.rs");
const STATE_SOURCE: &str = include_str!("../state/mod.rs");
const PERFORMANCE_SUPPORT_SOURCE: &str = include_str!("./performance_support.rs");
const PERFORMANCE_PROFILES_SOURCE: &str = include_str!("./performance_profiles.rs");
const PERFORMANCE_BASELINE_SOURCE: &str = include_str!("./performance_baseline.json");
const ENTRIES_SOURCE: &str = include_str!("../data/graph/storage/entries.rs");
const GRAPH_RUNTIME_SOURCE: &str = include_str!("../data/graph/runtime/graph.rs");
const SLOT_SOURCE: &str = include_str!("../data/graph/storage/slot.rs");
const DOT_SOURCE: &str = include_str!("../presentation/outputs/dot.rs");
const HARNESS_BRIDGE_SOURCE: &str = include_str!("../presentation/harness/bridge.rs");
const EXECUTION_FLOW_SOURCE: &str = include_str!("../diagnostics/runtime/execution_flow.rs");
const RECORDER_SOURCE: &str = include_str!("../diagnostics/runtime/recorder.rs");
const HISTORY_SOURCE: &str = include_str!("../diagnostics/inspection/history.rs");
const SUMMARY_SOURCE: &str = include_str!("../diagnostics/model/summary.rs");
const OBSERVER_SOURCE: &str = include_str!("../data/graph/runtime/observer.rs");
const FACADE_SOURCE: &str = include_str!("../facade.rs");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Domain {
    Cache,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Impact {
    One,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ev {
    Tick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Tier {
    Feature,
}

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
        BRANCH_BASIS_SOURCE.contains("validate_branch_basis_artifact"),
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
        include_str!("../logic/transaction/runtime/state/branching/lifecycle.rs")
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
        MERGE_RUNTIME_SOURCE.contains("request: &LoweredFoundationalMergeRequest"),
        "phase-4 merge planning should consume the lowered foundational request proof instead of a raw caller-local scope bag"
    );
    assert!(
        !MERGE_RUNTIME_SOURCE.contains("normalized_scope.is_full_branch()"),
        "once phase-4 lowering lands, planner admission should not keep deciding scoped merge eligibility from the native-only scope dialect"
    );
    assert!(
        !MERGE_RUNTIME_SOURCE.contains("expect(\"merge request normalization already validated above\")"),
        "merge planning should not re-normalize scope after the boundary has already established that proof"
    );
    assert!(
        FACADE_SOURCE.contains("NormalizedBranchMergeRequest")
            && FACADE_SOURCE.contains("BranchMergeRequestScopeFamily"),
        "facade surfaces should expose the explicit scoped merge request family and normalized proof type"
    );
}

#[test]
fn merge_foundational_lowering_api_is_source_visible_and_planner_gated() {
    assert!(
        MERGE_FOUNDATIONAL_SCOPE_SOURCE.contains("pub struct LoweredFoundationalMergeRequest"),
        "phase-4 should define a proof-bearing lowered foundational merge request wrapper"
    );
    assert!(
        MERGE_FOUNDATIONAL_SCOPE_SOURCE.contains("pub fn lower_to_foundational_scope"),
        "phase-4 should expose one explicit lowering lane from normalized signal requests into foundational scope"
    );
    assert!(
        MERGE_FOUNDATIONAL_SCOPE_SOURCE.contains("FoundationalMergeScope::selected_nodes")
            && MERGE_FOUNDATIONAL_SCOPE_SOURCE.contains("FoundationalMergeScope::selected_aspects"),
        "phase-4 lowering should use the shared foundational scope constructors instead of a signal-local parallel ontology"
    );
    assert!(
        MERGE_RUNTIME_SOURCE.contains("request: &LoweredFoundationalMergeRequest"),
        "merge planner and executor boundaries should consume the lowered foundational request proof"
    );
    assert!(
        MERGE_RUNTIME_SOURCE.contains("signal_scope_family_matches_foundational_family("),
        "phase-4 planner entry should verify foundational-family parity rather than silently trusting a parallel signal-local scope dialect"
    );
    assert!(
        MERGE_RUNTIME_SOURCE.contains("foundational_scope_lowering_count")
            && GUIDED_SOURCE.contains("lower_foundational_merge_request(&request)"),
        "phase-4 lowering should be measured at the runtime boundary and guided merge should route through that one counted lowering seam"
    );
}

#[test]
fn merge_candidate_scope_api_is_source_visible_and_planner_narrowing() {
    assert!(
        MERGE_CANDIDATE_SCOPE_SOURCE.contains("pub struct LoweredScopedMergeCandidateSet"),
        "phase-5 should define a proof-bearing scoped candidate artifact instead of narrowing candidates inline in the planner"
    );
    assert!(
        MERGE_CANDIDATE_SCOPE_SOURCE.contains("pub struct ScopedMergeCandidateBreadthSummary"),
        "phase-5 should preserve explicit requested/admitted/skipped/no-op/support breadth accounting"
    );
    assert!(
        MERGE_CANDIDATE_SCOPE_SOURCE.contains("pub fn lower(")
            && MERGE_CANDIDATE_SCOPE_SOURCE.contains("pub fn with_support_closure_nodes("),
        "phase-5 should expose one lowering seam for scoped candidates plus an explicit support-closure update path"
    );
    assert!(
        MERGE_RUNTIME_SOURCE.contains("LoweredScopedMergeCandidateSet::lower(")
            && MERGE_RUNTIME_SOURCE.contains("scoped_candidates.admitted_candidate_nodes().to_vec()"),
        "phase-5 planner should lower scoped candidates once and feed downstream planning from admitted candidates rather than raw source-journal breadth"
    );
    assert!(
        MERGE_PLAN_SOURCE.contains("scoped_candidates: LoweredScopedMergeCandidateSet")
            && MERGE_PLAN_SOURCE.contains("pub fn scoped_candidates(&self)"),
        "phase-5 merge plans should retain the scoped candidate proof artifact instead of dropping requested/skipped/support posture after planning"
    );
    assert!(
        MERGE_RUNTIME_SOURCE.contains("scoped_candidate_lowering_count")
            && MERGE_RUNTIME_SOURCE.contains("scoped_candidate_support_closure_breadth"),
        "phase-5 candidate lowering should be measured at the runtime boundary so breadth remains performance-visible"
    );
}

#[test]
fn merge_scoped_proof_api_is_source_visible_and_retained_across_plan_and_result() {
    assert!(
        MERGE_SCOPED_PROOF_SOURCE.contains("pub struct ScopedMergeProofPacket"),
        "phase-7 should define a dedicated retained scoped merge proof packet instead of leaking planner-local candidate structures across the proof boundary"
    );
    assert!(
        MERGE_SCOPED_PROOF_SOURCE.contains("from_request_and_candidates"),
        "phase-7 should derive retained scoped proof once from the admitted request and scoped candidate truth"
    );
    assert!(
        MERGE_PLAN_SOURCE.contains("scoped_merge_proof: ScopedMergeProofPacket")
            && MERGE_PLAN_SOURCE.contains("pub fn scoped_merge_proof(&self)"),
        "phase-7 plans should retain the scoped proof packet as a first-class artifact"
    );
    assert!(
        include_str!("../logic/transaction/runtime/state/merge/result.rs")
            .contains("pub scoped_merge_proof: ScopedMergeProofPacket"),
        "phase-7 results and execution summaries should retain the same scoped proof packet instead of degrading to counters-only merge summaries"
    );
    assert!(
        include_str!("../logic/transaction/runtime/state/merge/proof.rs")
            .contains("pub scoped_merge_proof: ScopedMergeProofPacket")
            && include_str!("../logic/transaction/runtime/state/merge/proof.rs")
                .contains("pub scoped_merge_proof: Option<ScopedMergeProofPacket>"),
        "phase-7 proof reports and replay proof inputs should carry the retained scoped proof packet directly instead of forcing replay to rediscover scope from ambient branch state"
    );
    assert!(
        include_str!("../diagnostics/model/replay.rs").contains("BranchMergeSummary")
            && include_str!("../diagnostics/model/replay.rs")
                .contains("as_scoped_merge_proof")
            && RECORDER_SOURCE.contains("scoped_merge_proof: summary.scoped_merge_proof.clone()"),
        "phase-7 retained branch-merge replay history should carry the scoped proof packet directly instead of collapsing it into message-only diagnostics"
    );
}

#[test]
fn merge_strategy_witness_api_is_source_visible_and_retained_across_plan_result_and_replay() {
    assert!(
        MERGE_STRATEGY_WITNESS_SOURCE.contains("pub struct SignalMergeStrategyWitness"),
        "phase-9 should define a dedicated retained merge strategy witness instead of leaving strategy identity smeared across plan and result fields"
    );
    assert!(
        MERGE_STRATEGY_IDENTITY_SOURCE.contains("pub struct SignalMergeStrategyIdentity")
            && MERGE_STRATEGY_IDENTITY_SOURCE
                .contains("pub struct SignalInvalidationStrategyIdentity")
            && MERGE_STRATEGY_IDENTITY_SOURCE
                .contains("pub struct SignalDeliveryStrategyIdentity"),
        "phase-9 should keep merge, invalidation, and delivery posture distinguishable inside the retained strategy witness"
    );
    assert!(
        MERGE_PLAN_SOURCE.contains("strategy_witness: SignalMergeStrategyWitness")
            && MERGE_PLAN_SOURCE.contains("pub fn strategy_witness(&self)"),
        "phase-9 plans should retain the strategy witness as first-class proof instead of rediscovering it from selected semantics"
    );
    assert!(
        include_str!("../logic/transaction/runtime/state/merge/result.rs")
            .contains("pub strategy_witness: SignalMergeStrategyWitness"),
        "phase-9 execution summaries and results should retain the same strategy witness"
    );
    assert!(
        include_str!("../logic/transaction/runtime/state/merge/proof.rs")
            .contains("pub strategy_witness: SignalMergeStrategyWitness")
            && include_str!("../logic/transaction/runtime/state/merge/proof.rs")
                .contains("pub strategy_witness: Option<SignalMergeStrategyWitness>"),
        "phase-9 proof reports and replay proof inputs should carry the retained strategy witness directly"
    );
    assert!(
        include_str!("../diagnostics/model/replay.rs").contains("as_strategy_witness")
            && RECORDER_SOURCE.contains("strategy_witness: summary.strategy_witness.clone()"),
        "phase-9 retained replay history should carry the strategy witness directly instead of collapsing strategy posture into message text"
    );
}

#[test]
fn merge_compatibility_witness_api_is_source_visible_and_readmission_prepared() {
    assert!(
        MERGE_COMPATIBILITY_WITNESS_SOURCE.contains("pub struct SignalMergeCompatibilityWitness")
            && MERGE_COMPATIBILITY_WITNESS_SOURCE
                .contains("pub struct SignalMergeCompatibilityBasis"),
        "phase-10 should define a retained compatibility witness and explicit compatibility basis instead of leaving consumers to rediscover compatibility from three separate retained artifacts"
    );
    assert!(
        MERGE_COMPATIBILITY_FACTS_SOURCE
            .contains("pub struct SignalMergeCompatibilityFactInventory")
            && MERGE_COMPATIBILITY_FACTS_SOURCE.contains("from_retained"),
        "phase-10 should project one lower-authority fact inventory directly from retained branch basis, scoped proof, and strategy witness"
    );
    assert!(
        MERGE_COMPATIBILITY_DENIAL_SOURCE.contains("pub enum SignalMergeCompatibilityDenial")
            && MERGE_COMPATIBILITY_DENIAL_SOURCE.contains("StaleBranchBasis")
            && MERGE_COMPATIBILITY_DENIAL_SOURCE.contains("ScopedMergeProofMismatch")
            && MERGE_COMPATIBILITY_DENIAL_SOURCE.contains("StrategyWitnessMismatch"),
        "phase-10 should preserve stale, missing, and mismatched compatibility posture as typed denial instead of collapsing it into generic replay or merge failure text"
    );
    assert!(
        MERGE_COMPATIBILITY_READMISSION_SOURCE
            .contains("pub fn merge_result_compatibility_artifact(")
            && MERGE_COMPATIBILITY_READMISSION_SOURCE
                .contains("pub fn replay_merge_compatibility_artifact(")
            && MERGE_COMPATIBILITY_READMISSION_SOURCE
                .contains("pub fn readmit_merge_compatibility_artifact("),
        "phase-10 should expose one retained compatibility seam across merge result, replay, and explicit authority-backed readmission"
    );
    assert!(
        !MERGE_COMPATIBILITY_READMISSION_SOURCE
            .contains("pub fn merge_compatibility_artifact_from_parts("),
        "phase-10 should not expose a public raw parts seam that can mint compatibility-looking artifacts from caller-assembled retained fragments"
    );
    assert!(
        MERGE_COMPATIBILITY_READMISSION_SOURCE.contains("validate_branch_basis_artifact(")
            && MERGE_COMPATIBILITY_READMISSION_SOURCE.contains("readmit_with_authority("),
        "phase-10 compatibility should be anchored to current branch-basis validation and explicit worth-proof readmission rather than helper-local folklore progression"
    );
    assert!(
        MERGE_COMPATIBILITY_READMISSION_SOURCE.contains("merge_compatibility_build_count")
            && MERGE_COMPATIBILITY_READMISSION_SOURCE
                .contains("merge_compatibility_readmission_denial_count"),
        "phase-10 compatibility and readmission preparation should be measured at the boundary so retained-proof preparation stays performance-visible"
    );
    assert!(
        include_str!("../logic/transaction/runtime/state/merge/result.rs")
            .contains("pub compatibility_witness: SignalMergeCompatibilityWitness")
            && include_str!("../diagnostics/model/replay.rs")
                .contains("as_compatibility_witness")
            && RECORDER_SOURCE
                .contains("compatibility_witness: summary.compatibility_witness.clone()"),
        "phase-10 compatibility witness should be retained on merge results and replay history instead of being rebuilt ad hoc from separate retained fragments"
    );
}

#[test]
fn merge_support_inspection_api_is_source_visible_and_support_ready() {
    assert!(
        MERGE_INSPECTION_SUPPORT_WITNESS_SOURCE
            .contains("pub struct SignalMergeSupportInspectionWitness"),
        "phase-11 should define one proof-bearing support inspection witness instead of leaving collaboration-critical posture split across ad hoc helpers"
    );
    assert!(
        MERGE_INSPECTION_SUPPORT_ROWS_SOURCE.contains("pub struct SignalBranchBasisInspectionRow")
            && MERGE_INSPECTION_SUPPORT_ROWS_SOURCE
                .contains("pub struct SignalScopedMergeInspectionRow")
            && MERGE_INSPECTION_SUPPORT_ROWS_SOURCE
                .contains("pub struct SignalStrategyInspectionRow")
            && MERGE_INSPECTION_SUPPORT_ROWS_SOURCE
                .contains("pub struct SignalCompatibilityInspectionRow"),
        "phase-11 should project separate summarized branch-basis, scope, strategy, and compatibility rows instead of flattening retained proof into one opaque support blob"
    );
    assert!(
        MERGE_INSPECTION_ABSENCE_SOURCE
            .contains("pub enum SignalMergeSupportInspectionAbsence"),
        "phase-11 inspection should type missing or incomplete retained proof instead of synthesizing fallback support rows"
    );
    assert!(
        MERGE_INSPECTION_SUPPORT_WITNESS_SOURCE.contains(
            "pub(crate) fn merge_support_inspection_from_retained_parts("
        ) && MERGE_INSPECTION_SUPPORT_WITNESS_SOURCE
            .contains("pub fn merge_result_support_inspection(")
            && MERGE_INSPECTION_SUPPORT_WITNESS_SOURCE
                .contains("pub fn merge_execution_summary_support_inspection(")
            && MERGE_INSPECTION_SUPPORT_WITNESS_SOURCE
            .contains("pub fn replay_merge_support_inspection("),
        "phase-11 should keep the raw retained-parts builder internal while exposing explicit result, execution-summary, and replay inspection lanes for real support consumption"
    );
    assert!(
        FACADE_SOURCE.contains("SignalMergeSupportInspectionWitness")
            && FACADE_SOURCE.contains("SignalMergeSupportReadinessPosture"),
        "facade runtime surfaces should expose the phase-11 support inspection witness and readiness summary"
    );
    assert!(
        MERGE_INSPECTION_SUPPORT_WITNESS_SOURCE.contains("compatibility_witness: Option<&SignalMergeCompatibilityWitness>")
            && MERGE_INSPECTION_SUPPORT_WITNESS_SOURCE.contains("ReplayDetailUnavailable"),
        "phase-11 support inspection should prefer retained compatibility witness when present and deny explicitly when replay does not retain merge support detail"
    );
}

#[test]
fn hot_apply_modules_do_not_use_broad_entry_accessors_for_reads() {
    for (name, source) in [
        ("apply", HOT_APPLY_SOURCE),
        ("prepared_apply", HOT_PREPARED_APPLY_SOURCE),
        ("semantic_finalize", HOT_SEMANTIC_FINALIZE_SOURCE),
        ("serial_batch", HOT_SERIAL_BATCH_SOURCE),
        ("planning", HOT_PLANNING_SOURCE),
        ("planning_validation", HOT_VALIDATION_SOURCE),
        ("precompute", HOT_PRECOMPUTE_SOURCE),
    ] {
        assert!(
            !source.contains("get_entry("),
            "{name} should use narrowed graph accessors instead of broad get_entry reads"
        );
        assert!(
            !source.contains("get_entry_mut("),
            "{name} should not require broad mutable entry access on the read-path seam"
        );
    }
}

#[test]
fn perf_harness_supports_hot_family_access_counter_budgets() {
    assert!(
        PERFORMANCE_SUPPORT_SOURCE.contains("access_counter_maxima"),
        "performance support should encode explicit access-counter budgets for hot families"
    );
    assert!(
        PERFORMANCE_SUPPORT_SOURCE
            .contains("for (counter, maximum) in contract.access_counter_maxima"),
        "performance support should certify access-counter maxima as part of perf-case enforcement"
    );
}

#[test]
fn hot_perf_families_forbid_broad_entry_access() {
    for suite in [
        "topology_rewiring_churn",
        "topology_rewiring_rotating_window",
        "chain_10k_bootstrap",
        "suppression_wide_fanout",
    ] {
        assert!(
            PERFORMANCE_PROFILES_SOURCE.contains(&format!("\"{suite}\"")),
            "{suite} perf family should remain source-visible in the cert profile file"
        );
    }
    assert!(
        PERFORMANCE_PROFILES_SOURCE.contains("ZERO_BROAD_ENTRY_ACCESS"),
        "perf profiles should define an explicit zero-broad-entry budget for narrowed hot families"
    );
    assert!(
        PERFORMANCE_PROFILES_SOURCE.contains("ZERO_BROAD_AND_ARTIFACT_ACCESS"),
        "perf profiles should define an explicit zero-broad-and-artifact budget for already-clean topology families"
    );
    assert!(
        PERFORMANCE_PROFILES_SOURCE.contains("hot_family_contract("),
        "hot perf families should use explicit hot-family contracts instead of generic perf contracts"
    );
    assert!(
        PERFORMANCE_PROFILES_SOURCE.contains("\"suppression_wide_fanout\"")
            && PERFORMANCE_PROFILES_SOURCE.contains("SignalRuntimePolicy::operational().with_history_limit(4)"),
        "suppression perf cert should run under explicit operational policy instead of paying development-mode diagnostic retention by default"
    );
}

#[test]
fn observability_perf_profiles_use_structural_only_certification() {
    assert!(
        PERFORMANCE_SUPPORT_SOURCE.contains("PerfTimingPolicy::StructuralOnly"),
        "performance support should expose a structural-only cert mode for rich observability workloads"
    );
    assert!(
        PERFORMANCE_SUPPORT_SOURCE
            .contains("if !matches!(contract.timing_policy, PerfTimingPolicy::StructuralOnly)"),
        "structural-only perf cases should skip timing-phase regression gates"
    );
    assert!(
        PERFORMANCE_PROFILES_SOURCE.contains("\"harness_observability_profile\"")
            && PERFORMANCE_PROFILES_SOURCE.contains("PerfTimingPolicy::StructuralOnly"),
        "observability perf profiles should certify structural/resource behavior without hard timing gating"
    );
}

#[test]
fn maybe_stale_validation_path_uses_narrowed_hot_accessors() {
    assert!(
        !HOT_VALIDATION_SOURCE.contains("get_entry("),
        "maybe-stale validation should not reintroduce broad entry reads"
    );
    assert!(
        !HOT_VALIDATION_SOURCE.contains("RuntimeArtifactState"),
        "maybe-stale validation should rely on hot artifact truth rather than broad runtime artifact state"
    );
    assert!(
        HOT_VALIDATION_SOURCE.contains("node_runtime_artifact_hot("),
        "maybe-stale validation should inspect changed scopes through the hot artifact lane"
    );
}

#[test]
fn hot_effect_runtime_path_avoids_broad_entry_reads() {
    assert!(
        !HOT_EFFECT_SOURCE.contains("get_entry("),
        "runtime effect hot path should not use broad get_entry reads"
    );
    assert!(
        !HOT_EFFECT_SOURCE.contains("node_runtime_artifact_state("),
        "runtime effect hot path should inspect partition scope changes through the hot artifact lane"
    );
    assert_eq!(
        HOT_EFFECT_SOURCE.matches("get_entry_mut(").count(),
        0,
        "runtime effect hot path should mutate through named graph transitions instead of broad mutable entry access"
    );
    assert!(
        HOT_EFFECT_SOURCE.contains("node_runtime_artifact_structural_state("),
        "runtime effect should derive previous lineage/hash/reuse truth through a narrowed graph accessor"
    );
    assert!(
        HOT_EFFECT_SOURCE.contains("apply_node_artifact_write_delta("),
        "runtime effect should publish runtime and retained artifact writes through a named graph operation"
    );
    assert!(
        HOT_EFFECT_SOURCE.contains("transition_node_clean("),
        "runtime effect suppression should clean nodes through a named graph transition"
    );
}

#[test]
fn invalidation_subscription_path_uses_narrowed_config_access() {
    assert!(
        !HOT_INVALIDATION_SUBSCRIPTION_SOURCE.contains("get_entry("),
        "subscription invalidation should not materialize broad node entries for partition policy checks"
    );
    assert!(
        HOT_INVALIDATION_SUBSCRIPTION_SOURCE.contains("node_eval_config("),
        "subscription invalidation should inspect partitioned-output policy through narrowed config access"
    );
}

#[test]
fn execution_context_and_reuse_paths_use_narrowed_graph_accessors() {
    for (name, source) in [
        ("context", HOT_CONTEXT_SOURCE),
        ("reuse_context", HOT_REUSE_CONTEXT_SOURCE),
    ] {
        assert!(
            !source.contains("get_entry("),
            "{name} should not rely on broad entry reads for execution-time version or config access"
        );
    }
    assert!(
        HOT_CONTEXT_SOURCE.contains("node_aspect_version("),
        "evaluation context should read aspect versions through the narrowed graph accessor"
    );
    assert!(
        HOT_CONTEXT_SOURCE.contains("node_partitioned_aspect_version("),
        "evaluation context should read partitioned versions through the narrowed graph accessor"
    );
    assert!(
        HOT_REUSE_CONTEXT_SOURCE.contains("node_eval_config("),
        "reuse boundary resolution should derive comparator/config truth through the named graph accessor"
    );
}

#[test]
fn invalidation_routing_uses_named_node_transitions() {
    assert!(
        !HOT_INVALIDATION_ROUTING_SOURCE.contains("get_entry("),
        "invalidation routing should not use broad entry reads"
    );
    assert!(
        !HOT_INVALIDATION_ROUTING_SOURCE.contains("get_entry_mut("),
        "invalidation routing should mutate node state through named graph transitions"
    );
    assert!(
        HOT_INVALIDATION_ROUTING_SOURCE.contains("transition_node_dirty("),
        "invalidation routing should use the named dirty transition"
    );
    assert!(
        HOT_INVALIDATION_ROUTING_SOURCE.contains("transition_node_maybe_stale("),
        "invalidation routing should use the named maybe-stale transition"
    );
}

#[test]
fn hot_stage_path_avoids_broad_entry_reads() {
    assert!(
        !HOT_STAGE_SOURCE.contains("get_entry("),
        "stage lowering should use narrowed graph accessors instead of broad get_entry reads"
    );
    assert!(
        !HOT_STAGE_SOURCE.contains("get_entry_mut("),
        "stage lowering should not require broad mutable entry access on the read-path seam"
    );
}

#[test]
fn gate3_finalize_paths_use_compact_artifact_images_instead_of_broad_runtime_state_snapshots() {
    for (name, source) in [
        ("semantic_finalize", HOT_SEMANTIC_FINALIZE_SOURCE),
        ("serial_batch", HOT_SERIAL_BATCH_SOURCE),
        ("stage", HOT_STAGE_SOURCE),
    ] {
        assert!(
            !source.contains("RuntimeArtifactState"),
            "{name} should not depend on broad RuntimeArtifactState in the finalize/apply carrier path"
        );
        assert!(
            !source.contains("node_runtime_artifact_state("),
            "{name} should not read broad runtime artifact state on the narrowed finalize/apply path"
        );
        assert!(
            source.contains("RuntimeArtifactFinalizeImage")
                || source.contains("node_runtime_artifact_finalize_image("),
            "{name} should consume the compact finalize image explicitly"
        );
    }
}

#[test]
fn gate4_stage_snapshot_commit_path_keeps_classified_snapshot_proofs() {
    assert!(
        HOT_STAGE_SOURCE.contains("apply_classified_snapshot_batch_commit("),
        "stage-owned snapshot publication should commit the already-classified proof form instead of reclassifying a generic batch late"
    );
    assert!(
        !HOT_STAGE_SOURCE.contains("apply_snapshot_batch_commit(stage_scratch.pending_snapshots)"),
        "stage-owned snapshot publication should not collapse back to generic snapshot batches once classification has occurred"
    );
}

#[test]
fn snapshot_proof_entries_are_not_publicly_forgeable() {
    assert!(
        PROOF_SOURCE.contains("pub struct PendingStableShapeSnapshotCommit {\r\n    node: NodeId,\r\n    update: VersionOnlySnapshotUpdate,\r\n    delta: SnapshotDeltaRecord,")
            || PROOF_SOURCE.contains("pub struct PendingStableShapeSnapshotCommit {\n    node: NodeId,\n    update: VersionOnlySnapshotUpdate,\n    delta: SnapshotDeltaRecord,"),
        "stable-shape snapshot proof entries should keep their fields private"
    );
    assert!(
        PROOF_SOURCE.contains("pub struct PendingReplacementSnapshotCommit {\r\n    node: NodeId,\r\n    update: ReplacementSnapshotUpdate,\r\n    delta: SnapshotDeltaRecord,")
            || PROOF_SOURCE.contains("pub struct PendingReplacementSnapshotCommit {\n    node: NodeId,\n    update: ReplacementSnapshotUpdate,\n    delta: SnapshotDeltaRecord,"),
        "replacement snapshot proof entries should keep their fields private"
    );
}

#[test]
fn lowered_execution_and_semantic_packets_use_constructors_instead_of_open_field_assembly() {
    assert!(
        PLANNER_MODEL_SOURCE.contains("impl LoweredTaskExecution"),
        "lowered execution should be mediated through an implementation boundary instead of remaining a raw field bag"
    );
    assert!(
        !HOT_STAGE_SOURCE.contains("LoweredTaskExecution {"),
        "stage lowering should construct lowered execution through its constructor rather than open field assembly"
    );
    assert!(
        HOT_STAGE_SOURCE.contains("LoweredTaskExecution::new("),
        "stage lowering should explicitly establish the lowered execution carrier through its constructor"
    );
    assert!(
        !HOT_STAGE_SOURCE.contains("SemanticTaskUpdate {"),
        "grouped apply reduction should construct semantic updates through a constructor rather than open field assembly"
    );
    assert!(
        SEMANTIC_SOURCE.contains("impl SemanticTaskUpdate"),
        "semantic update packets should be mediated through an implementation boundary"
    );
    assert!(
        HOT_SERIAL_BATCH_SOURCE.contains("ReadySerialFinalizeBatch::new("),
        "serial finalize readiness should be established through a constructor after width and snapshot checks"
    );
    assert!(
        !HOT_SERIAL_BATCH_SOURCE.contains("Ok(ReadySerialFinalizeBatch {"),
        "serial finalize readiness should not fall back to open struct assembly after proof checks"
    );
    assert!(
        WORKSPACE_SOURCE.contains("impl ConcurrentWorkerInput"),
        "parallel worker packets should be mediated through a construction boundary"
    );
    assert!(
        WORKSPACE_SOURCE.contains("impl ConcurrentApplyGroupInput"),
        "parallel grouped-input packets should be mediated through a construction boundary"
    );
    assert!(
        WORKSPACE_SOURCE.contains("impl GroupLocalTaskCommit"),
        "group-local commit packets should be mediated through a construction boundary"
    );
    assert!(
        WORKSPACE_SOURCE.contains("impl StageScratch"),
        "stage scratch should be mediated through owned transitions rather than open field access"
    );
    assert!(
        HOT_STAGE_SOURCE.contains("ConcurrentWorkerInput::new("),
        "parallel stage lowering should construct worker packets through their constructor"
    );
    assert!(
        HOT_STAGE_SOURCE.contains("ConcurrentApplyGroupInput::new("),
        "parallel stage lowering should construct grouped-input packets through their constructor"
    );
    assert!(
        HOT_STAGE_SOURCE.contains("GroupLocalTaskCommit::new("),
        "group-local apply packets should construct task commits through their constructor"
    );
    assert!(
        HOT_STAGE_SOURCE.contains("StageScratch::new("),
        "stage scratch should be constructed through its constructor on the grouped-apply path"
    );
    assert!(
        PLANNER_MODEL_SOURCE.contains("impl LoweredTask"),
        "lowered task packets should be mediated through an implementation boundary"
    );
    assert!(
        PLANNER_MODEL_SOURCE.contains("fn execution(&self) -> &LoweredTaskExecution"),
        "lowered task execution should be accessed through an accessor rather than a crate-visible field"
    );
    assert!(
        !PLANNER_MODEL_SOURCE.contains("pub(crate) execution: LoweredTaskExecution"),
        "lowered task should not expose its execution carrier as a crate-visible field"
    );
    assert!(
        HOT_STAGE_SOURCE.contains("LoweredTask::new("),
        "stage lowering should construct lowered tasks through their constructor"
    );
    assert!(
        !HOT_STAGE_SOURCE.contains("Ok(LoweredTask {"),
        "stage lowering should not fall back to open lowered-task assembly"
    );
    assert!(
        HOT_STAGE_SOURCE.contains("LoweredStagePlan::new("),
        "lowered stage plans should be constructed through their constructor"
    );
    assert!(
        !HOT_STAGE_SOURCE.contains("Ok(LoweredStagePlan {"),
        "stage lowering should not fall back to open lowered-stage assembly"
    );
}

#[test]
fn gate5_rollback_and_merge_paths_use_checkpoint_node_images_as_authority_boundary() {
    assert!(
        PATCH_BUFFER_SOURCE.contains("original: CheckpointNodeImage"),
        "transaction rollback patches should retain canonical checkpoint node images instead of raw NodeEntry clones"
    );
    assert!(
        PATCH_BUFFER_SOURCE.contains("node_checkpoint_image("),
        "transaction rollback should capture authority through the explicit checkpoint-image graph accessor"
    );
    assert!(
        PATCH_BUFFER_SOURCE.contains("replace_entry_from_checkpoint_image("),
        "transaction rollback should restore touched nodes through the checkpoint-image boundary"
    );
    assert!(
        !PATCH_BUFFER_SOURCE.contains("original: NodeEntry"),
        "transaction rollback should not keep raw NodeEntry snapshots as its authoritative rollback packet"
    );
    assert!(
        MERGE_EXECUTE_SOURCE.contains("node_checkpoint_image("),
        "merge adoption should request authority through the explicit checkpoint-image graph accessor"
    );
    assert!(
        MERGE_EXECUTE_SOURCE.contains("create_node_from_checkpoint_image("),
        "merge adoption should materialize introduced nodes through the checkpoint-image boundary"
    );
    assert!(
        MERGE_EXECUTE_SOURCE.contains("replace_node_from_checkpoint_image("),
        "merge adoption should route existing-target checkpoint replacement through the temporal-aware replacement boundary"
    );
    assert!(
        !MERGE_EXECUTE_SOURCE.contains("NodeEntry::from_checkpoint_image("),
        "merge adoption should not bounce checkpoint authority back through broad NodeEntry reconstruction"
    );
    assert!(
        MERGE_EXECUTE_SOURCE.contains("entry_image.set_eval_config("),
        "merge adoption should carry evaluation contract through the checkpoint image packet itself"
    );
    assert!(
        !MERGE_EXECUTE_SOURCE.contains("get_entry_mut("),
        "merge adoption should not fall back to broad mutable entry mutation after checkpoint-image materialization"
    );
    assert!(
        MERGE_RUNTIME_SOURCE.contains("replace_node_from_checkpoint_image("),
        "branch merge reconciliation should route existing targets through the branch-aware temporal replacement boundary"
    );
    assert!(
        MERGE_RUNTIME_SOURCE.contains("node_checkpoint_image("),
        "branch merge reconciliation should request checkpoint authority through the explicit graph checkpoint-image accessor"
    );
    assert!(
        !MERGE_RUNTIME_SOURCE.contains(".replace_entry(target_node, replacement)"),
        "branch merge reconciliation should not fall back to direct whole-entry replacement"
    );
    assert!(
        !MERGE_RUNTIME_SOURCE.contains("get_runtime_artifact_state()"),
        "branch merge planning should not read broad runtime artifact state when hot/warm lane projections are available"
    );
    assert!(
        MERGE_RUNTIME_SOURCE.contains("node_runtime_artifact_hot(")
            && MERGE_RUNTIME_SOURCE.contains("node_runtime_artifact_warm("),
        "branch merge planning should derive merge comparability from explicit hot and warm artifact lanes"
    );
}

#[test]
fn gate5_snapshot_restore_uses_classified_snapshot_commit_boundary() {
    assert!(
        SNAPSHOT_RESTORE_SOURCE.contains("checkpoint_image")
            && SNAPSHOT_RESTORE_SOURCE.contains("dependency_snapshot_batch")
            && SNAPSHOT_RESTORE_SOURCE.contains(".classify()"),
        "snapshot restore planning should retain the classified checkpoint-carried dependency snapshot rebuild batch rather than only a generic batch form"
    );
    assert!(
        SNAPSHOT_RESTORE_SOURCE.contains("apply_classified_snapshot_batch_commit("),
        "snapshot restore execution should rebuild dependency snapshot state through the classified snapshot commit boundary"
    );
    assert!(
        SNAPSHOT_RESTORE_SOURCE.contains("restore_plan.checkpoint_restore_batch().clone_inner()"),
        "snapshot restore rebuild should consume the already-classified restore-plan batch instead of reclassifying the checkpoint batch late"
    );
    assert!(
        !SNAPSHOT_RESTORE_SOURCE.contains("dependency_snapshot_batch\n                        .clone()\n                        .classify()")
            && !SNAPSHOT_RESTORE_SOURCE.contains("dependency_snapshot_batch\r\n                        .clone()\r\n                        .classify()"),
        "snapshot restore rebuild should not reclassify dependency snapshot batches during execution"
    );
    assert!(
        !SNAPSHOT_RESTORE_SOURCE.contains("apply_snapshot_batch_commit(\r\n                        snapshot.checkpoint_image.dependency_snapshot_batch.clone(),")
            && !SNAPSHOT_RESTORE_SOURCE.contains("apply_snapshot_batch_commit(\n                        snapshot.checkpoint_image.dependency_snapshot_batch.clone(),"),
        "snapshot restore execution should not fall back to the generic snapshot batch commit path"
    );
    assert!(
        RUNTIME_SNAPSHOTTING_SOURCE.contains("apply_classified_snapshot_batch_commit("),
        "runtime branch snapshot restore should rebuild dependency snapshot state through the classified snapshot commit boundary"
    );
    assert!(
        RUNTIME_SNAPSHOTTING_SOURCE.contains("restore_plan.checkpoint_restore_batch().clone_inner()"),
        "runtime branch snapshot restore should consume the already-classified restore-plan batch instead of reclassifying the checkpoint batch late"
    );
    assert!(
        !RUNTIME_SNAPSHOTTING_SOURCE.contains("dependency_snapshot_batch\n                        .clone()\n                        .classify()")
            && !RUNTIME_SNAPSHOTTING_SOURCE.contains("dependency_snapshot_batch\r\n                        .clone()\r\n                        .classify()"),
        "runtime branch snapshot restore should not reclassify dependency snapshot batches during execution"
    );
    assert!(
        !RUNTIME_SNAPSHOTTING_SOURCE.contains("apply_snapshot_batch_commit(\r\n                        snapshot.checkpoint_image.dependency_snapshot_batch.clone(),")
            && !RUNTIME_SNAPSHOTTING_SOURCE.contains("apply_snapshot_batch_commit(\n                        snapshot.checkpoint_image.dependency_snapshot_batch.clone(),"),
        "runtime branch snapshot restore should not fall back to the generic snapshot batch commit path"
    );
}

#[test]
fn checkpoint_authority_image_fields_are_sealed_behind_methods() {
    assert!(
        CHECKPOINT_IMAGE_SOURCE.contains("pub struct CheckpointNodeImage {\n    state: NodeState,")
            || CHECKPOINT_IMAGE_SOURCE
                .contains("pub struct CheckpointNodeImage {\r\n    state: NodeState,"),
        "checkpoint authority image should keep its storage fields private"
    );
    assert!(
        !CHECKPOINT_IMAGE_SOURCE.contains("pub state:"),
        "checkpoint authority image should not expose raw state fields"
    );
    assert!(
        !CHECKPOINT_IMAGE_SOURCE.contains("pub dependencies_id:")
            && !CHECKPOINT_IMAGE_SOURCE.contains("pub runtime_artifact_state:")
            && !CHECKPOINT_IMAGE_SOURCE.contains("pub retained_artifact:")
            && !CHECKPOINT_IMAGE_SOURCE.contains("pub causality:")
            && !CHECKPOINT_IMAGE_SOURCE.contains("pub eval_config:"),
        "checkpoint authority image should not expose forgeable public fields"
    );
    assert!(
        CHECKPOINT_IMAGE_SOURCE.contains("pub(crate) fn set_eval_config(")
            && CHECKPOINT_IMAGE_SOURCE.contains("pub(crate) fn set_runtime_artifact_state(")
            && CHECKPOINT_IMAGE_SOURCE
                .contains("pub(crate) fn clear_dependency_handles_for_adoption("),
        "checkpoint authority image mutation should be mediated through crate-scoped methods"
    );
}

#[test]
fn snapshot_restore_plan_separates_restore_proof_from_delta_accounting() {
    assert!(
        STATE_SOURCE.contains("pub struct CheckpointRestoreSnapshotBatch")
            && STATE_SOURCE.contains("classified: ClassifiedSnapshotBatchCommit"),
        "restore plan should name the classified checkpoint rebuild proof explicitly"
    );
    assert!(
        STATE_SOURCE.contains("pub struct RestoreDeltaAccounting")
            && STATE_SOURCE.contains("dependency_snapshot_delta_node_count: u64"),
        "restore plan should name delta accounting separately from the rebuild proof"
    );
    assert!(
        STATE_SOURCE.contains("checkpoint_restore_batch: CheckpointRestoreSnapshotBatch")
            && STATE_SOURCE.contains("delta_accounting: RestoreDeltaAccounting"),
        "snapshot restore plan should carry distinct proof and accounting fields"
    );
    assert!(
        STATE_SOURCE.contains("intent: SnapshotRestoreIntent")
            && STATE_SOURCE.contains("shared_node_count: u64")
            && STATE_SOURCE.contains("current_only_node_count: u64")
            && STATE_SOURCE.contains("snapshot_only_node_count: u64")
            && STATE_SOURCE.contains("coarse_replacement_required: bool")
            && STATE_SOURCE.contains("coarse_reasons: Vec<SnapshotRestoreCoarseReason>"),
        "snapshot restore plan should keep its restore-structure fields private"
    );
    assert!(
        STATE_SOURCE.contains("pub fn checkpoint_restore_batch(&self) -> &CheckpointRestoreSnapshotBatch")
            && STATE_SOURCE.contains("pub fn dependency_snapshot_delta_node_count(&self) -> u64")
            && STATE_SOURCE.contains("pub fn shared_node_count(&self) -> u64")
            && STATE_SOURCE.contains("pub fn current_only_node_count(&self) -> u64")
            && STATE_SOURCE.contains("pub fn snapshot_only_node_count(&self) -> u64")
            && STATE_SOURCE.contains("pub fn coarse_replacement_required(&self) -> bool")
            && STATE_SOURCE.contains("pub fn coarse_reasons(&self) -> &[SnapshotRestoreCoarseReason]"),
        "snapshot restore plan should expose restore proof and accounting only through explicit accessors"
    );
}

#[test]
fn merge_runtime_uses_sealed_projection_accessors_instead_of_rederiving_lane_state() {
    assert!(
        MERGE_RUNTIME_SOURCE.contains("struct NodeMergeProjection"),
        "merge runtime should define a single projection for merge-comparable state"
    );
    assert!(
        MERGE_RUNTIME_SOURCE.contains("fn node_merge_projection("),
        "merge runtime should centralize merge projection assembly behind one accessor"
    );
    assert!(
        !MERGE_RUNTIME_SOURCE.contains("node_merge_comparable(")
            && !MERGE_RUNTIME_SOURCE.contains("node_lineage_artifact_id(")
            && !MERGE_RUNTIME_SOURCE.contains("node_merge_authority("),
        "merge runtime should not fall back to separate comparable, lineage, and authority helpers"
    );
}

#[test]
fn merge_planning_packets_are_mediated_through_constructors_and_accessors() {
    assert!(
        MERGE_PLAN_SOURCE.contains("impl NodeMergeInputState")
            && MERGE_PLAN_SOURCE.contains("impl NodeMergePlan")
            && MERGE_PLAN_SOURCE.contains("impl LoweredMergePlan"),
        "merge planning packet families should be mediated through implementation boundaries"
    );
    assert!(
        !MERGE_RUNTIME_SOURCE.contains("NodeMergePlan {")
            && !MERGE_RUNTIME_SOURCE.contains("NodeMergeInputState {")
            && !MERGE_RUNTIME_SOURCE.contains("LoweredMergePlan {"),
        "merge runtime should not assemble merge planning packets by open struct literal"
    );
    assert!(
        MERGE_RUNTIME_SOURCE.contains("NodeMergePlan::new(")
            && MERGE_RUNTIME_SOURCE.contains("NodeMergeInputState::new(")
            && MERGE_RUNTIME_SOURCE.contains("LoweredMergePlan::new("),
        "merge runtime should construct merge planning packets through their constructors"
    );
}

#[test]
fn branch_snapshot_restore_packets_are_mediated_through_transition_helpers() {
    assert!(
        BRANCHES_SOURCE.contains("SnapshotBranchState")
            && BRANCHES_SOURCE.contains("into_branch_state("),
        "snapshot branch state should expose an explicit rehydration transition"
    );
    assert!(
        RUNTIME_SNAPSHOTTING_SOURCE.contains("snapshot_state.into_branch_state("),
        "branch snapshot restore should rebuild stored branch state through the snapshot transition helper"
    );
    assert!(
        !RUNTIME_SNAPSHOTTING_SOURCE.contains("let mut state = BranchState {")
            && !RUNTIME_SNAPSHOTTING_SOURCE.contains("let state = BranchState {"),
        "branch snapshot restore should not hand-assemble branch state by struct literal"
    );
    assert!(
        !RUNTIME_SNAPSHOTTING_SOURCE.contains("(snapshot, branch_catalog, state.clone())")
            && !RUNTIME_SNAPSHOTTING_SOURCE.contains("store_branch_state(branch.id, branch_state)")
            && !RUNTIME_SNAPSHOTTING_SOURCE.contains("store_branch_state(snapshot.meta.branch_id,")
            && !RUNTIME_SNAPSHOTTING_SOURCE.contains("insert_snapshot(snapshot.meta.snapshot_id,")
            && !MERGE_RUNTIME_SOURCE.contains("store_branch_state(request.target_branch.id,")
            && !MERGE_RUNTIME_SOURCE.contains("insert_snapshot(\n            merged_snapshot,")
            && !MERGE_RUNTIME_SOURCE.contains("insert_snapshot(\r\n            merged_snapshot,")
            && !RUNTIME_STATE_SOURCE.contains("store_branch_state(current.id,"),
        "inactive branch snapshot capture should not clone and re-store a full BranchState after mutating it in place"
    );
    assert!(
        RUNTIME_STATE_SOURCE.contains("AuthorityTransferPacket")
            && RUNTIME_STATE_SOURCE.contains("RestoreTransferPacket")
            && RUNTIME_STATE_SOURCE.contains("ExplicitBranchForkPacket")
            && RUNTIME_STATE_SOURCE
                .contains("pub fn new(branch_id: SignalBranchId, state: BranchState")
            && (RUNTIME_STATE_SOURCE
                .contains("pub fn new(\n        source_branch: SignalBranchId,")
                || RUNTIME_STATE_SOURCE
                    .contains("pub fn new(\r\n        source_branch: SignalBranchId,")),
        "branch lifecycle transfer packets should be mediated through implementation boundaries"
    );
    assert!(
        !RUNTIME_SNAPSHOTTING_SOURCE.contains("RestoreTransferPacket {")
            && !MERGE_RUNTIME_SOURCE.contains("AuthorityTransferPacket {")
            && !RUNTIME_SNAPSHOTTING_SOURCE.contains("AuthorityTransferPacket {")
            && !RUNTIME_STATE_SOURCE.contains("AuthorityTransferPacket { branch_id")
            && !RUNTIME_STATE_SOURCE.contains("RestoreTransferPacket { branch_id")
            && !BRANCHES_SOURCE.contains("pub authority:")
            && !BRANCHES_SOURCE.contains("pub derived:")
            && !BRANCHES_SOURCE.contains("pub ancestry:")
            && !BRANCHES_SOURCE.contains("pub mutation_ledger:")
            && !BRANCHES_SOURCE.contains("pub branch_id:")
            && !BRANCHES_SOURCE.contains("pub parent_branch_id:")
            && !BRANCHES_SOURCE.contains("pub forked_from_snapshot_id:")
            && !BRANCHES_SOURCE.contains("pub latest_merge_reference:")
            && BRANCHES_SOURCE.contains("pub(in crate::logic::transaction::runtime) struct SnapshotStatePacket")
            && BRANCHES_SOURCE.contains("pub fn packet(self, snapshot_id: SignalSnapshotId) -> SnapshotStatePacket"),
        "branch lifecycle transfer packets should not be assembled by open struct literal on runtime paths"
    );
}

#[test]
fn runtime_builder_supports_typed_runtime_configuration() {
    let graph = SignalGraph::new();
    let _ = Impact::One;
    let _ = Ev::Tick;
    let _ = Tier::Feature;
    let runtime = SignalRuntime::builder(graph)
        .with_kernel_defaults()
        .with_domains::<Domain>()
        .with_impacts::<Impact>()
        .with_events::<Ev>()
        .with_tiers::<Tier>()
        .checkpoint_barrier(CheckpointBarrier::PerOperation)
        .fallback_comparator(VersionComparatorPolicy::Exact)
        .build();

    assert_eq!(
        runtime.checkpoint().policy().barrier_for(Domain::Cache),
        CheckpointBarrier::PerOperation
    );
}

#[test]
fn transaction_helper_commits_on_success() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();

    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let outcome = runtime
        .transaction(&mut (), |transaction| {
            transaction.mark_dirty(source, ASPECT_A)?;
            Ok(())
        })
        .unwrap();

    assert_eq!(outcome.outcome, TransactionOutcome::Committed);
    assert_eq!(
        outcome.reconstructability.authority_branch_id,
        runtime.observe().current_branch().id
    );
    assert_eq!(
        outcome.reconstructability.authority_snapshot_id,
        runtime.observe().current_branch().head_snapshot_id
    );
    assert!(outcome.reconstructability.journal.replay_event_count >= 1);
    assert_eq!(
        outcome.reconstructability.checkpoint.journal_replay_span,
        outcome.reconstructability.journal.replay_event_count as u64
    );
    let metrics = runtime.observe().metrics();
    let graph_metrics = runtime.observe().graph().metrics();
    assert!(metrics.transaction.decision_log_event_count >= 1);
    assert!(graph_metrics.invalidation.batch_width >= 1);
    assert!(
        outcome
            .performance_accounting
            .transaction
            .decision_log_event_count
            >= 1
    );
    assert!(
        metrics.checkpoint.journal_replay_span
            >= outcome.reconstructability.journal.replay_event_count as u64
    );
    assert!(
        outcome
            .performance_accounting
            .checkpoint
            .journal_replay_span
            >= outcome.reconstructability.journal.replay_event_count as u64
    );
    assert_eq!(
        outcome.reconstructability.checkpoint.checkpoint_size,
        outcome.performance_accounting.checkpoint.checkpoint_size
    );
    let proof = outcome.reconstructability.proof();
    assert_eq!(
        proof.checkpoint.authority_branch_id,
        outcome.reconstructability.authority_branch_id
    );
    assert!(
        proof.required_rebuild.len() >= 2,
        "transaction proof should classify semantically required derived rebuild surfaces"
    );
    assert_eq!(
        runtime.graph().get_state(dependent).unwrap(),
        NodeState::Dirty
    );
}

#[test]
fn transaction_helper_rolls_back_on_error() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();
    let before = graph.get_state(dependent).unwrap();

    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let err = runtime
        .transaction(&mut (), |transaction| {
            transaction.mark_dirty(source, ASPECT_A)?;
            Err(SignalError::internal("fail the transaction"))
        })
        .unwrap_err();

    assert!(format!("{err}").contains("fail the transaction"));
    assert_eq!(runtime.graph().get_state(dependent).unwrap(), before);
}

#[test]
fn graph_node_builder_sets_accessible_configuration() {
    let mut graph = SignalGraph::new();
    let node = graph
        .node()
        .reads_aspects([ASPECT_A, ASPECT_B])
        .produces_aspects([ASPECT_B])
        .requires_context(ContextRequirement::DomainContext)
        .path_class(PathClass::Rich)
        .maintenance_mode(MaintenanceMode::RebuildAllowed)
        .artifact_policy(ArtifactPolicyClass::DevelopmentRetained)
        .on_demand()
        .tolerance(2)
        .build();

    let config = graph.get_entry(node).unwrap().get_eval_config().clone();
    assert_eq!(
        config.contract.semantics.reads,
        AspectMask::from([ASPECT_A, ASPECT_B])
    );
    assert_eq!(
        config.contract.semantics.produces,
        AspectMask::from([ASPECT_B])
    );
    assert_eq!(
        config.contract.semantics.required_context,
        ContextRequirement::DomainContext
    );
    assert_eq!(
        config.contract.projection.consumes,
        AspectMask::from([ASPECT_A, ASPECT_B])
    );
    assert_eq!(config.contract.execution.path_class, PathClass::Rich);
    assert_eq!(
        config.contract.execution.maintenance_mode,
        MaintenanceMode::RebuildAllowed
    );
    assert_eq!(
        config.contract.execution.artifact_policy,
        ArtifactPolicyClass::DevelopmentRetained
    );
    assert_eq!(
        config.contract.execution.equivalence,
        EquivalenceContract::for_comparator_override(&VersionComparatorPolicy::Tolerance {
            epsilon: 2,
        })
    );
    assert_eq!(
        config.contract.authority.policy,
        AuthorityPolicy::SpeculativeThenReconcile
    );
    assert_eq!(config.condition, EvaluationCondition::OnDemand);
    assert_eq!(
        config.comparator,
        Some(VersionComparatorPolicy::Tolerance { epsilon: 2 })
    );
}

#[test]
fn node_contract_uses_explicit_performance_defaults() {
    let contract = NodeContract::default();

    assert_eq!(
        contract.execution.equivalence,
        EquivalenceContract::default()
    );
    assert_eq!(contract.execution.path_class, PathClass::Operational);
    assert_eq!(
        contract.execution.maintenance_mode,
        MaintenanceMode::DensityAdaptive
    );
    assert_eq!(
        contract.execution.artifact_policy,
        ArtifactPolicyClass::OperationalMinimal
    );
    assert_eq!(
        contract.authority.policy,
        AuthorityPolicy::SpeculativeThenReconcile
    );
    assert_eq!(
        contract.reuse,
        NodeReuseContract {
            equivalence: ArtifactEquivalenceContract::strict(),
            retain_certification: true,
        }
    );
    assert_eq!(contract.projection.consumes, AspectMask::ALL);
}

#[test]
fn graph_node_builder_sets_reuse_contract_accessibly() {
    let mut graph = SignalGraph::new();
    let node = graph
        .node()
        .artifact_equivalence_contract(ArtifactEquivalenceContract {
            required_boundaries: vec![
                ArtifactSemanticBoundary::TopologyRegime,
                ArtifactSemanticBoundary::AuthorityLane,
            ],
            supported_strategies: vec![
                crate::data::reuse::ReuseStrategy::OutputSuppression,
                crate::data::reuse::ReuseStrategy::MemoizedArtifactReuse,
                crate::data::reuse::ReuseStrategy::SnapshotRestoreReuse,
                crate::data::reuse::ReuseStrategy::ReconciliationAdoption,
                crate::data::reuse::ReuseStrategy::CrossIdentityPersistentMatch,
                crate::data::reuse::ReuseStrategy::PartialArtifactSplicing,
            ],
            allows_snapshot_restore_reuse: true,
            allows_authority_reconciliation_reuse: false,
        })
        .retain_reuse_certification(false)
        .build();

    let config = graph.get_entry(node).unwrap().get_eval_config().clone();
    assert_eq!(
        config.contract.reuse.equivalence.required_boundaries,
        vec![
            ArtifactSemanticBoundary::TopologyRegime,
            ArtifactSemanticBoundary::AuthorityLane,
        ]
    );
    assert!(
        config
            .contract
            .reuse
            .equivalence
            .allows_snapshot_restore_reuse
    );
    assert!(
        !config
            .contract
            .reuse
            .equivalence
            .allows_authority_reconciliation_reuse
    );
    assert!(!config.contract.reuse.retain_certification);
}

#[test]
fn reuse_domain_types_are_publicly_reachable() {
    let basis = ReuseBasis::strategy(
        crate::data::reuse::ReuseStrategy::MemoizedArtifactReuse,
        ReuseSource::MemoizedArtifact,
        ReuseCrossing::None,
    );
    let record = ReuseCertificationRecord {
        strategy: crate::data::reuse::ReuseStrategy::SnapshotRestoreReuse,
        origin: crate::data::reuse::ReuseOrigin::SnapshotRestore,
        source: ReuseSource::SnapshotArtifact,
        crossing: ReuseCrossing::SnapshotRestore,
        proofs: vec![ReuseBoundaryProof {
            boundary: ArtifactSemanticBoundary::SnapshotLineage,
            satisfied: true,
        }],
    };

    assert_eq!(
        basis,
        ReuseBasis::strategy(
            crate::data::reuse::ReuseStrategy::MemoizedArtifactReuse,
            ReuseSource::MemoizedArtifact,
            ReuseCrossing::None,
        )
    );
    assert_eq!(record.proofs.len(), 1);
    assert_eq!(
        record.proofs[0].boundary,
        ArtifactSemanticBoundary::SnapshotLineage
    );
}

#[test]
fn runtime_policy_maps_into_s9_contract_and_strategy_defaults() {
    let operational = SignalRuntimePolicy::operational();
    let development = SignalRuntimePolicy::development();
    let forensic = SignalRuntimePolicy::forensic();

    assert_eq!(operational.default_path_class(), PathClass::Operational);
    assert_eq!(
        operational.default_artifact_policy_class(),
        ArtifactPolicyClass::OperationalMinimal
    );
    assert_eq!(
        operational.default_execution_strategy(),
        ResolvedExecutionStrategy::SparseIncremental
    );
    assert_eq!(
        operational.default_maintenance_strategy(),
        ResolvedMaintenanceStrategy::DensityAdaptive
    );
    assert_eq!(
        operational.default_authority_policy(),
        AuthorityPolicy::SpeculativeThenReconcile
    );

    assert_eq!(development.default_path_class(), PathClass::Rich);
    assert_eq!(
        development.default_artifact_policy_class(),
        ArtifactPolicyClass::DevelopmentRetained
    );
    assert_eq!(
        development.default_execution_strategy(),
        ResolvedExecutionStrategy::DenseStageBatched
    );
    assert_eq!(
        development.default_maintenance_strategy(),
        ResolvedMaintenanceStrategy::Incremental
    );
    assert_eq!(
        development.default_authority_policy(),
        AuthorityPolicy::SpeculativeThenReconcile
    );

    assert_eq!(forensic.default_path_class(), PathClass::Rich);
    assert_eq!(
        forensic.default_artifact_policy_class(),
        ArtifactPolicyClass::ForensicReconstructable
    );
    assert_eq!(
        forensic.default_execution_strategy(),
        ResolvedExecutionStrategy::DenseStageBatched
    );
    assert_eq!(
        forensic.default_maintenance_strategy(),
        ResolvedMaintenanceStrategy::Rebuild
    );
    assert_eq!(
        forensic.default_authority_policy(),
        AuthorityPolicy::SpeculativeThenReconcile
    );
}

#[test]
fn diagnostics_profile_reset_restores_stock_tier_policy() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(
        SignalRuntimePolicy::forensic()
            .with_history_limit(23)
            .with_detail_limit(41)
            .with_history_details(false),
    );

    graph.reset_runtime_policy_to_tier(DiagnosticsTier::Operational);

    assert_eq!(graph.runtime_policy(), SignalRuntimePolicy::operational());
}

#[test]
fn node_contract_and_runtime_policy_expose_s9_1_enforcement_surfaces() {
    let contract = NodeContract::reads([ASPECT_A])
        .with_equivalence(EquivalenceContract::for_comparator_override(
            &VersionComparatorPolicy::Exact,
        ))
        .with_path_class(PathClass::Rich)
        .with_maintenance_mode(MaintenanceMode::RebuildAllowed)
        .with_artifact_policy(ArtifactPolicyClass::DevelopmentRetained);
    let compile_time = contract.compile_time_performance_contract();
    let resolved = SignalRuntimePolicy::development().resolve_performance_policy();

    assert_eq!(PerformanceEnforcementLayer::CompileTime as u8, 0);
    assert_eq!(PerformanceEnforcementLayer::RuntimePolicy as u8, 1);
    assert_eq!(PerformanceEnforcementLayer::CounterTest as u8, 2);

    assert_eq!(compile_time.equivalence, contract.execution.equivalence);
    assert_eq!(compile_time.path_class, PathClass::Rich);
    assert_eq!(
        compile_time.maintenance_mode,
        MaintenanceMode::RebuildAllowed
    );
    assert_eq!(
        compile_time.artifact_policy,
        ArtifactPolicyClass::DevelopmentRetained
    );
    assert_eq!(
        compile_time.authority_policy,
        AuthorityPolicy::SpeculativeThenReconcile
    );

    assert_eq!(resolved.path_class, PathClass::Rich);
    assert_eq!(
        resolved.artifact_policy,
        ArtifactPolicyClass::DevelopmentRetained
    );
    assert_eq!(
        resolved.execution_strategy,
        ResolvedExecutionStrategy::DenseStageBatched
    );
    assert_eq!(
        resolved.maintenance_strategy,
        ResolvedMaintenanceStrategy::Incremental
    );
    assert_eq!(
        resolved.authority_policy,
        AuthorityPolicy::SpeculativeThenReconcile
    );
}

#[test]
fn runtime_telemetry_exposes_performance_counter_surface() {
    let telemetry = RuntimeTelemetry {
        evaluation: EvaluationTelemetry {
            evaluation_calls: 3,
            ..EvaluationTelemetry::default()
        },
        invalidation: InvalidationTelemetry {
            invalidation_nodes_visited: 5,
            ..InvalidationTelemetry::default()
        },
        transaction: TransactionTelemetry {
            transaction_commit_count: 2,
            ..TransactionTelemetry::default()
        },
        planner: PlannerTelemetry {
            stages_built: 7,
            ..PlannerTelemetry::default()
        },
        execution: ExecutionTelemetry {
            rewiring_apply_count: 11,
            ..ExecutionTelemetry::default()
        },
        storage: StorageTelemetry {
            graph_storage_snapshot_rewrites: 13,
            ..StorageTelemetry::default()
        },
        checkpoint: CheckpointTelemetry {
            checkpoint_flushes: 17,
            ..CheckpointTelemetry::default()
        },
        temporal: TemporalTelemetry {
            temporal_wake_count: 19,
            scheduled_frontier_width: 29,
            temporal_eligibility_lowering_count: 31,
            previous_value_reference_count: 23,
            ..TemporalTelemetry::default()
        },
        resource: ResourceTelemetry {
            resource_declaration_lowering_count: 37,
            resource_descriptor_count: 41,
            ..ResourceTelemetry::default()
        },
        host_computed: crate::data::telemetry::HostComputedTelemetry::default(),
    };
    let counters = telemetry.performance_counter_surface();

    assert_eq!(counters.evaluation.evaluation_calls, 3);
    assert_eq!(counters.invalidation.invalidation_nodes_visited, 5);
    assert_eq!(counters.transaction.transaction_commit_count, 2);
    assert_eq!(counters.planner.stages_built, 7);
    assert_eq!(counters.execution.rewiring_apply_count, 11);
    assert_eq!(counters.storage.graph_storage_snapshot_rewrites, 13);
    assert_eq!(counters.checkpoint.checkpoint_flushes, 17);
    assert_eq!(counters.temporal.temporal_wake_count, 19);
    assert_eq!(counters.temporal.scheduled_frontier_width, 29);
    assert_eq!(counters.temporal.temporal_eligibility_lowering_count, 31);
    assert_eq!(counters.temporal.previous_value_reference_count, 23);
    assert_eq!(counters.resource.resource_declaration_lowering_count, 37);
    assert_eq!(counters.resource.resource_descriptor_count, 41);
}

#[test]
fn performance_harness_emits_allocation_and_footprint_metrics() {
    assert!(
        PERFORMANCE_SUPPORT_SOURCE.contains("#[global_allocator]")
            && PERFORMANCE_SUPPORT_SOURCE.contains("StatsAlloc")
            && PERFORMANCE_SUPPORT_SOURCE.contains("INSTRUMENTED_SYSTEM"),
        "performance harness should provide a process-wide allocation instrumentation surface for certification runs"
    );
    assert!(
        PERFORMANCE_SUPPORT_SOURCE.contains("\"allocation_metrics\"")
            && PERFORMANCE_SUPPORT_SOURCE.contains("\"allocated_bytes\"")
            && PERFORMANCE_SUPPORT_SOURCE.contains("\"peak_live_bytes\"")
            && PERFORMANCE_SUPPORT_SOURCE.contains("\"access_counters\""),
        "performance harness should emit allocation, heap-footprint, and compatibility-access counters with each perf sample"
    );
    assert!(
        PERFORMANCE_SUPPORT_SOURCE.contains("PERF_ALLOC_LOCK")
            && PERFORMANCE_SUPPORT_SOURCE.contains("Region::new(GLOBAL_ALLOCATOR)")
            && PERFORMANCE_SUPPORT_SOURCE.contains("snapshot_allocation_stats(&region)")
            && PERFORMANCE_SUPPORT_SOURCE.contains("WORTH_SIGNAL_UPDATE_PERF_BASELINE")
            && PERFORMANCE_SUPPORT_SOURCE.contains("performance_baseline.json"),
        "allocation instrumentation should serialize perf measurements and persist a checked baseline/delta certification surface"
    );
}

#[test]
fn node_storage_is_physically_split_into_index_addressed_lanes() {
    assert!(
        GRAPH_RUNTIME_SOURCE.contains("pub(in crate::data::graph) hot: Vec<Option<NodeHotData>>")
            && GRAPH_RUNTIME_SOURCE.contains("pub(in crate::data::graph) warm: Vec<NodeWarmData>")
            && GRAPH_RUNTIME_SOURCE
                .contains("pub(in crate::data::graph) cold: Vec<Option<Box<NodeColdData>>>"),
        "node arena should store hot, warm, and cold node lanes explicitly"
    );
    assert!(
        !SLOT_SOURCE.contains("Option<NodeEntry>"),
        "slot metadata should no longer store whole NodeEntry payloads inline"
    );
    assert!(
        ENTRIES_SOURCE.contains("NodeEntry::from_storage_parts(")
            && ENTRIES_SOURCE.contains("entry.into_storage_parts()"),
        "broad NodeEntry access should now be compatibility assembly over split node lanes"
    );
}

#[test]
fn performance_profiles_are_baseline_gated_not_report_only() {
    assert!(
        PERFORMANCE_SUPPORT_SOURCE.contains("capture_and_certify_perf_samples")
            && PERFORMANCE_SUPPORT_SOURCE.contains("certify_against_baseline")
            && PERFORMANCE_SUPPORT_SOURCE.contains("performance_baseline.json"),
        "ignored performance profiles should certify against a committed baseline artifact"
    );
    assert!(
        PERFORMANCE_BASELINE_SOURCE.contains("\"version\"")
            && PERFORMANCE_BASELINE_SOURCE.contains("\"cases\""),
        "performance baseline artifact should be present in-repo for certification runs"
    );
}

#[test]
fn gate6_broad_entry_access_is_visibility_restricted_and_boundary_reads_are_explicit() {
    assert!(
        ENTRIES_SOURCE.contains("pub(crate) fn get_entry(")
            && ENTRIES_SOURCE.contains("pub(crate) fn get_entry_mut("),
        "broad entry accessors should be crate-visible compatibility seams, not public API"
    );
    assert!(
        !ENTRIES_SOURCE.contains("pub fn get_entry(")
            && !ENTRIES_SOURCE.contains("pub fn get_entry_mut("),
        "broad entry accessors should no longer be exported publicly"
    );
    assert!(
        DOT_SOURCE.contains("node_condition(")
            && HARNESS_BRIDGE_SOURCE.contains("node_eval_config(")
            && EXECUTION_FLOW_SOURCE.contains("node_lineage_artifact_id(")
            && RECORDER_SOURCE.contains("stamp_runtime_artifact_lineage_and_execution(")
            && HISTORY_SOURCE.contains("node_execution_trace_stamp(")
            && SUMMARY_SOURCE.contains("node_runtime_artifact_state_present("),
        "boundary modules should move onto explicit graph accessors instead of relying on public broad entry assembly"
    );
    assert!(
        !FACADE_SOURCE.contains("NodeEntry,"),
        "public facade types should not re-export broad NodeEntry compatibility storage"
    );
    assert!(
        !FACADE_SOURCE.contains("RuntimeArtifactState,"),
        "public facade types should not re-export broad RuntimeArtifactState compatibility state"
    );
    assert!(
        !OBSERVER_SOURCE.contains("pub fn runtime_artifact_state("),
        "graph observer should not expose broad runtime artifact compatibility state on the public API"
    );
}

#[test]
fn proof_bearing_form_families_exist_as_real_types() {
    fn assert_canonical<T: CanonicalForm>() {}
    fn assert_resolved<T: ResolvedForm>() {}
    fn assert_delta<T: DeltaForm>() {}
    fn assert_summary<T: SummaryForm>() {}

    assert_canonical::<CanonicalDependencies>();
    assert_canonical::<CanonicalChangedRegions>();
    assert_canonical::<DedupedNodeBatch>();
    assert_canonical::<DependencyBatchEdit>();
    assert_canonical::<PartitionScopeSet>();
    assert_canonical::<SortedSourceBatch>();
    assert_resolved::<ResolvedExecutionStrategy>();
    assert_resolved::<ResolvedMaintenanceStrategy>();
    assert_resolved::<ResolvedPerformancePolicy>();
    assert_delta::<DirtyBatch>();
    assert_delta::<DirtyDelta>();
    assert_delta::<StructuralDelta>();
    assert_delta::<PatchPlan>();
    assert_summary::<LocalityFootprint>();
    assert_summary::<NarrowedPropagationSet>();
    assert_summary::<FrontierWave>();
    assert_summary::<InvalidationFrontier>();
    assert_summary::<InvalidationSeedBatch>();
    assert_summary::<FrontierPlan>();
    assert_summary::<FrontierExecutionSummary>();
    assert_summary::<SemanticBatchCommit>();
    assert_summary::<TouchedScopeSummary>();
    assert_summary::<PendingSnapshotBatch>();
    assert_summary::<SnapshotBatchCommit>();
    assert_summary::<SubscriberRepairBatch>();
}

#[test]
fn single_consumer_preserves_one_way_packet_flow() {
    let packet = SingleConsumer::new(vec![1_u32, 2, 3]);

    assert_eq!(packet.as_ref(), &[1, 2, 3]);
    assert_eq!(packet.into_inner(), vec![1, 2, 3]);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OrderedTestItem(u32);

impl OrderedStreamItem for OrderedTestItem {
    type OrderKey = u32;

    fn order_key(&self) -> Self::OrderKey {
        self.0
    }
}

#[test]
fn mergeable_ordered_stream_merges_locally_ordered_shards_without_global_sort() {
    let left = LocallyOrderedShard::new(vec![OrderedTestItem(0), OrderedTestItem(2)]);
    let right = LocallyOrderedShard::new(vec![OrderedTestItem(1), OrderedTestItem(3)]);

    let merged = MergeableOrderedStream::new(vec![left, right])
        .try_into_vec()
        .unwrap();

    assert_eq!(
        merged,
        vec![
            OrderedTestItem(0),
            OrderedTestItem(1),
            OrderedTestItem(2),
            OrderedTestItem(3)
        ]
    );
}

#[test]
fn unordered_canonicalization_is_explicit_fallback_for_ordered_shards() {
    let shard = LocallyOrderedShard::canonicalize_unordered(vec![
        OrderedTestItem(3),
        OrderedTestItem(1),
        OrderedTestItem(2),
    ]);

    assert_eq!(
        shard.into_vec(),
        vec![OrderedTestItem(1), OrderedTestItem(2), OrderedTestItem(3)]
    );
}

#[test]
fn prepared_dependency_capture_recording_preserves_sorted_unique_order_without_resort() {
    let mut capture = crate::logic::prepared::PreparedDependencyCapture::new();
    let source_a = NodeId::new(9, 0);
    let source_b = NodeId::new(3, 1);

    capture.record(source_a, ASPECT_B, None);
    capture.record(source_b, ASPECT_A, None);
    capture.record(source_a, ASPECT_B, None);

    let capture = capture.into_sorted_unique();
    assert_eq!(capture.as_slice().len(), 2);
    assert!(capture.as_slice().windows(2).all(|pair| {
        (
            pair[0].source.index(),
            pair[0].source.generation(),
            pair[0].aspect.index(),
            pair[0].scope.as_ref(),
        ) < (
            pair[1].source.index(),
            pair[1].source.generation(),
            pair[1].aspect.index(),
            pair[1].scope.as_ref(),
        )
    }));
    assert_eq!(capture.as_slice()[0].source, source_b);
    assert_eq!(capture.as_slice()[1].source, source_a);
}

#[test]
fn proof_bearing_batches_and_summaries_canonicalize_their_inputs() {
    let node_a = NodeId::new(7, 1);
    let node_b = NodeId::new(3, 2);
    let changed_regions = CanonicalChangedRegions::new(vec![
        ChangedRegion {
            partition: "wing".into(),
            detail: Some("spar".into()),
        },
        ChangedRegion {
            partition: "wing".into(),
            detail: Some("spar".into()),
        },
        ChangedRegion {
            partition: "fuselage".into(),
            detail: None,
        },
    ]);
    let touched_nodes = DedupedNodeBatch::new([node_a, node_b, node_a]);
    let touched_sources = SortedSourceBatch::new([node_a, node_b, node_b]);
    let dirty_delta = DirtyDelta::new(AspectMask::from([ASPECT_A]), changed_regions, touched_nodes);
    let structural_delta = StructuralDelta::new(Some(dirty_delta.clone()), None);
    let patch_plan = PatchPlan::new(vec![node_a, node_b, node_a], structural_delta.clone());
    let touched_scope_summary = TouchedScopeSummary::new(
        vec![
            PartitionSubscription::partition_and_detail("wing", "spar"),
            PartitionSubscription::whole_partition("fuselage"),
            PartitionSubscription::partition_and_detail("wing", "spar"),
        ],
        vec![node_a, node_b, node_a],
        vec![node_a, node_b, node_b],
    );
    let snapshot_batch = PendingSnapshotBatch::from_pairs(vec![
        (node_a, crate::data::dependency::DependencySnapshot::empty()),
        (node_b, crate::data::dependency::DependencySnapshot::empty()),
        (node_a, crate::data::dependency::DependencySnapshot::empty()),
    ]);
    let subscriber_repairs = SubscriberRepairBatch::new(vec![
        SubscriberRepair {
            source: node_a,
            subscribers: DedupedNodeBatch::new([node_b, node_b]),
        },
        SubscriberRepair {
            source: node_b,
            subscribers: DedupedNodeBatch::new([node_a, node_a]),
        },
        SubscriberRepair {
            source: node_a,
            subscribers: DedupedNodeBatch::new([node_a, node_b]),
        },
    ]);
    let desired = DesiredState::new(AspectMask::from([ASPECT_A, ASPECT_B]));
    let dependency_batch = DependencyBatchEdit::from_pairs(vec![
        (
            node_a,
            CanonicalDependencies::new([DependencyEdge::new(node_b, ASPECT_A)]),
        ),
        (
            node_b,
            CanonicalDependencies::new([DependencyEdge::new(node_a, ASPECT_B)]),
        ),
    ]);
    let dirty_batch = DirtyBatch::new(vec![
        DirtyBatchEntry::new(node_a, ASPECT_A, vec![ChangedRegion::new("wing")]),
        DirtyBatchEntry::new(
            node_a,
            ASPECT_A,
            vec![ChangedRegion::new("wing"), ChangedRegion::new("fuselage")],
        ),
        DirtyBatchEntry::without_regions(node_b, ASPECT_B),
    ]);
    let semantic_batch_commit = SemanticBatchCommit::new(dirty_batch.clone());
    let locality = LocalityFootprint::new(
        vec![
            PartitionSubscription::partition_and_detail("wing", "spar"),
            PartitionSubscription::whole_partition("fuselage"),
            PartitionSubscription::partition_and_detail("wing", "spar"),
        ],
        vec![node_a, node_b, node_a],
        vec![node_a, node_b, node_b],
    );
    let snapshot_commit = SnapshotBatchCommit::from_pairs(vec![
        (node_a, crate::data::dependency::DependencySnapshot::empty()),
        (node_b, crate::data::dependency::DependencySnapshot::empty()),
        (node_a, crate::data::dependency::DependencySnapshot::empty()),
    ]);

    assert_eq!(dirty_delta.changed_regions.as_slice().len(), 2);
    assert_eq!(dirty_delta.touched_nodes.as_slice(), &[node_b, node_a]);
    assert!(!structural_delta.is_empty());
    assert!(!patch_plan.is_empty());
    assert_eq!(patch_plan.target_nodes.as_slice(), &[node_b, node_a]);
    assert_eq!(touched_sources.as_slice(), &[node_b, node_a]);
    assert_eq!(touched_scope_summary.seed_scopes.len(), 2);
    assert_eq!(
        touched_scope_summary.touched_nodes.as_slice(),
        &[node_b, node_a]
    );
    assert_eq!(
        touched_scope_summary.touched_sources.as_slice(),
        &[node_b, node_a]
    );
    assert_eq!(snapshot_batch.as_slice().len(), 2);
    assert_eq!(dependency_batch.as_slice().len(), 2);
    assert_eq!(dirty_batch.as_slice().len(), 2);
    assert_eq!(dirty_batch.changed_regions().as_slice().len(), 2);
    assert_eq!(dirty_batch.locality_footprint().partitions.len(), 2);
    assert_eq!(dirty_batch.touched_sources().as_slice(), &[node_b, node_a]);
    assert_eq!(locality.partitions.len(), 2);
    assert_eq!(locality.nodes.as_slice(), &[node_b, node_a]);
    assert_eq!(
        semantic_batch_commit.changed_aspects.bits(),
        AspectMask::from([ASPECT_A, ASPECT_B]).bits()
    );
    assert_eq!(semantic_batch_commit.locality.partitions.len(), 2);
    assert_eq!(snapshot_commit.target_nodes().as_slice(), &[node_b, node_a]);
    assert_eq!(subscriber_repairs.as_slice().len(), 2);
    assert_eq!(
        desired.value().bits(),
        AspectMask::from([ASPECT_A, ASPECT_B]).bits()
    );
}

#[test]
fn observer_exposes_runtime_and_retained_artifacts_separately() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::development());
    let node = graph.node().output_identity().build();
    let runtime_only = graph.node().build();

    let mut compute = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(7, 0))
            .with_output_identity("wing-surface")
            .with_continuity_token("wing-lineage")
            .with_label("forensic"))
    };
    evaluate(&mut graph, node, &mut compute).unwrap();
    let mut runtime_only_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(8, 0));
    evaluate(&mut graph, runtime_only, &mut runtime_only_compute).unwrap();
    graph
        .get_entry_mut(node)
        .unwrap()
        .set_causality(Some(CausalityMetadata {
            kind: "host_patch".to_string(),
            fields: std::collections::BTreeMap::from([(
                "patch_id".to_string(),
                "wing-42".to_string(),
            )]),
        }));

    let observer = graph.observe();
    let runtime = observer.runtime_artifact_state(node).unwrap().unwrap();
    let retained = observer
        .retained_diagnostic_artifact(node)
        .unwrap()
        .unwrap();
    let materializer = observer.materialize();
    let historical = materializer
        .materialize_historical_artifact_record(node)
        .unwrap()
        .unwrap();
    let trace = materializer
        .materialize_trace_summary(node)
        .unwrap()
        .unwrap();

    assert_eq!(
        runtime.output_identity().map(|id| id.as_str()),
        Some("wing-surface")
    );
    assert_eq!(
        runtime.continuity_token().map(|token| token.as_str()),
        Some("wing-lineage")
    );
    assert_eq!(
        runtime.memoized_origin(),
        MemoizedResultOrigin::DirectCompute
    );
    assert_eq!(
        runtime.reuse_basis().clone_inner(),
        ReuseBasis::fresh_compute()
    );
    assert_eq!(retained.labels, vec!["forensic".to_owned()]);
    assert_eq!(historical.node, node);
    assert_eq!(
        historical.runtime.output_identity().cloned(),
        runtime.output_identity().cloned()
    );
    assert_eq!(
        historical.runtime.reuse_basis().clone_inner(),
        runtime.reuse_basis().clone_inner()
    );
    assert_eq!(
        historical.retained.as_ref().unwrap().labels,
        retained.labels
    );
    assert_eq!(trace.reuse_basis, runtime.reuse_basis().clone_inner());
    assert_eq!(
        historical
            .causality
            .as_ref()
            .and_then(|causality| causality.fields.get("patch_id"))
            .map(|value| value.as_str()),
        Some("wing-42")
    );
    assert_eq!(trace.labels, vec!["forensic".to_owned()]);
    assert_eq!(
        trace.output_identity.as_ref().map(|id| id.as_str()),
        Some("wing-surface")
    );

    let runtime_only_state = observer
        .runtime_artifact_state(runtime_only)
        .unwrap()
        .unwrap();
    assert!(
        observer
            .retained_diagnostic_artifact(runtime_only)
            .unwrap()
            .is_none(),
        "runtime-only artifacts must not require retained richness"
    );
    let runtime_only_historical = materializer
        .materialize_historical_artifact_record(runtime_only)
        .unwrap()
        .unwrap();
    assert!(
        runtime_only_historical.retained.is_none(),
        "cold historical assembly should remain available without retained payload"
    );
    let runtime_only_trace = materializer
        .materialize_trace_summary(runtime_only)
        .unwrap()
        .unwrap();
    assert_eq!(
        runtime_only_trace.output_hash,
        runtime_only_state.output_hash(),
        "cold trace assembly should derive from runtime truth even when retained richness is absent"
    );
}

#[test]
fn dependency_snapshot_clone_shares_backing_storage() {
    let mut snapshot = crate::data::dependency::DependencySnapshot::empty();
    snapshot.record(NodeId::new(1, 0), ASPECT_A, 7, None);
    snapshot.record(NodeId::new(2, 0), ASPECT_B, 11, None);

    let cloned = snapshot.clone();

    assert!(std::sync::Arc::ptr_eq(
        &snapshot.shared_entries(),
        &cloned.shared_entries()
    ));
    assert_eq!(snapshot.entries(), cloned.entries());
}

#[test]
fn replacing_dependency_snapshot_reports_delta() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let source = graph.node().build();

    let mut baseline = crate::data::dependency::DependencySnapshot::empty();
    baseline.record(source, ASPECT_A, 3, None);
    graph.set_dep_snapshot(node, baseline).unwrap();

    let mut updated = crate::data::dependency::DependencySnapshot::empty();
    updated.record(source, ASPECT_A, 5, None);
    updated.record(source, ASPECT_B, 7, None);
    let mut shape_store = crate::data::dependency::DependencySnapshotShapeStore::default();

    let delta = graph
        .replace_dep_snapshot_committed(
            node,
            crate::data::dependency::CommittedSnapshotUpdate::Replace(
                crate::data::dependency::ReplacementSnapshotUpdate::from_snapshot(
                    updated,
                    &mut shape_store,
                ),
            ),
        )
        .unwrap();

    assert_eq!(delta.node, node);
    assert_eq!(delta.previous_entry_count, 1);
    assert_eq!(delta.next_entry_count, 2);
    assert_eq!(delta.changed_entry_count, 2);
    assert!(delta.changed());
}

#[test]
fn replacing_identical_dependency_snapshot_is_a_noop() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let source = graph.node().build();

    let mut baseline = crate::data::dependency::DependencySnapshot::empty();
    baseline.record(source, ASPECT_A, 3, None);
    graph.set_dep_snapshot(node, baseline.clone()).unwrap();
    let first_id = graph.get_entry(node).unwrap().get_dep_snapshot_id();
    let mut shape_store = crate::data::dependency::DependencySnapshotShapeStore::default();

    let delta = graph
        .replace_dep_snapshot_committed(
            node,
            crate::data::dependency::CommittedSnapshotUpdate::Replace(
                crate::data::dependency::ReplacementSnapshotUpdate::from_snapshot(
                    baseline,
                    &mut shape_store,
                ),
            ),
        )
        .unwrap();
    let second_id = graph.get_entry(node).unwrap().get_dep_snapshot_id();

    assert_eq!(first_id, second_id);
    assert_eq!(delta.changed_entry_count, 0);
    assert!(!delta.changed());
}

#[test]
fn dependency_snapshot_version_only_update_preserves_shape() {
    let source_a = NodeId::new(1, 0);
    let source_b = NodeId::new(2, 0);
    let mut baseline = crate::data::dependency::DependencySnapshot::empty();
    baseline.record(source_a, ASPECT_A, 3, None);
    baseline.record(source_b, ASPECT_B, 7, None);

    let updated = baseline.with_updated_versions(&[5, 7]);
    let delta = crate::data::dependency::SnapshotDeltaRecord::between(
        NodeId::new(9, 0),
        &baseline,
        &crate::data::dependency::SharedDependencySnapshot::new(updated.clone()),
    );

    assert_eq!(baseline.entries().len(), updated.entries().len());
    assert_eq!(
        baseline
            .entries()
            .iter()
            .map(|entry| entry.sort_key())
            .collect::<Vec<_>>(),
        updated
            .entries()
            .iter()
            .map(|entry| entry.sort_key())
            .collect::<Vec<_>>()
    );
    assert_eq!(updated.entries()[0].cached_version, 5);
    assert_eq!(updated.entries()[1].cached_version, 7);
    assert_eq!(delta.changed_entry_count, 1);
    assert!(delta.changed());
}

#[test]
fn shared_dependency_snapshot_reports_storage_sharing_without_implying_semantics() {
    let source = NodeId::new(1, 0);
    let mut baseline = crate::data::dependency::DependencySnapshot::empty();
    baseline.record(source, ASPECT_A, 3, None);

    let shared_left = crate::data::dependency::SharedDependencySnapshot::new(baseline.clone());
    let shared_right = crate::data::dependency::SharedDependencySnapshot::new(baseline.clone());

    assert!(
        baseline.shares_storage_with(shared_left.snapshot()),
        "shared snapshot wrapping should preserve shared backing"
    );
    assert!(
        shared_left.shares_storage_with(&shared_right),
        "cloned snapshots should report shared backing explicitly"
    );

    let mut shape_store = crate::data::dependency::DependencySnapshotShapeStore::default();
    let replace = crate::data::dependency::CommittedSnapshotUpdate::Replace(
        crate::data::dependency::ReplacementSnapshotUpdate::from_snapshot(
            shared_left.into_snapshot(),
            &mut shape_store,
        ),
    );
    let basis = crate::data::dependency::StableShapeSnapshotBasis::prove(
        &crate::data::dependency::DependencyInputScan::stable_shape(
            NodeId::new(0, 0),
            crate::data::dependency::DependencySnapshotId::EMPTY,
            1,
            1,
            vec![5],
        ),
        baseline.shape().intern(&mut shape_store),
    )
    .expect("stable shape proof should exist");
    let version_only = crate::data::dependency::CommittedSnapshotUpdate::VersionOnly(
        crate::data::dependency::VersionOnlySnapshotUpdate::from_basis_and_versions(
            basis.clone(),
            crate::data::dependency::VersionVector::from_scan(
                &basis,
                &crate::data::dependency::DependencyInputScan::stable_shape(
                    NodeId::new(0, 0),
                    crate::data::dependency::DependencySnapshotId::EMPTY,
                    1,
                    1,
                    vec![5],
                ),
            ),
        ),
    );

    assert_eq!(
        replace.storage_strategy(),
        crate::data::dependency::SnapshotStorageStrategy::SharedReplacement
    );
    assert_eq!(
        version_only.storage_strategy(),
        crate::data::dependency::SnapshotStorageStrategy::VersionOnlyDelta
    );
}

#[test]
fn snapshot_storage_telemetry_distinguishes_replacement_from_version_only_delta() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let source = graph.node().build();

    let mut baseline = crate::data::dependency::DependencySnapshot::empty();
    baseline.record(source, ASPECT_A, 3, None);
    graph.set_dep_snapshot(node, baseline.clone()).unwrap();

    let mut replaced = crate::data::dependency::DependencySnapshot::empty();
    replaced.record(source, ASPECT_A, 5, None);
    replaced.record(source, ASPECT_B, 7, None);
    graph
        .replace_dep_snapshot_committed(
            node,
            crate::data::dependency::CommittedSnapshotUpdate::Replace(
                crate::data::dependency::ReplacementSnapshotUpdate::from_snapshot(
                    replaced,
                    &mut crate::data::dependency::DependencySnapshotShapeStore::default(),
                ),
            ),
        )
        .unwrap();

    let mut proof_shape_store = crate::data::dependency::DependencySnapshotShapeStore::default();
    let current_snapshot = graph.get_dep_snapshot(node).unwrap().clone();
    let basis = crate::data::dependency::StableShapeSnapshotBasis::prove(
        &crate::data::dependency::DependencyInputScan::stable_shape(
            node,
            graph.get_entry(node).unwrap().get_dep_snapshot_id(),
            current_snapshot.entries().len(),
            current_snapshot.entries().len(),
            vec![11, 13],
        ),
        current_snapshot.shape().intern(&mut proof_shape_store),
    )
    .expect("stable shape proof should exist for version-only update");
    graph
        .replace_dep_snapshot_committed(
            node,
            crate::data::dependency::CommittedSnapshotUpdate::VersionOnly(
                crate::data::dependency::VersionOnlySnapshotUpdate::from_basis_and_versions(
                    basis.clone(),
                    crate::data::dependency::VersionVector::from_scan(
                        &basis,
                        &crate::data::dependency::DependencyInputScan::stable_shape(
                            node,
                            graph.get_entry(node).unwrap().get_dep_snapshot_id(),
                            current_snapshot.entries().len(),
                            current_snapshot.entries().len(),
                            vec![11, 13],
                        ),
                    ),
                ),
            ),
        )
        .unwrap();

    let storage = graph.observe().metrics().storage;
    assert!(
        storage.shared_snapshot_replacement_count >= 2,
        "snapshot telemetry should count full shared replacement boundaries"
    );
    assert!(
        storage.version_only_snapshot_update_count >= 1,
        "snapshot telemetry should count version-only delta boundaries separately"
    );
}

#[test]
fn version_only_commit_preserves_stable_shape_change_kind() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let source = graph.node().build();

    let mut baseline = crate::data::dependency::DependencySnapshot::empty();
    baseline.record(source, ASPECT_A, 3, None);
    baseline.record(source, ASPECT_B, 7, None);
    graph.set_dep_snapshot(node, baseline.clone()).unwrap();

    let mut proof_shape_store = crate::data::dependency::DependencySnapshotShapeStore::default();
    let current_snapshot = graph.get_dep_snapshot(node).unwrap().clone();
    let next_versions = vec![11, 13];
    let basis = crate::data::dependency::StableShapeSnapshotBasis::prove(
        &crate::data::dependency::DependencyInputScan::stable_shape(
            node,
            graph.get_entry(node).unwrap().get_dep_snapshot_id(),
            current_snapshot.entries().len(),
            current_snapshot.entries().len(),
            next_versions.clone(),
        ),
        current_snapshot.shape().intern(&mut proof_shape_store),
    )
    .expect("stable shape proof should exist for version-only update");

    let delta = graph
        .replace_dep_snapshot_committed(
            node,
            crate::data::dependency::CommittedSnapshotUpdate::VersionOnly(
                crate::data::dependency::VersionOnlySnapshotUpdate::from_basis_and_versions(
                    basis.clone(),
                    crate::data::dependency::VersionVector::from_scan(
                        &basis,
                        &crate::data::dependency::DependencyInputScan::stable_shape(
                            node,
                            graph.get_entry(node).unwrap().get_dep_snapshot_id(),
                            current_snapshot.entries().len(),
                            current_snapshot.entries().len(),
                            next_versions,
                        ),
                    ),
                ),
            ),
        )
        .unwrap();

    assert_eq!(
        delta.change_kind,
        crate::data::dependency::SnapshotChangeKind::StableShapeVersionOnly
    );
}

#[test]
fn set_dep_snapshot_uses_version_only_delta_when_snapshot_shape_is_stable() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let source_a = graph.node().build();
    let source_b = graph.node().build();

    let mut baseline = crate::data::dependency::DependencySnapshot::empty();
    baseline.record(source_a, ASPECT_A, 3, None);
    baseline.record(source_b, ASPECT_B, 7, None);
    graph.set_dep_snapshot(node, baseline).unwrap();

    let mut version_only = crate::data::dependency::DependencySnapshot::empty();
    version_only.record(source_a, ASPECT_A, 5, None);
    version_only.record(source_b, ASPECT_B, 11, None);
    graph.set_dep_snapshot(node, version_only).unwrap();

    let storage = graph.observe().metrics().storage;
    assert_eq!(
        storage.shared_snapshot_replacement_count, 1,
        "initial snapshot install should be the only full replacement when shape stays stable"
    );
    assert_eq!(
        storage.version_only_snapshot_update_count, 1,
        "stable-shape snapshot rewrite should narrow to a version-only delta"
    );
}

#[test]
fn derive_dependency_snapshot_restore_batch_uses_version_only_delta_for_shared_shape() {
    let mut current = SignalGraph::new();
    let source_a = current.node().build();
    let source_b = current.node().build();
    let target = current.node().build();

    let mut baseline = crate::data::dependency::DependencySnapshot::empty();
    baseline.record(source_a, ASPECT_A, 3, None);
    baseline.record(source_b, ASPECT_B, 7, None);
    current.set_dep_snapshot(target, baseline).unwrap();

    let mut restored = current.clone();
    let mut updated = crate::data::dependency::DependencySnapshot::empty();
    updated.record(source_a, ASPECT_A, 5, None);
    updated.record(source_b, ASPECT_B, 11, None);
    restored.set_dep_snapshot(target, updated).unwrap();

    let batch = current
        .derive_dependency_snapshot_restore_batch(&restored)
        .unwrap();
    let entries = batch.pending().as_slice();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].node, target);
    assert_eq!(
        entries[0].update.storage_strategy(),
        crate::data::dependency::SnapshotStorageStrategy::VersionOnlyDelta
    );
    assert_eq!(entries[0].delta.changed_entry_count, 2);
}

#[test]
fn locality_footprint_merges_and_detects_conflicts_canonically() {
    let node_a = NodeId::new(7, 1);
    let node_b = NodeId::new(3, 2);
    let node_c = NodeId::new(9, 1);

    let mut left = LocalityFootprint::new(
        vec![
            PartitionSubscription::whole_partition("wing"),
            PartitionSubscription::partition_and_detail("fuselage", "frame-2"),
        ],
        vec![node_a, node_b],
        vec![node_b],
    );
    let right = LocalityFootprint::new(
        vec![
            PartitionSubscription::partition_and_detail("fuselage", "frame-2"),
            PartitionSubscription::whole_partition("tail"),
        ],
        vec![node_b, node_c],
        vec![node_c],
    );

    assert!(left.conflicts_with(&right));
    left.merge(&right);

    assert_eq!(left.partitions.len(), 3);
    assert_eq!(left.nodes.as_slice(), &[node_b, node_a, node_c]);
    assert_eq!(left.sources.as_slice(), &[node_b, node_c]);
}

#[test]
fn graph_node_builder_accepts_explicit_node_contract() {
    let mut graph = SignalGraph::new();
    let contract = NodeContract::reads([ASPECT_A])
        .with_produces([ASPECT_B])
        .with_required_context(ContextRequirement::RelationalSnapshot);
    let node = graph.node().with_contract(contract.clone()).build();

    let stored = graph.get_contract(node).unwrap().clone();
    assert_eq!(stored, contract);
}

#[test]
fn transaction_batch_dirty_is_the_bulk_invalidation_surface() {
    let mut graph = SignalGraph::new();
    let source_a = graph.node().build();
    let source_b = graph.node().build();
    let dependent = graph.node().build();
    graph
        .set_dependencies(
            dependent,
            [
                DependencyEdge::new(source_a, ASPECT_A),
                DependencyEdge::new(source_b, ASPECT_B),
            ],
        )
        .unwrap();

    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    runtime
        .transaction(&mut (), |transaction| {
            transaction.mark_dirty_batch(&DirtyBatch::from_sources([
                (source_a, ASPECT_A),
                (source_b, ASPECT_B),
            ]))?;
            Ok(())
        })
        .unwrap();

    assert_eq!(
        runtime.graph().get_state(source_a).unwrap(),
        NodeState::Dirty
    );
    assert_eq!(
        runtime.graph().get_state(source_b).unwrap(),
        NodeState::Dirty
    );
    assert_eq!(
        runtime.graph().get_state(dependent).unwrap(),
        NodeState::Dirty
    );
}

#[test]
fn dependency_batch_edit_is_the_bulk_dependency_surface() {
    let mut graph = SignalGraph::new();
    let source_a = graph.node().build();
    let source_b = graph.node().build();
    let left = graph.node().build();
    let right = graph.node().build();

    graph
        .apply_dependency_batch_edit(&DependencyBatchEdit::from_pairs([
            (left, vec![DependencyEdge::new(source_a, ASPECT_A)]),
            (right, vec![DependencyEdge::new(source_b, ASPECT_B)]),
        ]))
        .unwrap();

    assert_eq!(graph.dependencies_of(left).unwrap().len(), 1);
    assert_eq!(graph.dependencies_of(right).unwrap().len(), 1);
    assert_eq!(graph.runtime_subscribers_of(source_a).unwrap(), &[left]);
    assert_eq!(graph.runtime_subscribers_of(source_b).unwrap(), &[right]);
}

#[test]
#[should_panic(expected = "dependency batch edit cannot contain multiple edits")]
fn dependency_batch_edit_rejects_duplicate_node_edits() {
    let node = NodeId::new(7, 1);
    let source = NodeId::new(3, 2);
    let _ = DependencyBatchEdit::from_pairs([
        (node, vec![DependencyEdge::new(source, ASPECT_A)]),
        (node, vec![DependencyEdge::new(source, ASPECT_B)]),
    ]);
}

#[test]
fn define_computation_applies_contract_comparator_and_tier_to_created_nodes() {
    let graph = SignalGraph::new();
    let mut runtime = SignalRuntime::builder(graph)
        .with_kernel_defaults()
        .with_tiers::<Tier>()
        .build();
    let contract = NodeContract::reads([ASPECT_A])
        .with_produces([ASPECT_B])
        .with_required_context(ContextRequirement::DomainContext);
    let computation = runtime
        .define(Recipe {
            family: "geometry".into(),
            contract: contract.clone(),
            tier: Tier::Feature,
            comparator: VersionComparatorPolicy::OutputIdentity,
            evaluator: |_ctx: &mut EvaluationContext<'_, ()>| {
                Ok::<EvaluationOutput, SignalError>(EvaluationOutput::from_result(
                    NodeEvaluationResult::from_version(version_ab(1, 0)),
                ))
            },
        })
        .unwrap();

    let node = computation.keyed("bulkhead").node(&mut runtime);
    let stored = runtime
        .graph()
        .get_entry(node)
        .unwrap()
        .get_eval_config()
        .clone();

    assert_eq!(runtime.graph().get_contract(node).unwrap(), &contract);
    assert_eq!(
        stored.comparator,
        Some(VersionComparatorPolicy::OutputIdentity)
    );
    assert_eq!(
        runtime.config().node_meta().tier_for_node(node),
        Some(Tier::Feature)
    );
}

#[test]
fn easy_mode_supports_input_computed_get_set_and_batch() {
    let mut graph = ReactiveGraph::new();
    let price = graph.input(100.0_f64);
    let tax = graph.input(0.08_f64);
    let total = graph.computed(move |context| context.get(price) * (1.0 + context.get(tax)));

    assert_eq!(graph.get(total), 108.0);

    graph.set(price, 200.0);
    assert_eq!(graph.get(total), 216.0);

    graph.batch(|reactive| {
        reactive.set(price, 300.0);
        reactive.set(tax, 0.10);
    });
    assert_eq!(graph.get(total), 330.0);
}

#[test]
fn easy_mode_computed_chains_observe_staged_upstream_values_in_the_same_pass() {
    let mut graph = ReactiveGraph::new();
    let source = graph.input(2_i32);
    let doubled = graph.computed(move |context| context.get(source) * 2);
    let chained = graph.computed(move |context| context.get(doubled) + 1);

    assert_eq!(graph.get(chained), 5);

    graph.set(source, 7);

    assert_eq!(
        graph.get(chained),
        15,
        "downstream computed nodes should see freshly staged upstream values, not the pre-plan cache"
    );
}

#[test]
fn easy_mode_failed_batch_restores_input_values() {
    let mut graph = ReactiveGraph::new();
    let price = graph.input(100_i32);
    let tax = graph.input(5_i32);

    let err = graph.try_batch(|reactive| {
        reactive.try_set(price, 200)?;
        reactive.try_set(tax, 9)?;
        Err(SignalError::invalid_input("force easy-mode rollback"))
    });
    assert!(err.is_err());

    assert_eq!(graph.get(price), 100);
    assert_eq!(graph.get(tax), 5);
}

#[test]
fn easy_mode_failed_batch_restores_downstream_invalidation_state() {
    let mut graph = ReactiveGraph::new();
    let source = graph.input(2_i32);
    let doubled = graph.computed(move |context| context.get(source) * 2);

    assert_eq!(graph.get(doubled), 4);

    let err = graph.try_batch(|reactive| {
        reactive.try_set(source, 9)?;
        reactive.try_get(doubled)?;
        Err(SignalError::invalid_input(
            "force rollback after dirty propagation",
        ))
    });
    assert!(err.is_err());

    assert_eq!(graph.get(source), 2);
    assert_eq!(graph.get(doubled), 4);
}

#[test]
fn observation_hot_paths_use_index_iteration_without_snapshot_allocations() {
    assert!(
        HOT_TRANSACTION_OBSERVATION_MUTATION_SOURCE.contains("for_each_matching_observer_for_node("),
        "transaction observation staging should iterate the node index directly instead of materializing temporary observer-id snapshots"
    );
    assert!(
        !HOT_TRANSACTION_OBSERVATION_MUTATION_SOURCE.contains(
            "matching_observers_for_node(node)"
        ),
        "transaction observation staging should not allocate matching-observer snapshots on the hot path"
    );
    assert!(
        HOT_EASY_OBSERVATION_SOURCE.contains("for_each_matching_observer_for_node("),
        "easy observation delivery should iterate the node index directly instead of materializing temporary observer-id snapshots"
    );
    assert!(
        HOT_EASY_OBSERVATION_SOURCE.contains("has_matching_observers_for_node("),
        "easy observation traversal should use the zero-allocation node-watch check instead of snapshotting observer ids"
    );
    assert!(
        HOT_EASY_OBSERVATION_SOURCE.contains(".filter(|node| app.computed.contains_key(node))"),
        "easy observation recompute prepass should narrow itself to impacted computed nodes instead of cloning the whole impacted set"
    );
}

#[test]
fn committed_observation_delivery_avoids_recloning_boundary_summaries() {
    assert!(
        HOT_RUNTIME_OBSERVATION_SOURCE.contains("for delivery in deliveries"),
        "committed observation delivery should stream committed packets directly instead of rebuilding a second summary vector"
    );
    assert!(
        !HOT_RUNTIME_OBSERVATION_SOURCE.contains(".collect::<Vec<_>>();\r\n        self.deliver_boundary_summaries(graph, &summaries)")
            && !HOT_RUNTIME_OBSERVATION_SOURCE.contains(".collect::<Vec<_>>();\n        self.deliver_boundary_summaries(graph, &summaries)"),
        "committed observation delivery should not clone whole boundary summaries into a second vector before listener dispatch"
    );
}

#[test]
fn easy_mode_watch_and_effect_use_observation_substrate() {
    let mut graph = ReactiveGraph::new();
    let count = graph.input(1_i32);
    let doubled = graph.computed({
        let count = count;
        move |context| context.get(count) * 2
    });

    let watch_hits = std::sync::Arc::new(std::sync::Mutex::new(Vec::<usize>::new()));
    let watch_hits_clone = std::sync::Arc::clone(&watch_hits);
    let effect_hits = std::sync::Arc::new(std::sync::Mutex::new(0_usize));
    let effect_hits_clone = std::sync::Arc::clone(&effect_hits);

    let watch_handle = graph.watch(doubled, move |notice| {
        watch_hits_clone
            .lock()
            .expect("easy watch mutex poisoned")
            .push(notice.matched_nodes().len());
        assert!(notice.trigger_matched());
        assert!(notice.meaningful_change());
    });
    let effect_handle = graph.effect(doubled, move || {
        *effect_hits_clone
            .lock()
            .expect("easy effect mutex poisoned") += 1;
    });

    graph.set(count, 2);
    assert_eq!(graph.get(doubled), 4);

    assert_eq!(
        watch_hits
            .lock()
            .expect("easy watch mutex poisoned")
            .as_slice(),
        &[1]
    );
    assert_eq!(*effect_hits.lock().expect("easy effect mutex poisoned"), 1);

    assert!(graph.unobserve(watch_handle));
    assert!(graph.unobserve(effect_handle));
}

#[test]
fn easy_mode_meaningful_change_watch_suppresses_recomputed_but_unchanged_values() {
    let mut graph = ReactiveGraph::new();
    let count = graph.input(1_i32);
    let parity = graph.computed({
        let count = count;
        move |context| context.get(count) % 2
    });

    let watch_hits = std::sync::Arc::new(std::sync::Mutex::new(0_usize));
    let effect_hits = std::sync::Arc::new(std::sync::Mutex::new(0_usize));
    let watch_hits_clone = std::sync::Arc::clone(&watch_hits);
    let effect_hits_clone = std::sync::Arc::clone(&effect_hits);

    let watch_handle = graph.watch(parity, move |notice| {
        *watch_hits_clone
            .lock()
            .expect("easy meaningful-change watch mutex poisoned") += 1;
        assert!(notice.recomputed());
        assert!(notice.meaningful_change());
    });
    let effect_handle = graph.effect(parity, move || {
        *effect_hits_clone
            .lock()
            .expect("easy meaningful-change effect mutex poisoned") += 1;
    });

    graph.set(count, 3);
    assert_eq!(graph.get(parity), 1);

    assert_eq!(
        *watch_hits
            .lock()
            .expect("easy meaningful-change watch mutex poisoned"),
        0
    );
    assert_eq!(
        *effect_hits
            .lock()
            .expect("easy meaningful-change effect mutex poisoned"),
        0
    );

    assert!(graph.unobserve(watch_handle));
    assert!(graph.unobserve(effect_handle));
}

#[test]
fn easy_mode_unobserve_stops_future_notifications() {
    let mut graph = ReactiveGraph::new();
    let count = graph.input(1_i32);
    let doubled = graph.computed({
        let count = count;
        move |context| context.get(count) * 2
    });

    let hits = std::sync::Arc::new(std::sync::Mutex::new(0_usize));
    let hits_clone = std::sync::Arc::clone(&hits);
    let handle = graph.watch(doubled, move |_notice| {
        *hits_clone
            .lock()
            .expect("easy unobserve watch mutex poisoned") += 1;
    });

    graph.set(count, 2);
    assert_eq!(
        *hits.lock().expect("easy unobserve watch mutex poisoned"),
        1
    );

    assert!(graph.unobserve(handle));
    graph.set(count, 3);
    assert_eq!(
        *hits.lock().expect("easy unobserve watch mutex poisoned"),
        1
    );
}
