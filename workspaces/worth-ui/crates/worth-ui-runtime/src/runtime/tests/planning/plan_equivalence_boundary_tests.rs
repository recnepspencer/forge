use super::allocation_planning_test_support::{
    admitted_measurement_basis, independent_allocation_planning, planning_graph_authority,
};
use super::durable_state_inventory_test_support::platform_inventory;
use super::plan_equivalence_topology_test_support::execution_plan_with_topology;
use super::query_binding_comparison_test_support::{
    fresh_standard_query_app, query_artifact, standard_query_app,
};
use super::replacement_impact_test_support::admitted_candidate;
use crate::facade::WorthUiApp;
use crate::runtime::WorthUiPendingActivation;
use crate::runtime::{
    WorthUiExecutablePlanDecisionKind, WorthUiExecutionPlan, WorthUiExecutionPlanInput,
    WorthUiPlanChildRange, WorthUiPlanExecutionLane, WorthUiPlanLanePartition, WorthUiPlanNode,
    WorthUiPlanNodeInputFamily, WorthUiPlanTopology, WorthUiRenderResourceRef, WorthUiRuntime,
    WorthUiRuntimeDiagnosticPolicy, WorthUiRuntimeLaunch,
};

#[test]
fn equal_definitions_from_independent_query_installations_are_not_reusable() {
    let (left_runtime, left_plan) = execution_plan_fixture_with_fresh_query_installation();
    let (_, right_plan) = execution_plan_fixture_with_fresh_query_installation();

    assert_ne!(
        left_plan.handle_receipt().arena_identity(),
        right_plan.handle_receipt().arena_identity(),
        "independent runtime sessions must own distinct handle arenas"
    );

    let equivalence = left_runtime.compare_execution_plans(&left_plan, &right_plan);

    assert_eq!(equivalence.previous_digest(), equivalence.next_digest());
    assert_eq!(
        equivalence.decision_kind(),
        WorthUiExecutablePlanDecisionKind::RebuildRequired
    );
    assert_eq!(equivalence.counters().plan_node_digest_count(), 0);
    assert_eq!(equivalence.counters().artifact_tree_scan_count(), 0);
    assert_eq!(
        equivalence.counters().pointer_identity_comparison_count(),
        0
    );
}

fn execution_plan_fixture_with_fresh_query_installation() -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    WorthUiExecutionPlan,
) {
    let (runtime, plan_input, planning, allocation) =
        super::plan_topology_test_support::topology_fixture_with_app(fresh_standard_query_app());
    let plan = super::plan_topology_test_support::assemble(&planning, &plan_input, &allocation);
    (runtime, plan)
}

#[test]
fn lane_or_handle_meaning_change_changes_equivalence() {
    let (runtime, stable_plan) = execution_plan_fixture();
    let (changed_runtime, changed_input) =
        plan_input_with_first_family(WorthUiPlanNodeInputFamily::TokenStyle);
    let changed_planning = independent_allocation_planning("plan-equivalence.changed");
    let changed_receipt = changed_runtime
        .commit_allocation_candidate_for_test(changed_planning)
        .expect("independently admitted changed meaning commits");
    let changed_facts = changed_runtime.execution_plan_lowering_facts_below_authority_for_test(
        changed_receipt
            .lowering_input()
            .expect("changed receipt admits execution lowering"),
        changed_input,
    );
    let changed_allocation = changed_runtime
        .allocate_runtime_handles(&changed_facts)
        .expect("changed handles allocate");
    let changed_plan = changed_runtime
        .assemble_execution_plan_topology(&changed_facts, &changed_allocation)
        .expect("changed topology assembles");

    let equivalence = runtime.compare_execution_plans(&stable_plan, &changed_plan);

    assert_eq!(
        equivalence.decision_kind(),
        WorthUiExecutablePlanDecisionKind::RebuildRequired
    );
    assert_ne!(
        equivalence
            .previous_digest()
            .basis()
            .executable_shape_fingerprint(),
        equivalence
            .next_digest()
            .basis()
            .executable_shape_fingerprint()
    );
}

#[test]
fn diagnostic_policy_does_not_change_plan_digest() {
    let (minimal_runtime, minimal_plan) =
        execution_plan_fixture_with_diagnostic_policy(WorthUiRuntimeDiagnosticPolicy::minimal());
    let (rich_runtime, rich_plan) =
        execution_plan_fixture_with_diagnostic_policy(WorthUiRuntimeDiagnosticPolicy::rich());

    let minimal_digest = minimal_runtime.digest_execution_plan(&minimal_plan);
    let rich_digest = rich_runtime.digest_execution_plan(&rich_plan);
    let equivalence = rich_runtime.compare_execution_plans(&minimal_plan, &rich_plan);

    assert_eq!(minimal_digest, rich_digest);
    assert_eq!(equivalence.counters().diagnostic_policy_read_count(), 0);
    assert_eq!(
        equivalence.decision_kind(),
        WorthUiExecutablePlanDecisionKind::ExactSemanticNoOp
    );
}

#[test]
fn reconstructive_lane_partition_tampering_cannot_override_regional_equivalence() {
    let (runtime, plan) = execution_plan_fixture();
    let repartitioned = plan_with_first_node_moved_to_next_lane(&plan);

    let original_digest = runtime.digest_execution_plan(&plan);
    let repartitioned_digest = runtime.digest_execution_plan(&repartitioned);
    let equivalence = runtime.compare_execution_plans(&plan, &repartitioned);

    assert_ne!(original_digest, repartitioned_digest);
    assert_ne!(
        original_digest.basis().executable_shape_fingerprint(),
        repartitioned_digest.basis().executable_shape_fingerprint()
    );
    assert_eq!(
        equivalence.decision_kind(),
        WorthUiExecutablePlanDecisionKind::ExactSemanticNoOp
    );
    assert_eq!(equivalence.counters().artifact_tree_scan_count(), 0);
}

#[test]
fn plan_equivalence_digest_covers_lookup_egui_and_render_surfaces() {
    let (runtime, plan) = execution_plan_fixture();
    let equivalence = runtime.compare_execution_plans(&plan, &plan);
    let basis = equivalence.previous_digest().basis();

    assert_eq!(
        basis.lookup_entry_count(),
        plan.lookup_index().entry_count()
    );
    assert_eq!(basis.lane_partition_count(), plan.lane_partitions().len());
    assert_eq!(equivalence.counters().lookup_index_digest_count(), 0);
    assert_eq!(equivalence.counters().equivalence_comparison_count(), 1);
}

#[test]
fn reconstructive_child_range_tampering_cannot_override_regional_equivalence() {
    let (runtime, plan) = execution_plan_fixture();
    let changed_topology = plan_with_added_child_range_topology(&plan);

    let original_digest = runtime.digest_execution_plan(&plan);
    let changed_digest = runtime.digest_execution_plan(&changed_topology);
    let equivalence = runtime.compare_execution_plans(&plan, &changed_topology);

    assert_eq!(
        original_digest.basis().plan_node_count(),
        changed_digest.basis().plan_node_count()
    );
    assert_ne!(original_digest, changed_digest);
    assert_eq!(
        equivalence.decision_kind(),
        WorthUiExecutablePlanDecisionKind::ExactSemanticNoOp
    );
}

#[test]
fn reconstructive_render_ref_tampering_cannot_override_regional_equivalence() {
    let (runtime, plan) = execution_plan_fixture();
    let changed_render_ref = plan_with_first_node_render_resource_ref_changed(&plan);

    let original_digest = runtime.digest_execution_plan(&plan);
    let changed_digest = runtime.digest_execution_plan(&changed_render_ref);
    let equivalence = runtime.compare_execution_plans(&plan, &changed_render_ref);

    assert_ne!(original_digest, changed_digest);
    assert_eq!(
        changed_digest.basis().render_resource_ref_count(),
        original_digest.basis().render_resource_ref_count() + 1
    );
    assert_eq!(
        equivalence.decision_kind(),
        WorthUiExecutablePlanDecisionKind::ExactSemanticNoOp
    );
}

fn execution_plan_fixture() -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    WorthUiExecutionPlan,
) {
    let (runtime, plan_input, planning, allocation) =
        super::plan_topology_test_support::topology_fixture();
    let plan = super::plan_topology_test_support::assemble(&planning, &plan_input, &allocation);
    (runtime, plan)
}

fn plan_input_with_first_family(
    family: WorthUiPlanNodeInputFamily,
) -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    WorthUiExecutionPlanInput,
) {
    let (runtime, plan_input, _, _) = super::plan_topology_test_support::topology_fixture();
    let mut node_inputs = plan_input.node_inputs().to_vec();
    let index = node_inputs
        .iter()
        .position(|input| input.family() != family)
        .expect("fixture includes a different plan input family");
    node_inputs[index] = node_inputs[index].clone().with_family_for_test(family);
    (
        runtime,
        WorthUiExecutionPlanInput::new(
            plan_input.basis().clone(),
            plan_input.context().clone(),
            node_inputs,
            plan_input.counters(),
        ),
    )
}

fn execution_plan_fixture_with_diagnostic_policy(
    policy: WorthUiRuntimeDiagnosticPolicy,
) -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    WorthUiExecutionPlan,
) {
    let app = standard_query_app();
    let active = query_artifact(&app, "workspace.view_binding.selection");
    let candidate = query_artifact(&app, "workspace.view_binding.selection");
    let runtime = launch_runtime_with_policy(&app, active, policy);
    let pending = stage_query_replacement_for_policy_runtime(&app, &runtime, candidate);
    let plan = assemble_plan_from_pending_activation(&runtime, pending);
    (runtime, plan)
}

fn stage_query_replacement_for_policy_runtime(
    app: &WorthUiApp,
    runtime: &WorthUiRuntime,
    candidate: crate::source::WorthUiArtifact,
) -> WorthUiPendingActivation {
    let admitted = admitted_candidate(app, runtime, candidate);
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
    runtime
        .stage_replacement_activation(
            admitted,
            &impact,
            &narrowing,
            &node_plan,
            crate::runtime::WorthUiActivationStagingPlans::new(
                Some(&reconciliation_plan),
                Some(&query_rebind_plan),
            ),
        )
        .expect("activation staging succeeds")
}

fn assemble_plan_from_pending_activation(
    runtime: &WorthUiRuntime,
    pending: WorthUiPendingActivation,
) -> WorthUiExecutionPlan {
    let plan_input = runtime
        .prepare_execution_plan_input(&pending)
        .expect("plan input prepares after candidate staging");
    let measurement_basis = admitted_measurement_basis("plan-equivalence.fixture");
    let (snapshot, selected) =
        planning_graph_authority("plan-equivalence.fixture", "operator:stack");
    let planning = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, measurement_basis, &selected)
            .expect("equivalence planning admits through graph authority"),
    );
    let facts = runtime.detached_execution_plan_lowering_facts_for_test(&planning, plan_input);
    let allocation = runtime
        .allocate_runtime_handles(&facts)
        .expect("handles allocate");
    runtime
        .assemble_execution_plan_topology(&facts, &allocation)
        .expect("topology assembles")
}

fn launch_runtime_with_policy(
    app: &WorthUiApp,
    artifact: crate::source::WorthUiArtifact,
    policy: WorthUiRuntimeDiagnosticPolicy,
) -> crate::runtime::WorthUiRuntimeFrameworkLoop {
    app.launch_runtime(
        WorthUiRuntimeLaunch::from_canonical_artifact(artifact).with_diagnostics(policy),
    )
    .expect("runtime launches")
}

fn plan_with_first_node_moved_to_next_lane(plan: &WorthUiExecutionPlan) -> WorthUiExecutionPlan {
    let mut partitions = plan.lane_partitions().to_vec();
    let source = partitions
        .iter()
        .position(|partition| !partition.plan_indexes().is_empty())
        .expect("fixture has a populated source lane");
    let current_lane = partitions[source].lane();
    let indexes = partitions[source].plan_indexes().to_vec();
    let alternate_lane = match current_lane {
        WorthUiPlanExecutionLane::QueryView => WorthUiPlanExecutionLane::UiStructure,
        _ => WorthUiPlanExecutionLane::QueryView,
    };
    partitions[source] = WorthUiPlanLanePartition::new(alternate_lane, indexes);

    plan.with_test_parts(
        plan.topology().clone(),
        partitions,
        plan.lookup_index().clone(),
        plan.counters(),
    )
}

fn plan_with_added_child_range_topology(plan: &WorthUiExecutionPlan) -> WorthUiExecutionPlan {
    let mut traversal_order = plan.topology().traversal_order().to_vec();
    let mut child_ranges = plan.topology().child_ranges().to_vec();
    let first_plan_index = traversal_order
        .first()
        .expect("fixture has plan nodes")
        .runtime_handle()
        .plan_index();
    let added_range = WorthUiPlanChildRange::from_root_region_count(first_plan_index, 1);
    child_ranges.push(added_range);
    if traversal_order[0].child_range().is_none() {
        traversal_order[0] =
            plan_node_with_child_range(traversal_order[0].clone(), Some(added_range));
    }

    execution_plan_with_topology(
        plan,
        WorthUiPlanTopology::new(traversal_order, child_ranges),
    )
}

fn plan_node_with_child_range(
    node: WorthUiPlanNode,
    child_range: Option<WorthUiPlanChildRange>,
) -> WorthUiPlanNode {
    WorthUiPlanNode::new(
        node.runtime_handle(),
        node.family(),
        child_range,
        node.region_structure(),
        node.render_resource_ref(),
    )
}

fn plan_with_first_node_render_resource_ref_changed(
    plan: &WorthUiExecutionPlan,
) -> WorthUiExecutionPlan {
    let mut traversal_order = plan.topology().traversal_order().to_vec();
    let node_index = traversal_order
        .iter()
        .position(|node| node.render_resource_ref().is_none())
        .expect("fixture has a plan node without a render-resource ref");
    let node = traversal_order[node_index].clone();
    let render_ref = WorthUiRenderResourceRef::new(node.runtime_handle().locator());
    traversal_order[node_index] = WorthUiPlanNode::new(
        node.runtime_handle(),
        node.family(),
        node.child_range(),
        node.region_structure(),
        Some(render_ref),
    );
    execution_plan_with_topology(
        plan,
        WorthUiPlanTopology::new(traversal_order, plan.topology().child_ranges().to_vec()),
    )
}
