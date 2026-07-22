use std::io::{Read, Seek, SeekFrom};

use cap_fs_ext::{FollowSymlinks, OpenOptionsFollowExt};
use cap_std::fs::OpenOptions;

use super::super::artifact_tree_effects::{artifact_file_length, begin, write_all_interposed};
use super::{
    ArtifactRangeWriteOutcome, ArtifactTreeFailure, ArtifactTreeFailureKind, ArtifactTreeFile,
    ArtifactTreeMedia, CompletedArtifactRangeWrite, IndeterminateArtifactRangeWrite,
};
use crate::filesystem_media::{FilesystemMediaOwner, MediaOperationRole};

/// A newly-created artifact whose bytes may be supplied in bounded chunks.
pub struct ArtifactTreeNewFile<'media> {
    owner: &'media FilesystemMediaOwner,
    store: worth_store_physical_format::store_namespace::StableStoreIdentity,
    artifact: ArtifactTreeFile,
    file: cap_std::fs::File,
    mutation_sequence: crate::filesystem_media::file_mutation_sequence::FileMutationSequence,
    _coordination:
        crate::filesystem_media::artifact_mutation_coordinator::CoordinatedArtifactMutation<'media>,
    completed_bytes: u64,
}

impl ArtifactTreeNewFile<'_> {
    pub fn write_chunk(&mut self, bytes: &[u8]) -> Result<(), ArtifactTreeFailure> {
        let _sequence = self.mutation_sequence.lock();
        write_all_interposed(self.owner, &mut self.file, bytes)?;
        self.completed_bytes = self
            .completed_bytes
            .checked_add(bytes.len() as u64)
            .ok_or_else(|| ArtifactTreeFailure::structural(ArtifactTreeFailureKind::Damaged))?;
        Ok(())
    }

    pub const fn completed_bytes(&self) -> u64 {
        self.completed_bytes
    }

    pub fn write_exact_chunk(
        &mut self,
        coordinate: worth_store_physical_format::RecordFrameCoordinate,
        bytes: &[u8],
    ) -> ArtifactRangeWriteOutcome {
        if coordinate.offset() != self.completed_bytes
            || coordinate.length() as usize != bytes.len()
            || coordinate.artifact().file_name() != self.artifact.file_name
        {
            return ArtifactRangeWriteOutcome::DeniedBeforeEffect(ArtifactTreeFailure::structural(
                ArtifactTreeFailureKind::AccessLimitExceeded,
            ));
        }
        let _sequence = self.mutation_sequence.lock();
        match super::exact_write_effect::execute(self.owner, &mut self.file, bytes) {
            super::exact_write_effect::ExactWriteEffect::DeniedBeforeEffect(failure) => {
                ArtifactRangeWriteOutcome::DeniedBeforeEffect(failure)
            }
            super::exact_write_effect::ExactWriteEffect::Indeterminate {
                failure,
                completed_bytes,
                operation,
            } => {
                self.completed_bytes = self.completed_bytes.saturating_add(completed_bytes);
                ArtifactRangeWriteOutcome::Indeterminate(IndeterminateArtifactRangeWrite::new(
                    failure,
                    coordinate,
                    completed_bytes,
                    operation,
                ))
            }
            super::exact_write_effect::ExactWriteEffect::Completed(operation) => {
                self.completed_bytes += bytes.len() as u64;
                ArtifactRangeWriteOutcome::Completed(CompletedArtifactRangeWrite::buffered(
                    self.owner.identity(),
                    self.store,
                    coordinate,
                    bytes,
                    operation,
                ))
            }
        }
    }
}

impl ArtifactTreeMedia<'_> {
    pub fn create_new_file(
        &self,
        artifact: &ArtifactTreeFile,
    ) -> Result<ArtifactTreeNewFile<'_>, ArtifactTreeFailure> {
        let coordination = self
            .owner
            .begin_artifact_namespace_mutation(vec![artifact.coordination_key()])
            .map_err(|_| {
                ArtifactTreeFailure::structural(ArtifactTreeFailureKind::DeniedBeforeEffect)
            })?;
        let directory = self.open_directory(&artifact.directory)?;
        let mut options = OpenOptions::new();
        options
            .read(true)
            .write(true)
            .create_new(true)
            .follow(FollowSymlinks::No);
        let create = begin(self.owner, MediaOperationRole::CreateNew, 0);
        if let Some(error) = create.fail_before_error() {
            create.denied();
            return Err(ArtifactTreeFailure::io(
                ArtifactTreeFailureKind::DeniedBeforeEffect,
                &error,
            ));
        }
        match directory.open_with(&artifact.file_name, &options) {
            Ok(file) => {
                let sequence_file = match file.try_clone().map(cap_std::fs::File::into_std) {
                    Ok(file) => file,
                    Err(_) => {
                        create.indeterminate(0);
                        return Err(ArtifactTreeFailure::structural(
                            ArtifactTreeFailureKind::IndeterminateEffect,
                        ));
                    }
                };
                let mutation_sequence = match self.owner.mutation_sequence_for(&sequence_file) {
                    Ok(sequence) => sequence,
                    Err(_) => {
                        create.indeterminate(0);
                        return Err(ArtifactTreeFailure::structural(
                            ArtifactTreeFailureKind::IndeterminateEffect,
                        ));
                    }
                };
                create.completed(0);
                Ok(ArtifactTreeNewFile {
                    owner: self.owner,
                    store: self.store,
                    artifact: artifact.clone(),
                    file,
                    mutation_sequence,
                    _coordination: coordination.release_namespace(),
                    completed_bytes: 0,
                })
            }
            Err(error) => {
                create.denied();
                let kind = if error.kind() == std::io::ErrorKind::AlreadyExists {
                    ArtifactTreeFailureKind::AlreadyExists
                } else {
                    ArtifactTreeFailureKind::DeniedBeforeEffect
                };
                Err(ArtifactTreeFailure::io(kind, &error))
            }
        }
    }

    pub fn file_length(&self, artifact: &ArtifactTreeFile) -> Result<u64, ArtifactTreeFailure> {
        let directory = self.open_directory(&artifact.directory)?;
        let file = self.open_readable_file(&directory, &artifact.file_name)?;
        artifact_file_length(self.owner, &file)
    }

    pub fn read_exact_at(
        &self,
        artifact: &ArtifactTreeFile,
        offset: u64,
        target: &mut [u8],
    ) -> Result<(), ArtifactTreeFailure> {
        let directory = self.open_directory(&artifact.directory)?;
        let mut file = self.open_readable_file(&directory, &artifact.file_name)?;
        let end = offset.checked_add(target.len() as u64).ok_or_else(|| {
            ArtifactTreeFailure::structural(ArtifactTreeFailureKind::AccessLimitExceeded)
        })?;
        if end > artifact_file_length(self.owner, &file)? {
            return Err(ArtifactTreeFailure::structural(
                ArtifactTreeFailureKind::Damaged,
            ));
        }
        file.seek(SeekFrom::Start(offset))
            .map_err(|error| ArtifactTreeFailure::io(ArtifactTreeFailureKind::Damaged, &error))?;
        let read = begin(
            self.owner,
            MediaOperationRole::PositionedRead,
            target.len() as u64,
        );
        if let Some(error) = read.fail_before_error() {
            read.denied();
            return Err(ArtifactTreeFailure::io(
                ArtifactTreeFailureKind::DeniedBeforeEffect,
                &error,
            ));
        }
        let requested = target.len() as u64;
        if read.transfer_limit(requested) as usize != target.len() {
            read.denied();
            return Err(ArtifactTreeFailure::structural(
                ArtifactTreeFailureKind::AccessLimitExceeded,
            ));
        }
        match file.read_exact(target) {
            Ok(()) => {
                read.completed(requested);
                Ok(())
            }
            Err(error) => {
                read.denied();
                Err(ArtifactTreeFailure::io(
                    ArtifactTreeFailureKind::Damaged,
                    &error,
                ))
            }
        }
    }
}
