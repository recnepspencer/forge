use worth_store::physical_runtime::ObservedRecoveryArtifact;
use worth_store_physical_format::{
    inspect_checkpoint_stream, CheckpointStreamDecodeDenial, PhysicalCheckpointIdentity,
    VerifiedCheckpointStream,
};

use super::{
    IntegrityAdmittedCheckpointBinding, IntegrityAdmittedCheckpointBindingCompaction,
    IntegrityAdmittedCheckpointDirtyBasis, IntegrityAdmittedCheckpointFooter,
    IntegrityAdmittedCheckpointStreamHeader,
};
use crate::integrity_ingress::{
    ObservedRecoverySource, RecoveryIntegrityIngressCounters, RecoveryIntegrityIngressRejection,
};

pub(crate) struct IntegrityAdmittedCheckpointStream<'media> {
    observed: &'media ObservedRecoveryArtifact,
    _header: IntegrityAdmittedCheckpointStreamHeader<'media>,
    _dirty: Vec<IntegrityAdmittedCheckpointDirtyBasis<'media>>,
    _compaction: IntegrityAdmittedCheckpointBindingCompaction<'media>,
    _bindings: Vec<IntegrityAdmittedCheckpointBinding<'media>>,
    _footer: IntegrityAdmittedCheckpointFooter<'media>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CheckpointStreamProjectionDenial {
    Decode(CheckpointStreamDecodeDenial),
}

impl<'media> IntegrityAdmittedCheckpointStream<'media> {
    pub(crate) fn assemble(
        header: IntegrityAdmittedCheckpointStreamHeader<'media>,
        dirty: Vec<IntegrityAdmittedCheckpointDirtyBasis<'media>>,
        compaction: IntegrityAdmittedCheckpointBindingCompaction<'media>,
        bindings: Vec<IntegrityAdmittedCheckpointBinding<'media>>,
        footer: IntegrityAdmittedCheckpointFooter<'media>,
    ) -> Result<Self, RecoveryIntegrityIngressRejection> {
        let observed = header.source().observed();
        let identity = header.checkpoint_identity();
        let mut next_offset = 0_u64;
        require_record(header.source(), observed, None, &mut next_offset)?;
        for record in &dirty {
            require_record(record.source(), observed, Some(identity), &mut next_offset)?;
        }
        require_record(
            compaction.source(),
            observed,
            Some(identity),
            &mut next_offset,
        )?;
        for record in &bindings {
            require_record(record.source(), observed, Some(identity), &mut next_offset)?;
        }
        require_record(footer.source(), observed, Some(identity), &mut next_offset)?;
        let observed_bytes = observed
            .bytes()
            .ok_or(RecoveryIntegrityIngressRejection::MissingBoundedArtifact)?;
        if next_offset != observed_bytes.len() as u64 {
            return Err(RecoveryIntegrityIngressRejection::ScopeMismatch);
        }
        Ok(Self {
            observed,
            _header: header,
            _dirty: dirty,
            _compaction: compaction,
            _bindings: bindings,
            _footer: footer,
        })
    }

    pub(crate) fn project(
        self,
        maximum_dirty_records: u64,
        maximum_binding_records: u64,
        counters: &mut RecoveryIntegrityIngressCounters,
    ) -> Result<VerifiedCheckpointStream, CheckpointStreamProjectionDenial> {
        counters.record_owner_projection();
        counters.record_owner_decoder();
        inspect_checkpoint_stream(
            self.observed
                .bytes()
                .expect("an admitted checkpoint stream retains its C.4 observation"),
            maximum_dirty_records,
            maximum_binding_records,
        )
        .map_err(CheckpointStreamProjectionDenial::Decode)
    }
}

fn require_record(
    source: &ObservedRecoverySource<'_>,
    observed: &ObservedRecoveryArtifact,
    expected_identity: Option<PhysicalCheckpointIdentity>,
    next_offset: &mut u64,
) -> Result<(), RecoveryIntegrityIngressRejection> {
    if !core::ptr::eq(source.observed(), observed)
        || expected_identity.is_some() && source.scope().checkpoint_identity() != expected_identity
    {
        return Err(RecoveryIntegrityIngressRejection::SourceIncarnationMismatch);
    }
    let range = source.selected_range();
    if range.offset() != *next_offset || range != source.scope().byte_range() {
        return Err(RecoveryIntegrityIngressRejection::ScopeMismatch);
    }
    *next_offset = range.end_exclusive();
    Ok(())
}
