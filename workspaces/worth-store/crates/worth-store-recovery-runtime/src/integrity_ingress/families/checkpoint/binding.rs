use worth_store_physical_integrity::IntegrityValidatedCheckpointBinding;

use super::super::super::admission::require_observed_recovery_source;
use super::super::super::{
    ObservedRecoverySource, RecoveryIntegrityIngressCounters, RecoveryIntegrityIngressRejection,
};

pub(crate) struct IntegrityAdmittedCheckpointBinding<'media> {
    source: ObservedRecoverySource<'media>,
    validated: IntegrityValidatedCheckpointBinding<'media>,
}

pub(crate) struct CheckpointBindingProjection {
    pub payload_bytes: u32,
    pub encoded_bytes: u64,
}

impl<'media> IntegrityAdmittedCheckpointBinding<'media> {
    pub(in crate::integrity_ingress) fn bind(
        source: ObservedRecoverySource<'media>,
        validated: IntegrityValidatedCheckpointBinding<'media>,
    ) -> Result<Self, RecoveryIntegrityIngressRejection> {
        require_observed_recovery_source(&source, validated.scope(), |input| {
            validated.matches_input(input)
        })?;
        Ok(Self { source, validated })
    }

    pub(crate) fn project(
        &self,
        counters: &mut RecoveryIntegrityIngressCounters,
    ) -> CheckpointBindingProjection {
        counters.record_owner_projection();
        CheckpointBindingProjection {
            payload_bytes: self.validated.payload_bytes(),
            encoded_bytes: self.validated.encoded_bytes(),
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
        validated: IntegrityValidatedCheckpointBinding<'media>,
    ) {
        let _ = IntegrityAdmittedCheckpointBinding::bind(source, validated);
    }
    let _ = bind;
}
