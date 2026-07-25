use std::time::Duration;

use worth_store::physical_runtime::PhysicalWorkHostileTruthScenario;

use crate::courtroom_campaign::report_publication::CourtroomReportSession;

use super::{
    CampaignPhase, CampaignTimings, ScenarioStage, TimingIdentity,
    EXECUTABLE_VERIFICATION_BUDGET_MS, FINAL_SOURCE_BINDING_BUDGET_MS, MUTATION_EVIDENCE_BUDGET_MS,
    POSTBUILD_BINARY_BINDING_BUDGET_MS, POSTBUILD_SOURCE_BINDING_BUDGET_MS,
    PREBUILD_SOURCE_BINDING_BUDGET_MS, REPORT_ENCODING_BUDGET_MS, SCENARIO_STAGE_BUDGET_MS,
    SOURCE_INVENTORY_BUDGET_MS, WORLD_CREATION_BUDGET_MS,
};

#[test]
fn cold_build_is_the_only_exclusion_from_completed_campaign_budget() {
    let timings = complete_timings(Duration::from_millis(10));
    assert_eq!(
        timings
            .validate_completed_campaign(Duration::from_secs(305))
            .unwrap(),
        5_000
    );
    assert_postpublication_rejection(&timings, Duration::from_secs(331), "runner-controlled work");
}

#[test]
fn runtime_budget_rejects_each_bounded_stage_class() {
    for (phase, budget) in [
        (CampaignPhase::MutationEvidence, MUTATION_EVIDENCE_BUDGET_MS),
        (CampaignPhase::World, WORLD_CREATION_BUDGET_MS),
        (CampaignPhase::SourceInventory, SOURCE_INVENTORY_BUDGET_MS),
        (
            CampaignPhase::PrebuildSourceBinding,
            PREBUILD_SOURCE_BINDING_BUDGET_MS,
        ),
        (
            CampaignPhase::PostbuildBinaryBinding,
            POSTBUILD_BINARY_BINDING_BUDGET_MS,
        ),
        (
            CampaignPhase::PostbuildSourceBinding,
            POSTBUILD_SOURCE_BINDING_BUDGET_MS,
        ),
        (
            CampaignPhase::FinalSourceBinding,
            FINAL_SOURCE_BINDING_BUDGET_MS,
        ),
        (
            CampaignPhase::ExecutableVerification,
            EXECUTABLE_VERIFICATION_BUDGET_MS,
        ),
    ] {
        let mut timings = runtime_timings(Duration::from_millis(10));
        phase_mut(&mut timings, TimingIdentity::Campaign(phase)).elapsed_ms = budget + 1;
        assert_prepublication_rejection(&timings, false, phase.label());
    }

    for scenario in PhysicalWorkHostileTruthScenario::ALL {
        for stage in ScenarioStage::ALL {
            let identity = TimingIdentity::Scenario { scenario, stage };
            let mut timings = runtime_timings(Duration::from_millis(10));
            phase_mut(&mut timings, identity).elapsed_ms = SCENARIO_STAGE_BUDGET_MS + 1;
            assert_prepublication_rejection(&timings, false, &identity.label());
        }
        let identity = TimingIdentity::CaseVerification(scenario);
        let mut timings = runtime_timings(Duration::from_millis(10));
        phase_mut(&mut timings, identity).elapsed_ms = SCENARIO_STAGE_BUDGET_MS + 1;
        assert_prepublication_rejection(&timings, false, &identity.label());
    }
}

#[test]
fn scenario_aggregate_and_report_encoding_have_independent_budgets() {
    let timings = runtime_timings(Duration::from_millis(700));
    assert_prepublication_rejection(&timings, false, "scenario stages took");

    let mut timings = complete_timings(Duration::from_millis(10));
    phase_mut(
        &mut timings,
        TimingIdentity::Campaign(CampaignPhase::ReportEncoding),
    )
    .elapsed_ms = REPORT_ENCODING_BUDGET_MS + 1;
    assert_prepublication_rejection(&timings, true, CampaignPhase::ReportEncoding.label());
}

#[test]
fn missing_substituted_and_duplicate_scenario_stages_are_rejected() {
    let mut missing = complete_timings(Duration::from_millis(10));
    missing.phases.retain(|phase| {
        phase.identity
            != TimingIdentity::Scenario {
                scenario: PhysicalWorkHostileTruthScenario::DuringShortWrite,
                stage: ScenarioStage::Fault,
            }
    });
    assert!(missing.validate_complete_budget().is_err());

    let mut substituted = complete_timings(Duration::from_millis(10));
    phase_mut(
        &mut substituted,
        TimingIdentity::Scenario {
            scenario: PhysicalWorkHostileTruthScenario::DuringShortWrite,
            stage: ScenarioStage::Fault,
        },
    )
    .identity =
        TimingIdentity::CaseVerification(PhysicalWorkHostileTruthScenario::BeforeBackendDispatch);
    assert!(substituted.validate_complete_budget().is_err());

    let mut duplicate = complete_timings(Duration::from_millis(10));
    duplicate.phases.push(duplicate.phases[0].clone());
    assert!(duplicate.validate_complete_budget().is_err());
}

#[test]
fn completed_wall_cannot_be_smaller_than_the_recorded_cold_build() {
    let timings = complete_timings(Duration::from_millis(10));
    assert_postpublication_rejection(
        &timings,
        Duration::from_secs(299),
        "cold-build timing exceeded",
    );
}

#[test]
fn timing_fixtures_are_valid_before_hostile_deltas() {
    assert!(runtime_timings(Duration::from_millis(10))
        .validate_runtime_budget()
        .is_ok());
    assert!(complete_timings(Duration::from_millis(10))
        .validate_complete_budget()
        .is_ok());
}

fn runtime_timings(scenario_stage: Duration) -> CampaignTimings {
    let mut timings = CampaignTimings::new();
    for phase in CampaignPhase::BEFORE_REPORT {
        let elapsed = match phase {
            CampaignPhase::BinaryBuild => Duration::from_secs(300),
            CampaignPhase::CampaignBeforeReport => Duration::from_secs(305),
            _ => Duration::from_millis(10),
        };
        timings.record_campaign(phase, elapsed);
    }
    for scenario in PhysicalWorkHostileTruthScenario::ALL {
        for stage in ScenarioStage::ALL {
            timings.record_scenario(scenario, stage, scenario_stage);
        }
        timings.record_case_verification(scenario, Duration::from_millis(10));
    }
    timings
}

fn complete_timings(scenario_stage: Duration) -> CampaignTimings {
    let mut timings = runtime_timings(scenario_stage);
    timings.record_campaign(CampaignPhase::ReportEncoding, Duration::from_millis(1));
    timings
}

fn phase_mut(
    timings: &mut CampaignTimings,
    identity: TimingIdentity,
) -> &mut super::TimedCampaignPhase {
    timings
        .phases
        .iter_mut()
        .find(|phase| phase.identity == identity)
        .unwrap()
}

fn assert_prepublication_rejection(
    timings: &CampaignTimings,
    complete: bool,
    expected_cause: &str,
) {
    let temporary = tempfile::tempdir().unwrap();
    let report = temporary.path().join("courtroom-b-stage-budget.json");
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
    timings: &CampaignTimings,
    completed_wall: Duration,
    expected_cause: &str,
) {
    let temporary = tempfile::tempdir().unwrap();
    let report = temporary.path().join("courtroom-b-total-budget.json");
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
