use crate::{
    CounterContractKind, ExpectedFaultLocalization, PhysicalBoundarySeam,
    PhysicalCounterEvidenceReceipt, PhysicalFaultEvent, PhysicalSimulationPlan,
    PhysicalSimulationScenarioFamily, S6IoPressureHarnessEvidenceDenial,
    S6IoPressureHarnessScenario, S6IoPressureOracleObservation,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S6IoPressureExecutionCounters {
    queue_depth: u64,
    interference_events: u64,
    allocation_bytes: u64,
}

impl S6IoPressureExecutionCounters {
    pub(crate) fn from_receipt(
        receipt: &PhysicalCounterEvidenceReceipt,
    ) -> Result<Self, S6IoPressureHarnessEvidenceDenial> {
        let queue_depth = required_counter(receipt, CounterContractKind::IoQueueDepth)?;
        let interference_events =
            required_counter(receipt, CounterContractKind::IoInterferenceEvents)?;
        let allocation_bytes = required_counter(receipt, CounterContractKind::AllocationBytes)?;
        if queue_depth == 0 || interference_events == 0 || allocation_bytes == 0 {
            return Err(S6IoPressureHarnessEvidenceDenial::MissingPressureCounterEvidence);
        }
        Ok(Self {
            queue_depth,
            interference_events,
            allocation_bytes,
        })
    }

    pub const fn queue_depth(self) -> u64 {
        self.queue_depth
    }

    pub const fn interference_events(self) -> u64 {
        self.interference_events
    }

    pub const fn allocation_bytes(self) -> u64 {
        self.allocation_bytes
    }
}

pub(crate) fn materialize_s6_pressure_observation(
    plan: &PhysicalSimulationPlan,
    fault: &PhysicalFaultEvent,
    receipt: &PhysicalCounterEvidenceReceipt,
    scenario: &S6IoPressureHarnessScenario,
) -> Result<S6IoPressureOracleObservation, S6IoPressureHarnessEvidenceDenial> {
    require_s6_pressure_plan(plan)?;
    require_s6_pressure_fault(fault, scenario)?;
    let counters = S6IoPressureExecutionCounters::from_receipt(receipt)?;
    Ok(S6IoPressureOracleObservation::from_executed_pressure(
        scenario,
        counters,
        scenario.expected_status(),
    ))
}

fn require_s6_pressure_plan(
    plan: &PhysicalSimulationPlan,
) -> Result<(), S6IoPressureHarnessEvidenceDenial> {
    if plan.scenario_family() != PhysicalSimulationScenarioFamily::S6IoPressureHarness {
        return Err(S6IoPressureHarnessEvidenceDenial::ScenarioFamilyMismatch);
    }
    let binding = plan.yieldpoint_binding();
    if binding.declared_yieldpoint().seam() != PhysicalBoundarySeam::IoPressure
        || binding.scheduled_yieldpoint() != binding.declared_yieldpoint().name()
    {
        return Err(S6IoPressureHarnessEvidenceDenial::MissingIoPressureYieldpoint);
    }
    Ok(())
}

fn require_s6_pressure_fault(
    fault: &PhysicalFaultEvent,
    scenario: &S6IoPressureHarnessScenario,
) -> Result<(), S6IoPressureHarnessEvidenceDenial> {
    let PhysicalFaultEvent::IoStall(event) = fault else {
        return Err(S6IoPressureHarnessEvidenceDenial::MissingIoPressureFaultEvent);
    };
    if event.s6_pressure_fault_kind() != Some(scenario.fault_kind()) {
        return Err(S6IoPressureHarnessEvidenceDenial::PressureFaultKindMismatch);
    }
    if event.locus().expected_localization() != ExpectedFaultLocalization::ProductionDriverBoundary
    {
        return Err(S6IoPressureHarnessEvidenceDenial::PressureFaultNotProductionBoundaryLocalized);
    }
    Ok(())
}

fn required_counter(
    receipt: &PhysicalCounterEvidenceReceipt,
    kind: CounterContractKind,
) -> Result<u64, S6IoPressureHarnessEvidenceDenial> {
    receipt
        .rows()
        .iter()
        .find(|row| row.kind() == kind)
        .map(|row| row.observed_count())
        .ok_or(S6IoPressureHarnessEvidenceDenial::MissingPressureCounterEvidence)
}
