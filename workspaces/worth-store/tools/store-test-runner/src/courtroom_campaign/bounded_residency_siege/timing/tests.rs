use std::time::Duration;

use crate::courtroom_campaign::report_publication::CourtroomReportSession;

use super::{
    BoundedResidencySiegePhase, BoundedResidencySiegeTimings, CHILD_STAGE_BUDGET_MS,
    EXECUTABLE_VERIFICATION_BUDGET_MS, FINAL_SOURCE_BINDING_BUDGET_MS, MUTATION_EVIDENCE_BUDGET_MS,
    POSTBUILD_BINARY_BINDING_BUDGET_MS, POSTBUILD_SOURCE_BINDING_BUDGET_MS,
    PREBUILD_SOURCE_BINDING_BUDGET_MS, REPORT_ENCODING_BUDGET_MS,
    RUNNER_CONTROLLED_TOTAL_BUDGET_MS, SERVING_STAGE_BUDGET_MS, SOURCE_INVENTORY_BUDGET_MS,
    WORLD_BUDGET_MS,
};

#[test]
fn reconstructive_build_and_producer_are_excluded_from_runtime_acceptance() {
    let timings = complete_timings();
    assert_eq!(
        timings
            .validate_completed_campaign(Duration::from_secs(605))
            .unwrap(),
        4_990
    );
    assert_postpublication_rejection(
        &timings,
        Duration::from_millis(600_010 + RUNNER_CONTROLLED_TOTAL_BUDGET_MS + 1),
        "runner-controlled work",
    );

    let mut slow_setup = complete_timings();
    phase_mut(&mut slow_setup, BoundedResidencySiegePhase::SiegeProducer).elapsed_ms = 120_000;
    assert!(slow_setup.validate_complete_budget().is_ok());
}

#[test]
fn runtime_budget_rejects_each_enforced_stage_for_its_own_cause() {
    for (phase, budget) in [
        (
            BoundedResidencySiegePhase::MutationEvidence,
            MUTATION_EVIDENCE_BUDGET_MS,
        ),
        (BoundedResidencySiegePhase::World, WORLD_BUDGET_MS),
        (
            BoundedResidencySiegePhase::SourceInventory,
            SOURCE_INVENTORY_BUDGET_MS,
        ),
        (
            BoundedResidencySiegePhase::PrebuildSourceBinding,
            PREBUILD_SOURCE_BINDING_BUDGET_MS,
        ),
        (
            BoundedResidencySiegePhase::PostbuildBinaryBinding,
            POSTBUILD_BINARY_BINDING_BUDGET_MS,
        ),
        (
            BoundedResidencySiegePhase::PostbuildSourceBinding,
            POSTBUILD_SOURCE_BINDING_BUDGET_MS,
        ),
        (
            BoundedResidencySiegePhase::SiegeServing,
            SERVING_STAGE_BUDGET_MS,
        ),
        (
            BoundedResidencySiegePhase::OfflineObserver,
            CHILD_STAGE_BUDGET_MS,
        ),
        (
            BoundedResidencySiegePhase::FreshReopener,
            CHILD_STAGE_BUDGET_MS,
        ),
        (
            BoundedResidencySiegePhase::FinalSourceBinding,
            FINAL_SOURCE_BINDING_BUDGET_MS,
        ),
        (
            BoundedResidencySiegePhase::ExecutableVerification,
            EXECUTABLE_VERIFICATION_BUDGET_MS,
        ),
    ] {
        let mut timings = runtime_timings();
        phase_mut(&mut timings, phase).elapsed_ms = budget + 1;
        assert_prepublication_rejection(
            &timings,
            false,
            phase.label(),
            "courtroom-c-stage-budget.json",
        );
    }
}

#[test]
fn missing_substituted_and_duplicate_phases_are_rejected() {
    let mut missing = complete_timings();
    missing
        .phases
        .retain(|phase| phase.identity != BoundedResidencySiegePhase::OracleVerification);
    assert!(missing.validate_complete_budget().is_err());

    let mut substituted = complete_timings();
    phase_mut(
        &mut substituted,
        BoundedResidencySiegePhase::OracleVerification,
    )
    .identity = BoundedResidencySiegePhase::RunProvenance;
    assert!(substituted.validate_complete_budget().is_err());

    let mut duplicate = complete_timings();
    duplicate.phases.push(duplicate.phases[0].clone());
    assert!(duplicate.validate_complete_budget().is_err());
}

#[test]
fn report_encoding_and_completed_wall_are_independently_bounded() {
    let mut timings = complete_timings();
    phase_mut(&mut timings, BoundedResidencySiegePhase::ReportEncoding).elapsed_ms =
        REPORT_ENCODING_BUDGET_MS + 1;
    assert_prepublication_rejection(
        &timings,
        true,
        BoundedResidencySiegePhase::ReportEncoding.label(),
        "courtroom-c-report-encoding.json",
    );

    let timings = complete_timings();
    assert_postpublication_rejection(&timings, Duration::from_secs(599), "setup timing exceeded");
}

#[test]
fn source_bound_report_encoding_retains_observed_scheduler_headroom() {
    let mut timings = complete_timings();
    phase_mut(&mut timings, BoundedResidencySiegePhase::ReportEncoding).elapsed_ms = 2_500;
    if timings.validate_complete_budget().is_err() {
        panic!("MUTANT_PREDICATE:report-encoding-budget-regressed");
    }
}

#[test]
fn timing_fixtures_are_valid_before_hostile_deltas() {
    assert!(runtime_timings().validate_runtime_budget().is_ok());
    assert!(complete_timings().validate_complete_budget().is_ok());
}

fn runtime_timings() -> BoundedResidencySiegeTimings {
    let mut timings = BoundedResidencySiegeTimings::new();
    for phase in BoundedResidencySiegePhase::BEFORE_REPORT {
        let elapsed = match phase {
            BoundedResidencySiegePhase::BinaryBuild => Duration::from_secs(600),
            BoundedResidencySiegePhase::CampaignBeforeReport => Duration::from_secs(605),
            _ => Duration::from_millis(10),
        };
        timings.record(phase, elapsed);
    }
    timings
}

fn complete_timings() -> BoundedResidencySiegeTimings {
    let mut timings = runtime_timings();
    timings.record(
        BoundedResidencySiegePhase::ReportEncoding,
        Duration::from_millis(1),
    );
    timings
}

fn phase_mut(
    timings: &mut BoundedResidencySiegeTimings,
    identity: BoundedResidencySiegePhase,
) -> &mut super::TimedSiegePhase {
    timings
        .phases
        .iter_mut()
        .find(|phase| phase.identity == identity)
        .unwrap()
}

fn assert_prepublication_rejection(
    timings: &BoundedResidencySiegeTimings,
    complete: bool,
    expected_cause: &str,
    report_name: &str,
) {
    let temporary = tempfile::tempdir().unwrap();
    let report = temporary.path().join(report_name);
    std::fs::write(&report, b"stale success").unwrap();
    let session = CourtroomReportSession::begin(&report).unwrap();
    let result = if complete {
        timings.validate_complete_budget()
    } else {
        timings.validate_runtime_budget()
    };
    let error = result.expect_err("hostile timing must reject report publication");
    assert!(error.contains(expected_cause), "{error}");
    drop(session);
    assert_no_report_artifact(temporary.path());
}

fn assert_postpublication_rejection(
    timings: &BoundedResidencySiegeTimings,
    completed_wall: Duration,
    expected_cause: &str,
) {
    let temporary = tempfile::tempdir().unwrap();
    let report = temporary.path().join("courtroom-c-total-budget.json");
    let publication = CourtroomReportSession::begin(&report)
        .unwrap()
        .publish(b"{\"accepted\":true}")
        .unwrap();
    assert!(report.exists());
    let error = timings
        .validate_completed_campaign(completed_wall)
        .expect_err("hostile completed wall must reject report publication");
    assert!(error.contains(expected_cause), "{error}");
    drop(publication);
    assert_no_report_artifact(temporary.path());
}

fn assert_no_report_artifact(directory: &std::path::Path) {
    assert_eq!(std::fs::read_dir(directory).unwrap().count(), 0);
}
