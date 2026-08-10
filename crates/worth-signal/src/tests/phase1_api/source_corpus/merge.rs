pub(in crate::tests::phase1_api) const MERGE_EXECUTE_SOURCE: &str =
    include_str!("../../../logic/transaction/runtime/state/merge/execute.rs");
pub(in crate::tests::phase1_api) const MERGE_FOUNDATIONAL_SCOPE_SOURCE: &str =
    include_str!("../../../logic/transaction/runtime/state/merge/foundational_scope.rs");
pub(in crate::tests::phase1_api) const MERGE_CANDIDATE_SCOPE_SOURCE: &str =
    include_str!("../../../logic/transaction/runtime/state/merge/candidate_scope.rs");
pub(in crate::tests::phase1_api) const MERGE_SCOPED_PROOF_SOURCE: &str =
    include_str!("../../../logic/transaction/runtime/state/merge/scoped_proof.rs");
pub(in crate::tests::phase1_api) const MERGE_COMPATIBILITY_WITNESS_SOURCE: &str =
    include_str!("../../../logic/transaction/runtime/state/merge/compatibility/witness.rs");
pub(in crate::tests::phase1_api) const MERGE_COMPATIBILITY_FACTS_SOURCE: &str =
    include_str!("../../../logic/transaction/runtime/state/merge/compatibility/facts.rs");
pub(in crate::tests::phase1_api) const MERGE_COMPATIBILITY_DENIAL_SOURCE: &str =
    include_str!("../../../logic/transaction/runtime/state/merge/compatibility/denial.rs");
pub(in crate::tests::phase1_api) const MERGE_COMPATIBILITY_READMISSION_SOURCE: &str =
    include_str!("../../../logic/transaction/runtime/state/merge/compatibility/readmission.rs");
pub(in crate::tests::phase1_api) const MERGE_INSPECTION_SUPPORT_WITNESS_SOURCE: &str =
    include_str!("../../../logic/transaction/runtime/state/inspection/support_witness.rs");
pub(in crate::tests::phase1_api) const MERGE_INSPECTION_SUPPORT_ROWS_SOURCE: &str =
    include_str!("../../../logic/transaction/runtime/state/inspection/support_rows.rs");
pub(in crate::tests::phase1_api) const MERGE_INSPECTION_ABSENCE_SOURCE: &str =
    include_str!("../../../logic/transaction/runtime/state/inspection/absence.rs");
pub(in crate::tests::phase1_api) const MERGE_STRATEGY_IDENTITY_SOURCE: &str =
    include_str!("../../../logic/transaction/runtime/state/merge/strategy_identity.rs");
pub(in crate::tests::phase1_api) const MERGE_STRATEGY_WITNESS_SOURCE: &str =
    include_str!("../../../logic/transaction/runtime/state/merge/strategy_witness.rs");
pub(in crate::tests::phase1_api) const MERGE_PLAN_SOURCE: &str = concat!(
    include_str!("../../../logic/transaction/runtime/state/merge/plan.rs"),
    include_str!("../../../logic/transaction/runtime/state/merge/plan/accessors.rs"),
    include_str!("../../../logic/transaction/runtime/state/merge/plan/candidates.rs"),
    include_str!("../../../logic/transaction/runtime/state/merge/plan/construction.rs"),
);
pub(in crate::tests::phase1_api) const MERGE_PROOF_SOURCE: &str = concat!(
    include_str!("../../../logic/transaction/runtime/state/merge/proof.rs"),
    include_str!("../../../logic/transaction/runtime/state/merge/proof/digest.rs"),
    include_str!("../../../logic/transaction/runtime/state/merge/proof/replay.rs"),
    include_str!("../../../logic/transaction/runtime/state/merge/proof/reports.rs"),
);
pub(in crate::tests::phase1_api) const MERGE_RESULT_SOURCE: &str =
    include_str!("../../../logic/transaction/runtime/state/merge/result.rs");
pub(in crate::tests::phase1_api) const MERGE_REQUEST_SOURCE: &str =
    include_str!("../../../logic/transaction/runtime/state/merge/request.rs");
pub(in crate::tests::phase1_api) const REPLAY_SOURCE: &str =
    include_str!("../../../diagnostics/model/replay.rs");
pub(in crate::tests::phase1_api) const GUIDED_SOURCE: &str =
    include_str!("../../../logic/transaction/runtime/state/guided.rs");
pub(in crate::tests::phase1_api) const MERGE_RUNTIME_REQUEST_BOUNDARY_SOURCE: &str = include_str!(
    "../../../logic/transaction/runtime/state/branching/merge_runtime/request_boundary.rs"
);
pub(in crate::tests::phase1_api) const MERGE_RUNTIME_CANDIDATES_SOURCE: &str =
    include_str!("../../../logic/transaction/runtime/state/branching/merge_runtime/candidates.rs");
pub(in crate::tests::phase1_api) const MERGE_RUNTIME_PLAN_SOURCE: &str = include_str!(
    "../../../logic/transaction/runtime/state/branching/merge_runtime/plan_compiler.rs"
);
pub(in crate::tests::phase1_api) const MERGE_RUNTIME_ARTIFACT_PROJECTION_SOURCE: &str = include_str!(
    "../../../logic/transaction/runtime/state/branching/merge_runtime/artifact_projection.rs"
);
pub(in crate::tests::phase1_api) const MERGE_RUNTIME_NODE_PLAN_SOURCE: &str =
    include_str!("../../../logic/transaction/runtime/state/branching/merge_runtime/node_plan.rs");
pub(in crate::tests::phase1_api) const MERGE_RUNTIME_EXECUTION_APPLICATION_SOURCE: &str = include_str!(
    "../../../logic/transaction/runtime/state/branching/merge_runtime/execution_application.rs"
);
pub(in crate::tests::phase1_api) const MERGE_RUNTIME_EXECUTION_FINALIZATION_SOURCE: &str = include_str!(
    "../../../logic/transaction/runtime/state/branching/merge_runtime/execution_finalization.rs"
);
