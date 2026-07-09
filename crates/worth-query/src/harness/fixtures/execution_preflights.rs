use crate::facade::{
    plan_validated_bundle, plan_validated_bundle_for_collection_family,
    planning_request_context_for_direct, preflight_execution_basis, seed_execution_plan,
    CollectionResultFamily, ExecutionPreflightBundle, FallbackDisposition, PlannedExecutionRoute,
};
use crate::memory_workspace::WorthQuerySnapshotIdentity;
use crate::planning::{
    plan_validated_bundle_for_requested_aggregate_family,
    plan_validated_bundle_for_requested_derived_field_family, RequestedAggregateFamily,
    RequestedDerivedFieldFamily,
};

pub fn direct_runtime_preflight() -> ExecutionPreflightBundle {
    runtime_preflight_with_snapshot_identity(super::resolved_bases::primary_snapshot_identity())
}

pub fn runtime_preflight_with_snapshot_identity(
    snapshot_identity: WorthQuerySnapshotIdentity,
) -> ExecutionPreflightBundle {
    let bundle = super::validated_bundles::runtime_detail_bundle();
    let plan = plan_validated_bundle(
        &bundle,
        super::planning_requests::direct_runtime_request(&bundle),
    )
    .unwrap();
    preflight_execution_basis(
        plan,
        super::resolved_bases::runtime_basis(&bundle, &snapshot_identity),
    )
    .unwrap()
}

pub fn store_detail_preflight() -> ExecutionPreflightBundle {
    let bundle = super::validated_bundles::runtime_detail_bundle();
    let request =
        planning_request_context_for_direct(&bundle, super::resolved_bases::store_basis_intent())
            .unwrap();
    let plan = seed_execution_plan(
        &bundle,
        request,
        PlannedExecutionRoute::StoreSnapshotRead,
        FallbackDisposition::Forbidden,
    )
    .unwrap();
    preflight_execution_basis(
        plan,
        super::resolved_bases::store_basis(
            &bundle,
            &super::resolved_bases::primary_snapshot_identity(),
        ),
    )
    .unwrap()
}

pub fn replay_runtime_preflight() -> ExecutionPreflightBundle {
    direct_runtime_preflight()
}

pub fn alternate_basis_runtime_preflight() -> ExecutionPreflightBundle {
    let bundle = super::validated_bundles::runtime_detail_bundle();
    let plan = plan_validated_bundle(
        &bundle,
        super::planning_requests::direct_runtime_request(&bundle),
    )
    .unwrap();
    preflight_execution_basis(
        plan,
        super::resolved_bases::runtime_basis(
            &bundle,
            &super::resolved_bases::alternate_snapshot_identity(),
        ),
    )
    .unwrap()
}

pub fn alternate_basis_ordered_collection_preflight() -> ExecutionPreflightBundle {
    let bundle = super::validated_bundles::ordered_collection_without_traversal_bundle();
    let plan = plan_validated_bundle(
        &bundle,
        super::planning_requests::direct_runtime_request(&bundle),
    )
    .unwrap();
    preflight_execution_basis(
        plan,
        super::resolved_bases::runtime_basis(
            &bundle,
            &super::resolved_bases::alternate_snapshot_identity(),
        ),
    )
    .unwrap()
}

pub fn alternate_basis_bounded_materialization_preflight() -> ExecutionPreflightBundle {
    let bundle = super::validated_bundles::ordered_collection_bundle();
    let plan = plan_validated_bundle(
        &bundle,
        super::planning_requests::direct_runtime_request(&bundle),
    )
    .unwrap();
    preflight_execution_basis(
        plan,
        super::resolved_bases::runtime_basis(
            &bundle,
            &super::resolved_bases::alternate_snapshot_identity(),
        ),
    )
    .unwrap()
}

pub fn expanded_runtime_preflight() -> ExecutionPreflightBundle {
    let bundle = super::validated_bundles::legal_detail_bundle();
    let plan = plan_validated_bundle(
        &bundle,
        super::planning_requests::direct_runtime_request(&bundle),
    )
    .unwrap();
    preflight_execution_basis(
        plan,
        super::resolved_bases::runtime_basis(
            &bundle,
            &super::resolved_bases::primary_snapshot_identity(),
        ),
    )
    .unwrap()
}

pub fn bound_runtime_preflight(value: &str) -> ExecutionPreflightBundle {
    let bundle = super::validated_bundles::runtime_bound_detail_bundle();
    let plan = plan_validated_bundle(
        &bundle,
        super::planning_requests::bound_runtime_request(&bundle, value),
    )
    .unwrap();
    preflight_execution_basis(
        plan,
        super::resolved_bases::runtime_basis(
            &bundle,
            &super::resolved_bases::primary_snapshot_identity(),
        ),
    )
    .unwrap()
}

pub fn pre_resolved_bound_runtime_preflight(value: &str) -> ExecutionPreflightBundle {
    let bundle = super::validated_bundles::runtime_bound_detail_bundle();
    let plan = plan_validated_bundle(
        &bundle,
        super::planning_requests::pre_resolved_bound_runtime_request(&bundle, value),
    )
    .unwrap();
    preflight_execution_basis(
        plan,
        super::resolved_bases::runtime_basis(
            &bundle,
            &super::resolved_bases::primary_snapshot_identity(),
        ),
    )
    .unwrap()
}

pub fn ordered_collection_preflight() -> ExecutionPreflightBundle {
    let bundle = super::validated_bundles::ordered_collection_bundle();
    let plan = plan_validated_bundle(
        &bundle,
        super::planning_requests::direct_runtime_request(&bundle),
    )
    .unwrap();
    preflight_execution_basis(
        plan,
        super::resolved_bases::runtime_basis(
            &bundle,
            &super::resolved_bases::primary_snapshot_identity(),
        ),
    )
    .unwrap()
}

pub fn ordered_collection_without_traversal_preflight() -> ExecutionPreflightBundle {
    let bundle = super::validated_bundles::ordered_collection_without_traversal_bundle();
    let plan = plan_validated_bundle(
        &bundle,
        super::planning_requests::direct_runtime_request(&bundle),
    )
    .unwrap();
    preflight_execution_basis(
        plan,
        super::resolved_bases::runtime_basis(
            &bundle,
            &super::resolved_bases::primary_snapshot_identity(),
        ),
    )
    .unwrap()
}

pub fn replay_ordered_collection_preflight() -> ExecutionPreflightBundle {
    ordered_collection_preflight()
}

pub fn descending_collection_preflight() -> ExecutionPreflightBundle {
    let bundle = super::validated_bundles::descending_collection_bundle();
    let plan = plan_validated_bundle(
        &bundle,
        super::planning_requests::direct_runtime_request(&bundle),
    )
    .unwrap();
    preflight_execution_basis(
        plan,
        super::resolved_bases::runtime_basis(
            &bundle,
            &super::resolved_bases::primary_snapshot_identity(),
        ),
    )
    .unwrap()
}

pub fn cdc_collection_preflight() -> ExecutionPreflightBundle {
    let bundle = super::validated_bundles::ordered_collection_bundle();
    let plan = plan_validated_bundle_for_collection_family(
        &bundle,
        super::planning_requests::direct_runtime_request(&bundle),
        CollectionResultFamily::CdcCollection,
    )
    .unwrap();
    preflight_execution_basis(
        plan,
        super::resolved_bases::runtime_basis(
            &bundle,
            &super::resolved_bases::primary_snapshot_identity(),
        ),
    )
    .unwrap()
}

pub fn aggregate_rollup_collection_preflight() -> ExecutionPreflightBundle {
    let bundle = super::validated_bundles::ordered_collection_bundle();
    let plan = plan_validated_bundle_for_requested_aggregate_family(
        &bundle,
        super::planning_requests::direct_runtime_request(&bundle),
        RequestedAggregateFamily::CountRows,
    )
    .unwrap();
    preflight_execution_basis(
        plan,
        super::resolved_bases::runtime_basis(
            &bundle,
            &super::resolved_bases::primary_snapshot_identity(),
        ),
    )
    .unwrap()
}

pub fn replay_aggregate_rollup_collection_preflight() -> ExecutionPreflightBundle {
    aggregate_rollup_collection_preflight()
}

pub fn derived_field_collection_preflight() -> ExecutionPreflightBundle {
    let bundle = super::validated_bundles::ordered_collection_bundle();
    let plan = plan_validated_bundle_for_requested_derived_field_family(
        &bundle,
        super::planning_requests::direct_runtime_request(&bundle),
        RequestedDerivedFieldFamily::DisplayLabel,
    )
    .unwrap();
    preflight_execution_basis(
        plan,
        super::resolved_bases::runtime_basis(
            &bundle,
            &super::resolved_bases::primary_snapshot_identity(),
        ),
    )
    .unwrap()
}

pub fn replay_derived_field_collection_preflight() -> ExecutionPreflightBundle {
    derived_field_collection_preflight()
}
