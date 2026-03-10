use forge_harness::facade::{
    ArtifactSurface, RegressionTargetKind, WorkflowCertificationRunner, WorkflowRuntimeProfile,
    WorkflowState,
};

use super::adapter::RelationalFintechWorkflowCertificationAdapter;
use super::plans::{
    relational_fintech_analysis_baseline_plan, relational_fintech_intraday_risk_plan,
    relational_fintech_replay_regression_plan, relational_fintech_settlement_repair_plan,
};

fn development_profile() -> WorkflowRuntimeProfile {
    WorkflowRuntimeProfile::new("fintech-development")
}

#[test]
fn workflow_certification_runner_proves_relational_fintech_analysis_baseline() {
    let runner = WorkflowCertificationRunner::new(RelationalFintechWorkflowCertificationAdapter);
    let plan = relational_fintech_analysis_baseline_plan();
    let report = runner.certify(&plan, &development_profile()).unwrap();

    assert_eq!(report.session.state, WorkflowState::Completed);
    assert!(report.failure_bundle.is_none());
    assert!(report
        .session
        .session_data
        .named_snapshots
        .contains_key("baseline_snapshot"));
    assert!(report
        .session
        .session_data
        .named_reads
        .contains_key("analysis_correction_case"));
    assert!(report.session.artifacts.iter().any(|artifact| {
        artifact.surface == ArtifactSurface::Diagnostics
            && artifact.boundary == WorkflowState::Inspected
    }));
    assert!(report.session.artifacts.iter().any(|artifact| {
        artifact.surface == ArtifactSurface::PatchChangeSurface
            && artifact.boundary == WorkflowState::Inspected
    }));
    assert!(report.session.artifacts.iter().any(|artifact| {
        artifact.surface == ArtifactSurface::ComplexityCounters
            && artifact.boundary == WorkflowState::Inspected
    }));
    let budget = report
        .session
        .artifacts
        .iter()
        .find(|artifact| {
            artifact.surface == ArtifactSurface::BudgetOutcome
                && artifact.boundary == WorkflowState::Inspected
        })
        .unwrap();
    assert_eq!(
        budget
            .payload
            .get("all_passed")
            .and_then(|value| value.as_bool()),
        Some(true)
    );
}

#[test]
fn workflow_certification_runner_tracks_relational_fintech_replay_regression() {
    let runner = WorkflowCertificationRunner::new(RelationalFintechWorkflowCertificationAdapter);
    let plan = relational_fintech_replay_regression_plan();
    let report = runner.certify(&plan, &development_profile()).unwrap();
    let replay = report
        .session
        .session_data
        .named_replays
        .get("analysis_replay")
        .unwrap();

    assert!(matches!(
        plan.regression_target.as_ref().map(|target| &target.kind),
        Some(RegressionTargetKind::KnownFailing)
    ));
    assert_eq!(replay.requested.branch_id.0, "analysis");

    match (
        &report.session.state,
        &report.failure_bundle,
        &replay.failure,
    ) {
        (WorkflowState::Failed, Some(bundle), Some(_)) => {
            assert!(bundle
                .invariant_failures
                .iter()
                .any(
                    |report| report.check_id == "replay_has_no_failure:analysis_replay"
                        && !report.passed
                ));
        }
        (WorkflowState::Completed, None, None) => {}
        other => panic!("unexpected regression workflow outcome: {other:?}"),
    }
}

#[test]
fn workflow_certification_runner_proves_relational_fintech_intraday_risk() {
    let runner = WorkflowCertificationRunner::new(RelationalFintechWorkflowCertificationAdapter);
    let plan = relational_fintech_intraday_risk_plan();
    let report = runner.certify(&plan, &development_profile()).unwrap();

    assert_eq!(report.session.state, WorkflowState::Completed);
    assert!(report.failure_bundle.is_none());
    assert!(report
        .session
        .session_data
        .named_reads
        .contains_key("analysis_intraday_case"));
    assert!(report
        .session
        .session_data
        .named_replays
        .contains_key("analysis_intraday_replay"));
}

#[test]
fn workflow_certification_runner_proves_relational_fintech_settlement_repair() {
    let runner = WorkflowCertificationRunner::new(RelationalFintechWorkflowCertificationAdapter);
    let plan = relational_fintech_settlement_repair_plan();
    let report = runner.certify(&plan, &development_profile()).unwrap();

    assert_eq!(report.session.state, WorkflowState::Completed);
    assert!(report.failure_bundle.is_none());
    assert!(report
        .session
        .session_data
        .named_reads
        .contains_key("analysis_repair_case"));
    assert!(report
        .session
        .session_data
        .named_replays
        .contains_key("analysis_repair_replay"));
}
