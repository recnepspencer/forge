use super::Roadmap2CoverageRegistry;

use super::super::{
    CoverageGapDenial, CoverageRowDimension, CoverageSurfaceKind, MutationValidationPosture,
    PhysicalCoverageMatrixRow, PhysicalMutationCoverageEvidence,
};

impl Roadmap2CoverageRegistry {
    pub fn register_mutation_result(
        mut self,
        mutation: &PhysicalMutationCoverageEvidence,
    ) -> Result<Self, CoverageGapDenial> {
        self.require_surface_not_registered(CoverageSurfaceKind::MutationResult)?;
        if mutation.sequence() != self.sequence {
            return Err(CoverageGapDenial::MissingMutationResult);
        }
        let plan =
            self.plan
                .as_ref()
                .ok_or(CoverageGapDenial::MissingPlanBeforeDependentSurface {
                    surface: CoverageSurfaceKind::MutationResult,
                })?;
        if mutation.plan_identity() != plan.identity().digest_bytes() {
            return Err(CoverageGapDenial::MutationPlanIdentityMismatch);
        }
        let identity = mutation_identity(mutation.posture());
        let mut dimensions = vec![CoverageRowDimension::MutationValidationPosture(
            mutation.posture(),
        )];
        dimensions.extend(
            mutation
                .compaction_mutations()
                .iter()
                .map(|row| CoverageRowDimension::CompactionMutation(row.kind())),
        );
        dimensions.extend(
            mutation
                .s5_physical_isolation_mutations()
                .iter()
                .copied()
                .map(CoverageRowDimension::S5PhysicalIsolationMutation),
        );
        self.rows.push(PhysicalCoverageMatrixRow::generated(
            self.sequence,
            CoverageSurfaceKind::MutationResult,
            identity,
            dimensions,
        ));
        Ok(self)
    }
}

fn mutation_identity(posture: MutationValidationPosture) -> [u8; 32] {
    let mut identity = [0_u8; 32];
    let token = match posture {
        MutationValidationPosture::ExpectedFailureObserved => b"expected-failure-observed",
    };
    for (slot, byte) in identity.iter_mut().zip(token.iter().copied()) {
        *slot = byte;
    }
    identity
}
