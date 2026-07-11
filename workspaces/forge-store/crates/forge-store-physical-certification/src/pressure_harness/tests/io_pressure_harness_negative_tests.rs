use crate::pressure_harness::fixtures::{
    mislocalized_io_pressure_fault_event, replay_bundle_with_delivered_fault,
    replay_bundle_with_fault_event,
};
use crate::{
    IoPressureFaultKind, IoPressureHarnessEvidence, IoPressureHarnessEvidenceDenial,
    IoPressureHarnessScenario, PhysicalProofOracleKind, PhysicalProofOracleVerdictKind,
    PhysicalSimulationProfile,
};
use forge_store_io_scheduler::LatencyEnvelopeAssessmentStatus;

#[test]
fn wrong_pressure_fault_kind_cannot_publish_io_pressure_replay_evidence() {
    let scenario = IoPressureHarnessScenario::deterministic_read_under_repair_pressure();
    let replay = replay_bundle_with_delivered_fault(
        scenario.clone(),
        PhysicalSimulationProfile::DeveloperSmoke,
        IoPressureFaultKind::DelayedSync,
    );

    let denial = IoPressureHarnessEvidence::from_replay_bundle(scenario, &replay).unwrap_err();

    assert_eq!(
        denial,
        IoPressureHarnessEvidenceDenial::PressureFaultKindMismatch
    );
}

#[test]
fn non_held_pressure_envelope_is_failed_verdict_not_oracle_denial() {
    let scenario = IoPressureHarnessScenario::deterministic_read_under_repair_pressure()
        .with_expected_status(LatencyEnvelopeAssessmentStatus::EnvelopeExceeded);
    let replay = replay_bundle_with_delivered_fault(
        scenario.clone(),
        PhysicalSimulationProfile::DeveloperSmoke,
        scenario.fault_kind(),
    );
    let verdict = replay
        .oracle_verdicts()
        .iter()
        .find(|verdict| verdict.oracle() == PhysicalProofOracleKind::IoPressureSimulation)
        .unwrap();

    assert_eq!(verdict.kind(), PhysicalProofOracleVerdictKind::Failed);
    assert_eq!(
        IoPressureHarnessEvidence::from_replay_bundle(scenario, &replay).unwrap_err(),
        IoPressureHarnessEvidenceDenial::PostAdmissionEnvelopeViolation(
            LatencyEnvelopeAssessmentStatus::EnvelopeExceeded
        )
    );
}

#[test]
fn pressure_fault_must_be_localized_to_production_driver_boundary() {
    let scenario = IoPressureHarnessScenario::deterministic_read_under_repair_pressure();
    let replay = replay_bundle_with_fault_event(
        scenario.clone(),
        PhysicalSimulationProfile::DeveloperSmoke,
        mislocalized_io_pressure_fault_event(scenario.fault_kind()),
    );

    assert_eq!(
        IoPressureHarnessEvidence::from_replay_bundle(scenario, &replay).unwrap_err(),
        IoPressureHarnessEvidenceDenial::PressureFaultNotProductionBoundaryLocalized
    );
}
