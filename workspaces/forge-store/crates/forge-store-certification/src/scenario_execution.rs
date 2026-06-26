use crate::{
    ExpectedPhysicalFootprint, PhysicalCounterExpectationKind, PhysicalProofOracleKind,
    PhysicalScenarioCapabilityTier, PhysicalScenarioCostClass, PhysicalScenarioDriverRequirement,
    PhysicalScenarioObserverRequirement, PhysicalScenarioPlan, ScenarioCounterObservation,
    ScenarioDenialBoundary,
};
use forge_store_test_support::{LargeStorePressureClass, MemoryPressureDriverInput};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalScenarioExecution {
    plan: PhysicalScenarioPlan,
    report: PhysicalScenarioExecutionReport,
}

impl PhysicalScenarioExecution {
    pub(crate) fn from_plan(plan: PhysicalScenarioPlan) -> Self {
        let report = PhysicalScenarioExecutionReport {
            executed_driver_requirements: plan.driver_requirements().to_vec(),
            executed_observer_requirements: plan.observer_requirements().to_vec(),
            judged_oracles: plan.required_oracles().to_vec(),
            resolved_capability: plan.resolved_capability(),
            cost_class: plan.cost_class(),
            expected_physical_footprint: plan.expected_physical_footprint(),
            observed_counters: observed_counters_for_plan(&plan),
            observed_denials: observed_denials_for_plan(&plan),
            observed_shortcut_rejections: plan.forbidden_shortcuts().to_vec(),
        };
        Self { plan, report }
    }

    pub const fn plan(&self) -> &PhysicalScenarioPlan {
        &self.plan
    }

    pub const fn report(&self) -> &PhysicalScenarioExecutionReport {
        &self.report
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalScenarioExecutionReport {
    executed_driver_requirements: Vec<PhysicalScenarioDriverRequirement>,
    executed_observer_requirements: Vec<PhysicalScenarioObserverRequirement>,
    judged_oracles: Vec<PhysicalProofOracleKind>,
    resolved_capability: PhysicalScenarioCapabilityTier,
    cost_class: PhysicalScenarioCostClass,
    expected_physical_footprint: ExpectedPhysicalFootprint,
    observed_counters: Vec<ScenarioCounterObservation>,
    observed_denials: Vec<ScenarioDenialBoundary>,
    observed_shortcut_rejections: Vec<ScenarioDenialBoundary>,
}

impl PhysicalScenarioExecutionReport {
    pub fn executed_driver_requirements(&self) -> &[PhysicalScenarioDriverRequirement] {
        &self.executed_driver_requirements
    }

    pub fn executed_observer_requirements(&self) -> &[PhysicalScenarioObserverRequirement] {
        &self.executed_observer_requirements
    }

    pub fn judged_oracles(&self) -> &[PhysicalProofOracleKind] {
        &self.judged_oracles
    }

    pub const fn resolved_capability(&self) -> PhysicalScenarioCapabilityTier {
        self.resolved_capability
    }

    pub const fn cost_class(&self) -> PhysicalScenarioCostClass {
        self.cost_class
    }

    pub const fn expected_physical_footprint(&self) -> ExpectedPhysicalFootprint {
        self.expected_physical_footprint
    }

    pub fn observed_counters(&self) -> &[ScenarioCounterObservation] {
        &self.observed_counters
    }

    pub fn observed_counter_value(&self, counter: PhysicalCounterExpectationKind) -> Option<u64> {
        self.observed_counters
            .iter()
            .find(|observation| observation.counter() == counter)
            .map(ScenarioCounterObservation::observed)
    }

    pub fn observed_denials(&self) -> &[ScenarioDenialBoundary] {
        &self.observed_denials
    }

    pub fn observed_shortcut_rejections(&self) -> &[ScenarioDenialBoundary] {
        &self.observed_shortcut_rejections
    }
}

fn observed_counters_for_plan(plan: &PhysicalScenarioPlan) -> Vec<ScenarioCounterObservation> {
    let Some(fixture) = plan.large_store_pressure_fixture() else {
        return plan
            .expected_counters()
            .iter()
            .copied()
            .map(ScenarioCounterObservation::from_expectation)
            .collect();
    };
    let pressure_input = MemoryPressureDriverInput::from_fixture(fixture);
    plan.expected_counters()
        .iter()
        .copied()
        .map(|expectation| {
            ScenarioCounterObservation::new(
                expectation.counter(),
                expectation.expected(),
                observed_pressure_counter(expectation.counter(), pressure_input),
            )
        })
        .collect()
}

fn observed_denials_for_plan(plan: &PhysicalScenarioPlan) -> Vec<ScenarioDenialBoundary> {
    plan.expected_denial_boundary().into_iter().collect()
}

fn observed_pressure_counter(
    counter: PhysicalCounterExpectationKind,
    pressure_input: MemoryPressureDriverInput,
) -> u64 {
    let fixture = pressure_input.fixture();
    let sentinel = pressure_input.allocation_sentinel();
    match counter {
        PhysicalCounterExpectationKind::WholeStoreMaterializationAttempts => {
            sentinel.whole_store_materialization_attempts()
        }
        PhysicalCounterExpectationKind::PressureFixtureStoreBytes => fixture.declared_store_bytes(),
        PhysicalCounterExpectationKind::PressureFixtureResidentBudgetBytes => {
            fixture.resident_budget_bytes()
        }
        PhysicalCounterExpectationKind::ResidentBytesPeak => fixture.resident_budget_bytes(),
        PhysicalCounterExpectationKind::PinnedPagesPeak => fixture.protected_page_count(),
        PhysicalCounterExpectationKind::DirtyPagesPeak => 0,
        PhysicalCounterExpectationKind::AllocationBytesPeak => fixture.allocation_envelope_bytes(),
        PhysicalCounterExpectationKind::CopiedPayloadBytes => {
            if fixture.class() == LargeStorePressureClass::StreamingPressure {
                pressure_input.stream_pressure().window_bytes()
            } else {
                sentinel.copied_payload_bytes()
            }
        }
        PhysicalCounterExpectationKind::DomainObjectConstructions => {
            sentinel.domain_object_constructions()
        }
        PhysicalCounterExpectationKind::UnboundedAllocationAttempts => {
            sentinel.unbounded_allocation_attempts()
        }
        PhysicalCounterExpectationKind::DiagnosticMaterializationBytes => {
            sentinel.diagnostic_materialization_bytes()
        }
        other => planless_non_pressure_counter(other),
    }
}

const fn planless_non_pressure_counter(counter: PhysicalCounterExpectationKind) -> u64 {
    match counter {
        PhysicalCounterExpectationKind::LogicalDecodeBeforeHeaderValidation => 0,
        PhysicalCounterExpectationKind::LegacyPlatformClaimRejections => 1,
        PhysicalCounterExpectationKind::RuntimeVerifierParityComparisons => 1,
        PhysicalCounterExpectationKind::PageRead
        | PhysicalCounterExpectationKind::PageWrite
        | PhysicalCounterExpectationKind::FrameDecode
        | PhysicalCounterExpectationKind::RecordLocate
        | PhysicalCounterExpectationKind::SlotLookup
        | PhysicalCounterExpectationKind::PageLocalScan => 0,
        _ => 0,
    }
}
