use worth_store_physical_format::{
    CHECKPOINT_BINDING_COMPACTION_HEADER_RECORD_BYTES, CHECKPOINT_BINDING_RECORD_PREFIX_BYTES,
    CHECKPOINT_DIRTY_FRAME_RECORD_BYTES, CHECKPOINT_STREAM_HEADER_RECORD_BYTES,
};
use worth_store_physical_integrity::{
    project_checkpoint_binding_frame_length, validate_checkpoint_binding,
    validate_checkpoint_binding_compaction, validate_checkpoint_dirty_basis,
    validate_checkpoint_footer, CheckpointBindingCompactionIntegrityValidation,
    CheckpointBindingIntegrityValidation, CheckpointDirtyBasisIntegrityValidation,
    CheckpointFooterIntegrityValidation, CheckpointFooterValidationBasis, PhysicalArtifactScope,
};

use crate::integrity_ingress::OwnerCheckpointProjection;
use crate::integrity_ingress::{
    families::checkpoint::{
        IntegrityAdmittedCheckpointBinding, IntegrityAdmittedCheckpointBindingCompaction,
        IntegrityAdmittedCheckpointDirtyBasis, IntegrityAdmittedCheckpointStream,
    },
    RecoveryIntegrityIngressCounters, RecoveryIntegrityIngressRejection,
};

use super::envelope::CheckpointEnvelopeAdmission;
use super::{
    bind_binding, bind_compaction, bind_dirty, bind_footer, bounded, physical_range,
    physical_range_u64, record_integrity_rejection, record_recovery_rejection,
    CheckpointStreamAdmissionFailure,
};

pub(super) struct CheckpointBodyAdmission<'media> {
    envelope: CheckpointEnvelopeAdmission<'media>,
    dirty: Vec<IntegrityAdmittedCheckpointDirtyBasis<'media>>,
    compaction: IntegrityAdmittedCheckpointBindingCompaction<'media>,
    bindings: Vec<IntegrityAdmittedCheckpointBinding<'media>>,
}

impl<'media> CheckpointBodyAdmission<'media> {
    pub(super) fn admit(
        envelope: CheckpointEnvelopeAdmission<'media>,
        counters: &mut RecoveryIntegrityIngressCounters,
    ) -> Result<Self, CheckpointStreamAdmissionFailure> {
        let identity = envelope.header.checkpoint_identity();
        let mut offset = CHECKPOINT_STREAM_HEADER_RECORD_BYTES;
        let mut dirty = Vec::with_capacity(
            usize::try_from(envelope.footer.dirty_record_count())
                .map_err(|_| layout_rejection(&envelope, counters))?,
        );
        for _ in 0..envelope.footer.dirty_record_count() {
            let range = physical_range(offset, CHECKPOINT_DIRTY_FRAME_RECORD_BYTES)?;
            let scope = PhysicalArtifactScope::checkpoint_dirty_basis(identity, range);
            let validation = validate_checkpoint_dirty_basis(
                bounded(envelope.bytes, range, scope, counters)?,
                scope,
            )
            .0;
            let CheckpointDirtyBasisIntegrityValidation::Intact(validated) = validation else {
                let CheckpointDirtyBasisIntegrityValidation::Rejected(rejection) = validation
                else {
                    unreachable!()
                };
                return Err(record_integrity_rejection(scope, rejection, counters));
            };
            dirty.push(bind_dirty(
                envelope.observed,
                scope,
                range,
                validated,
                counters,
            )?);
            offset = range.end_exclusive() as usize;
        }

        let compaction_range =
            physical_range(offset, CHECKPOINT_BINDING_COMPACTION_HEADER_RECORD_BYTES)?;
        let compaction_scope =
            PhysicalArtifactScope::checkpoint_binding_compaction(identity, compaction_range);
        let validation = validate_checkpoint_binding_compaction(
            bounded(envelope.bytes, compaction_range, compaction_scope, counters)?,
            compaction_scope,
        )
        .0;
        let CheckpointBindingCompactionIntegrityValidation::Intact(validated) = validation else {
            let CheckpointBindingCompactionIntegrityValidation::Rejected(rejection) = validation
            else {
                unreachable!()
            };
            return Err(record_integrity_rejection(
                compaction_scope,
                rejection,
                counters,
            ));
        };
        let compaction = bind_compaction(
            envelope.observed,
            compaction_scope,
            compaction_range,
            validated,
            counters,
        )?;
        offset = compaction_range.end_exclusive() as usize;

        let mut bindings = Vec::with_capacity(
            usize::try_from(envelope.footer.binding_record_count())
                .map_err(|_| layout_rejection(&envelope, counters))?,
        );
        for _ in 0..envelope.footer.binding_record_count() {
            let prefix_range = physical_range(offset, CHECKPOINT_BINDING_RECORD_PREFIX_BYTES)?;
            let prefix_scope = PhysicalArtifactScope::checkpoint_binding(identity, prefix_range);
            let frame = project_checkpoint_binding_frame_length(
                bounded(envelope.bytes, prefix_range, prefix_scope, counters)?,
                prefix_scope,
            )
            .map_err(|rejection| record_integrity_rejection(prefix_scope, rejection, counters))?;
            let range = physical_range_u64(offset as u64, frame.encoded_bytes())?;
            if range.end_exclusive() > envelope.footer_range.offset() {
                return Err(layout_rejection(&envelope, counters));
            }
            let scope = PhysicalArtifactScope::checkpoint_binding(identity, range);
            let validation = validate_checkpoint_binding(
                bounded(envelope.bytes, range, scope, counters)?,
                scope,
            )
            .0;
            let CheckpointBindingIntegrityValidation::Intact(validated) = validation else {
                let CheckpointBindingIntegrityValidation::Rejected(rejection) = validation else {
                    unreachable!()
                };
                return Err(record_integrity_rejection(scope, rejection, counters));
            };
            bindings.push(bind_binding(
                envelope.observed,
                scope,
                range,
                validated,
                counters,
            )?);
            offset = range.end_exclusive() as usize;
        }
        if offset as u64 != envelope.footer_range.offset() {
            return Err(layout_rejection(&envelope, counters));
        }
        Ok(Self {
            envelope,
            dirty,
            compaction,
            bindings,
        })
    }

    pub(super) fn finish(
        self,
        counters: &mut RecoveryIntegrityIngressCounters,
    ) -> Result<OwnerCheckpointProjection, CheckpointStreamAdmissionFailure> {
        let maximum_binding_records = self.envelope.maximum_binding_records;
        let dirty = self
            .dirty
            .iter()
            .map(IntegrityAdmittedCheckpointDirtyBasis::validated)
            .collect::<Vec<_>>();
        let bindings = self
            .bindings
            .iter()
            .map(IntegrityAdmittedCheckpointBinding::validated)
            .collect::<Vec<_>>();
        let validation = validate_checkpoint_footer(
            bounded(
                self.envelope.bytes,
                self.envelope.footer_range,
                self.envelope.footer_scope,
                counters,
            )?,
            self.envelope.footer_scope,
            CheckpointFooterValidationBasis::from_record_references(
                self.envelope.header.validated(),
                &dirty,
                self.compaction.validated(),
                &bindings,
            ),
        )
        .0;
        let CheckpointFooterIntegrityValidation::Intact(validated) = validation else {
            let CheckpointFooterIntegrityValidation::Rejected(rejection) = validation else {
                unreachable!()
            };
            return Err(record_integrity_rejection(
                self.envelope.footer_scope,
                rejection,
                counters,
            ));
        };
        let footer = bind_footer(
            self.envelope.observed,
            self.envelope.footer_scope,
            self.envelope.footer_range,
            validated,
            counters,
        )?;
        let admitted = IntegrityAdmittedCheckpointStream::assemble(
            self.envelope.header,
            self.dirty,
            self.compaction,
            self.bindings,
            footer,
        )
        .map_err(|rejection| {
            record_recovery_rejection(self.envelope.footer_scope, rejection, counters)
        })?;
        admitted
            .into_owner_checkpoint(maximum_binding_records, counters)
            .map_err(|rejection| {
                record_recovery_rejection(self.envelope.footer_scope, rejection, counters)
            })
    }
}

fn layout_rejection(
    envelope: &CheckpointEnvelopeAdmission<'_>,
    counters: &mut RecoveryIntegrityIngressCounters,
) -> CheckpointStreamAdmissionFailure {
    record_recovery_rejection(
        envelope.footer_scope,
        RecoveryIntegrityIngressRejection::ScopeMismatch,
        counters,
    )
}
