use std::time::Duration;

use super::{
    C6SiegeTimings, SiegePhase, POSTBUILD_BINARY_BINDING_BUDGET_MS,
    PREBUILD_SOURCE_BINDING_BUDGET_MS, SOURCE_INVENTORY_BUDGET_MS,
};

#[test]
fn cold_build_is_the_only_exclusion_from_completed_campaign_budget() {
    let timings = complete_timings();
    assert_eq!(
        timings
            .validate_completed_campaign(Duration::from_secs(605))
            .unwrap(),
        5_000
    );
    assert!(timings
        .validate_completed_campaign(Duration::from_secs(631))
        .is_err());
}

#[test]
fn each_child_stage_has_an_independent_five_second_budget() {
    for phase in [
        SiegePhase::SiegeWriter,
        SiegePhase::OfflineObserver,
        SiegePhase::FreshReopener,
    ] {
        let mut timings = complete_timings();
        phase_mut(&mut timings, phase).elapsed_ms = 5_001;
        assert!(timings.validate_runtime_budget().is_err(), "{phase:?}");
    }
}

#[test]
fn each_binding_stage_has_an_independent_workload_budget() {
    for (phase, budget) in [
        (SiegePhase::SourceInventory, SOURCE_INVENTORY_BUDGET_MS),
        (
            SiegePhase::PrebuildSourceBinding,
            PREBUILD_SOURCE_BINDING_BUDGET_MS,
        ),
        (
            SiegePhase::PostbuildBinaryBinding,
            POSTBUILD_BINARY_BINDING_BUDGET_MS,
        ),
    ] {
        let mut timings = complete_timings();
        phase_mut(&mut timings, phase).elapsed_ms = budget + 1;
        assert!(timings.validate_runtime_budget().is_err(), "{phase:?}");
    }
}

#[test]
fn missing_substituted_and_duplicate_phases_are_rejected() {
    let mut missing = complete_timings();
    missing
        .phases
        .retain(|phase| phase.identity != SiegePhase::OracleVerification);
    assert!(missing.validate_complete_budget().is_err());

    let mut substituted = complete_timings();
    phase_mut(&mut substituted, SiegePhase::OracleVerification).identity =
        SiegePhase::RunProvenance;
    assert!(substituted.validate_complete_budget().is_err());

    let mut duplicate = complete_timings();
    duplicate.phases.push(duplicate.phases[0].clone());
    assert!(duplicate.validate_complete_budget().is_err());
}

#[test]
fn report_encoding_and_completed_wall_are_independently_bounded() {
    let mut timings = complete_timings();
    phase_mut(&mut timings, SiegePhase::ReportEncoding).elapsed_ms = 501;
    assert!(timings.validate_complete_budget().is_err());

    let timings = complete_timings();
    assert!(timings
        .validate_completed_campaign(Duration::from_secs(599))
        .is_err());
}

fn complete_timings() -> C6SiegeTimings {
    let mut timings = C6SiegeTimings::new();
    for phase in SiegePhase::BEFORE_REPORT {
        let elapsed = match phase {
            SiegePhase::BinaryBuild => Duration::from_secs(600),
            SiegePhase::CampaignBeforeReport => Duration::from_secs(605),
            _ => Duration::from_millis(10),
        };
        timings.record(phase, elapsed);
    }
    timings.record(SiegePhase::ReportEncoding, Duration::from_millis(1));
    timings
}

fn phase_mut(timings: &mut C6SiegeTimings, identity: SiegePhase) -> &mut super::TimedSiegePhase {
    timings
        .phases
        .iter_mut()
        .find(|phase| phase.identity == identity)
        .unwrap()
}
