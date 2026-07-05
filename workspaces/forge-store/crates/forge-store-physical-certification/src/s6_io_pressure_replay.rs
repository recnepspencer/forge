use forge_store_io_scheduler::LatencyEnvelopeAssessmentStatus;

use crate::s6_io_pressure_execution::materialize_s6_pressure_observation;
use crate::{
    PhysicalBoundarySeam, PhysicalFaultEvent, PhysicalProofOracleKind,
    PhysicalProofOracleVerdictKind, PhysicalSimulationScenarioFamily, S6IoPressureHarnessScenario,
    SimulationReplayBundle,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S6IoPressureHarnessEvidenceDenial {
    ScenarioFamilyMismatch,
    ScheduleReplayIdentityMismatch,
    MissingIoPressureYieldpoint,
    MissingIoPressureFaultEvent,
    PressureFaultKindMismatch,
    PressureFaultNotProductionBoundaryLocalized,
    MissingS6PressureObservation,
    PressureObservationMismatch,
    MissingS6PressureOracleVerdict,
    PostAdmissionEnvelopeViolation(LatencyEnvelopeAssessmentStatus),
    S6PressureOracleNotSatisfied,
    MissingCounterReceiptRows,
    MissingPressureCounterEvidence,
    ForbiddenShortcutEvidencePresent,
}

pub(crate) fn require_s6_replay_bundle(
    scenario: &S6IoPressureHarnessScenario,
    replay: &SimulationReplayBundle,
) -> Result<(), S6IoPressureHarnessEvidenceDenial> {
    if replay.plan().scenario_family() != PhysicalSimulationScenarioFamily::S6IoPressureHarness {
        return Err(S6IoPressureHarnessEvidenceDenial::ScenarioFamilyMismatch);
    }
    if !replay
        .schedule()
        .replay_identity_matches_plan(replay.plan())
    {
        return Err(S6IoPressureHarnessEvidenceDenial::ScheduleReplayIdentityMismatch);
    }
    if replay
        .plan()
        .yieldpoint_binding()
        .declared_yieldpoint()
        .seam()
        != PhysicalBoundarySeam::IoPressure
    {
        return Err(S6IoPressureHarnessEvidenceDenial::MissingIoPressureYieldpoint);
    }
    if !replay.trace().shortcut_rejections().is_empty() {
        return Err(S6IoPressureHarnessEvidenceDenial::ForbiddenShortcutEvidencePresent);
    }
    if !replay
        .fault_events()
        .iter()
        .any(|fault| matches_s6_pressure_fault_kind(fault, scenario))
    {
        if replay
            .fault_events()
            .iter()
            .any(|fault| matches_s6_pressure_fault_without_localization(fault, scenario))
        {
            return Err(
                S6IoPressureHarnessEvidenceDenial::PressureFaultNotProductionBoundaryLocalized,
            );
        }
        if replay
            .fault_events()
            .iter()
            .any(is_io_pressure_fault_delivery)
        {
            return Err(S6IoPressureHarnessEvidenceDenial::PressureFaultKindMismatch);
        }
        return Err(S6IoPressureHarnessEvidenceDenial::MissingIoPressureFaultEvent);
    }
    let matched_fault = replay
        .fault_events()
        .iter()
        .find(|fault| matches_s6_pressure_fault_kind(fault, scenario))
        .expect("matching fault was checked above");
    let observation = replay
        .trace()
        .s6_io_pressure_observation()
        .ok_or(S6IoPressureHarnessEvidenceDenial::MissingS6PressureObservation)?;
    let executed_observation = materialize_s6_pressure_observation(
        replay.plan(),
        matched_fault,
        replay.counter_receipt(),
        scenario,
    )?;
    if observation != executed_observation || !observation.matches_scenario(scenario) {
        return Err(S6IoPressureHarnessEvidenceDenial::PressureObservationMismatch);
    }
    let verdict = replay
        .oracle_verdicts()
        .iter()
        .find(|verdict| verdict.oracle() == PhysicalProofOracleKind::S6IoPressureSimulation)
        .ok_or(S6IoPressureHarnessEvidenceDenial::MissingS6PressureOracleVerdict)?;
    if verdict.kind() == PhysicalProofOracleVerdictKind::Failed
        && observation.envelope_status() != LatencyEnvelopeAssessmentStatus::Held
    {
        return Err(
            S6IoPressureHarnessEvidenceDenial::PostAdmissionEnvelopeViolation(
                observation.envelope_status(),
            ),
        );
    }
    if verdict.kind() != PhysicalProofOracleVerdictKind::Satisfied {
        return Err(S6IoPressureHarnessEvidenceDenial::S6PressureOracleNotSatisfied);
    }
    if replay.counter_receipt().rows().is_empty() {
        return Err(S6IoPressureHarnessEvidenceDenial::MissingCounterReceiptRows);
    }
    Ok(())
}

fn is_io_pressure_fault_delivery(fault: &PhysicalFaultEvent) -> bool {
    matches!(fault, PhysicalFaultEvent::IoStall(_))
        && fault.required_seam() == PhysicalBoundarySeam::IoPressure
}

fn matches_s6_pressure_fault_without_localization(
    fault: &PhysicalFaultEvent,
    scenario: &S6IoPressureHarnessScenario,
) -> bool {
    match fault {
        PhysicalFaultEvent::IoStall(event) => {
            is_io_pressure_fault_delivery(fault)
                && event.s6_pressure_fault_kind() == Some(scenario.fault_kind())
        }
        _ => false,
    }
}

fn matches_s6_pressure_fault_kind(
    fault: &PhysicalFaultEvent,
    scenario: &S6IoPressureHarnessScenario,
) -> bool {
    match fault {
        PhysicalFaultEvent::IoStall(event) => {
            is_io_pressure_fault_delivery(fault)
                && event.s6_pressure_fault_kind() == Some(scenario.fault_kind())
                && event.locus().expected_localization()
                    == crate::ExpectedFaultLocalization::ProductionDriverBoundary
        }
        _ => false,
    }
}
