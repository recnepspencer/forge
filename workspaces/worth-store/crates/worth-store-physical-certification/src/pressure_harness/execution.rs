use crate::{
    CounterContractKind, ExpectedFaultLocalization, IoPressureHarnessEvidenceDenial,
    IoPressureHarnessScenario, IoPressureOracleObservation, PhysicalBoundarySeam,
    PhysicalCounterEvidenceReceipt, PhysicalFaultEvent, PhysicalSimulationPlan,
    PhysicalSimulationScenarioFamily,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IoPressureExecutionCounters {
    queue_depth: u64,
    interference_events: u64,
    allocation_bytes: u64,
}

impl IoPressureExecutionCounters {
    pub(crate) fn from_receipt(
        receipt: &PhysicalCounterEvidenceReceipt,
    ) -> Result<Self, IoPressureHarnessEvidenceDenial> {
        let queue_depth = required_counter(receipt, CounterContractKind::IoQueueDepth)?;
        let interference_events =
            required_counter(receipt, CounterContractKind::IoInterferenceEvents)?;
        let allocation_bytes = required_counter(receipt, CounterContractKind::AllocationBytes)?;
        if queue_depth == 0 || interference_events == 0 || allocation_bytes == 0 {
            return Err(IoPressureHarnessEvidenceDenial::MissingPressureCounterEvidence);
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

pub(crate) fn materialize_io_pressure_observation(
    plan: &PhysicalSimulationPlan,
    fault: &PhysicalFaultEvent,
    receipt: &PhysicalCounterEvidenceReceipt,
    scenario: &IoPressureHarnessScenario,
) -> Result<IoPressureOracleObservation, IoPressureHarnessEvidenceDenial> {
    require_io_pressure_plan(plan)?;
    require_io_pressure_fault(fault, scenario)?;
    let counters = IoPressureExecutionCounters::from_receipt(receipt)?;
    Ok(IoPressureOracleObservation::from_executed_pressure(
        scenario,
        counters,
        scenario.expected_status(),
    ))
}

fn require_io_pressure_plan(
    plan: &PhysicalSimulationPlan,
) -> Result<(), IoPressureHarnessEvidenceDenial> {
    if plan.scenario_family() != PhysicalSimulationScenarioFamily::IoPressureHarness {
        return Err(IoPressureHarnessEvidenceDenial::ScenarioFamilyMismatch);
    }
    let binding = plan.yieldpoint_binding();
    if binding.declared_yieldpoint().seam() != PhysicalBoundarySeam::IoPressure
        || binding.scheduled_yieldpoint() != binding.declared_yieldpoint().name()
    {
        return Err(IoPressureHarnessEvidenceDenial::MissingIoPressureYieldpoint);
    }
    Ok(())
}

fn require_io_pressure_fault(
    fault: &PhysicalFaultEvent,
    scenario: &IoPressureHarnessScenario,
) -> Result<(), IoPressureHarnessEvidenceDenial> {
    let PhysicalFaultEvent::IoStall(event) = fault else {
        return Err(IoPressureHarnessEvidenceDenial::MissingIoPressureFaultEvent);
    };
    if event.io_pressure_fault_kind() != Some(scenario.fault_kind()) {
        return Err(IoPressureHarnessEvidenceDenial::PressureFaultKindMismatch);
    }
    if event.locus().expected_localization() != ExpectedFaultLocalization::ProductionDriverBoundary
    {
        return Err(IoPressureHarnessEvidenceDenial::PressureFaultNotProductionBoundaryLocalized);
    }
    Ok(())
}

fn required_counter(
    receipt: &PhysicalCounterEvidenceReceipt,
    kind: CounterContractKind,
) -> Result<u64, IoPressureHarnessEvidenceDenial> {
    receipt
        .rows()
        .iter()
        .find(|row| row.kind() == kind)
        .map(|row| row.observed_count())
        .ok_or(IoPressureHarnessEvidenceDenial::MissingPressureCounterEvidence)
}
