use worth_query::facade::runtime::WorthQueryGraphReadAccessRequirementKind;

use crate::support::graph_index_inventory::runtime_profiles::{
    profile_with_ephemeral_graph_support, workspace_with_graph_support,
};
use crate::support::graph_read_access::read_surface_assertions::assert_success_counters_are_executor_observed;
use crate::support::graph_read_access::read_surface_declarations::graph_access_family;

#[test]
fn closeout_covered_read_receipt_proves_zero_caller_owned_n_plus_one_paths() {
    let mut workspace = workspace_with_graph_support(
        "graph-read-access.closeout.no-n-plus-one.ephemeral",
        profile_with_ephemeral_graph_support(
            WorthQueryGraphReadAccessRequirementKind::DirectionalAdjacency,
        ),
    );
    let family = graph_access_family(&mut workspace, "closeout-no-n-plus-one");
    let result = workspace
        .execute_read_family(&family)
        .expect("covered read should execute through admitted access plan");
    let counters = result
        .receipt()
        .graph_read_access_complexity_counters()
        .expect("receipt should expose access complexity counters");
    let consumption = result
        .receipt()
        .graph_read_access_plan_consumption()
        .expect("receipt should expose access-plan consumption");
    let ephemeral_receipt = result
        .receipt()
        .ephemeral_graph_index_receipt()
        .expect("ephemeral read should expose provisioning receipt");

    assert_success_counters_are_executor_observed(counters);
    assert_eq!(
        consumption
            .execution_counters()
            .ephemeral_index_allocation_count(),
        ephemeral_receipt.counters().successful_allocation_count()
    );
    assert_eq!(consumption.execution_counters().edge_scan_count(), 0);
    assert_eq!(
        consumption
            .execution_counters()
            .per_result_neighbor_lookup_count(),
        0
    );
    assert_eq!(
        consumption
            .execution_counters()
            .persistent_artifact_bypass_count(),
        0
    );
}
