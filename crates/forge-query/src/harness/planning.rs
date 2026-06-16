use crate::authoring::{RawAuthoredQuery, RawAuthoredResultShape};
use crate::facade::{
    canonicalize_request, derive_binding_requirements, execute_preflight_bundle,
    plan_validated_bundle, planning_request_context_for_bound, planning_request_context_for_direct,
    preflight_execution_basis, resolve_bindings, resolve_snapshot_basis, validate_canonical_bundle,
    AspectFieldSelector, AuthoredResultShapeField, BasisAuthorityFamily, BasisResolutionError,
    BasisResolutionMode, BoundBinding, BoundBindings, CollectionResultFamily, ExecutionBasisIntent,
    GuidedAuthoringPath, IdentityBindingDescriptor, QueryBindingDescriptor, QueryBindingSlot,
    QueryBindingSubject, ResolvedSnapshotIdentity, RootEntityKey, SnapshotLineageClass,
};
use crate::memory_workspace::ForgeQuerySnapshotIdentity;
use crate::planning::{
    plan_validated_bundle_for_requested_aggregate_family,
    plan_validated_bundle_for_requested_derived_field_family, RequestedAggregateFamily,
    RequestedDerivedFieldFamily,
};

fn direct_validated_bundle() -> crate::facade::ValidatedQueryBundle {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .build()
        .unwrap();

    let request = GuidedAuthoringPath::pair_detail(query, shape).unwrap();
    let canonical = canonicalize_request(request).unwrap();
    validate_canonical_bundle(
        canonical,
        crate::harness::fixtures::schema_view::detail_schema_view(),
    )
    .unwrap()
}

fn bound_validated_bundle() -> crate::facade::ValidatedQueryBundle {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .build()
        .unwrap();
    let bindings = QueryBindingDescriptor::new().with_identity(IdentityBindingDescriptor::new(
        QueryBindingSlot::new("root").unwrap(),
        QueryBindingSubject::RootEntity,
    ));

    let request = GuidedAuthoringPath::pair_detail_with_bindings(query, shape, bindings).unwrap();
    let canonical = canonicalize_request(request).unwrap();
    validate_canonical_bundle(
        canonical,
        crate::harness::fixtures::schema_view::detail_schema_view(),
    )
    .unwrap()
}

fn expanded_validated_bundle() -> crate::facade::ValidatedQueryBundle {
    crate::harness::fixtures::validated_bundles::legal_detail_bundle()
}

fn collection_validated_bundle() -> crate::facade::ValidatedQueryBundle {
    let query = RawAuthoredQuery::collection_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .project(AspectFieldSelector::new("profile", "display_name").unwrap())
        .order_by(crate::facade::OrderingSelector::ascending("profile", "display_name").unwrap())
        .traverse(crate::facade::TraversalSelector::bounded("manager", 1).unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::collection_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap())
        .build()
        .unwrap();

    let request = GuidedAuthoringPath::pair_collection(query, shape).unwrap();
    let canonical = canonicalize_request(request).unwrap();
    validate_canonical_bundle(
        canonical,
        crate::harness::fixtures::schema_view::detail_schema_view(),
    )
    .unwrap()
}

fn descending_collection_validated_bundle() -> crate::facade::ValidatedQueryBundle {
    let query = RawAuthoredQuery::collection_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .project(AspectFieldSelector::new("profile", "display_name").unwrap())
        .order_by(crate::facade::OrderingSelector::descending("profile", "display_name").unwrap())
        .traverse(crate::facade::TraversalSelector::bounded("manager", 1).unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::collection_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap())
        .build()
        .unwrap();

    let request = GuidedAuthoringPath::pair_collection(query, shape).unwrap();
    let canonical = canonicalize_request(request).unwrap();
    validate_canonical_bundle(
        canonical,
        crate::harness::fixtures::schema_view::detail_schema_view(),
    )
    .unwrap()
}

fn runtime_basis_intent() -> ExecutionBasisIntent {
    ExecutionBasisIntent::new(
        BasisAuthorityFamily::Runtime,
        SnapshotLineageClass::CurrentHead,
        false,
    )
}

fn runtime_resolved_identity(
    schema_basis: crate::facade::SchemaBasisDigest,
) -> ResolvedSnapshotIdentity {
    ResolvedSnapshotIdentity::new(
        BasisAuthorityFamily::Runtime,
        Some("workspace-main".to_string()),
        crate::memory_workspace::admit_external_snapshot_label("snapshot-1").evidence_identity(),
        schema_basis,
        SnapshotLineageClass::CurrentHead,
    )
}

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
    let seeded_direct = plan_validated_bundle(&bundle, direct).unwrap();

    let requirements = derive_binding_requirements(&bundle);
    let bound_resolution = resolve_bindings(requirements, BoundBindings::new(Vec::new())).unwrap();
    let bound = crate::facade::PlanningRequestContext::new(
        crate::facade::PlanningSemanticInputs::new(Some(bound_resolution), runtime_basis_intent()),
        crate::facade::PlanningAmbientContext::new(Vec::new()),
    );
    let seeded_bound = plan_validated_bundle(&bundle, bound).unwrap();

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

#[test]
fn detail_queries_do_not_emit_collection_plan_artifacts() {
    let bundle = direct_validated_bundle();
    let request = planning_request_context_for_direct(&bundle, runtime_basis_intent()).unwrap();
    let planned = plan_validated_bundle(&bundle, request).unwrap();

    assert!(planned.collection().is_none());
}

#[test]
fn collection_queries_emit_collection_plan_artifacts() {
    let bundle = collection_validated_bundle();
    let request = planning_request_context_for_direct(&bundle, runtime_basis_intent()).unwrap();
    let planned = plan_validated_bundle(&bundle, request).unwrap();

    let collection = planned.collection().expect("collection plan");
    assert_eq!(
        collection.planning_context().query_family(),
        &crate::facade::QueryFamily::Collection
    );
    assert_eq!(
        collection.planning_context().result_family(),
        &crate::facade::CollectionResultFamily::OrdinaryCollection
    );
    assert_eq!(
        collection.ordering_basis().entries().len(),
        bundle.query().ordering().entries().len()
    );
    assert_eq!(
        collection.traversal_bound().depth_limit().value(),
        bundle
            .query()
            .traversal()
            .iter()
            .map(|entry| entry.max_depth())
            .max()
            .unwrap_or(0)
    );
    assert_eq!(
        collection
            .post_read_shaping()
            .aggregate_shape()
            .input_breadth()
            .value(),
        bundle.query().projection().len()
            + bundle.query().predicates().entries().len()
            + bundle.query().traversal().len()
            + bundle.query().ordering().entries().len()
    );
    assert_eq!(
        collection.cursor_contract(),
        &crate::facade::CursorAdvanceContract::BasisBoundOpaque
    );
    assert_eq!(
        collection.window_policy(),
        &crate::facade::CollectionWindowPolicy::FullSnapshotRead
    );
    assert!(!collection.digest().as_str().is_empty());
    assert_eq!(
        planned
            .counters()
            .planned_materialization_edge_class_count(),
        1
    );
    assert_eq!(planned.counters().planned_traversal_depth_limit(), 1);
    assert_eq!(
        planned.counters().planned_aggregate_input_breadth(),
        collection
            .post_read_shaping()
            .aggregate_shape()
            .input_breadth()
            .value()
    );
    assert_eq!(planned.counters().planned_cdc_family_count(), 0);
}

#[test]
fn collection_ordering_changes_plan_and_collection_digests() {
    let ascending = collection_validated_bundle();
    let descending = descending_collection_validated_bundle();

    let ascending_request =
        planning_request_context_for_direct(&ascending, runtime_basis_intent()).unwrap();
    let descending_request =
        planning_request_context_for_direct(&descending, runtime_basis_intent()).unwrap();

    let ascending_plan = plan_validated_bundle(&ascending, ascending_request).unwrap();
    let descending_plan = plan_validated_bundle(&descending, descending_request).unwrap();

    assert_ne!(
        ascending_plan.query().plan_digest(),
        descending_plan.query().plan_digest()
    );
    assert_ne!(
        ascending_plan.collection().unwrap().digest(),
        descending_plan.collection().unwrap().digest()
    );
    assert_ne!(
        ascending_plan
            .collection()
            .unwrap()
            .ordering_basis()
            .entries()[0]
            .direction(),
        descending_plan
            .collection()
            .unwrap()
            .ordering_basis()
            .entries()[0]
            .direction()
    );
}

#[test]
fn repeated_collection_planning_preserves_collection_digest() {
    let bundle = collection_validated_bundle();
    let first_request =
        planning_request_context_for_direct(&bundle, runtime_basis_intent()).unwrap();
    let second_request =
        planning_request_context_for_direct(&bundle, runtime_basis_intent()).unwrap();

    let first = plan_validated_bundle(&bundle, first_request).unwrap();
    let second = plan_validated_bundle(&bundle, second_request).unwrap();

    assert_eq!(
        first.collection().unwrap().digest(),
        second.collection().unwrap().digest()
    );
    assert_eq!(first.query().plan_digest(), second.query().plan_digest());
}

#[test]
fn cdc_collection_family_changes_collection_and_plan_digests() {
    let bundle = collection_validated_bundle();
    let ordinary_request =
        planning_request_context_for_direct(&bundle, runtime_basis_intent()).unwrap();
    let cdc_request = planning_request_context_for_direct(&bundle, runtime_basis_intent()).unwrap();

    let ordinary = plan_validated_bundle(&bundle, ordinary_request).unwrap();
    let cdc = crate::facade::plan_validated_bundle_for_collection_family(
        &bundle,
        cdc_request,
        CollectionResultFamily::CdcCollection,
    )
    .unwrap();

    assert_ne!(ordinary.query().plan_digest(), cdc.query().plan_digest());
    assert_ne!(
        ordinary.collection().unwrap().digest(),
        cdc.collection().unwrap().digest()
    );
    assert_eq!(
        ordinary
            .collection()
            .unwrap()
            .planning_context()
            .result_family(),
        &CollectionResultFamily::OrdinaryCollection
    );
    assert_eq!(
        cdc.collection().unwrap().planning_context().result_family(),
        &CollectionResultFamily::CdcCollection
    );
    assert_eq!(ordinary.counters().planned_cdc_family_count(), 0);
    assert_eq!(cdc.counters().planned_cdc_family_count(), 1);
}

#[test]
fn aggregate_rollup_collection_family_changes_plan_and_rollup_semantics() {
    let bundle = collection_validated_bundle();
    let ordinary_request =
        planning_request_context_for_direct(&bundle, runtime_basis_intent()).unwrap();
    let aggregate_request =
        planning_request_context_for_direct(&bundle, runtime_basis_intent()).unwrap();

    let ordinary = plan_validated_bundle(&bundle, ordinary_request).unwrap();
    let aggregate = plan_validated_bundle_for_requested_aggregate_family(
        &bundle,
        aggregate_request,
        RequestedAggregateFamily::CountRows,
    )
    .unwrap();

    assert_ne!(
        ordinary.query().plan_digest(),
        aggregate.query().plan_digest()
    );
    assert_ne!(
        ordinary.collection().unwrap().digest(),
        aggregate.collection().unwrap().digest()
    );
    assert_eq!(
        aggregate
            .collection()
            .unwrap()
            .post_read_shaping()
            .aggregate_shape()
            .function_family(),
        &crate::facade::AggregateFunctionFamily::CountRows
    );
    assert_eq!(
        aggregate
            .collection()
            .unwrap()
            .post_read_shaping()
            .rollup_shape()
            .edge_class(),
        &crate::facade::RollupEdgeClass::RootCollection
    );
}

#[test]
fn derived_field_collection_family_changes_plan_and_derived_shape_semantics() {
    let bundle = collection_validated_bundle();
    let ordinary_request =
        planning_request_context_for_direct(&bundle, runtime_basis_intent()).unwrap();
    let derived_request =
        planning_request_context_for_direct(&bundle, runtime_basis_intent()).unwrap();

    let ordinary = plan_validated_bundle(&bundle, ordinary_request).unwrap();
    let derived = plan_validated_bundle_for_requested_derived_field_family(
        &bundle,
        derived_request,
        RequestedDerivedFieldFamily::DisplayLabel,
    )
    .unwrap();

    assert_ne!(
        ordinary.query().plan_digest(),
        derived.query().plan_digest()
    );
    assert_ne!(
        ordinary.collection().unwrap().digest(),
        derived.collection().unwrap().digest()
    );
    assert_eq!(
        derived
            .collection()
            .unwrap()
            .post_read_shaping()
            .derived_field_plan()
            .computation_class(),
        &crate::facade::DerivedFieldComputationClass::DisplayLabelFromIdentityAndProfile
    );
    assert_eq!(
        derived
            .collection()
            .unwrap()
            .post_read_shaping()
            .derived_field_plan()
            .derived_field_count(),
        1
    );
}

#[test]
fn traversal_bearing_runtime_queries_lower_to_expanded_runtime_route() {
    let bundle = expanded_validated_bundle();
    let request = planning_request_context_for_direct(&bundle, runtime_basis_intent()).unwrap();
    let planned = plan_validated_bundle(&bundle, request).unwrap();

    assert_eq!(
        planned.query().route(),
        &crate::facade::PlannedExecutionRoute::RuntimeExpandedSnapshotRead
    );
    assert_eq!(planned.counters().route_candidate_count(), 2);
    assert_eq!(planned.counters().planned_projection_entry_count(), 2);
    assert_eq!(planned.counters().planned_traversal_clause_count(), 1);
    assert_eq!(planned.counters().planned_read_surface_count(), 3);
}

#[test]
fn plan_and_resolved_basis_preflight_successfully_couple() {
    let bundle = direct_validated_bundle();
    let request = planning_request_context_for_direct(&bundle, runtime_basis_intent()).unwrap();
    let planned = plan_validated_bundle(&bundle, request).unwrap();
    let basis = resolve_snapshot_basis(
        runtime_basis_intent(),
        runtime_resolved_identity(bundle.query().schema_basis().clone()),
        BasisResolutionMode::RuntimeDirect,
    )
    .unwrap();

    let preflight = preflight_execution_basis(planned, basis).unwrap();
    assert_eq!(
        preflight.report().basis_digest(),
        preflight.basis().proof().digest()
    );
    assert_eq!(preflight.report().snapshot_basis_resolution_count(), 1);
}

#[test]
fn store_backend_planning_is_rejected_until_parity_is_admitted() {
    let bundle = direct_validated_bundle();
    let store_intent = ExecutionBasisIntent::new(
        BasisAuthorityFamily::Store,
        SnapshotLineageClass::CurrentHead,
        false,
    );
    let request = planning_request_context_for_direct(&bundle, store_intent.clone()).unwrap();
    let error = plan_validated_bundle(&bundle, request).unwrap_err();
    assert_eq!(
        error,
        crate::facade::PlanningError::UnsupportedBackendParityRequest
    );
}

#[test]
fn resolve_snapshot_basis_rejects_identity_mismatch() {
    let bundle = direct_validated_bundle();
    let error = resolve_snapshot_basis(
        runtime_basis_intent(),
        ResolvedSnapshotIdentity::new(
            BasisAuthorityFamily::Store,
            Some("workspace-main".to_string()),
            crate::memory_workspace::admit_external_snapshot_label("snapshot-2")
                .evidence_identity(),
            bundle.query().schema_basis().clone(),
            SnapshotLineageClass::CurrentHead,
        ),
        BasisResolutionMode::StoreDirect,
    )
    .unwrap_err();

    assert_eq!(error, BasisResolutionError::ResolutionIdentityMismatch);
}

#[test]
fn fallback_admission_is_rejected_until_supported_shape_exists() {
    let bundle = direct_validated_bundle();
    let request = planning_request_context_for_direct(
        &bundle,
        ExecutionBasisIntent::new(
            BasisAuthorityFamily::Runtime,
            SnapshotLineageClass::CurrentHead,
            true,
        ),
    )
    .unwrap();
    let error = plan_validated_bundle(&bundle, request).unwrap_err();
    assert_eq!(
        error,
        crate::facade::PlanningError::UnsupportedFallbackShape
    );
}

#[test]
fn execution_is_deterministic_for_same_preflight_bundle() {
    let bundle = direct_validated_bundle();
    let request = planning_request_context_for_direct(&bundle, runtime_basis_intent()).unwrap();
    let planned = plan_validated_bundle(&bundle, request).unwrap();
    let basis = resolve_snapshot_basis(
        runtime_basis_intent(),
        runtime_resolved_identity(bundle.query().schema_basis().clone()),
        BasisResolutionMode::RuntimeDirect,
    )
    .unwrap();
    let preflight = preflight_execution_basis(planned, basis).unwrap();

    let first = execute_preflight_bundle(&preflight).unwrap();
    let second = execute_preflight_bundle(&preflight).unwrap();

    assert_eq!(
        first.report().result_digest(),
        second.report().result_digest()
    );
    assert_eq!(first.rows(), second.rows());
    assert_eq!(first.counters(), second.counters());
    assert_eq!(first.counters().executor_semantic_rediscovery_count(), 0);
}

#[test]
fn execution_counters_reflect_planned_shape() {
    let bundle = direct_validated_bundle();
    let request = planning_request_context_for_direct(&bundle, runtime_basis_intent()).unwrap();
    let planned = plan_validated_bundle(&bundle, request).unwrap();
    let basis = resolve_snapshot_basis(
        runtime_basis_intent(),
        runtime_resolved_identity(bundle.query().schema_basis().clone()),
        BasisResolutionMode::RuntimeDirect,
    )
    .unwrap();
    let preflight = preflight_execution_basis(planned, basis).unwrap();
    let envelope = execute_preflight_bundle(&preflight).unwrap();

    assert_eq!(envelope.counters().execution_read_operation_count(), 1);
    assert_eq!(envelope.counters().execution_records_examined_count(), 1);
    assert_eq!(envelope.counters().execution_records_emitted_count(), 1);
    assert_eq!(
        envelope.counters().execution_result_shape_binding_count(),
        1
    );
    assert_eq!(envelope.counters().page_width(), 1);
    assert_eq!(envelope.counters().page_truncation_count(), 0);
    assert_eq!(envelope.counters().cursor_advance_count(), 0);
    assert_eq!(envelope.counters().post_read_shape_field_count(), 1);
}

#[test]
fn expanded_runtime_route_increases_execution_read_surface_counts() {
    let bundle = expanded_validated_bundle();
    let request = planning_request_context_for_direct(&bundle, runtime_basis_intent()).unwrap();
    let planned = plan_validated_bundle(&bundle, request).unwrap();
    let basis = resolve_snapshot_basis(
        runtime_basis_intent(),
        runtime_resolved_identity(bundle.query().schema_basis().clone()),
        BasisResolutionMode::RuntimeDirect,
    )
    .unwrap();
    let preflight = preflight_execution_basis(planned, basis).unwrap();
    let envelope = execute_preflight_bundle(&preflight).unwrap();

    assert_eq!(envelope.counters().execution_read_operation_count(), 3);
    assert_eq!(envelope.counters().execution_records_examined_count(), 3);
    assert_eq!(envelope.counters().execution_records_emitted_count(), 2);
}

#[test]
fn collection_execution_counters_reflect_materialization_and_aggregate_breadth() {
    let bundle = collection_validated_bundle();
    let request = planning_request_context_for_direct(&bundle, runtime_basis_intent()).unwrap();
    let planned = plan_validated_bundle(&bundle, request).unwrap();
    let basis = resolve_snapshot_basis(
        runtime_basis_intent(),
        runtime_resolved_identity(bundle.query().schema_basis().clone()),
        BasisResolutionMode::RuntimeDirect,
    )
    .unwrap();
    let preflight = preflight_execution_basis(planned, basis).unwrap();
    let envelope = execute_preflight_bundle(&preflight).unwrap();

    assert_eq!(envelope.counters().materialized_relation_count(), 1);
    assert_eq!(envelope.counters().page_width(), 2);
    assert_eq!(envelope.counters().page_truncation_count(), 0);
    assert_eq!(envelope.counters().cursor_advance_count(), 1);
    assert_eq!(
        envelope.counters().aggregate_input_count(),
        preflight
            .plan()
            .collection()
            .unwrap()
            .post_read_shaping()
            .aggregate_shape()
            .input_breadth()
            .value()
    );
    assert_eq!(envelope.counters().rollup_input_count(), 0);
    assert_eq!(envelope.counters().derived_field_evaluation_count(), 0);
    assert_eq!(envelope.counters().cdc_output_count(), 0);
}

#[test]
fn cdc_collection_execution_emits_distinct_payload_and_cdc_counters() {
    let bundle = collection_validated_bundle();
    let request = planning_request_context_for_direct(&bundle, runtime_basis_intent()).unwrap();
    let planned = crate::facade::plan_validated_bundle_for_collection_family(
        &bundle,
        request,
        CollectionResultFamily::CdcCollection,
    )
    .unwrap();
    let basis = resolve_snapshot_basis(
        runtime_basis_intent(),
        runtime_resolved_identity(bundle.query().schema_basis().clone()),
        BasisResolutionMode::RuntimeDirect,
    )
    .unwrap();
    let preflight = preflight_execution_basis(planned, basis).unwrap();
    let envelope = execute_preflight_bundle(&preflight).unwrap();

    assert!(envelope
        .rows()
        .iter()
        .all(|entry| entry.starts_with("cdc:")));
    assert_eq!(envelope.counters().cursor_advance_count(), 1);
    assert_eq!(envelope.counters().cdc_output_count(), 1);
}

#[test]
fn aggregate_rollup_execution_emits_distinct_payload_and_rollup_counters() {
    let bundle = collection_validated_bundle();
    let request = planning_request_context_for_direct(&bundle, runtime_basis_intent()).unwrap();
    let planned = plan_validated_bundle_for_requested_aggregate_family(
        &bundle,
        request,
        RequestedAggregateFamily::CountRows,
    )
    .unwrap();
    let basis = resolve_snapshot_basis(
        runtime_basis_intent(),
        runtime_resolved_identity(bundle.query().schema_basis().clone()),
        BasisResolutionMode::RuntimeDirect,
    )
    .unwrap();
    let preflight = preflight_execution_basis(planned, basis).unwrap();
    let envelope = execute_preflight_bundle(&preflight).unwrap();

    assert!(envelope
        .rows()
        .iter()
        .all(|entry| entry.starts_with("aggregate:count_rows:")));
    assert_eq!(envelope.counters().cursor_advance_count(), 1);
    assert_eq!(envelope.counters().rollup_input_count(), 1);
}

#[test]
fn derived_field_execution_emits_distinct_payload_and_shape_counts() {
    let bundle = collection_validated_bundle();
    let request = planning_request_context_for_direct(&bundle, runtime_basis_intent()).unwrap();
    let planned = plan_validated_bundle_for_requested_derived_field_family(
        &bundle,
        request,
        RequestedDerivedFieldFamily::DisplayLabel,
    )
    .unwrap();
    let basis = resolve_snapshot_basis(
        runtime_basis_intent(),
        runtime_resolved_identity(bundle.query().schema_basis().clone()),
        BasisResolutionMode::RuntimeDirect,
    )
    .unwrap();
    let preflight = preflight_execution_basis(planned, basis).unwrap();
    let envelope = execute_preflight_bundle(&preflight).unwrap();

    assert!(envelope
        .rows()
        .iter()
        .all(|entry| entry.starts_with("derived:display_label:")));
    assert_eq!(envelope.counters().cursor_advance_count(), 1);
    assert_eq!(envelope.counters().derived_field_evaluation_count(), 1);
    assert_eq!(envelope.counters().post_read_shape_field_count(), 3);
}
