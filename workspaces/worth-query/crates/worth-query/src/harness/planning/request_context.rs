use super::{bound_validated_bundle, direct_validated_bundle, runtime_basis_intent};
use crate::binding::resolve_bindings;
use crate::facade::foundation::{
    BoundBinding, BoundBindings, QueryBindingSlot, QueryBindingSubject,
};
use crate::facade::policy::{
    planning_request_context_for_bound, planning_request_context_for_direct,
};

#[test]
fn direct_planning_request_context_requires_no_binding_resolution() {
    let bundle = direct_validated_bundle();
    let request_context =
        planning_request_context_for_direct(&bundle, runtime_basis_intent()).unwrap();
    assert!(request_context.semantic().binding_resolution().is_none());
}

#[test]
fn bound_planning_request_context_resolves_through_query_owned_requirements() {
    let bundle = bound_validated_bundle();
    let request_context = planning_request_context_for_bound(
        &bundle,
        runtime_basis_intent(),
        BoundBindings::new(vec![BoundBinding::new(
            QueryBindingSlot::new("root").unwrap(),
            QueryBindingSubject::RootEntity,
            "user-1",
        )]),
        Vec::new(),
    )
    .unwrap();

    let resolution = request_context.semantic().binding_resolution().unwrap();
    assert_eq!(resolution.requirements().requirements().len(), 1);
    assert_eq!(resolution.bindings().bindings().len(), 1);
}

#[test]
fn direct_and_pre_resolved_bound_requests_seed_identical_plans_for_same_semantics() {
    let bundle = direct_validated_bundle();
    let direct = planning_request_context_for_direct(&bundle, runtime_basis_intent()).unwrap();
    let seeded_direct = crate::facade::policy::plan_validated_bundle(&bundle, direct).unwrap();

    let requirements = crate::facade::foundation::derive_binding_requirements(&bundle);
    let bound_resolution = resolve_bindings(requirements, BoundBindings::new(Vec::new())).unwrap();
    let bound = crate::facade::policy::PlanningRequestContext::new(
        crate::facade::policy::PlanningSemanticInputs::new(
            Some(bound_resolution),
            runtime_basis_intent(),
        ),
        crate::facade::policy::PlanningAmbientContext::new(Vec::new()),
    );
    let seeded_bound = crate::facade::policy::plan_validated_bundle(&bundle, bound).unwrap();

    assert_eq!(
        seeded_direct.query().plan_digest(),
        seeded_bound.query().plan_digest()
    );
    assert_eq!(seeded_direct.query().projection_count(), 1);
    assert_eq!(seeded_direct.query().traversal_count(), 0);
    assert_eq!(seeded_direct.query().predicate_count(), 0);
    assert_eq!(seeded_direct.query().ordering_count(), 0);
    assert_eq!(seeded_direct.result_shape().binding_count(), 1);
    assert_eq!(seeded_direct.report().projection_count(), 1);
    assert_eq!(seeded_direct.report().result_shape_binding_count(), 1);
    assert_eq!(seeded_direct.counters().planned_projection_entry_count(), 1);
    assert_eq!(seeded_direct.counters().planned_traversal_clause_count(), 0);
    assert_eq!(seeded_direct.counters().route_candidate_count(), 2);
    assert_eq!(seeded_direct.counters().planned_read_surface_count(), 1);
    assert_eq!(seeded_direct.counters().fallback_denial_count(), 0);
    assert_eq!(
        seeded_direct
            .counters()
            .planned_materialization_edge_class_count(),
        0
    );
    assert_eq!(seeded_direct.counters().planned_traversal_depth_limit(), 0);
    assert_eq!(
        seeded_direct.counters().planned_aggregate_input_breadth(),
        0
    );
    assert_eq!(seeded_direct.counters().planned_cdc_family_count(), 0);
}
