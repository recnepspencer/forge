use crate::facade::{
    execute_preflight_bundle, plan_validated_bundle, plan_validated_bundle_for_collection_family,
    planning_request_context_for_direct, BasisAuthorityFamily, CollectionResultFamily,
    ExecutionBasisIntent, PlanningError, SnapshotLineageClass,
};
use crate::collection::page_cursor_for_collection;
use crate::planning::{
    plan_validated_bundle_for_requested_aggregate_family,
    plan_validated_bundle_for_requested_traversal_bound, RequestedAggregateFamily,
    RequestedTraversalBound,
};

use super::super::collection_matrix::{
    CollectionCertificationBundle, CollectionCertificationRow, CollectionHostileExpectation,
    CollectionPerturbationClass, CollectionRejectionBundle, CollectionRejectionRow,
};
use super::super::profiles::CertificationProfile;

pub(super) fn to_bundle(
    profile: CertificationProfile,
    preflight: &crate::facade::ExecutionPreflightBundle,
) -> CollectionCertificationBundle {
    let envelope = execute_preflight_bundle(preflight).unwrap();
    let cursor_progress_report = if let Some(collection) = preflight.plan().collection() {
        let cursor = page_cursor_for_collection(
            collection,
            preflight.plan().query().plan_digest().as_str(),
            preflight.basis().proof().digest().as_str(),
            envelope.counters().page_width(),
        );
        format!(
            "family:{:?}:cursor:{}:basis:{}:advance_count:{}",
            collection.planning_context().result_family(),
            cursor.boundary().as_str(),
            preflight.basis().proof().digest().as_str()
            ,
            envelope.counters().cursor_advance_count()
        )
    } else {
        "no-collection-plan".to_string()
    };
    CollectionCertificationBundle {
        profile,
        query_digest: preflight.plan().query().validated_query_digest().as_str().to_string(),
        plan_digest: preflight.plan().query().plan_digest().as_str().to_string(),
        result_digest: envelope.report().result_digest().as_str().to_string(),
        basis_digest: preflight.basis().proof().digest().as_str().to_string(),
        delivery_digest: envelope.report().result_digest().as_str().to_string(),
        cursor_progress_report,
        counter_snapshot: envelope.counters().clone(),
    }
}

pub(super) fn to_rejection_bundle(
    profile: CertificationProfile,
    failure_class: &str,
    failure_digest: String,
) -> CollectionRejectionBundle {
    CollectionRejectionBundle {
        profile,
        failure_class: failure_class.to_string(),
        failure_digest,
    }
}

pub(super) fn canonical_row(
    row_name: &'static str,
    perturbation_class: CollectionPerturbationClass,
    hostile_expectation: CollectionHostileExpectation,
    control: crate::facade::ExecutionPreflightBundle,
    hostile: crate::facade::ExecutionPreflightBundle,
    parity: crate::facade::ExecutionPreflightBundle,
) -> CollectionCertificationRow {
    CollectionCertificationRow {
        row_name,
        perturbation_class,
        hostile_expectation,
        parity_anchor: crate::harness::certification::ParityAnchor::Control,
        control_lane: to_bundle(CertificationProfile::DirectConstruction, &control),
        hostile_lane: to_bundle(CertificationProfile::BindingVariation, &hostile),
        parity_lane: to_bundle(CertificationProfile::ReplayParity, &parity),
    }
}

pub(super) fn rejection_row(
    row_name: &'static str,
    perturbation_class: CollectionPerturbationClass,
    control: &crate::facade::ExecutionPreflightBundle,
    hostile: Result<(), PlanningError>,
) -> CollectionRejectionRow {
    let hostile = hostile.unwrap_err();
    CollectionRejectionRow {
        row_name,
        perturbation_class,
        control_lane: to_bundle(CertificationProfile::DirectConstruction, control),
        hostile_lane: to_rejection_bundle(
            CertificationProfile::BindingVariation,
            match hostile {
                PlanningError::UnsupportedOrderingFamily => "unsupported-ordering-family",
                PlanningError::UnsupportedCursorShape => "unstable-cursor-shape",
                PlanningError::UnsupportedTraversalBound => "unsupported-traversal-bound",
                PlanningError::UnsupportedAggregateFamily => "unsupported-aggregate-family",
                PlanningError::UnsupportedCollectionResultFamily => "unsupported-cdc-result-family",
                _ => "other",
            },
            match hostile {
                PlanningError::UnsupportedOrderingFamily => "unsupported-ordering-family".to_string(),
                PlanningError::UnsupportedCursorShape => "unstable-cursor-shape".to_string(),
                PlanningError::UnsupportedTraversalBound => "unsupported-traversal-bound".to_string(),
                PlanningError::UnsupportedAggregateFamily => "unsupported-aggregate-family".to_string(),
                PlanningError::UnsupportedCollectionResultFamily => {
                    "unsupported-cdc-result-family".to_string()
                }
                _ => "other".to_string(),
            },
        ),
        parity_lane: to_bundle(CertificationProfile::ReplayParity, control),
    }
}

pub(super) fn unsupported_ordering_family_hostile() -> Result<(), PlanningError> {
    let bundle = crate::harness::fixtures::validated_bundles::multi_order_collection_bundle();
    let request = planning_request_context_for_direct(
        &bundle,
        ExecutionBasisIntent::new(
            BasisAuthorityFamily::Runtime,
            SnapshotLineageClass::CurrentHead,
            false,
        ),
    )
    .unwrap();
    plan_validated_bundle(&bundle, request).map(|_| ())
}

pub(super) fn unstable_cursor_shape_hostile() -> Result<(), PlanningError> {
    let bundle = crate::harness::fixtures::validated_bundles::unordered_collection_bundle();
    let request = planning_request_context_for_direct(
        &bundle,
        ExecutionBasisIntent::new(
            BasisAuthorityFamily::Runtime,
            SnapshotLineageClass::CurrentHead,
            false,
        ),
    )
    .unwrap();
    plan_validated_bundle(&bundle, request).map(|_| ())
}

pub(super) fn unsupported_cdc_result_family_hostile() -> Result<(), PlanningError> {
    let bundle = crate::harness::fixtures::validated_bundles::runtime_detail_bundle();
    let request = planning_request_context_for_direct(
        &bundle,
        ExecutionBasisIntent::new(
            BasisAuthorityFamily::Runtime,
            SnapshotLineageClass::CurrentHead,
            false,
        ),
    )
    .unwrap();
    plan_validated_bundle_for_collection_family(
        &bundle,
        request,
        CollectionResultFamily::CdcCollection,
    )
    .map(|_| ())
}

pub(super) fn unsupported_traversal_bound_hostile() -> Result<(), PlanningError> {
    let bundle = crate::harness::fixtures::validated_bundles::ordered_collection_bundle();
    let request = planning_request_context_for_direct(
        &bundle,
        ExecutionBasisIntent::new(
            BasisAuthorityFamily::Runtime,
            SnapshotLineageClass::CurrentHead,
            false,
        ),
    )
    .unwrap();
    plan_validated_bundle_for_requested_traversal_bound(
        &bundle,
        request,
        RequestedTraversalBound::UnboundedExpansion,
    )
    .map(|_| ())
}

pub(super) fn unsupported_aggregate_family_hostile() -> Result<(), PlanningError> {
    let bundle = crate::harness::fixtures::validated_bundles::ordered_collection_bundle();
    let request = planning_request_context_for_direct(
        &bundle,
        ExecutionBasisIntent::new(
            BasisAuthorityFamily::Runtime,
            SnapshotLineageClass::CurrentHead,
            false,
        ),
    )
    .unwrap();
    plan_validated_bundle_for_requested_aggregate_family(
        &bundle,
        request,
        RequestedAggregateFamily::GroupedIntegerSum,
    )
    .map(|_| ())
}
