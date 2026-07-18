use super::durable_state_inventory_test_support::platform_inventory;
use crate::runtime::{
    WorthUiAdmittedReplacementCandidate, WorthUiPendingActivation, WorthUiRuntime,
};

pub(super) fn pending_plan_input(
    runtime: &WorthUiRuntime,
    admitted: WorthUiAdmittedReplacementCandidate,
) -> WorthUiPendingActivation {
    let comparison = runtime
        .compare_admitted_replacement(&admitted)
        .expect("runtime comparison succeeds");
    let impact = runtime
        .classify_replacement_impact(&comparison, &admitted)
        .expect("impact classification succeeds");
    let narrowing = runtime
        .narrow_replacement_impact(&impact, &admitted)
        .expect("impact narrowing succeeds");
    let identity_report = runtime
        .build_identity_match_graph(&narrowing, &admitted)
        .expect("identity matching succeeds");
    let node_plan = runtime
        .classify_node_replacements(&impact, &narrowing, &identity_report)
        .expect("node replacement plan succeeds");
    let inventory = platform_inventory(runtime)
        .build_for_replacement(&node_plan)
        .expect("inventory builds");
    let reconciliation_plan = runtime
        .reconcile_durable_state(&node_plan, &inventory)
        .expect("state reconciliation succeeds");
    let query_comparison = runtime
        .compare_query_bindings(&node_plan, &narrowing, &admitted)
        .expect("query comparison succeeds");
    let query_rebind_plan = runtime
        .plan_query_live_rebinds(&query_comparison, &node_plan, &narrowing, &admitted)
        .expect("query rebind planning succeeds");
    let pending_input = runtime.prepare_pending_execution_plan_lowering_input(
        &node_plan,
        &reconciliation_plan,
        &query_rebind_plan,
    );
    runtime
        .stage_replacement_activation(
            admitted,
            &impact,
            &narrowing,
            &node_plan,
            crate::runtime::WorthUiActivationStagingPlans::new(
                Some(&reconciliation_plan),
                Some(&query_rebind_plan),
                Some(&pending_input),
            ),
        )
        .expect("activation staging succeeds")
}
