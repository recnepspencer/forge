use crate::runtime::planning::execution_plan_input::{
    WorthUiChildRangePlanMeaning, WorthUiPlanOrdinaryMeaning, WorthUiRealtimePlanMeaning,
    WorthUiSpatialPlanMeaning,
};
use crate::runtime::planning::plan_topology::WorthUiPlanRegionStore;
use crate::runtime::{
    WorthUiExecutablePlanDecisionKind, WorthUiExecutionLaneSupport, WorthUiExecutionPlanInput,
    WorthUiLaneAdmission, WorthUiNodeLifecycleTransition, WorthUiPlanNodeInput,
    WorthUiPlanNodeInputFamily, WorthUiPlanNodeTopologyInput,
};

use super::plan_topology_test_support::{allocate_handles, assemble, topology_fixture};

#[test]
fn canonical_region_order_is_source_order_independent() {
    let first = child_range_row("workspace.range.first", "workspace.owner.first", 11);
    let second = child_range_row("workspace.range.second", "workspace.owner.second", 12);
    let left = WorthUiPlanRegionStore::launch([first.clone(), second.clone()]).into_store();
    let right = WorthUiPlanRegionStore::launch([second, first]).into_store();

    assert!(left.semantically_matches(&right).0);
    assert_eq!(left.canonical_identities(), right.canonical_identities());
}

#[test]
fn source_provenance_is_non_operational_but_changed_meaning_is_exactly_rejected() {
    let baseline = child_range_row("workspace.range", "workspace.owner", 21);
    let provenance_only = child_range_row("workspace.range", "workspace.owner", 99);
    let changed_meaning = child_range_row("workspace.range", "workspace.other-owner", 21);
    let baseline = WorthUiPlanRegionStore::launch([baseline]).into_store();
    let provenance_only = WorthUiPlanRegionStore::launch([provenance_only]).into_store();
    let changed_meaning = WorthUiPlanRegionStore::launch([changed_meaning]).into_store();

    assert!(baseline.semantically_matches(&provenance_only).0);
    let (matches, counters) = baseline.semantically_matches(&changed_meaning);
    assert!(!matches);
    assert_eq!(
        counters.fingerprint_rejection_count(),
        0,
        "the hostile rows intentionally share a narrowing fingerprint"
    );
    assert_eq!(counters.exact_comparison_count(), 1);
}

#[test]
fn fresh_handle_arenas_do_not_change_executable_equivalence() {
    let (runtime, plan_input, planning, _) = topology_fixture();
    let query_free = query_free(plan_input);
    let left_handles = allocate_handles(&planning, &query_free);
    let right_handles = allocate_handles(&planning, &query_free);
    assert_ne!(
        left_handles.receipt().arena_identity(),
        right_handles.receipt().arena_identity()
    );
    let left = assemble(&planning, &query_free, &left_handles);
    let right = assemble(&planning, &query_free, &right_handles);

    assert_eq!(
        runtime
            .compare_execution_plans(&left, &right)
            .decision_kind(),
        WorthUiExecutablePlanDecisionKind::ExactSemanticNoOp
    );
}

#[test]
fn executable_schema_field_matrix_classifies_every_semantic_constituent() {
    let (_, plan_input, _, _) = topology_fixture();
    let ordinary = child_range_row("workspace.range.matrix", "workspace.owner.matrix", 7);
    let query = plan_input
        .node_inputs()
        .iter()
        .find(|row| row.family() == WorthUiPlanNodeInputFamily::QueryViewBinding)
        .expect("fixture carries installed Query executable meaning")
        .clone();
    assert!(query.query_binding_identity().is_some());
    assert!(query.query_settled_fact_link().is_some());

    let spatial = spatial_row(64);
    let realtime = realtime_row(8, 4, 16);
    let hook = crate::runtime::WorthUiComponentLoweringHook::registered(
        "platform.hook.equivalence",
        WorthUiPlanNodeInputFamily::Accessibility,
    );
    let hook =
        WorthUiPlanNodeInput::from_component_hook(&hook, WorthUiPlanNodeInputFamily::Accessibility);
    let cases = vec![
        schema_case(
            "source provenance excluded",
            ordinary.clone(),
            ordinary
                .clone()
                .with_authored_provenance_digest_for_test(99),
            true,
        ),
        schema_case(
            "lifecycle transition excluded",
            ordinary.clone(),
            ordinary
                .clone()
                .with_transition_for_test(WorthUiNodeLifecycleTransition::Replace),
            true,
        ),
        schema_case(
            "identity",
            ordinary.clone(),
            ordinary
                .clone()
                .with_identity_basis_for_test("workspace.other.identity"),
            false,
        ),
        schema_case(
            "lane family",
            hook.clone(),
            hook.with_family_for_test(WorthUiPlanNodeInputFamily::TokenStyle),
            false,
        ),
        schema_case(
            "topology",
            ordinary.clone(),
            ordinary.clone().with_declared_topology_input_for_test(),
            false,
        ),
        schema_case(
            "owner",
            ordinary.clone(),
            ordinary
                .clone()
                .with_owner_identity_basis_for_test("workspace.other.owner"),
            false,
        ),
        schema_case(
            "owned region set",
            ordinary.clone(),
            ordinary
                .clone()
                .with_owned_region_identity_for_test("workspace.region.additional"),
            false,
        ),
        schema_case(
            "ordinary and durable-state meaning",
            ordinary.clone(),
            ordinary.clone().without_ordinary_meaning_for_test(),
            false,
        ),
        schema_case(
            "spatial hook/resource meaning",
            spatial.clone(),
            spatial_row(65),
            false,
        ),
        schema_case(
            "realtime hook/resource/frame policy",
            realtime.clone(),
            realtime_row(8, 5, 16),
            false,
        ),
        schema_case(
            "Query binding identity",
            query.clone(),
            query.clone().without_query_binding_identity_for_test(),
            false,
        ),
        schema_case(
            "Query settled fact link",
            query.clone(),
            query.clone().without_query_installed_reference_for_test(),
            false,
        ),
    ];

    for case in cases {
        assert_eq!(
            case.left.executable_schema_matches(&case.right),
            case.expected_equivalent,
            "schema matrix row `{}` drifted",
            case.name
        );
    }
}

#[test]
fn lane_admission_equivalence_includes_support_but_excludes_provenance() {
    let (runtime, plan_input, planning, _) = super::lane_admission_fixture::lane_fixture();
    let baseline = super::lane_admission_fixture::admit_lanes(
        &runtime,
        &planning,
        &plan_input,
        &WorthUiExecutionLaneSupport::platform_default(),
    );
    let same_contract_new_basis = WorthUiLaneAdmission::new(
        baseline.rows().to_vec(),
        baseline.query_fact_links().to_vec(),
        baseline.plan_input_basis_digest().wrapping_add(1),
        baseline.counters(),
    );
    let mut fewer_rows = baseline.rows().to_vec();
    fewer_rows.pop().expect("fixture admits supported lanes");
    let changed_lane_support = WorthUiLaneAdmission::new(
        fewer_rows,
        baseline.query_fact_links().to_vec(),
        baseline.plan_input_basis_digest(),
        baseline.counters(),
    );

    let cases = [
        ("identical", &baseline, true),
        ("plan-input provenance", &same_contract_new_basis, true),
        ("lane support row", &changed_lane_support, false),
    ];
    for (name, candidate, expected) in cases {
        assert_eq!(
            baseline.executable_contract_matches(candidate),
            expected,
            "lane-admission matrix row `{name}` drifted"
        );
    }

    if !baseline.query_fact_links().is_empty() {
        let changed_query_links = WorthUiLaneAdmission::new(
            baseline.rows().to_vec(),
            Vec::new(),
            baseline.plan_input_basis_digest(),
            baseline.counters(),
        );
        assert!(!baseline.executable_contract_matches(&changed_query_links));
    }
}

struct ExecutableSchemaCase {
    name: &'static str,
    left: WorthUiPlanNodeInput,
    right: WorthUiPlanNodeInput,
    expected_equivalent: bool,
}

fn schema_case(
    name: &'static str,
    left: WorthUiPlanNodeInput,
    right: WorthUiPlanNodeInput,
    expected_equivalent: bool,
) -> ExecutableSchemaCase {
    ExecutableSchemaCase {
        name,
        left,
        right,
        expected_equivalent,
    }
}

fn spatial_row(visible_primitives: u32) -> WorthUiPlanNodeInput {
    let descriptor = component_descriptor("workspace.component.equivalence_canvas");
    let contract = crate::capability::ComponentCanvasSpatialContract::new(visible_primitives, 2, 1)
        .expect("spatial matrix contract");
    WorthUiPlanNodeInput::from_spatial_component(
        "workspace.component.equivalence_canvas".to_owned(),
        None,
        WorthUiNodeLifecycleTransition::Create,
        WorthUiPlanNodeTopologyInput::empty(),
        WorthUiSpatialPlanMeaning::new(descriptor, contract),
    )
}

fn realtime_row(rows: u16, cost: u16, budget: u32) -> WorthUiPlanNodeInput {
    let descriptor = component_descriptor("workspace.component.equivalence_realtime");
    let contract = crate::capability::ComponentRealtimeOverlayContract::new(
        rows,
        cost,
        budget,
        crate::capability::ComponentRealtimeOverlayPriority::HudOverlay,
    )
    .expect("realtime matrix contract");
    WorthUiPlanNodeInput::from_realtime_component(
        "workspace.component.equivalence_realtime".to_owned(),
        None,
        WorthUiNodeLifecycleTransition::Create,
        WorthUiPlanNodeTopologyInput::empty(),
        WorthUiRealtimePlanMeaning::new(descriptor, contract),
    )
}

fn component_descriptor(identity: &str) -> crate::capability::ComponentDescriptor {
    crate::capability::ComponentDescriptor::new(
        crate::capability::ComponentId::new(identity).expect("matrix component identity"),
        crate::capability::ComponentPropSchema::named("equivalence.matrix.props"),
        crate::capability::ComponentChildPolicy::no_children(),
        crate::capability::ComponentStateOwnership::runtime_owned(),
    )
}

fn child_range_row(identity: &str, owner: &str, provenance: u64) -> WorthUiPlanNodeInput {
    WorthUiPlanNodeInput::from_ordinary_row(
        identity.to_owned(),
        Some(provenance),
        WorthUiPlanNodeInputFamily::ChildRange,
        WorthUiNodeLifecycleTransition::Create,
        WorthUiPlanNodeTopologyInput::empty(),
        None,
        WorthUiPlanOrdinaryMeaning::ChildRange(WorthUiChildRangePlanMeaning::new(
            owner.to_owned(),
            Vec::new(),
        )),
    )
}

fn query_free(input: WorthUiExecutionPlanInput) -> WorthUiExecutionPlanInput {
    let rows = input
        .node_inputs()
        .iter()
        .filter(|row| row.family() != WorthUiPlanNodeInputFamily::QueryViewBinding)
        .cloned()
        .collect();
    WorthUiExecutionPlanInput::new(
        input.basis().clone(),
        input.context().clone(),
        rows,
        input.counters(),
    )
}
