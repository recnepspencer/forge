use super::PhysicalCoverageRegistry;
use crate::{AdmittedDriverContractSet, PhysicalInterleavingSchedule};

use super::super::{
    CoverageGapDenial, CoverageRowDimension, CoverageSurfaceKind, PhysicalCoverageMatrixRow,
};

impl PhysicalCoverageRegistry {
    pub fn register_schedule(
        mut self,
        schedule: &PhysicalInterleavingSchedule,
    ) -> Result<Self, CoverageGapDenial> {
        self.require_surface_not_registered(CoverageSurfaceKind::YieldpointSchedule)?;
        let plan =
            self.plan
                .as_ref()
                .ok_or(CoverageGapDenial::MissingPlanBeforeDependentSurface {
                    surface: CoverageSurfaceKind::YieldpointSchedule,
                })?;
        if !schedule.replay_identity_matches_plan(plan) {
            return Err(CoverageGapDenial::PlanScheduleIdentityMismatch);
        }
        self.rows.push(PhysicalCoverageMatrixRow::generated(
            self.sequence,
            CoverageSurfaceKind::YieldpointSchedule,
            *schedule.identity().digest_bytes(),
            [CoverageRowDimension::ProductionBoundaryYieldpoint(
                plan.yieldpoint_binding().scheduled_yieldpoint().to_owned(),
            )],
        ));
        Ok(self)
    }

    pub fn register_actor_set(mut self) -> Result<Self, CoverageGapDenial> {
        self.require_surface_not_registered(CoverageSurfaceKind::Actor)?;
        let plan =
            self.plan
                .as_ref()
                .ok_or(CoverageGapDenial::MissingPlanBeforeDependentSurface {
                    surface: CoverageSurfaceKind::Actor,
                })?;
        if plan.actors().len() == 0 {
            return Err(CoverageGapDenial::EmptyActorRegistration);
        }
        self.rows.push(PhysicalCoverageMatrixRow::generated(
            self.sequence,
            CoverageSurfaceKind::Actor,
            *plan.identity().digest_bytes(),
            plan.actors()
                .iter()
                .map(|actor| CoverageRowDimension::ActorRole(actor.role())),
        ));
        Ok(self)
    }

    pub fn register_driver_contracts(
        mut self,
        contracts: &AdmittedDriverContractSet,
    ) -> Result<Self, CoverageGapDenial> {
        self.require_surface_not_registered(CoverageSurfaceKind::Driver)?;
        if contracts.iter().next().is_none() {
            return Err(CoverageGapDenial::EmptyDriverRegistration);
        }
        let plan =
            self.plan
                .as_ref()
                .ok_or(CoverageGapDenial::MissingPlanBeforeDependentSurface {
                    surface: CoverageSurfaceKind::Driver,
                })?;
        if contracts != plan.driver_contracts() {
            return Err(CoverageGapDenial::DriverContractPlanMismatch);
        }
        let identity = *plan.identity().digest_bytes();
        self.rows.push(PhysicalCoverageMatrixRow::generated(
            self.sequence,
            CoverageSurfaceKind::Driver,
            identity,
            contracts
                .iter()
                .map(|driver| CoverageRowDimension::BackgroundInterference(driver.kind())),
        ));
        Ok(self)
    }
}
