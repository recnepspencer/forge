use super::PhysicalCoverageRegistry;
use crate::{PhysicalProofOracleVerdict, PhysicalProofOracleVerdictKind};

use super::super::{
    CoverageGapDenial, CoverageRowDimension, CoverageSurfaceKind, PhysicalCoverageMatrixRow,
};

impl PhysicalCoverageRegistry {
    pub fn register_required_oracle_families_from_plan(
        mut self,
    ) -> Result<Self, CoverageGapDenial> {
        self.require_surface_not_registered(CoverageSurfaceKind::Oracle)?;
        let plan =
            self.plan
                .as_ref()
                .ok_or(CoverageGapDenial::MissingPlanBeforeDependentSurface {
                    surface: CoverageSurfaceKind::Oracle,
                })?;
        if plan.oracle_families().iter().next().is_none() {
            return Err(CoverageGapDenial::EmptyOracleVerdictRegistration);
        }
        self.rows.push(PhysicalCoverageMatrixRow::generated(
            self.sequence,
            CoverageSurfaceKind::Oracle,
            *plan.identity().digest_bytes(),
            plan.oracle_families()
                .iter()
                .map(CoverageRowDimension::AuthorityFamily),
        ));
        Ok(self)
    }

    pub fn register_oracle_verdicts(
        mut self,
        verdicts: &[PhysicalProofOracleVerdict],
    ) -> Result<Self, CoverageGapDenial> {
        self.require_surface_not_registered(CoverageSurfaceKind::Oracle)?;
        if verdicts.is_empty() {
            return Err(CoverageGapDenial::EmptyOracleVerdictRegistration);
        }
        if verdicts.iter().any(|verdict| {
            verdict.kind() != PhysicalProofOracleVerdictKind::Satisfied
                || !self
                    .plan
                    .as_ref()
                    .is_some_and(|plan| plan.oracle_families().contains(verdict.family()))
        }) {
            return Err(CoverageGapDenial::UnsatisfiedOracleVerdict);
        }
        let plan =
            self.plan
                .as_ref()
                .ok_or(CoverageGapDenial::MissingPlanBeforeDependentSurface {
                    surface: CoverageSurfaceKind::Oracle,
                })?;
        for required_family in plan.oracle_families().iter() {
            if !verdicts.iter().any(|verdict| {
                verdict.family() == required_family
                    && verdict.kind() == PhysicalProofOracleVerdictKind::Satisfied
            }) {
                return Err(CoverageGapDenial::MissingRequiredOracleVerdict);
            }
        }
        let identity = *plan.identity().digest_bytes();
        self.rows.push(PhysicalCoverageMatrixRow::generated(
            self.sequence,
            CoverageSurfaceKind::Oracle,
            identity,
            verdicts.iter().flat_map(|verdict| {
                [
                    CoverageRowDimension::AuthorityFamily(verdict.family()),
                    CoverageRowDimension::Oracle(verdict.oracle()),
                ]
            }),
        ));
        Ok(self)
    }
}
