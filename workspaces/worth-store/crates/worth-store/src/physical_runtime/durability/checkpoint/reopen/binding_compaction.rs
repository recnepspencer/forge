use sha2::{Digest, Sha256};
use worth_store_physical_backend::{
    ArtifactTreeDirectory, ArtifactTreeFile, ArtifactTreeMedia, QualifiedFilesystemMedia,
};
use worth_store_physical_format::{
    decode_checkpoint_binding_record, CheckpointBindingCompactionHeader,
    CheckpointBindingRecordFrameLength, CheckpointStreamDecoder, CheckpointStreamFooter,
    PhysicalCheckpointIdentity, CHECKPOINT_BINDING_COMPACTION_HEADER_RECORD_BYTES,
    CHECKPOINT_BINDING_RECORD_PREFIX_BYTES, CHECKPOINT_DIRTY_FRAME_RECORD_BYTES,
    CHECKPOINT_STREAM_FOOTER_RECORD_BYTES, CHECKPOINT_STREAM_HEADER_RECORD_BYTES,
};

use super::{
    PhysicalBindingCompactionReopenCounters, PhysicalBindingCompactionReopenFailure,
    ReopenedPhysicalBindingCompaction,
};

pub(in crate::physical_runtime) struct NamespaceDurablePhysicalBindingCompactionReopen {
    artifact: ArtifactTreeFile,
    checkpoint: PhysicalCheckpointIdentity,
    header: CheckpointBindingCompactionHeader,
    footer: CheckpointStreamFooter,
    artifact_bytes: u64,
    compaction_offset: u64,
    records_offset: u64,
    footer_offset: u64,
}

pub(in crate::physical_runtime) enum PhysicalBindingCompactionRecordStreamFailure<E> {
    Reopen(PhysicalBindingCompactionReopenFailure),
    Consumer(E),
}

pub(in crate::physical_runtime) fn reopen_binding_compaction(
    media: &QualifiedFilesystemMedia,
) -> Result<ReopenedPhysicalBindingCompaction, PhysicalBindingCompactionReopenFailure> {
    let tree = media.artifact_tree();
    let artifact = checkpoint_artifact();
    if !tree
        .file_exists(&artifact)
        .map_err(PhysicalBindingCompactionReopenFailure::Media)?
    {
        return Ok(ReopenedPhysicalBindingCompaction::GenerationZero);
    }
    let artifact_bytes = tree
        .file_length(&artifact)
        .map_err(PhysicalBindingCompactionReopenFailure::Media)?;
    let reopened =
        NamespaceDurablePhysicalBindingCompactionReopen::read(&tree, artifact, artifact_bytes)?;
    if reopened.checkpoint.store_identity() != media.store_identity() {
        return Err(PhysicalBindingCompactionReopenFailure::ForeignStore);
    }
    Ok(ReopenedPhysicalBindingCompaction::NamespaceDurable(
        reopened,
    ))
}

impl NamespaceDurablePhysicalBindingCompactionReopen {
    pub(in crate::physical_runtime) const fn generation(&self) -> u64 {
        self.header.generation()
    }

    pub(in crate::physical_runtime) const fn wal_cutoff_lsn_exclusive(&self) -> u64 {
        self.header.wal_cutoff_lsn_exclusive()
    }

    pub(in crate::physical_runtime) fn stream_records<E>(
        &self,
        media: &QualifiedFilesystemMedia,
        mut consume: impl FnMut(&[u8]) -> Result<(), E>,
    ) -> Result<
        PhysicalBindingCompactionReopenCounters,
        PhysicalBindingCompactionRecordStreamFailure<E>,
    > {
        if media.store_identity() != self.checkpoint.store_identity() {
            return Err(PhysicalBindingCompactionRecordStreamFailure::Reopen(
                PhysicalBindingCompactionReopenFailure::ForeignStore,
            ));
        }
        let tree = media.artifact_tree();
        let mut digest = Sha256::new();
        let mut offset = self.records_offset;
        let mut records_read = 0_u64;
        while offset < self.footer_offset {
            let prefix = read_fixed::<{ CHECKPOINT_BINDING_RECORD_PREFIX_BYTES }>(
                &tree,
                &self.artifact,
                offset,
            )
            .map_err(PhysicalBindingCompactionRecordStreamFailure::Reopen)?;
            let frame_bytes = CheckpointBindingRecordFrameLength::decode_prefix(&prefix)
                .map_err(|denial| {
                    PhysicalBindingCompactionRecordStreamFailure::Reopen(
                        PhysicalBindingCompactionReopenFailure::Format(denial),
                    )
                })?
                .encoded_bytes();
            let end = offset.checked_add(frame_bytes as u64).ok_or_else(|| {
                PhysicalBindingCompactionRecordStreamFailure::Reopen(
                    PhysicalBindingCompactionReopenFailure::CounterOverflow,
                )
            })?;
            if end > self.footer_offset {
                return Err(PhysicalBindingCompactionRecordStreamFailure::Reopen(
                    PhysicalBindingCompactionReopenFailure::ArtifactLayoutMismatch,
                ));
            }
            let mut record = Vec::new();
            record.try_reserve_exact(frame_bytes).map_err(|_| {
                PhysicalBindingCompactionRecordStreamFailure::Reopen(
                    PhysicalBindingCompactionReopenFailure::AllocationRejected,
                )
            })?;
            record.resize(frame_bytes, 0);
            record[..CHECKPOINT_BINDING_RECORD_PREFIX_BYTES].copy_from_slice(&prefix);
            tree.read_exact_at(
                &self.artifact,
                offset + CHECKPOINT_BINDING_RECORD_PREFIX_BYTES as u64,
                &mut record[CHECKPOINT_BINDING_RECORD_PREFIX_BYTES..],
            )
            .map_err(|failure| {
                PhysicalBindingCompactionRecordStreamFailure::Reopen(
                    PhysicalBindingCompactionReopenFailure::Media(failure),
                )
            })?;
            let payload = decode_checkpoint_binding_record(&record).map_err(|denial| {
                PhysicalBindingCompactionRecordStreamFailure::Reopen(
                    PhysicalBindingCompactionReopenFailure::Format(denial),
                )
            })?;
            consume(payload).map_err(PhysicalBindingCompactionRecordStreamFailure::Consumer)?;
            digest.update(&record);
            records_read = records_read.checked_add(1).ok_or_else(|| {
                PhysicalBindingCompactionRecordStreamFailure::Reopen(
                    PhysicalBindingCompactionReopenFailure::CounterOverflow,
                )
            })?;
            offset = end;
        }
        self.verify_stream(offset, records_read, digest)
            .map_err(PhysicalBindingCompactionRecordStreamFailure::Reopen)?;
        let checkpoint_bytes_read = fixed_read_bytes()
            .and_then(|bytes| {
                bytes
                    .checked_add(self.footer.binding_record_bytes())
                    .ok_or(PhysicalBindingCompactionReopenFailure::CounterOverflow)
            })
            .map_err(PhysicalBindingCompactionRecordStreamFailure::Reopen)?;
        Ok(PhysicalBindingCompactionReopenCounters {
            checkpoint_artifact_bytes: self.artifact_bytes,
            checkpoint_bytes_read,
            dirty_body_bytes_skipped: self.compaction_offset
                - CHECKPOINT_STREAM_HEADER_RECORD_BYTES as u64,
            binding_records_read: records_read,
        })
    }

    fn read(
        tree: &ArtifactTreeMedia<'_>,
        artifact: ArtifactTreeFile,
        artifact_bytes: u64,
    ) -> Result<Self, PhysicalBindingCompactionReopenFailure> {
        if artifact_bytes < fixed_read_bytes()? {
            return Err(PhysicalBindingCompactionReopenFailure::ArtifactTooShort);
        }
        let header_record =
            read_fixed::<CHECKPOINT_STREAM_HEADER_RECORD_BYTES>(tree, &artifact, 0)?;
        let source = CheckpointStreamDecoder::begin(&header_record)
            .map_err(PhysicalBindingCompactionReopenFailure::Format)?
            .source();
        let footer_offset = artifact_bytes
            .checked_sub(CHECKPOINT_STREAM_FOOTER_RECORD_BYTES as u64)
            .ok_or(PhysicalBindingCompactionReopenFailure::ArtifactTooShort)?;
        let footer_record =
            read_fixed::<CHECKPOINT_STREAM_FOOTER_RECORD_BYTES>(tree, &artifact, footer_offset)?;
        let footer = CheckpointStreamFooter::decode_record(&footer_record)
            .map_err(PhysicalBindingCompactionReopenFailure::Format)?;
        if source.identity() != footer.identity() {
            return Err(PhysicalBindingCompactionReopenFailure::ArtifactLayoutMismatch);
        }
        let compaction_offset = footer.binding_compaction_header_offset();
        let records_offset = compaction_offset
            .checked_add(CHECKPOINT_BINDING_COMPACTION_HEADER_RECORD_BYTES as u64)
            .ok_or(PhysicalBindingCompactionReopenFailure::CounterOverflow)?;
        if compaction_offset < CHECKPOINT_STREAM_HEADER_RECORD_BYTES as u64
            || records_offset > footer_offset
            || footer_offset - records_offset != footer.binding_record_bytes()
        {
            return Err(PhysicalBindingCompactionReopenFailure::ArtifactLayoutMismatch);
        }
        let expected_compaction_offset = (CHECKPOINT_STREAM_HEADER_RECORD_BYTES as u64)
            .checked_add(
                footer
                    .dirty_record_count()
                    .checked_mul(CHECKPOINT_DIRTY_FRAME_RECORD_BYTES as u64)
                    .ok_or(PhysicalBindingCompactionReopenFailure::CounterOverflow)?,
            )
            .ok_or(PhysicalBindingCompactionReopenFailure::CounterOverflow)?;
        if compaction_offset != expected_compaction_offset {
            return Err(PhysicalBindingCompactionReopenFailure::ArtifactLayoutMismatch);
        }
        let compaction_record = read_fixed::<CHECKPOINT_BINDING_COMPACTION_HEADER_RECORD_BYTES>(
            tree,
            &artifact,
            compaction_offset,
        )?;
        let header = CheckpointBindingCompactionHeader::decode_record(&compaction_record)
            .map_err(PhysicalBindingCompactionReopenFailure::Format)?;
        if footer.binding_compaction_generation() != header.generation()
            || footer.binding_wal_cutoff_lsn_exclusive() != header.wal_cutoff_lsn_exclusive()
        {
            return Err(PhysicalBindingCompactionReopenFailure::ArtifactLayoutMismatch);
        }
        let minimum_record_bytes = (CHECKPOINT_BINDING_RECORD_PREFIX_BYTES + 1 + 4) as u64;
        if footer.binding_record_count() > footer.binding_record_bytes() / minimum_record_bytes {
            return Err(PhysicalBindingCompactionReopenFailure::ArtifactLayoutMismatch);
        }
        Ok(Self {
            artifact,
            checkpoint: source.identity(),
            header,
            footer,
            artifact_bytes,
            compaction_offset,
            records_offset,
            footer_offset,
        })
    }

    fn verify_stream(
        &self,
        offset: u64,
        records_read: u64,
        digest: Sha256,
    ) -> Result<(), PhysicalBindingCompactionReopenFailure> {
        if offset != self.footer_offset
            || records_read != self.footer.binding_record_count()
            || <[u8; 32]>::from(digest.finalize()) != self.footer.binding_records_digest()
        {
            return Err(PhysicalBindingCompactionReopenFailure::ArtifactLayoutMismatch);
        }
        Ok(())
    }
}

fn read_fixed<const BYTES: usize>(
    tree: &ArtifactTreeMedia<'_>,
    artifact: &ArtifactTreeFile,
    offset: u64,
) -> Result<[u8; BYTES], PhysicalBindingCompactionReopenFailure> {
    let mut bytes = [0; BYTES];
    tree.read_exact_at(artifact, offset, &mut bytes)
        .map_err(PhysicalBindingCompactionReopenFailure::Media)?;
    Ok(bytes)
}

fn fixed_read_bytes() -> Result<u64, PhysicalBindingCompactionReopenFailure> {
    (CHECKPOINT_STREAM_HEADER_RECORD_BYTES as u64)
        .checked_add(CHECKPOINT_BINDING_COMPACTION_HEADER_RECORD_BYTES as u64)
        .and_then(|bytes| bytes.checked_add(CHECKPOINT_STREAM_FOOTER_RECORD_BYTES as u64))
        .ok_or(PhysicalBindingCompactionReopenFailure::CounterOverflow)
}

fn checkpoint_artifact() -> ArtifactTreeFile {
    ArtifactTreeDirectory::families()
        .file("checkpoint.current")
        .expect("the canonical checkpoint publication name is portable")
}
