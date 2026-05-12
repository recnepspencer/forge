use crate::runtime::worker_host::{
    certify_worker_compatibility, certify_worker_unavailable_compatibility_artifact,
};

use crate::runtime::tests::support::*;
use crate::runtime::tests::worker_runtime::fixtures::portable_dashboard_graph::{
    portable_dashboard_certification_scenario, portable_dashboard_scenario_with_unpublished_region,
};

#[test]
fn worker_compatibility_certification_covers_dense_non_host_runtime_truth() {
    let report = certify_worker_compatibility(portable_dashboard_certification_scenario()).unwrap();

    assert!(report.committed_truth_report.committed_truth_matches);
    assert!(
        report
            .branch_lifecycle_report
            .main_branch_report
            .branch_truth_matches
    );
    assert!(
        report
            .branch_lifecycle_report
            .restored_branch_report
            .branch_truth_matches
    );
    assert!(report.observation_report.observation_truth_matches);
    assert!(report.diagnostics_report.diagnostics_truth_matches);
    assert!(report.async_lifecycle_report.async_lifecycle_truth_matches);
    assert!(report.async_lifecycle_report.request_admitted);
    assert!(report.async_lifecycle_report.completion_committed);
    assert!(report.isolation_report.all_regions_remain_worker_owned);
    assert!(!report.isolation_report.broad_placement_collapse_detected);
    assert_digest_pair_matches(
        &report.committed_truth_report.worker_first_truth_digest,
        &report
            .committed_truth_report
            .compatibility_mode_truth_digest,
    );
    assert_digest_pair_matches(
        &report
            .branch_lifecycle_report
            .main_branch_report
            .worker_first_truth_digest,
        &report
            .branch_lifecycle_report
            .main_branch_report
            .compatibility_mode_truth_digest,
    );
    assert_digest_pair_matches(
        &report
            .branch_lifecycle_report
            .restored_branch_report
            .worker_first_truth_digest,
        &report
            .branch_lifecycle_report
            .restored_branch_report
            .compatibility_mode_truth_digest,
    );
    assert_eq!(report.isolation_report.declared_independent_region_count, 3);
    assert_eq!(
        report
            .isolation_report
            .declared_independent_region_recipe_ids,
        ["inventoryRegion", "trafficRegion", "financeRegion"]
    );
    assert_eq!(report.isolation_report.worker_admitted_recipe_count, 4);
    assert_eq!(report.isolation_report.transaction_op_count, 3);
    assert_digest_pair_matches(
        &report.observation_report.worker_first_observation_digest,
        &report
            .observation_report
            .compatibility_mode_observation_digest,
    );
    assert_digest_pair_matches(
        &report.diagnostics_report.worker_first_diagnostics_digest,
        &report
            .diagnostics_report
            .compatibility_mode_diagnostics_digest,
    );
    assert_digest_pair_matches(
        &report
            .async_lifecycle_report
            .worker_first_async_lifecycle_digest,
        &report
            .async_lifecycle_report
            .compatibility_mode_async_lifecycle_digest,
    );
    assert_digest_shape(&report.isolation_report.placement_frontier_digest);
    assert_digest_shape(&report.isolation_report.worker_breadth_digest);
    assert_digest_shape(&report.isolation_report.main_thread_hosted_digest);
    assert_eq!(
        report.isolation_report.broadening_denial_artifact,
        "noBroadeningDetected"
    );
}

#[test]
fn worker_compatibility_certification_emits_final_envelope_vocabulary() {
    let report = certify_worker_compatibility(portable_dashboard_certification_scenario()).unwrap();

    assert_eq!(
        report.committed_truth_report.worker_envelope_family,
        "transactionResult"
    );
    assert_eq!(
        report
            .branch_lifecycle_report
            .restored_branch_report
            .worker_envelope_family,
        "lifecycleControl"
    );
    assert_eq!(report.worker_publication_summary.denied_callback_count, 0);
    assert_eq!(
        report
            .compatibility_publication_summary
            .denied_callback_count,
        0
    );
}

#[test]
fn worker_compatibility_certification_detects_unpublished_region_frontier() {
    let report =
        certify_worker_compatibility(portable_dashboard_scenario_with_unpublished_region())
            .unwrap();

    assert!(!report.isolation_report.all_regions_remain_worker_owned);
    assert!(report.isolation_report.broad_placement_collapse_detected);
    assert_eq!(
        report
            .isolation_report
            .declared_independent_region_recipe_ids,
        ["inventoryRegion", "trafficRegion", "unpublishedRegion"]
    );
    assert_eq!(
        report.isolation_report.broadening_denial_artifact,
        "workerRegionPublicationMismatch"
    );
    assert_digest_shape(&report.isolation_report.placement_frontier_digest);
    assert_digest_shape(&report.isolation_report.worker_breadth_digest);
    assert_digest_shape(&report.isolation_report.main_thread_hosted_digest);
}

#[test]
fn worker_unavailable_compatibility_artifact_certifies_explicit_main_thread_posture() {
    let package = certify_worker_unavailable_compatibility_artifact(
        portable_dashboard_certification_scenario(),
    )
    .unwrap();

    assert_eq!(
        package.certification_family,
        "workerUnavailableCompatibilityCertification"
    );
    assert_eq!(package.covered_suite_count, 1);
    assert_eq!(package.worker_support_posture, "workerUnavailable");
    assert_eq!(
        package.selected_deployment_posture,
        "mainThreadCompatibility"
    );
    assert_eq!(package.runtime_authority, "mainThreadRuntime");
    assert_eq!(
        package.compatibility_artifact,
        "explicitMainThreadCompatibilityRuntime"
    );
    assert_eq!(
        package.incompatibility_artifact,
        "dedicatedWorkerUnavailable"
    );
    assert_eq!(package.fallback_policy, "productDeclaredFallbackOnly");
    assert!(!package.hidden_fallback_allowed);
    assert!(package.denial_artifact_required);
    assert_eq!(package.fallback_count, 0);
    assert_eq!(package.callback_declaration_count, 0);
    assert_eq!(package.main_thread_hosted_callback_count, 0);
    assert_eq!(package.unavailable_callback_count, 0);
    assert_digest_pair_matches(
        &package.worker_first_reference_truth_digest,
        &package.compatibility_mode_truth_digest,
    );
    assert_digest_shape(&package.compatibility_truth_digest);
    assert_digest_shape(&package.deployment_posture_digest);
    assert_digest_shape(&package.fallback_policy_digest);
    assert_digest_shape(&package.denial_digest);
    assert_digest_shape(&package.fallback_digest);
    assert_digest_shape(&package.capability_availability_digest);
    assert_digest_shape(&package.replay_import_compatibility_digest);
    assert_digest_shape(&package.placement_identity_digest);
    assert_digest_shape(&package.historical_capability_digest);
    assert_digest_shape(&package.certification_digest);
}

#[test]
fn worker_unavailable_compatibility_artifact_rejects_non_convergent_truth() {
    let error = certify_worker_unavailable_compatibility_artifact(
        portable_dashboard_scenario_with_unpublished_region(),
    )
    .unwrap_err();

    assert!(error.message.contains("compatibility truth convergence"));
}

fn assert_digest_pair_matches(worker_first_digest: &str, compatibility_mode_digest: &str) {
    assert_digest_shape(worker_first_digest);
    assert_digest_shape(compatibility_mode_digest);
    assert_eq!(worker_first_digest, compatibility_mode_digest);
}
