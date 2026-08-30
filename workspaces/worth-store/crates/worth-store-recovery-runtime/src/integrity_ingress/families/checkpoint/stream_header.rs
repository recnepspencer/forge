use worth_store_physical_format::{PhysicalCheckpointIdentity, PhysicalCheckpointSource};
use worth_store_physical_integrity::IntegrityValidatedCheckpointStreamHeader;

use super::super::super::admission::require_observed_recovery_source;
use super::super::super::{
    ObservedRecoverySource, RecoveryIntegrityIngressCounters, RecoveryIntegrityIngressRejection,
};

pub(crate) struct IntegrityAdmittedCheckpointStreamHeader<'media> {
    source: ObservedRecoverySource<'media>,
    validated: IntegrityValidatedCheckpointStreamHeader<'media>,
}

pub(crate) struct CheckpointStreamHeaderProjection {
    pub checkpoint_identity: PhysicalCheckpointIdentity,
    pub source: PhysicalCheckpointSource,
}

impl<'media> IntegrityAdmittedCheckpointStreamHeader<'media> {
    pub(in crate::integrity_ingress) fn bind(
        source: ObservedRecoverySource<'media>,
        validated: IntegrityValidatedCheckpointStreamHeader<'media>,
    ) -> Result<Self, RecoveryIntegrityIngressRejection> {
        require_observed_recovery_source(&source, validated.scope(), |input| {
            validated.matches_input(input)
        })?;
        Ok(Self { source, validated })
    }

    pub(crate) fn project(
        &self,
        counters: &mut RecoveryIntegrityIngressCounters,
    ) -> CheckpointStreamHeaderProjection {
        counters.record_owner_projection();
        CheckpointStreamHeaderProjection {
            checkpoint_identity: self.validated.checkpoint_identity(),
            source: self.validated.source(),
        }
    }

    pub(crate) fn scope(&self) -> worth_store_physical_integrity::PhysicalArtifactScope {
        self.source.scope()
    }
}

#[cfg(test)]
pub(super) fn owner_valid_compile_contract() {
    fn bind<'media>(
        source: ObservedRecoverySource<'media>,
        validated: IntegrityValidatedCheckpointStreamHeader<'media>,
    ) {
        let _ = IntegrityAdmittedCheckpointStreamHeader::bind(source, validated);
    }
    let _ = bind;
}
