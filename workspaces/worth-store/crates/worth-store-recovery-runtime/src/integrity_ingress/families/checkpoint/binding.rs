use sha2::{Digest, Sha256};
use worth_store_physical_format::decode_checkpoint_binding_record;
use worth_store_physical_integrity::IntegrityValidatedCheckpointBinding;

use super::super::super::admission::require_observed_recovery_source;
use super::super::super::{
    ObservedRecoverySource, RecoveryIntegrityIngressCounters, RecoveryIntegrityIngressRejection,
};

pub(crate) struct IntegrityAdmittedCheckpointBinding<'media> {
    source: ObservedRecoverySource<'media>,
    validated: IntegrityValidatedCheckpointBinding<'media>,
}

pub(crate) struct CheckpointBindingProjection<'media> {
    pub payload_bytes: u32,
    pub encoded_bytes: u64,
    pub binding: IntegrityAdmittedCheckpointBindingPayload<'media>,
}

/// Opaque, source-borrowed mutation-binding content. It exposes no raw bytes.
pub(crate) struct IntegrityAdmittedCheckpointBindingPayload<'media> {
    payload: &'media [u8],
    digest: [u8; 32],
}

impl IntegrityAdmittedCheckpointBindingPayload<'_> {
    pub(crate) fn byte_count(&self) -> u64 {
        self.payload.len() as u64
    }

    pub(crate) const fn digest(&self) -> [u8; 32] {
        self.digest
    }
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
    ) -> CheckpointBindingProjection<'media> {
        counters.record_owner_projection();
        counters.record_owner_decoder();
        let input = self
            .source
            .input()
            .expect("an admitted checkpoint binding retains its exact C.4 observation");
        let payload = decode_checkpoint_binding_record(input.bytes())
            .expect("an intact Phase 4 checkpoint binding retains canonical framing");
        CheckpointBindingProjection {
            payload_bytes: self.validated.payload_bytes(),
            encoded_bytes: self.validated.encoded_bytes(),
            binding: IntegrityAdmittedCheckpointBindingPayload {
                payload,
                digest: Sha256::digest(payload).into(),
            },
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
        validated: IntegrityValidatedCheckpointBinding<'media>,
    ) {
        let _ = IntegrityAdmittedCheckpointBinding::bind(source, validated);
    }
    let _ = bind;
}
