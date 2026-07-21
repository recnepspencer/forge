use crate::runtime::planning::plan_topology::{
    WorthUiPlanRegionIdentity, WorthUiPlanRegionMutation, WorthUiPlanRegionSchema,
    WorthUiPlanRegionStore, WorthUiPlanRegionTransition,
};
use crate::runtime::WorthUiPlanNodeInput;
use crate::runtime::WorthUiPlanNodeInputFamily;

#[test]
fn lawful_delta_orders_converge_on_canonical_complete_successor() {
    let inputs = fixture_inputs();
    let base = WorthUiPlanRegionStore::launch(inputs.clone()).into_store();
    let first = inserted_schema(&inputs[0], "delta.first");
    let second = inserted_schema(&inputs[0], "delta.second");

    let left = base
        .successor(vec![
            WorthUiPlanRegionMutation::Upsert(first.clone()),
            WorthUiPlanRegionMutation::Upsert(second.clone()),
        ])
        .into_store();
    let right = base
        .successor(vec![
            WorthUiPlanRegionMutation::Upsert(second),
            WorthUiPlanRegionMutation::Upsert(first),
        ])
        .into_store();

    assert_eq!(left.canonical_identities(), right.canonical_identities());
    for identity in left.canonical_identities() {
        assert_eq!(left.handle_for(&identity), right.handle_for(&identity));
    }
}

#[test]
fn equal_narrowing_fingerprints_cannot_authorize_structural_reuse() {
    let active_app = super::query_binding_comparison_test_support::standard_query_app();
    let candidate_app = super::query_binding_comparison_test_support::lifecycle_drift_query_app();
    let active_artifact = super::query_binding_comparison_test_support::query_artifact(
        &active_app,
        "workspace.view_binding.selection",
    );
    let candidate_artifact = super::query_binding_comparison_test_support::query_artifact(
        &candidate_app,
        "workspace.view_binding.selection",
    );
    let active_runtime =
        super::replacement_impact_test_support::launch_runtime(&active_app, active_artifact);
    let candidate_runtime =
        super::replacement_impact_test_support::launch_runtime(&candidate_app, candidate_artifact);
    let identity = WorthUiPlanRegionIdentity::from_exact_basis("workspace.view_binding.selection");
    let active_plan = active_runtime.active.active_plan();
    let candidate_plan = candidate_runtime.active.active_plan();
    let previous = active_plan
        .exact_plan()
        .region_store()
        .schema_for(&identity)
        .expect("active production plan contains the query region")
        .clone();
    let candidate = candidate_plan
        .exact_plan()
        .region_store()
        .schema_for(&identity)
        .expect("candidate production plan contains the query region")
        .clone();
    assert_eq!(
        previous.narrowing_fingerprint(),
        candidate.narrowing_fingerprint(),
        "two valid production plans deliberately collide at the narrowing layer"
    );
    assert!(!previous.exactly_matches_after_narrowing(&candidate));

    let successor = active_plan
        .exact_plan()
        .region_store()
        .successor(vec![WorthUiPlanRegionMutation::Upsert(candidate)]);
    assert_eq!(successor.counters().exact_comparison_count(), 1);
    assert_eq!(successor.counters().reuse_count(), 0);
    assert_eq!(
        successor.evidence()[0].transition(),
        WorthUiPlanRegionTransition::Replaced
    );
}

#[test]
fn one_changed_region_cost_is_independent_of_unrelated_predecessor_scale() {
    let inputs = fixture_inputs();
    let small = scaled_inputs(&inputs, 2);
    let large = scaled_inputs(&inputs, 200);
    let small_result = changed_query_successor(small);
    let large_result = changed_query_successor(large);

    assert_eq!(
        small_result.counters().region_construction_count(),
        large_result.counters().region_construction_count()
    );
    assert_eq!(
        small_result.counters().exact_comparison_count(),
        large_result.counters().exact_comparison_count()
    );
    assert_eq!(
        small_result.counters().trie_node_copy_count(),
        large_result.counters().trie_node_copy_count()
    );
    assert_eq!(
        small_result.counters().storage_pointer_copy_count(),
        large_result.counters().storage_pointer_copy_count()
    );
    assert_eq!(large_result.counters().trie_node_copy_count(), 68);
}

#[test]
fn unchanged_slots_share_storage_while_changed_and_retired_handles_go_stale() {
    let inputs = scaled_inputs(&fixture_inputs(), 2);
    let base = WorthUiPlanRegionStore::launch(inputs.clone()).into_store();
    let arena_identity = super::plan_topology_test_support::topology_fixture()
        .3
        .receipt()
        .arena_identity();
    let unchanged_identity =
        WorthUiPlanRegionIdentity::from_exact_basis(inputs[1].identity_basis());
    let changed_input = query_input(&inputs).clone();
    let changed_identity =
        WorthUiPlanRegionIdentity::from_exact_basis(changed_input.identity_basis());
    let stale = base
        .handle_for(&changed_identity)
        .expect("query region exists")
        .clone();
    let unchanged_before = base
        .runtime_handle_for_stable_slot(
            base.handle_for(&unchanged_identity)
                .expect("unchanged region exists")
                .stable_slot(),
            arena_identity,
        )
        .expect("unchanged runtime locator resolves");
    let stale_runtime = base
        .runtime_handle_for_stable_slot(stale.stable_slot(), arena_identity)
        .expect("changed predecessor runtime locator resolves");
    let successor = base
        .successor(vec![WorthUiPlanRegionMutation::Upsert(
            WorthUiPlanRegionSchema::from_node_input(
                changed_input.with_family_for_test(WorthUiPlanNodeInputFamily::TokenStyle),
            ),
        )])
        .into_store();

    assert!(base.shares_exact_region_storage_with(&successor, &unchanged_identity));
    assert_eq!(
        successor
            .runtime_handle_for_stable_slot(unchanged_before.plan_index().into(), arena_identity)
            .expect("unchanged successor locator resolves"),
        unchanged_before
    );
    assert!(!successor.resolves(&stale));
    let fresh = successor
        .handle_for(&changed_identity)
        .expect("replacement region exists");
    assert_eq!(fresh.stable_slot(), stale.stable_slot());
    assert_eq!(fresh.slot_generation(), stale.slot_generation() + 1);
    let fresh_runtime = successor
        .runtime_handle_for_stable_slot(fresh.stable_slot(), arena_identity)
        .expect("replacement runtime locator resolves");
    assert_eq!(fresh_runtime.plan_index(), stale_runtime.plan_index());
    assert_eq!(
        fresh_runtime.arena_identity(),
        stale_runtime.arena_identity()
    );
    assert_ne!(
        fresh_runtime.slot_generation(),
        stale_runtime.slot_generation()
    );
    assert_eq!(
        successor.handle_for_stable_slot(stale.stable_slot()),
        Some(fresh),
        "the execution index resolves the stable slot directly to its current generation"
    );

    let retired = successor
        .successor(vec![WorthUiPlanRegionMutation::Retire(changed_identity)])
        .into_store();
    assert!(!retired.resolves(fresh));
}

#[test]
fn reparent_authority_retires_a_slot_even_when_the_early_schema_is_equal() {
    let inputs = fixture_inputs();
    let original = inputs[0].clone();
    let identity = WorthUiPlanRegionIdentity::from_exact_basis(original.identity_basis());
    let predecessor = WorthUiPlanRegionStore::launch(inputs).into_store();
    let stale = predecessor
        .handle_for(&identity)
        .expect("predecessor region exists")
        .clone();
    let successor = predecessor.successor(vec![WorthUiPlanRegionMutation::Reparent(
        WorthUiPlanRegionSchema::from_node_input(original),
    )]);

    assert_eq!(successor.counters().exact_comparison_count(), 0);
    assert_eq!(
        successor.evidence()[0].transition(),
        WorthUiPlanRegionTransition::Reparented
    );
    let successor = successor.into_store();
    assert!(!successor.resolves(&stale));
    assert_eq!(
        successor
            .handle_for(&identity)
            .expect("reparented region exists")
            .stable_slot(),
        stale.stable_slot()
    );
}

#[test]
fn lane_transition_authority_retires_the_predecessor_slot_generation() {
    let inputs = fixture_inputs();
    let original = inputs[0].clone();
    let identity = WorthUiPlanRegionIdentity::from_exact_basis(original.identity_basis());
    let predecessor = WorthUiPlanRegionStore::launch(inputs).into_store();
    let stale = predecessor
        .handle_for(&identity)
        .expect("predecessor region exists")
        .clone();
    let successor = predecessor.successor(vec![WorthUiPlanRegionMutation::LaneTransition(
        WorthUiPlanRegionSchema::from_node_input(original),
    )]);

    assert_eq!(
        successor.evidence()[0].transition(),
        WorthUiPlanRegionTransition::LaneTransitioned
    );
    let successor = successor.into_store();
    assert!(!successor.resolves(&stale));
    assert_eq!(
        successor
            .handle_for(&identity)
            .expect("lane-transitioned region exists")
            .stable_slot(),
        stale.stable_slot()
    );
}

#[test]
fn lifecycle_lane_change_lowers_to_a_distinct_regional_transition() {
    let inputs = super::activation_staging_test_support::activation_staging_inputs();
    let node_plan = inputs.node_plan.clone();
    let (_, plan_input, _, _) = super::plan_topology_test_support::topology_fixture();
    let changed_identity = plan_input.node_inputs()[0].identity_basis().to_owned();
    let node_inputs = plan_input
        .node_inputs()
        .iter()
        .cloned()
        .map(|input| {
            if input.identity_basis() == changed_identity {
                input.with_transition_for_test(
                    crate::runtime::WorthUiNodeLifecycleTransition::LaneChange,
                )
            } else {
                input
            }
        })
        .collect();
    let lane_changed_input = crate::runtime::WorthUiExecutionPlanInput::new(
        plan_input.basis().clone(),
        plan_input.context().clone(),
        node_inputs,
        plan_input.counters(),
    );
    let delta = crate::runtime::planning::plan_topology::WorthUiPlanRegionDelta::from_replacement(
        &node_plan,
        &lane_changed_input,
        0x1234,
        0x5678,
    )
    .expect("lane-changing regional delta seals");

    assert!(delta.mutations().iter().any(|mutation| {
        matches!(mutation, WorthUiPlanRegionMutation::LaneTransition(schema)
            if schema.identity().exact_basis() == changed_identity)
    }));
}

#[test]
fn churn_reclaims_predecessor_only_paths_instead_of_retaining_history() {
    let inputs = scaled_inputs(&fixture_inputs(), 100);
    let query = query_input(&inputs).clone();
    let mut active = WorthUiPlanRegionStore::launch(inputs.clone()).into_store();
    for generation in 0..500 {
        let family = if generation % 2 == 0 {
            WorthUiPlanNodeInputFamily::TokenStyle
        } else {
            WorthUiPlanNodeInputFamily::ComponentInvocation
        };
        let changed = query.clone().with_family_for_test(family);
        active = active
            .successor(vec![WorthUiPlanRegionMutation::Upsert(
                WorthUiPlanRegionSchema::from_node_input(changed),
            )])
            .into_store();
    }

    assert_eq!(active.region_count(), 100);
    let expected_inputs = inputs
        .into_iter()
        .map(|input| {
            if input.identity_basis() == query.identity_basis() {
                input.with_family_for_test(WorthUiPlanNodeInputFamily::ComponentInvocation)
            } else {
                input
            }
        })
        .collect::<Vec<_>>();
    let history_free = WorthUiPlanRegionStore::launch(expected_inputs).into_store();
    assert_eq!(
        active.retained_storage_node_count(),
        history_free.retained_storage_node_count(),
        "retained nodes equal a fresh store for the same final family distribution"
    );
}

fn fixture_inputs() -> Vec<WorthUiPlanNodeInput> {
    super::plan_topology_test_support::topology_fixture()
        .1
        .node_inputs()
        .to_vec()
}

fn query_input(inputs: &[WorthUiPlanNodeInput]) -> &WorthUiPlanNodeInput {
    inputs
        .iter()
        .find(|input| input.query_binding_identity().is_some())
        .expect("fixture includes a query region")
}

fn inserted_schema(input: &WorthUiPlanNodeInput, identity: &str) -> WorthUiPlanRegionSchema {
    WorthUiPlanRegionSchema::from_node_input(input.clone().with_identity_basis_for_test(identity))
}

fn scaled_inputs(inputs: &[WorthUiPlanNodeInput], count: usize) -> Vec<WorthUiPlanNodeInput> {
    let query = query_input(inputs).clone();
    std::iter::once(query.clone())
        .chain((1..count).map(|index| {
            query
                .clone()
                .with_identity_basis_for_test(format!("unrelated.region.{index:04}"))
        }))
        .collect()
}

fn changed_query_successor(
    inputs: Vec<WorthUiPlanNodeInput>,
) -> crate::runtime::planning::plan_topology::WorthUiPlanRegionSuccessor {
    let changed = query_input(&inputs)
        .clone()
        .with_family_for_test(WorthUiPlanNodeInputFamily::TokenStyle);
    WorthUiPlanRegionStore::launch(inputs)
        .into_store()
        .successor(vec![WorthUiPlanRegionMutation::Upsert(
            WorthUiPlanRegionSchema::from_node_input(changed),
        )])
}
