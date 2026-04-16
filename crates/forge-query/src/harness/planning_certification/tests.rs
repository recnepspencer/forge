use super::MilestoneThreePlanningCertificationAdapter;
use crate::harness::planning_matrix::PlanningPerturbationClass;

#[test]
fn planner_executor_binding_parity_adapter_emits_named_matrix() {
    let matrix = MilestoneThreePlanningCertificationAdapter::planner_executor_binding_parity_test();

    assert_eq!(
        matrix.suite_name,
        "Planner / Executor / Binding Parity Test"
    );
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "direct-runtime-plan-parity"));
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "replanned-runtime-parity"));
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "type-bound-runtime-parity"));
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "runtime-basis-repeatability"));
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "identity-bearing-binding-difference"));
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "basis-difference"));
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "route-semantic-difference"));
    assert!(matrix
        .rejection_rows
        .iter()
        .any(|row| row.row_name == "unsupported-backend-route"));
    assert!(matrix
        .rejection_rows
        .iter()
        .any(|row| row.row_name == "unsupported-fallback-shape"));
    assert!(matrix
        .rejection_rows
        .iter()
        .any(|row| row.row_name == "binding-fulfillment-conflict"));
    assert!(matrix
        .rejection_rows
        .iter()
        .any(|row| row.row_name == "snapshot-basis-resolution-failure"));
}

#[test]
fn planner_executor_binding_parity_artifact_is_offline_ready_for_current_scope() {
    let artifact =
        MilestoneThreePlanningCertificationAdapter::planner_executor_binding_parity_certification_artifact();

    assert_eq!(
        artifact.suite_name,
        "Planner / Executor / Binding Parity Test"
    );
    assert!(!artifact.certification_bundle_digest.is_empty());
    assert!(!artifact.coverage_matrix_digest.is_empty());
    assert_eq!(
        artifact.bundle_completeness_report.successful_lane_count,
        artifact.bundle_completeness_report.supported_lane_count
    );
    assert_eq!(
        artifact.bundle_completeness_report.zero_fallback_lane_count,
        artifact.bundle_completeness_report.supported_lane_count
    );
    assert_eq!(
        artifact
            .bundle_completeness_report
            .zero_rediscovery_lane_count,
        artifact.bundle_completeness_report.supported_lane_count
    );
    assert!(artifact
        .bundle_completeness_report
        .covered_perturbation_classes
        .contains(&PlanningPerturbationClass::DirectRuntimeParity));
    assert!(artifact
        .bundle_completeness_report
        .covered_perturbation_classes
        .contains(&PlanningPerturbationClass::BindingParity));
    assert!(
        artifact
            .bundle_completeness_report
            .covers_all_currently_implemented_normative_scenarios
    );
    assert!(
        artifact
            .bundle_completeness_report
            .covers_full_milestone_three_spec_matrix
    );
    assert!(artifact
        .bundle_completeness_report
        .unmet_required_rows
        .is_empty());
    assert!(artifact
        .bundle_completeness_report
        .unmet_required_assertion_classes
        .is_empty());
    assert!(artifact.bundle_completeness_report.offline_analysis_ready);
}

#[test]
fn planner_executor_binding_parity_artifact_is_deterministic() {
    let left =
        MilestoneThreePlanningCertificationAdapter::planner_executor_binding_parity_certification_artifact();
    let right =
        MilestoneThreePlanningCertificationAdapter::planner_executor_binding_parity_certification_artifact();

    assert_eq!(
        left.certification_bundle_digest,
        right.certification_bundle_digest
    );
    assert_eq!(left.coverage_matrix_digest, right.coverage_matrix_digest);
    assert_eq!(
        left.bundle_completeness_report,
        right.bundle_completeness_report
    );
    assert_eq!(left.counter_snapshot, right.counter_snapshot);
}
