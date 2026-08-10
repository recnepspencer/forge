use crate::facade::foundation::{
    promote_preflight_bundle_to_live, LocalityPredicateContract, StreamConsumerShape,
};
use crate::harness::live_certification::LiveCertificationBundle;
use crate::harness::profiles::CertificationProfile;
use crate::live::{
    admit_region_scoped_live_plan, execute_live_change, execute_region_scoped_live_change,
    lower_region_scoped_execution_to_stream_contract,
};

use super::bundle_evidence::{
    bundle_from_live_execution, bundle_from_region_execution, bundle_from_stream_contract,
};
use super::change_scenarios::{
    bounded_in_region_change, detail_in_region_change, detail_off_region_change,
    detail_region_widening_change, single_field_partition_change,
};

pub(in crate::harness::region_live_certification) fn detail_region_convergence_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::region("assembly-a"))
            .expect("detail region admission should succeed");
    let execution = execute_region_scoped_live_change(&plan, &detail_in_region_change())
        .expect("in-region detail change should execute");
    bundle_from_region_execution(profile, &execution)
}

pub(in crate::harness::region_live_certification) fn off_region_suppression_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::region("assembly-a"))
            .expect("detail region admission should succeed");
    let execution = execute_region_scoped_live_change(&plan, &detail_off_region_change())
        .expect("off-region detail change should suppress");
    bundle_from_region_execution(profile, &execution)
}

pub(in crate::harness::region_live_certification) fn detail_region_widening_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::region("assembly-a"))
            .expect("detail region admission should succeed");
    let execution = execute_region_scoped_live_change(&plan, &detail_region_widening_change())
        .expect("detail region widening should execute");
    bundle_from_region_execution(profile, &execution)
}

pub(in crate::harness::region_live_certification) fn ordered_collection_partition_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight = crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");
    let plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::partition("tenant-a"))
            .expect("collection partition admission should succeed");
    let execution = execute_region_scoped_live_change(&plan, &single_field_partition_change())
        .expect("single-field in-partition change should execute");
    bundle_from_region_execution(profile, &execution)
}

pub(in crate::harness::region_live_certification) fn bounded_materialization_region_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight = crate::harness::fixtures::execution_preflights::ordered_collection_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("bounded preflight should promote");
    let plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::region("assembly-a"))
            .expect("bounded materialization should admit region scope");
    let execution = execute_region_scoped_live_change(&plan, &bounded_in_region_change())
        .expect("bounded in-region change should execute");
    bundle_from_region_execution(profile, &execution)
}

pub(in crate::harness::region_live_certification) fn broad_control_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let execution = execute_live_change(&live, &detail_in_region_change())
        .expect("broad live control should execute");
    bundle_from_live_execution(profile, &execution)
}

pub(in crate::harness::region_live_certification) fn stream_contract_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::region("assembly-a"))
            .expect("detail region admission should succeed");
    let execution = execute_region_scoped_live_change(&plan, &detail_in_region_change())
        .expect("in-region detail change should execute");
    let contract = lower_region_scoped_execution_to_stream_contract(
        &plan,
        &execution,
        StreamConsumerShape::DetailCurrentState,
    )
    .expect("detail stream lowering should succeed");
    bundle_from_stream_contract(profile, &execution, &contract)
}

pub(in crate::harness::region_live_certification) fn cdc_stream_contract_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    let preflight = crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");
    let plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::partition("tenant-a"))
            .expect("collection partition admission should succeed");
    let execution = execute_region_scoped_live_change(&plan, &single_field_partition_change())
        .expect("single-field in-partition change should execute");
    let contract = lower_region_scoped_execution_to_stream_contract(
        &plan,
        &execution,
        StreamConsumerShape::CdcCollectionPatch,
    )
    .expect("single-field collection patch should lower into the CDC stream contract");
    bundle_from_stream_contract(profile, &execution, &contract)
}

pub(in crate::harness::region_live_certification) fn locality_breadth_budget_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    detail_region_convergence_bundle(profile)
}

pub(in crate::harness::region_live_certification) fn stream_member_width_budget_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    cdc_stream_contract_bundle(profile)
}

pub(in crate::harness::region_live_certification) fn locality_work_avoided_bundle(
    profile: CertificationProfile,
) -> LiveCertificationBundle {
    off_region_suppression_bundle(profile)
}
