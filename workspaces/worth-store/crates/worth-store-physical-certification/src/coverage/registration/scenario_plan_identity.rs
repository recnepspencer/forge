use super::PhysicalCoverageRegistry;
use crate::{CertifiedPhysicalScenario, PhysicalSimulationPlan};

use super::super::{
    blob_dimensions::{append_blob_plan_dimensions, blob_scenario_dimensions},
    CoverageGapDenial, CoverageRowDimension, CoverageSurfaceKind, PhysicalCoverageMatrixRow,
};

impl PhysicalCoverageRegistry {
    pub fn register_scenario(
        mut self,
        scenario: &CertifiedPhysicalScenario,
    ) -> Result<Self, CoverageGapDenial> {
        self.require_surface_not_registered(CoverageSurfaceKind::Scenario)?;
        if let Some(plan) = self.plan.as_ref() {
            if scenario.identity() != plan.scenario_identity() {
                return Err(CoverageGapDenial::PlanScenarioIdentityMismatch);
            }
        }
        self.scenario_identity = Some(scenario.identity().clone());
        self.rows.push(PhysicalCoverageMatrixRow::generated(
            self.sequence,
            CoverageSurfaceKind::Scenario,
            *scenario.identity().digest_bytes(),
            [
                CoverageRowDimension::ProductionBoundaryYieldpoint(
                    scenario
                        .definition()
                        .schedule()
                        .production_boundary_yieldpoint()
                        .to_owned(),
                ),
                CoverageRowDimension::FaultPhase(scenario.definition().fault().kind()),
            ]
            .into_iter()
            .chain(blob_scenario_dimensions(scenario)),
        ));
        Ok(self)
    }

    pub fn register_plan(
        mut self,
        plan: &PhysicalSimulationPlan,
    ) -> Result<Self, CoverageGapDenial> {
        self.require_surface_not_registered(CoverageSurfaceKind::Plan)?;
        if let Some(scenario_identity) = self.scenario_identity.as_ref() {
            if scenario_identity != plan.scenario_identity() {
                return Err(CoverageGapDenial::PlanScenarioIdentityMismatch);
            }
        }
        let dimensions = plan_dimensions(plan);
        self.rows.push(PhysicalCoverageMatrixRow::generated(
            self.sequence,
            CoverageSurfaceKind::Plan,
            *plan.identity().digest_bytes(),
            dimensions,
        ));
        self.plan = Some(plan.clone());
        Ok(self)
    }
}

fn plan_dimensions(plan: &PhysicalSimulationPlan) -> Vec<CoverageRowDimension> {
    let mut dimensions = vec![CoverageRowDimension::ResourceEnvelopeProfile(
        plan.profile(),
    )];
    append_blob_plan_dimensions(&mut dimensions, plan);
    dimensions.extend(
        plan.fixture_classes()
            .iter()
            .map(CoverageRowDimension::ArtifactClass),
    );
    dimensions.extend(
        plan.observers()
            .iter()
            .map(CoverageRowDimension::OfflineVerifier),
    );
    dimensions
}
