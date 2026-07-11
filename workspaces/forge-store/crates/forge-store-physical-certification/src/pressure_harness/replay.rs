use forge_store_io_scheduler::LatencyEnvelopeAssessmentStatus;

use super::execution::materialize_io_pressure_observation;
use crate::{
    IoPressureHarnessScenario, PhysicalBoundarySeam, PhysicalFaultEvent, PhysicalProofOracleKind,
    PhysicalProofOracleVerdictKind, PhysicalSimulationScenarioFamily, SimulationReplayBundle,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoPressureHarnessEvidenceDenial {
    ScenarioFamilyMismatch,
    ScheduleReplayIdentityMismatch,
    MissingIoPressureYieldpoint,
    MissingIoPressureFaultEvent,
    PressureFaultKindMismatch,
    PressureFaultNotProductionBoundaryLocalized,
    MissingIoPressureObservation,
    PressureObservationMismatch,
    MissingIoPressureOracleVerdict,
    PostAdmissionEnvelopeViolation(LatencyEnvelopeAssessmentStatus),
    IoPressureOracleNotSatisfied,
    MissingCounterReceiptRows,
    MissingPressureCounterEvidence,
    ForbiddenShortcutEvidencePresent,
}

pub(crate) fn require_io_pressure_replay_bundle(
    scenario: &IoPressureHarnessScenario,
    replay: &SimulationReplayBundle,
) -> Result<(), IoPressureHarnessEvidenceDenial> {
    if replay.plan().scenario_family() != PhysicalSimulationScenarioFamily::IoPressureHarness {
        return Err(IoPressureHarnessEvidenceDenial::ScenarioFamilyMismatch);
    }
    if !replay
        .schedule()
        .replay_identity_matches_plan(replay.plan())
    {
        return Err(IoPressureHarnessEvidenceDenial::ScheduleReplayIdentityMismatch);
    }
    if replay
        .plan()
        .yieldpoint_binding()
        .declared_yieldpoint()
        .seam()
        != PhysicalBoundarySeam::IoPressure
    {
        return Err(IoPressureHarnessEvidenceDenial::MissingIoPressureYieldpoint);
    }
    if !replay.trace().shortcut_rejections().is_empty() {
        return Err(IoPressureHarnessEvidenceDenial::ForbiddenShortcutEvidencePresent);
    }
    if !replay
        .fault_events()
        .iter()
        .any(|fault| matches_io_pressure_fault_kind(fault, scenario))
    {
        if replay
            .fault_events()
            .iter()
            .any(|fault| matches_io_pressure_fault_without_localization(fault, scenario))
        {
            return Err(
                IoPressureHarnessEvidenceDenial::PressureFaultNotProductionBoundaryLocalized,
            );
        }
        if replay
            .fault_events()
            .iter()
            .any(is_io_pressure_fault_delivery)
        {
            return Err(IoPressureHarnessEvidenceDenial::PressureFaultKindMismatch);
        }
        return Err(IoPressureHarnessEvidenceDenial::MissingIoPressureFaultEvent);
    }
    let matched_fault = replay
        .fault_events()
        .iter()
        .find(|fault| matches_io_pressure_fault_kind(fault, scenario))
        .expect("matching fault was checked above");
    let observation = replay
        .trace()
        .io_pressure_observation()
        .ok_or(IoPressureHarnessEvidenceDenial::MissingIoPressureObservation)?;
    let executed_observation = materialize_io_pressure_observation(
        replay.plan(),
        matched_fault,
        replay.counter_receipt(),
        scenario,
    )?;
    if observation != executed_observation || !observation.matches_scenario(scenario) {
        return Err(IoPressureHarnessEvidenceDenial::PressureObservationMismatch);
    }
    let verdict = replay
        .oracle_verdicts()
        .iter()
        .find(|verdict| verdict.oracle() == PhysicalProofOracleKind::IoPressureSimulation)
        .ok_or(IoPressureHarnessEvidenceDenial::MissingIoPressureOracleVerdict)?;
    if verdict.kind() == PhysicalProofOracleVerdictKind::Failed
        && observation.envelope_status() != LatencyEnvelopeAssessmentStatus::Held
    {
        return Err(
            IoPressureHarnessEvidenceDenial::PostAdmissionEnvelopeViolation(
                observation.envelope_status(),
            ),
        );
    }
    if verdict.kind() != PhysicalProofOracleVerdictKind::Satisfied {
        return Err(IoPressureHarnessEvidenceDenial::IoPressureOracleNotSatisfied);
    }
    if replay.counter_receipt().rows().is_empty() {
        return Err(IoPressureHarnessEvidenceDenial::MissingCounterReceiptRows);
    }
    Ok(())
}

fn is_io_pressure_fault_delivery(fault: &PhysicalFaultEvent) -> bool {
    matches!(fault, PhysicalFaultEvent::IoStall(_))
        && fault.required_seam() == PhysicalBoundarySeam::IoPressure
}

fn matches_io_pressure_fault_without_localization(
    fault: &PhysicalFaultEvent,
    scenario: &IoPressureHarnessScenario,
) -> bool {
    match fault {
        PhysicalFaultEvent::IoStall(event) => {
            is_io_pressure_fault_delivery(fault)
                && event.io_pressure_fault_kind() == Some(scenario.fault_kind())
        }
        _ => false,
    }
}

fn matches_io_pressure_fault_kind(
    fault: &PhysicalFaultEvent,
    scenario: &IoPressureHarnessScenario,
) -> bool {
    match fault {
        PhysicalFaultEvent::IoStall(event) => {
            is_io_pressure_fault_delivery(fault)
                && event.io_pressure_fault_kind() == Some(scenario.fault_kind())
                && event.locus().expected_localization()
                    == crate::ExpectedFaultLocalization::ProductionDriverBoundary
        }
        _ => false,
    }
}
