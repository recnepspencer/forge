use super::ObservationDenial;
use crate::{
    PhysicalBoundarySeam, PhysicalInterleavingSchedule, PhysicalScenarioCanonicalIdentity,
    PhysicalScheduleExecution, PhysicalSimulationPlan, PhysicalSimulationPlanIdentity,
    ProductionBoundaryDriverTrace,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalSimulationObservationBasis {
    ScheduledStorageOwnerExecution,
    DeclaredDriverShapeProbe,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalSimulationBoundaryObservation {
    basis: PhysicalSimulationObservationBasis,
    scenario_identity: PhysicalScenarioCanonicalIdentity,
    plan_identity: PhysicalSimulationPlanIdentity,
    runtime_trace: ProductionBoundaryDriverTrace,
}

impl PhysicalSimulationBoundaryObservation {
    pub fn from_scheduled_storage_execution(
        plan: &PhysicalSimulationPlan,
        schedule: &PhysicalInterleavingSchedule,
        execution: &PhysicalScheduleExecution,
    ) -> Result<Self, ObservationDenial> {
        let completed_steps_match_schedule = execution
            .completed_steps()
            .iter()
            .map(|completed| completed.step())
            .eq(schedule.actor_steps());
        if !schedule.replay_identity_matches_plan(plan)
            || execution.schedule_identity() != schedule.identity()
            || !completed_steps_match_schedule
        {
            return Err(ObservationDenial::ScheduleExecutionMismatch);
        }
        let PhysicalBoundarySeam::ProductionStorage(seam) =
            plan.yieldpoint_binding().declared_yieldpoint().seam()
        else {
            return Err(ObservationDenial::StorageExecutionDidNotReachScheduledSeam);
        };
        if execution.storage_seam() != seam
            || execution
                .completed_steps()
                .iter()
                .any(|completed| !completed.storage_trace().reached().contains(&seam))
        {
            return Err(ObservationDenial::StorageExecutionDidNotReachScheduledSeam);
        }
        Self::from_plan(
            plan,
            PhysicalSimulationObservationBasis::ScheduledStorageOwnerExecution,
        )
    }

    pub fn from_declared_driver_shape_probe(
        plan: &PhysicalSimulationPlan,
    ) -> Result<Self, ObservationDenial> {
        Self::from_plan(
            plan,
            PhysicalSimulationObservationBasis::DeclaredDriverShapeProbe,
        )
    }

    fn from_plan(
        plan: &PhysicalSimulationPlan,
        basis: PhysicalSimulationObservationBasis,
    ) -> Result<Self, ObservationDenial> {
        let runtime_trace = plan
            .driver_contracts()
            .iter()
            .find_map(|driver| driver.production_boundary_trace())
            .ok_or(ObservationDenial::MissingExecutedProductionBoundaryTrace)?;
        Ok(Self {
            basis,
            scenario_identity: plan.scenario_identity().clone(),
            plan_identity: plan.identity().clone(),
            runtime_trace,
        })
    }

    pub const fn basis(&self) -> PhysicalSimulationObservationBasis {
        self.basis
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
