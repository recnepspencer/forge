use super::ExactBoundarySymbol;

pub(super) const PLANNER_AND_EXECUTOR_BOUNDARIES: &[ExactBoundarySymbol] = &[
    boundary!(
        "evaluation plan construction entry",
        "logic/planner/planning/mod.rs",
        "../../../../../logic/planner/planning/mod.rs",
        "build_evaluation_plan_with_policy_resolver",
        2
    ),
    boundary!(
        "readiness prevalidation owner",
        "logic/planner/precompute/eligibility.rs",
        "../../../../../logic/planner/precompute/eligibility.rs",
        "prevalidate_stage_tasks",
        1
    ),
    boundary!(
        "serial and parallel readiness call sites",
        "logic/planner/precompute/read_preparation.rs",
        "../../../../../logic/planner/precompute/read_preparation.rs",
        "prevalidate_stage_tasks",
        4
    ),
    boundary!(
        "prepared-plan execution facade",
        "logic/planner/execution/mod.rs",
        "../../../../../logic/planner/execution/mod.rs",
        "execute_prepared_plan",
        1
    ),
    boundary!(
        "policy-governed execution entry",
        "logic/planner/execution/mod.rs",
        "../../../../../logic/planner/execution/mod.rs",
        "execute_prepared_plan_with_policy",
        2
    ),
    boundary!(
        "temporal-readiness execution entry",
        "logic/planner/execution/mod.rs",
        "../../../../../logic/planner/execution/mod.rs",
        "execute_prepared_plan_with_policy_and_temporal_lowering",
        2
    ),
    boundary!(
        "stage dispatch call site",
        "logic/planner/execution/mod.rs",
        "../../../../../logic/planner/execution/mod.rs",
        "execute_stage",
        2
    ),
    boundary!(
        "stage execution owner",
        "logic/planner/execution/stage.rs",
        "../../../../../logic/planner/execution/stage.rs",
        "execute_stage",
        1
    ),
    boundary!(
        "stage precompute orchestration call",
        "logic/planner/execution/stage.rs",
        "../../../../../logic/planner/execution/stage.rs",
        "perform_stage_precompute",
        2
    ),
    boundary!(
        "stage precompute owner",
        "logic/planner/precompute/stage.rs",
        "../../../../../logic/planner/precompute/stage.rs",
        "perform_stage_precompute",
        1
    ),
    boundary!(
        "stage application orchestration call",
        "logic/planner/execution/stage.rs",
        "../../../../../logic/planner/execution/stage.rs",
        "apply_stage",
        2
    ),
    boundary!(
        "stage application owner",
        "logic/planner/apply/stage.rs",
        "../../../../../logic/planner/apply/stage.rs",
        "apply_stage",
        1
    ),
    boundary!(
        "lowered execution-form call",
        "logic/planner/apply/stage.rs",
        "../../../../../logic/planner/apply/stage.rs",
        "build_stage_execution_form",
        1
    ),
    boundary!(
        "lowered execution-form owner",
        "logic/planner/apply/stage/lowering.rs",
        "../../../../../logic/planner/apply/stage/lowering.rs",
        "build_stage_execution_form",
        1
    ),
    boundary!(
        "lowered apply pass owner and call",
        "logic/planner/apply/stage.rs",
        "../../../../../logic/planner/apply/stage.rs",
        "run_lowered_apply_pass",
        2
    ),
    boundary!(
        "snapshot publication owner and call",
        "logic/planner/apply/stage.rs",
        "../../../../../logic/planner/apply/stage.rs",
        "publish_pending_snapshots",
        2
    ),
    boundary!(
        "stage finalization owner and call",
        "logic/planner/apply/stage.rs",
        "../../../../../logic/planner/apply/stage.rs",
        "finalize_stage_results",
        2
    ),
    boundary!(
        "serial semantic finalization call",
        "logic/planner/apply/stage.rs",
        "../../../../../logic/planner/apply/stage.rs",
        "finalize_serial_stage_batch",
        2
    ),
    boundary!(
        "serial semantic finalization owner",
        "logic/planner/semantic/finalization.rs",
        "../../../../../logic/planner/semantic/finalization.rs",
        "finalize_serial_stage_batch",
        1
    ),
    boundary!(
        "parallel semantic finalization call",
        "logic/planner/apply/stage.rs",
        "../../../../../logic/planner/apply/stage.rs",
        "finalize_stage_batch",
        1
    ),
    boundary!(
        "parallel semantic finalization owner",
        "logic/planner/semantic/finalization.rs",
        "../../../../../logic/planner/semantic/finalization.rs",
        "finalize_stage_batch",
        1
    ),
    boundary!(
        "prepared custom-precompute execution entry",
        "logic/planner/execution/mod.rs",
        "../../../../../logic/planner/execution/mod.rs",
        "execute_prepared_plan_with_precompute",
        1
    ),
    boundary!(
        "scratch-session execution entry",
        "logic/planner/execution/mod.rs",
        "../../../../../logic/planner/execution/mod.rs",
        "execute_evaluation_session_with_policy",
        1
    ),
    boundary!(
        "shared plan and session stage-slice executor",
        "logic/planner/execution/mod.rs",
        "../../../../../logic/planner/execution/mod.rs",
        "execute_plan_stage_slices_with_policy",
        4
    ),
    boundary!(
        "stage precompute dispatch call",
        "logic/planner/precompute/stage.rs",
        "../../../../../logic/planner/precompute/stage.rs",
        "dispatch_stage_precompute",
        2
    ),
    boundary!(
        "stage precompute dispatch owner",
        "logic/planner/precompute/dispatch.rs",
        "../../../../../logic/planner/precompute/dispatch.rs",
        "dispatch_stage_precompute",
        1
    ),
    boundary!(
        "serial precompute dispatch",
        "logic/planner/precompute/dispatch.rs",
        "../../../../../logic/planner/precompute/dispatch.rs",
        "dispatch_stage_precompute_serial",
        3
    ),
    boundary!(
        "parallel precompute dispatch",
        "logic/planner/precompute/dispatch.rs",
        "../../../../../logic/planner/precompute/dispatch.rs",
        "dispatch_stage_precompute_parallel",
        2
    ),
    boundary!(
        "serial precompute owner",
        "logic/planner/precompute/read_preparation.rs",
        "../../../../../logic/planner/precompute/read_preparation.rs",
        "precompute_stage_serial",
        1
    ),
    boundary!(
        "staged-parallel precompute owner",
        "logic/planner/precompute/read_preparation.rs",
        "../../../../../logic/planner/precompute/read_preparation.rs",
        "precompute_stage_parallel",
        1
    ),
    boundary!(
        "full-parallel patch preparation owner",
        "logic/planner/precompute/read_preparation.rs",
        "../../../../../logic/planner/precompute/read_preparation.rs",
        "build_parallel_stage_patches",
        1
    ),
    boundary!(
        "grouped concurrent apply call",
        "logic/planner/apply/stage.rs",
        "../../../../../logic/planner/apply/stage.rs",
        "run_grouped_concurrent_apply_pass",
        1
    ),
    boundary!(
        "grouped concurrent apply owner",
        "logic/planner/apply/stage/concurrent.rs",
        "../../../../../logic/planner/apply/stage/concurrent.rs",
        "run_grouped_concurrent_apply_pass",
        1
    ),
    boundary!(
        "parallel apply input lowering",
        "logic/planner/apply/stage/concurrent.rs",
        "../../../../../logic/planner/apply/stage/concurrent.rs",
        "build_concurrent_apply_group_inputs",
        1
    ),
    boundary!(
        "parallel group packet construction",
        "logic/planner/apply/stage/concurrent.rs",
        "../../../../../logic/planner/apply/stage/concurrent.rs",
        "build_group_packet",
        1
    ),
    boundary!(
        "parallel packet reduction",
        "logic/planner/apply/stage/concurrent.rs",
        "../../../../../logic/planner/apply/stage/concurrent.rs",
        "reduce_grouped_concurrent_packets",
        1
    ),
    boundary!(
        "parallel prepared-commit publication call",
        "logic/planner/apply/stage/concurrent_packets.rs",
        "../../../../../logic/planner/apply/stage/concurrent_packets.rs",
        "publish_prepared_parallel_apply_commit_packet",
        1
    ),
    boundary!(
        "parallel prepared-commit publication owner",
        "data/graph/runtime/effect/output_commit.rs",
        "../../../../../data/graph/runtime/effect/output_commit.rs",
        "publish_prepared_parallel_apply_commit_packet",
        1
    ),
];
