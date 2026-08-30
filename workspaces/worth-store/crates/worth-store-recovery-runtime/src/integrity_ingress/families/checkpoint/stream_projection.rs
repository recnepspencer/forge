use worth_store::physical_runtime::ObservedRecoveryArtifact;
use worth_store_physical_format::{
    CheckpointDirtyFrameBasis, CheckpointStreamFooter, PhysicalCheckpointIdentity,
    PhysicalCheckpointSource,
};

use super::binding::CheckpointBindingProjection;
use super::{
    IntegrityAdmittedCheckpointBinding, IntegrityAdmittedCheckpointBindingCompaction,
    IntegrityAdmittedCheckpointDirtyBasis, IntegrityAdmittedCheckpointFooter,
    IntegrityAdmittedCheckpointStreamHeader,
};
use crate::integrity_ingress::{
    ObservedRecoverySource, RecoveryIntegrityIngressCounters, RecoveryIntegrityIngressRejection,
};

pub(crate) struct IntegrityAdmittedCheckpointStream<'media> {
    encoded_bytes: u64,
    header: IntegrityAdmittedCheckpointStreamHeader<'media>,
    dirty: Vec<IntegrityAdmittedCheckpointDirtyBasis<'media>>,
    compaction: IntegrityAdmittedCheckpointBindingCompaction<'media>,
    bindings: Vec<IntegrityAdmittedCheckpointBinding<'media>>,
    footer: IntegrityAdmittedCheckpointFooter<'media>,
}

/// Recovery-owner facts projected only from the complete admitted record set.
pub(crate) struct IntegrityAdmittedCheckpointProjection<'media> {
    source: PhysicalCheckpointSource,
    checkpoint_identity: PhysicalCheckpointIdentity,
    dirty_bases: Box<[CheckpointDirtyFrameBasis]>,
    compaction_generation: u64,
    wal_cutoff_lsn_exclusive: u64,
    bindings: Box<[CheckpointBindingProjection<'media>]>,
    footer: CheckpointStreamFooter,
    encoded_bytes: u64,
}

impl<'media> IntegrityAdmittedCheckpointProjection<'media> {
    pub(crate) const fn source(&self) -> PhysicalCheckpointSource {
        self.source
    }

    pub(crate) const fn checkpoint_identity(&self) -> PhysicalCheckpointIdentity {
        self.checkpoint_identity
    }

    pub(crate) fn dirty_bases(&self) -> &[CheckpointDirtyFrameBasis] {
        &self.dirty_bases
    }

    pub(crate) const fn compaction_generation(&self) -> u64 {
        self.compaction_generation
    }

    pub(crate) const fn wal_cutoff_lsn_exclusive(&self) -> u64 {
        self.wal_cutoff_lsn_exclusive
    }

    pub(crate) fn bindings(&self) -> &[CheckpointBindingProjection<'media>] {
        &self.bindings
    }

    pub(crate) const fn footer(&self) -> CheckpointStreamFooter {
        self.footer
    }

    pub(crate) const fn encoded_bytes(&self) -> u64 {
        self.encoded_bytes
    }
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
            encoded_bytes: next_offset,
            header,
            dirty,
            compaction,
            bindings,
            footer,
        })
    }

    pub(crate) fn project(
        self,
        counters: &mut RecoveryIntegrityIngressCounters,
    ) -> IntegrityAdmittedCheckpointProjection<'media> {
        let header = self.header.project(counters);
        let dirty_bases = self
            .dirty
            .iter()
            .map(|record| record.project(counters).basis)
            .collect();
        let compaction = self.compaction.project(counters);
        let bindings = self
            .bindings
            .iter()
            .map(|record| record.project(counters))
            .collect();
        let footer = self.footer.project(counters).footer;
        IntegrityAdmittedCheckpointProjection {
            source: header.source,
            checkpoint_identity: header.checkpoint_identity,
            dirty_bases,
            compaction_generation: compaction.generation,
            wal_cutoff_lsn_exclusive: compaction.wal_cutoff_lsn_exclusive,
            bindings,
            footer,
            encoded_bytes: self.encoded_bytes,
        }
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
