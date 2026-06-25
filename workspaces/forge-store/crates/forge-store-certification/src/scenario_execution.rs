use crate::{
    ExpectedPhysicalFootprint, PhysicalProofOracleKind, PhysicalScenarioCapabilityTier,
    PhysicalScenarioCostClass, PhysicalScenarioDriverRequirement,
    PhysicalScenarioObserverRequirement, PhysicalScenarioPlan,
};

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
}
