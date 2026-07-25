use std::time::Duration;

use worth_store::physical_runtime::PhysicalWorkHostileTruthScenario;

use super::{
    CampaignPhase, CampaignTimings, ScenarioStage, TimingIdentity,
    POSTBUILD_BINARY_BINDING_BUDGET_MS, PREBUILD_SOURCE_BINDING_BUDGET_MS,
    SOURCE_INVENTORY_BUDGET_MS,
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
    assert!(timings
        .validate_completed_campaign(Duration::from_secs(331))
        .is_err());
}

#[test]
fn runtime_budget_rejects_each_bounded_stage_class() {
    for (phase, elapsed) in [
        (CampaignPhase::MutationEvidence, 30_001),
        (CampaignPhase::World, 1_001),
        (
            CampaignPhase::SourceInventory,
            SOURCE_INVENTORY_BUDGET_MS + 1,
        ),
        (
            CampaignPhase::PrebuildSourceBinding,
            PREBUILD_SOURCE_BINDING_BUDGET_MS + 1,
        ),
        (
            CampaignPhase::PostbuildBinaryBinding,
            POSTBUILD_BINARY_BINDING_BUDGET_MS + 1,
        ),
        (CampaignPhase::ExecutableVerification, 1_001),
    ] {
        let mut timings = complete_timings(Duration::from_millis(10));
        phase_mut(&mut timings, TimingIdentity::Campaign(phase)).elapsed_ms = elapsed;
        assert!(timings.validate_runtime_budget().is_err(), "{phase:?}");
    }

    let mut timings = complete_timings(Duration::from_millis(10));
    phase_mut(
        &mut timings,
        TimingIdentity::Scenario {
            scenario: PhysicalWorkHostileTruthScenario::BeforeBackendDispatch,
            stage: ScenarioStage::Seed,
        },
    )
    .elapsed_ms = 5_001;
    assert!(timings.validate_runtime_budget().is_err());
}

#[test]
fn scenario_aggregate_and_report_encoding_have_independent_budgets() {
    let timings = complete_timings(Duration::from_millis(700));
    assert!(timings.validate_runtime_budget().is_err());

    let mut timings = complete_timings(Duration::from_millis(10));
    phase_mut(
        &mut timings,
        TimingIdentity::Campaign(CampaignPhase::ReportEncoding),
    )
    .elapsed_ms = 501;
    assert!(timings.validate_complete_budget().is_err());
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
    assert!(timings
        .validate_completed_campaign(Duration::from_secs(299))
        .is_err());
}

fn complete_timings(scenario_stage: Duration) -> CampaignTimings {
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
