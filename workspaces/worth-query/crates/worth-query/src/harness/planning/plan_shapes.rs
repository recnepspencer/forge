use super::{
    collection_validated_bundle, descending_collection_validated_bundle, direct_validated_bundle,
};
use crate::facade::foundation::CollectionResultFamily;
use crate::facade::policy::{plan_validated_bundle, planning_request_context_for_direct};

#[test]
fn detail_queries_do_not_emit_collection_plan_artifacts() {
    let bundle = direct_validated_bundle();
    let request =
        planning_request_context_for_direct(&bundle, super::runtime_basis_intent()).unwrap();
    let planned = plan_validated_bundle(&bundle, request).unwrap();

    assert!(planned.collection().is_none());
}

#[test]
fn collection_queries_emit_collection_plan_artifacts() {
    let bundle = collection_validated_bundle();
    let request =
        planning_request_context_for_direct(&bundle, super::runtime_basis_intent()).unwrap();
    let planned = plan_validated_bundle(&bundle, request).unwrap();

    let collection = planned.collection().expect("collection plan");
    assert_eq!(
        collection.planning_context().query_family(),
        &crate::facade::foundation::QueryFamily::Collection
    );
    assert_eq!(
        collection.planning_context().result_family(),
        &crate::facade::foundation::CollectionResultFamily::OrdinaryCollection
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
        &crate::facade::foundation::CursorAdvanceContract::BasisBoundOpaque
    );
    assert_eq!(
        collection.window_policy(),
        &crate::facade::foundation::CollectionWindowPolicy::FullSnapshotRead
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
        planning_request_context_for_direct(&ascending, super::runtime_basis_intent()).unwrap();
    let descending_request =
        planning_request_context_for_direct(&descending, super::runtime_basis_intent()).unwrap();

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
        planning_request_context_for_direct(&bundle, super::runtime_basis_intent()).unwrap();
    let second_request =
        planning_request_context_for_direct(&bundle, super::runtime_basis_intent()).unwrap();

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
        planning_request_context_for_direct(&bundle, super::runtime_basis_intent()).unwrap();
    let cdc_request =
        planning_request_context_for_direct(&bundle, super::runtime_basis_intent()).unwrap();

    let ordinary = plan_validated_bundle(&bundle, ordinary_request).unwrap();
    let cdc = crate::facade::policy::plan_validated_bundle_for_collection_family(
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
        planning_request_context_for_direct(&bundle, super::runtime_basis_intent()).unwrap();
    let aggregate_request =
        planning_request_context_for_direct(&bundle, super::runtime_basis_intent()).unwrap();

    let ordinary = plan_validated_bundle(&bundle, ordinary_request).unwrap();
    let aggregate =
        crate::planning::plan_validated_bundle_for_count_aggregate(&bundle, aggregate_request)
            .unwrap();

    assert_ne!(
        ordinary.query().plan_digest(),
        aggregate.query().plan_digest()
    );
    assert_ne!(
        ordinary.collection().unwrap().digest(),
        aggregate.collection().unwrap().digest()
    );
    assert_ne!(
        ordinary.result_shape().validated_result_shape_digest(),
        aggregate.result_shape().validated_result_shape_digest()
    );
    assert_ne!(
        ordinary.result_shape().canonical_result_shape_digest(),
        aggregate.result_shape().canonical_result_shape_digest()
    );
    assert_eq!(aggregate.result_shape().binding_count(), 1);
    assert_eq!(
        aggregate
            .collection()
            .unwrap()
            .post_read_shaping()
            .aggregate_shape()
            .function_family(),
        &crate::facade::foundation::AggregateFunctionFamily::CountRows
    );
    assert_eq!(
        aggregate
            .collection()
            .unwrap()
            .post_read_shaping()
            .rollup_shape()
            .edge_class(),
        &crate::facade::foundation::RollupEdgeClass::RootCollection
    );
    assert_eq!(
        aggregate
            .collection()
            .unwrap()
            .planning_context()
            .result_family(),
        &CollectionResultFamily::CountAggregate
    );
}

#[test]
fn traversal_bearing_runtime_queries_lower_to_expanded_runtime_route() {
    let bundle = super::expanded_validated_bundle();
    let request =
        planning_request_context_for_direct(&bundle, super::runtime_basis_intent()).unwrap();
    let planned = plan_validated_bundle(&bundle, request).unwrap();

    assert_eq!(
        planned.query().route(),
        &crate::facade::policy::PlannedExecutionRoute::RuntimeExpandedSnapshotRead
    );
    assert_eq!(planned.counters().route_candidate_count(), 2);
    assert_eq!(planned.counters().planned_projection_entry_count(), 2);
    assert_eq!(planned.counters().planned_traversal_clause_count(), 1);
    assert_eq!(planned.counters().planned_read_surface_count(), 3);
}
