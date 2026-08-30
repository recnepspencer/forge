use worth_store::physical_runtime::ObservedRecoveryArtifact;
use worth_store_physical_format::{
    store_namespace::StableStoreIdentity, CheckpointStreamFooter,
    CHECKPOINT_BINDING_COMPACTION_HEADER_RECORD_BYTES, CHECKPOINT_STREAM_FOOTER_RECORD_BYTES,
    CHECKPOINT_STREAM_HEADER_RECORD_BYTES,
};
use worth_store_physical_integrity::{
    validate_checkpoint_footer_envelope, validate_checkpoint_stream_header,
    CheckpointFooterEnvelopeIntegrityValidation, CheckpointStreamHeaderIntegrityValidation,
    CheckpointStreamHeaderScopeIdentity, PhysicalArtifactScope, PhysicalByteRange,
    UntrustedPhysicalArtifact,
};

use crate::integrity_ingress::{
    families::checkpoint::IntegrityAdmittedCheckpointStreamHeader,
    RecoveryIntegrityIngressCounters, RecoveryIntegrityIngressObservation,
    RecoveryIntegrityIngressRejection,
};

use super::{
    bind_header, bounded, physical_range, record_integrity_rejection, record_recovery_rejection,
    CheckpointStreamAdmissionFailure,
};

pub(super) struct CheckpointEnvelopeAdmission<'media> {
    pub(super) observed: &'media ObservedRecoveryArtifact,
    pub(super) bytes: &'media [u8],
    pub(super) header: IntegrityAdmittedCheckpointStreamHeader<'media>,
    pub(super) footer_range: PhysicalByteRange,
    pub(super) footer_scope: PhysicalArtifactScope,
    pub(super) footer: CheckpointStreamFooter,
    pub(super) maximum_binding_records: u64,
}

impl<'media> CheckpointEnvelopeAdmission<'media> {
    pub(super) fn admit(
        observed: &'media ObservedRecoveryArtifact,
        store: StableStoreIdentity,
        maximum_dirty_records: u64,
        maximum_binding_records: u64,
        counters: &mut RecoveryIntegrityIngressCounters,
    ) -> Result<Option<Self>, CheckpointStreamAdmissionFailure> {
        let Some(bytes) = observed.bytes() else {
            return Ok(None);
        };
        let header_range = physical_range(0, CHECKPOINT_STREAM_HEADER_RECORD_BYTES)?;
        let header_scope = PhysicalArtifactScope::checkpoint_stream_header(
            CheckpointStreamHeaderScopeIdentity::staged(store),
            header_range,
        );
        let header_input = if bytes.len() < CHECKPOINT_STREAM_HEADER_RECORD_BYTES {
            UntrustedPhysicalArtifact::from_bounded_bytes(bytes)
        } else {
            bounded(bytes, header_range, header_scope, counters)?
        };
        let header_validation = validate_checkpoint_stream_header(header_input, header_scope).0;
        let CheckpointStreamHeaderIntegrityValidation::Intact(validated) = header_validation else {
            let CheckpointStreamHeaderIntegrityValidation::Rejected(rejection) = header_validation
            else {
                unreachable!()
            };
            return Err(record_integrity_rejection(
                header_scope,
                rejection,
                counters,
            ));
        };
        let header = bind_header(observed, header_scope, header_range, validated, counters)?;
        let identity = header.checkpoint_identity();

        let minimum = CHECKPOINT_STREAM_HEADER_RECORD_BYTES
            + CHECKPOINT_BINDING_COMPACTION_HEADER_RECORD_BYTES
            + CHECKPOINT_STREAM_FOOTER_RECORD_BYTES;
        if bytes.len() < minimum {
            let footer_range = physical_range(
                CHECKPOINT_STREAM_HEADER_RECORD_BYTES
                    + CHECKPOINT_BINDING_COMPACTION_HEADER_RECORD_BYTES,
                CHECKPOINT_STREAM_FOOTER_RECORD_BYTES,
            )?;
            let scope = PhysicalArtifactScope::checkpoint_footer(identity, footer_range);
            return Err(record_recovery_rejection(
                scope,
                RecoveryIntegrityIngressRejection::SourceRangeOutsideObservation,
                counters,
            ));
        }
        let footer_offset = bytes.len() - CHECKPOINT_STREAM_FOOTER_RECORD_BYTES;
        let footer_range = physical_range(footer_offset, CHECKPOINT_STREAM_FOOTER_RECORD_BYTES)?;
        let footer_scope = PhysicalArtifactScope::checkpoint_footer(identity, footer_range);
        let validation = validate_checkpoint_footer_envelope(
            bounded(bytes, footer_range, footer_scope, counters)?,
            footer_scope,
        )
        .0;
        let CheckpointFooterEnvelopeIntegrityValidation::Intact(envelope) = validation else {
            let CheckpointFooterEnvelopeIntegrityValidation::Rejected(rejection) = validation
            else {
                unreachable!()
            };
            return Err(record_integrity_rejection(
                footer_scope,
                rejection,
                counters,
            ));
        };
        counters.record(RecoveryIntegrityIngressObservation::admitted(footer_scope));
        let footer = envelope.routing_projection().footer();
        enforce_record_limits(
            footer.dirty_record_count(),
            footer.binding_record_count(),
            maximum_dirty_records,
            maximum_binding_records,
        )?;
        Ok(Some(Self {
            observed,
            bytes,
            header,
            footer_range,
            footer_scope,
            footer,
            maximum_binding_records,
        }))
    }
}

fn enforce_record_limits(
    dirty: u64,
    bindings: u64,
    maximum_dirty: u64,
    maximum_bindings: u64,
) -> Result<(), CheckpointStreamAdmissionFailure> {
    if dirty > maximum_dirty {
        return Err(CheckpointStreamAdmissionFailure::DirtyRecordLimit {
            observed: dirty,
            admitted: maximum_dirty,
        });
    }
    if bindings > maximum_bindings {
        return Err(CheckpointStreamAdmissionFailure::BindingRecordLimit {
            observed: bindings,
            admitted: maximum_bindings,
        });
    }
    Ok(())
}
