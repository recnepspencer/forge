pub(super) const HOT_APPLY_SOURCE: &str = include_str!("../../logic/evaluation/engine/apply.rs");
pub(super) const HOT_PREPARED_APPLY_SOURCE: &str =
    include_str!("../../logic/evaluation/engine/prepared_apply.rs");
pub(super) const HOT_SEMANTIC_FINALIZE_SOURCE: &str =
    include_str!("../../logic/planner/semantic/mod.rs");
pub(super) const HOT_EFFECT_SOURCE: &str = include_str!("../../data/graph/runtime/effect.rs");
pub(super) const HOT_SERIAL_BATCH_SOURCE: &str =
    include_str!("../../logic/planner/apply/serial_batch.rs");
pub(super) const HOT_STAGE_SOURCE: &str = include_str!("../../logic/planner/apply/stage.rs");
pub(super) const HOT_PLANNING_SOURCE: &str = include_str!("../../logic/planner/planning/mod.rs");
pub(super) const HOT_VALIDATION_SOURCE: &str =
    include_str!("../../logic/planner/planning/validation.rs");
pub(super) const HOT_PRECOMPUTE_SOURCE: &str =
    include_str!("../../logic/planner/precompute/mod.rs");
pub(super) const HOT_CONTEXT_SOURCE: &str = include_str!("../../logic/context.rs");
pub(super) const HOT_REUSE_CONTEXT_SOURCE: &str =
    include_str!("../../logic/evaluation/reuse/context_resolution.rs");
pub(super) const HOT_INVALIDATION_ROUTING_SOURCE: &str =
    include_str!("../../logic/invalidation/routing.rs");
pub(super) const HOT_INVALIDATION_SUBSCRIPTION_SOURCE: &str =
    include_str!("../../logic/invalidation/subscription.rs");
pub(super) const HOT_TRANSACTION_OBSERVATION_MUTATION_SOURCE: &str =
    include_str!("../../logic/transaction/runtime/transaction/transaction_mutation.rs");
pub(super) const HOT_EASY_OBSERVATION_SOURCE: &str = include_str!("../../easy/observation.rs");
pub(super) const HOT_RUNTIME_OBSERVATION_SOURCE: &str =
    include_str!("../../logic/transaction/runtime/state/runtime_observation.rs");

pub(super) const PROOF_SOURCE: &str = include_str!("../../data/proof.rs");
pub(super) const PLANNER_MODEL_SOURCE: &str = include_str!("../../logic/planner/model/mod.rs");
pub(super) const SEMANTIC_SOURCE: &str = include_str!("../../logic/planner/semantic/mod.rs");
pub(super) const WORKSPACE_SOURCE: &str = include_str!("../../logic/planner/apply/workspace.rs");
pub(super) const PATCH_BUFFER_SOURCE: &str =
    include_str!("../../logic/transaction/patch_buffer.rs");

pub(super) const MERGE_EXECUTE_SOURCE: &str =
    include_str!("../../logic/transaction/runtime/state/merge/execute.rs");
pub(super) const MERGE_FOUNDATIONAL_SCOPE_SOURCE: &str =
    include_str!("../../logic/transaction/runtime/state/merge/foundational_scope.rs");
pub(super) const MERGE_CANDIDATE_SCOPE_SOURCE: &str =
    include_str!("../../logic/transaction/runtime/state/merge/candidate_scope.rs");
pub(super) const MERGE_SCOPED_PROOF_SOURCE: &str =
    include_str!("../../logic/transaction/runtime/state/merge/scoped_proof.rs");
pub(super) const MERGE_COMPATIBILITY_WITNESS_SOURCE: &str =
    include_str!("../../logic/transaction/runtime/state/merge/compatibility/witness.rs");
pub(super) const MERGE_COMPATIBILITY_FACTS_SOURCE: &str =
    include_str!("../../logic/transaction/runtime/state/merge/compatibility/facts.rs");
pub(super) const MERGE_COMPATIBILITY_DENIAL_SOURCE: &str =
    include_str!("../../logic/transaction/runtime/state/merge/compatibility/denial.rs");
pub(super) const MERGE_COMPATIBILITY_READMISSION_SOURCE: &str =
    include_str!("../../logic/transaction/runtime/state/merge/compatibility/readmission.rs");
pub(super) const MERGE_INSPECTION_SUPPORT_WITNESS_SOURCE: &str =
    include_str!("../../logic/transaction/runtime/state/inspection/support_witness.rs");
pub(super) const MERGE_INSPECTION_SUPPORT_ROWS_SOURCE: &str =
    include_str!("../../logic/transaction/runtime/state/inspection/support_rows.rs");
pub(super) const MERGE_INSPECTION_ABSENCE_SOURCE: &str =
    include_str!("../../logic/transaction/runtime/state/inspection/absence.rs");
pub(super) const MERGE_STRATEGY_IDENTITY_SOURCE: &str =
    include_str!("../../logic/transaction/runtime/state/merge/strategy_identity.rs");
pub(super) const MERGE_STRATEGY_WITNESS_SOURCE: &str =
    include_str!("../../logic/transaction/runtime/state/merge/strategy_witness.rs");
pub(super) const MERGE_PLAN_SOURCE: &str =
    include_str!("../../logic/transaction/runtime/state/merge/plan.rs");
pub(super) const MERGE_PROOF_SOURCE: &str =
    include_str!("../../logic/transaction/runtime/state/merge/proof.rs");
pub(super) const MERGE_RESULT_SOURCE: &str =
    include_str!("../../logic/transaction/runtime/state/merge/result.rs");
pub(super) const MERGE_REQUEST_SOURCE: &str =
    include_str!("../../logic/transaction/runtime/state/merge/request.rs");
pub(super) const REPLAY_SOURCE: &str = include_str!("../../diagnostics/model/replay.rs");
pub(super) const RECORDER_SOURCE: &str = include_str!("../../diagnostics/runtime/recorder.rs");
pub(super) const GUIDED_SOURCE: &str =
    include_str!("../../logic/transaction/runtime/state/guided.rs");

pub(super) const MERGE_RUNTIME_REQUEST_BOUNDARY_SOURCE: &str = include_str!(
    "../../logic/transaction/runtime/state/branching/merge_runtime/request_boundary.rs"
);
pub(super) const MERGE_RUNTIME_CANDIDATES_SOURCE: &str =
    include_str!("../../logic/transaction/runtime/state/branching/merge_runtime/candidates.rs");
pub(super) const MERGE_RUNTIME_PLAN_SOURCE: &str =
    include_str!("../../logic/transaction/runtime/state/branching/merge_runtime/plan_compiler.rs");
pub(super) const MERGE_RUNTIME_ARTIFACT_PROJECTION_SOURCE: &str = include_str!(
    "../../logic/transaction/runtime/state/branching/merge_runtime/artifact_projection.rs"
);
pub(super) const MERGE_RUNTIME_NODE_PLAN_SOURCE: &str =
    include_str!("../../logic/transaction/runtime/state/branching/merge_runtime/node_plan.rs");
pub(super) const MERGE_RUNTIME_EXECUTION_APPLICATION_SOURCE: &str = include_str!(
    "../../logic/transaction/runtime/state/branching/merge_runtime/execution_application.rs"
);
pub(super) const MERGE_RUNTIME_EXECUTION_FINALIZATION_SOURCE: &str = include_str!(
    "../../logic/transaction/runtime/state/branching/merge_runtime/execution_finalization.rs"
);
pub(super) const BRANCH_BASIS_SOURCE: &str =
    include_str!("../../logic/transaction/runtime/state/branching/basis.rs");
pub(super) const BRANCH_FORK_SOURCE: &str =
    include_str!("../../logic/transaction/runtime/state/branching/fork.rs");
pub(super) const BRANCHES_SOURCE: &str =
    include_str!("../../logic/transaction/runtime/state/branching/branches.rs");
pub(super) const LIFECYCLE_SOURCE: &str =
    include_str!("../../logic/transaction/runtime/state/branching/lifecycle.rs");
pub(super) const RUNTIME_STATE_SOURCE: &str =
    include_str!("../../logic/transaction/runtime/state/runtime_state.rs");
pub(super) const SNAPSHOT_RESTORE_SOURCE: &str =
    include_str!("../../data/graph/diagnostics_access/artifacts.rs");
pub(super) const RUNTIME_SNAPSHOTTING_SOURCE: &str =
    include_str!("../../logic/transaction/runtime/state/branching/snapshotting.rs");
pub(super) const CHECKPOINT_IMAGE_SOURCE: &str =
    include_str!("../../data/node/checkpoint_image.rs");
pub(super) const STATE_SOURCE: &str = include_str!("../../state/mod.rs");

pub(super) const PERFORMANCE_SUPPORT_SOURCE: &str = include_str!("../performance_support.rs");
pub(super) const PERFORMANCE_PROFILES_SOURCE: &str = include_str!("../performance_profiles.rs");
pub(super) const PERFORMANCE_BASELINE_SOURCE: &str = include_str!("../performance_baseline.json");
pub(super) const ENTRIES_SOURCE: &str = include_str!("../../data/graph/storage/entries.rs");
pub(super) const GRAPH_RUNTIME_SOURCE: &str = include_str!("../../data/graph/runtime/graph.rs");
pub(super) const SLOT_SOURCE: &str = include_str!("../../data/graph/storage/slot.rs");
pub(super) const DOT_SOURCE: &str = include_str!("../../presentation/outputs/dot.rs");
pub(super) const HARNESS_BRIDGE_SOURCE: &str = include_str!("../../presentation/harness/bridge.rs");
pub(super) const EXECUTION_FLOW_SOURCE: &str =
    include_str!("../../diagnostics/runtime/execution_flow.rs");
pub(super) const HISTORY_SOURCE: &str = include_str!("../../diagnostics/inspection/history.rs");
pub(super) const SUMMARY_SOURCE: &str = include_str!("../../diagnostics/model/summary.rs");
pub(super) const OBSERVER_SOURCE: &str = include_str!("../../data/graph/runtime/observer.rs");
pub(super) const FACADE_SOURCE: &str = include_str!("../../facade.rs");
