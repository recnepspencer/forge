use super::activation_staging_test_support::activation_staging_inputs;
use super::durable_state_inventory_test_support::platform_inventory;
use super::query_binding_comparison_test_support::{
    basis_drift_query_app, denial_presentation_drift_query_app, query_artifact, standard_query_app,
};
use super::replacement_impact_test_support::{admitted_candidate, launch_runtime};
use crate::facade::WorthUiApp;
use crate::runtime::{
    WorthUiActivationGateDenial, WorthUiExecutionPlan, WorthUiExecutionPlanInput,
    WorthUiLaneParityReport, WorthUiNodeLifecycleTransition, WorthUiNodeReplacementClassification,
    WorthUiNodeReplacementCounters, WorthUiNodeReplacementPlan, WorthUiPendingActivation,
    WorthUiReadyActivation, WorthUiRuntimeHandleAllocation, WorthUiRuntimeHost,
};
use crate::source::WorthUiArtifact;

pub(super) struct ReadyFixture {
    pub(super) runtime: WorthUiRuntimeHost,
    pub(super) plan_input: WorthUiExecutionPlanInput,
    pub(super) handle_allocation: WorthUiRuntimeHandleAllocation,
    pub(super) candidate_plan: WorthUiExecutionPlan,
    pub(super) reconciliation_receipt_count: usize,
    pub(super) query_rebind_entry_count: usize,
    pub(super) ready: WorthUiReadyActivation,
}

pub(super) fn ready_activation_fixture() -> ReadyFixture {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    ready_activation_fixture_from(runtime, pending)
}

pub(super) fn ready_activation_fixture_after_frame_advance() -> ReadyFixture {
    let inputs = activation_staging_inputs();
    let mut runtime = inputs.runtime;
    runtime.advance_frame_epoch_for_test();
    let pending = runtime
        .stage_replacement_activation(
            inputs.admitted,
            &inputs.impact,
            &inputs.narrowing,
            &inputs.node_plan,
            Some(&inputs.reconciliation_plan),
            Some(&inputs.query_rebind_plan),
            Some(&inputs.pending_execution_plan_lowering_input),
        )
        .expect("activation staging succeeds at advanced epoch");
    ready_activation_fixture_from(runtime, pending)
}

pub(super) fn query_posture_drift_ready_activation_fixture() -> ReadyFixture {
    let active_app = standard_query_app();
    let candidate_app = basis_drift_query_app();
    let active = query_artifact(&active_app, "workspace.view_binding.selection");
    let candidate = query_artifact(&candidate_app, "workspace.view_binding.selection");
    let (runtime, pending) = staged_query_replacement(active_app, active, candidate);
    ready_activation_fixture_from(runtime, pending)
}

fn ready_activation_fixture_from(
    runtime: WorthUiRuntimeHost,
    pending: WorthUiPendingActivation,
) -> ReadyFixture {
    let reconciliation_receipt_count = pending
        .staged_replacement()
        .reconciliation_plan()
        .receipts()
        .len();
    let query_rebind_entry_count = pending
        .staged_replacement()
        .query_rebind_plan()
        .entries()
        .len();
    let plan_input = runtime
        .prepare_execution_plan_input(&pending)
        .expect("plan input prepares");
    let handle_allocation = runtime
        .allocate_runtime_handles(&plan_input)
        .expect("runtime handles allocate");
    let candidate_plan = runtime
        .assemble_execution_plan_topology(&plan_input, &handle_allocation)
        .expect("execution plan topology assembles");
    let ready = runtime
        .prepare_ready_activation(
            pending,
            &plan_input,
            &handle_allocation,
            &candidate_plan,
            None,
        )
        .expect("ready activation prepares");
    ReadyFixture {
        runtime,
        plan_input,
        handle_allocation,
        candidate_plan,
        reconciliation_receipt_count,
        query_rebind_entry_count,
        ready,
    }
}

pub(super) fn denied_query_ready_activation() -> WorthUiActivationGateDenial {
    let active_app = standard_query_app();
    let candidate_app = denial_presentation_drift_query_app();
    let active = query_artifact(&active_app, "workspace.view_binding.selection");
    let candidate = query_artifact(&candidate_app, "workspace.view_binding.selection");
    let (runtime, pending) = staged_query_replacement(active_app, active, candidate);
    let plan_input = runtime
        .prepare_execution_plan_input(&pending)
        .expect("plan input prepares");
    let handles = runtime
        .allocate_runtime_handles(&plan_input)
        .expect("handles allocate");
    let plan = runtime
        .assemble_execution_plan_topology(&plan_input, &handles)
        .expect("topology assembles");
    runtime
        .prepare_ready_activation(pending, &plan_input, &handles, &plan, None)
        .expect_err("Query denial blocks ready activation")
}

fn staged_query_replacement(
    active_app: WorthUiApp,
    active: WorthUiArtifact,
    candidate: WorthUiArtifact,
) -> (WorthUiRuntimeHost, WorthUiPendingActivation) {
    let runtime = launch_runtime(&active_app, active);
    let admitted = admitted_candidate(&active_app, &runtime, candidate);
    let comparison = runtime
        .compare_admitted_replacement(&admitted)
        .expect("runtime comparison succeeds");
    let impact = runtime
        .classify_replacement_impact(&comparison, &admitted)
        .expect("impact classification succeeds");
    let narrowing = runtime
        .narrow_replacement_impact(&impact, &admitted)
        .expect("impact narrowing succeeds");
    let identity = runtime
        .build_identity_match_graph(&narrowing, &admitted)
        .expect("identity graph succeeds");
    let node_plan = runtime
        .classify_node_replacements(&impact, &narrowing, &identity)
        .expect("node plan succeeds");
    let inventory = platform_inventory(&runtime)
        .build_for_replacement(&node_plan)
        .expect("inventory builds");
    let reconciliation = runtime
        .reconcile_durable_state(&node_plan, &inventory)
        .expect("state reconciliation succeeds");
    let query_comparison = runtime
        .compare_query_bindings(&node_plan, &narrowing, &admitted)
        .expect("query comparison succeeds");
    let query_rebind = runtime
        .plan_query_live_rebinds(&query_comparison, &node_plan, &narrowing, &admitted)
        .expect("query rebind plan succeeds");
    let lowering_input = runtime.prepare_pending_execution_plan_lowering_input(
        &node_plan,
        &reconciliation,
        &query_rebind,
    );
    let pending = runtime
        .stage_replacement_activation(
            admitted,
            &impact,
            &narrowing,
            &node_plan,
            Some(&reconciliation),
            Some(&query_rebind),
            Some(&lowering_input),
        )
        .expect("activation staging accepts Query plan for gate evaluation");
    (runtime, pending)
}

pub(super) struct LaneChangeFixture {
    pub(super) runtime: WorthUiRuntimeHost,
    pub(super) pending: WorthUiPendingActivation,
    pub(super) plan_input: WorthUiExecutionPlanInput,
    pub(super) handle_allocation: WorthUiRuntimeHandleAllocation,
    pub(super) candidate_plan: WorthUiExecutionPlan,
    pub(super) parity_report: Option<WorthUiLaneParityReport>,
}

pub(super) fn lane_change_fixture(include_parity: bool) -> LaneChangeFixture {
    let inputs = activation_staging_inputs();
    let runtime = inputs.runtime;
    let node_plan = lane_change_plan(&inputs.node_plan);
    let inventory = platform_inventory(&runtime)
        .build_for_replacement(&node_plan)
        .expect("inventory builds");
    let reconciliation = runtime
        .reconcile_durable_state(&node_plan, &inventory)
        .expect("state reconciliation succeeds");
    let query_comparison = runtime
        .compare_query_bindings(&node_plan, &inputs.narrowing, &inputs.admitted)
        .expect("query comparison succeeds");
    let query_rebind = runtime
        .plan_query_live_rebinds(
            &query_comparison,
            &node_plan,
            &inputs.narrowing,
            &inputs.admitted,
        )
        .expect("query rebind succeeds");
    let lowering_input = runtime.prepare_pending_execution_plan_lowering_input(
        &node_plan,
        &reconciliation,
        &query_rebind,
    );
    let pending = runtime
        .stage_replacement_activation(
            inputs.admitted,
            &inputs.impact,
            &inputs.narrowing,
            &node_plan,
            Some(&reconciliation),
            Some(&query_rebind),
            Some(&lowering_input),
        )
        .expect("lane-change activation stages");
    let plan_input = runtime
        .prepare_execution_plan_input(&pending)
        .expect("plan input prepares");
    let handle_allocation = runtime
        .allocate_runtime_handles(&plan_input)
        .expect("handles allocate");
    let candidate_plan = runtime
        .assemble_execution_plan_topology(&plan_input, &handle_allocation)
        .expect("topology assembles");
    let parity_report = include_parity.then(|| {
        runtime
            .certify_lane_meaning_parity(
                &node_plan,
                &inputs.narrowing,
                &candidate_plan,
                &candidate_plan,
                &query_comparison,
                Some(&query_rebind),
            )
            .expect("lane parity certifies")
    });
    LaneChangeFixture {
        runtime,
        pending,
        plan_input,
        handle_allocation,
        candidate_plan,
        parity_report,
    }
}

fn lane_change_plan(plan: &WorthUiNodeReplacementPlan) -> WorthUiNodeReplacementPlan {
    let mut counters = WorthUiNodeReplacementCounters::default();
    let classifications = plan
        .classifications()
        .iter()
        .map(|classification| {
            counters.record_transition(WorthUiNodeLifecycleTransition::LaneChange);
            WorthUiNodeReplacementClassification::new(
                classification.identity_basis().to_owned(),
                WorthUiNodeLifecycleTransition::LaneChange,
                classification.active_kind(),
                classification.candidate_kind(),
                classification.active_durable_state_eligible(),
                classification.candidate_durable_state_eligible(),
            )
        })
        .collect();
    WorthUiNodeReplacementPlan::new(
        plan.active_artifact_digest(),
        plan.candidate_artifact_digest(),
        classifications,
        counters,
    )
}
