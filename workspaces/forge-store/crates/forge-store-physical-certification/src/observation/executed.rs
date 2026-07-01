use super::ObservationDenial;
use crate::{
    PhysicalScenarioCanonicalIdentity, PhysicalSimulationPlan, PhysicalSimulationPlanIdentity,
    ProductionBoundaryDriverTrace,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutedPhysicalSimulationObservation {
    scenario_identity: PhysicalScenarioCanonicalIdentity,
    plan_identity: PhysicalSimulationPlanIdentity,
    runtime_trace: ProductionBoundaryDriverTrace,
}

impl ExecutedPhysicalSimulationObservation {
    pub fn from_executed_plan(plan: &PhysicalSimulationPlan) -> Result<Self, ObservationDenial> {
        let runtime_trace = plan
            .driver_contracts()
            .iter()
            .find_map(|driver| driver.production_boundary_trace())
            .ok_or(ObservationDenial::MissingExecutedProductionBoundaryTrace)?;

        Ok(Self {
            scenario_identity: plan.scenario_identity().clone(),
            plan_identity: plan.identity().clone(),
            runtime_trace,
        })
    }

    pub const fn scenario_identity(&self) -> &PhysicalScenarioCanonicalIdentity {
        &self.scenario_identity
    }

    pub const fn plan_identity(&self) -> &PhysicalSimulationPlanIdentity {
        &self.plan_identity
    }

    pub const fn runtime_trace(&self) -> &ProductionBoundaryDriverTrace {
        &self.runtime_trace
    }
}
