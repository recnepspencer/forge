use worth_store_physical_format::CheckpointDirtyFrameBasis;
use worth_store_physical_integrity::IntegrityValidatedCheckpointDirtyBasis;

use super::super::super::admission::require_observed_recovery_source;
use super::super::super::{
    ObservedRecoverySource, RecoveryIntegrityIngressCounters, RecoveryIntegrityIngressRejection,
};

pub(crate) struct IntegrityAdmittedCheckpointDirtyBasis<'media> {
    source: ObservedRecoverySource<'media>,
    validated: IntegrityValidatedCheckpointDirtyBasis<'media>,
}

pub(crate) struct CheckpointDirtyBasisProjection {
    pub basis: CheckpointDirtyFrameBasis,
}

impl<'media> IntegrityAdmittedCheckpointDirtyBasis<'media> {
    pub(in crate::integrity_ingress) fn bind(
        source: ObservedRecoverySource<'media>,
        validated: IntegrityValidatedCheckpointDirtyBasis<'media>,
    ) -> Result<Self, RecoveryIntegrityIngressRejection> {
        require_observed_recovery_source(&source, validated.scope(), |input| {
            validated.matches_input(input)
        })?;
        Ok(Self { source, validated })
    }

    pub(crate) fn project(
        &self,
        counters: &mut RecoveryIntegrityIngressCounters,
    ) -> CheckpointDirtyBasisProjection {
        counters.record_owner_projection();
        CheckpointDirtyBasisProjection {
            basis: self.validated.basis(),
        }
    }

    pub(crate) fn scope(&self) -> worth_store_physical_integrity::PhysicalArtifactScope {
        self.source.scope()
    }

    pub(in crate::integrity_ingress) const fn source(&self) -> &ObservedRecoverySource<'media> {
        &self.source
    }
}

#[cfg(test)]
pub(super) fn owner_valid_compile_contract() {
    fn bind<'media>(
        source: ObservedRecoverySource<'media>,
        validated: IntegrityValidatedCheckpointDirtyBasis<'media>,
    ) {
        let _ = IntegrityAdmittedCheckpointDirtyBasis::bind(source, validated);
    }
    let _ = bind;
}
