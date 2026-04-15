use crate::facade::{
    plan_validated_bundle, plan_validated_bundle_for_collection_family, preflight_execution_basis,
    CollectionResultFamily, ExecutionPreflightBundle,
};
use crate::planning::{
    plan_validated_bundle_for_requested_aggregate_family,
    plan_validated_bundle_for_requested_derived_field_family, RequestedAggregateFamily,
    RequestedDerivedFieldFamily,
};

pub fn direct_runtime_preflight() -> ExecutionPreflightBundle {
    let bundle = super::validated_bundles::runtime_detail_bundle();
    let plan = plan_validated_bundle(&bundle, super::planning_requests::direct_runtime_request(&bundle)).unwrap();
    preflight_execution_basis(plan, super::resolved_bases::runtime_basis(&bundle, "snapshot-1")).unwrap()
}

pub fn replay_runtime_preflight() -> ExecutionPreflightBundle {
    direct_runtime_preflight()
}

pub fn alternate_basis_runtime_preflight() -> ExecutionPreflightBundle {
    let bundle = super::validated_bundles::runtime_detail_bundle();
    let plan = plan_validated_bundle(&bundle, super::planning_requests::direct_runtime_request(&bundle)).unwrap();
    preflight_execution_basis(plan, super::resolved_bases::runtime_basis(&bundle, "snapshot-2")).unwrap()
}

pub fn expanded_runtime_preflight() -> ExecutionPreflightBundle {
    let bundle = super::validated_bundles::legal_detail_bundle();
    let plan = plan_validated_bundle(&bundle, super::planning_requests::direct_runtime_request(&bundle)).unwrap();
    preflight_execution_basis(plan, super::resolved_bases::runtime_basis(&bundle, "snapshot-1")).unwrap()
}

pub fn bound_runtime_preflight(value: &str) -> ExecutionPreflightBundle {
    let bundle = super::validated_bundles::runtime_bound_detail_bundle();
    let plan = plan_validated_bundle(&bundle, super::planning_requests::bound_runtime_request(&bundle, value)).unwrap();
    preflight_execution_basis(plan, super::resolved_bases::runtime_basis(&bundle, "snapshot-1")).unwrap()
}

pub fn pre_resolved_bound_runtime_preflight(value: &str) -> ExecutionPreflightBundle {
    let bundle = super::validated_bundles::runtime_bound_detail_bundle();
    let plan = plan_validated_bundle(
        &bundle,
        super::planning_requests::pre_resolved_bound_runtime_request(&bundle, value),
    )
    .unwrap();
    preflight_execution_basis(plan, super::resolved_bases::runtime_basis(&bundle, "snapshot-1")).unwrap()
}

pub fn ordered_collection_preflight() -> ExecutionPreflightBundle {
    let bundle = super::validated_bundles::ordered_collection_bundle();
    let plan = plan_validated_bundle(&bundle, super::planning_requests::direct_runtime_request(&bundle)).unwrap();
    preflight_execution_basis(plan, super::resolved_bases::runtime_basis(&bundle, "snapshot-1")).unwrap()
}

pub fn replay_ordered_collection_preflight() -> ExecutionPreflightBundle {
    ordered_collection_preflight()
}

pub fn descending_collection_preflight() -> ExecutionPreflightBundle {
    let bundle = super::validated_bundles::descending_collection_bundle();
    let plan = plan_validated_bundle(&bundle, super::planning_requests::direct_runtime_request(&bundle)).unwrap();
    preflight_execution_basis(plan, super::resolved_bases::runtime_basis(&bundle, "snapshot-1")).unwrap()
}

pub fn cdc_collection_preflight() -> ExecutionPreflightBundle {
    let bundle = super::validated_bundles::ordered_collection_bundle();
    let plan = plan_validated_bundle_for_collection_family(
        &bundle,
        super::planning_requests::direct_runtime_request(&bundle),
        CollectionResultFamily::CdcCollection,
    )
    .unwrap();
    preflight_execution_basis(plan, super::resolved_bases::runtime_basis(&bundle, "snapshot-1")).unwrap()
}

pub fn aggregate_rollup_collection_preflight() -> ExecutionPreflightBundle {
    let bundle = super::validated_bundles::ordered_collection_bundle();
    let plan = plan_validated_bundle_for_requested_aggregate_family(
        &bundle,
        super::planning_requests::direct_runtime_request(&bundle),
        RequestedAggregateFamily::CountRows,
    )
    .unwrap();
    preflight_execution_basis(plan, super::resolved_bases::runtime_basis(&bundle, "snapshot-1")).unwrap()
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
    preflight_execution_basis(plan, super::resolved_bases::runtime_basis(&bundle, "snapshot-1")).unwrap()
}

pub fn replay_derived_field_collection_preflight() -> ExecutionPreflightBundle {
    derived_field_collection_preflight()
}
