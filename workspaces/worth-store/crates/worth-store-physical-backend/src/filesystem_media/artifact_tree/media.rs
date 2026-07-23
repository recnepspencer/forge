use std::io::Read;

use cap_fs_ext::{FollowSymlinks, OpenOptionsFollowExt};
use cap_std::fs::OpenOptions;

use super::super::artifact_tree_effects::{
    artifact_file_length, begin, create_directory, directory_has_entries,
    directory_has_other_entry, synchronize_directory, synchronize_file,
};
use super::path::validate_component;
use super::{
    ArtifactTreeDirectory, ArtifactTreeFailure, ArtifactTreeFailureKind, ArtifactTreeFile,
};
use crate::filesystem_media::{FilesystemMediaOwner, MediaOperationRole, QualifiedFilesystemMedia};

pub struct ArtifactTreeMedia<'media> {
    pub(super) owner: &'media FilesystemMediaOwner,
    pub(super) store: worth_store_physical_format::store_namespace::StableStoreIdentity,
    pub(super) execution_capability: &'media crate::AdmittedBackendCapabilityWitness,
}

impl QualifiedFilesystemMedia {
    pub fn artifact_tree(&self) -> ArtifactTreeMedia<'_> {
        ArtifactTreeMedia {
            owner: self.artifact_tree_owner(),
            store: self.store_identity(),
            execution_capability: self.execution_capability(),
        }
    }
}

impl ArtifactTreeMedia<'_> {
    pub fn directory_exists(
        &self,
        directory: &ArtifactTreeDirectory,
    ) -> Result<bool, ArtifactTreeFailure> {
        Ok(self.open_optional_directory(directory)?.is_some())
    }

    pub fn file_exists(&self, artifact: &ArtifactTreeFile) -> Result<bool, ArtifactTreeFailure> {
        let Some(directory) = self.open_optional_directory(&artifact.directory)? else {
            return Ok(false);
        };
        let mut options = OpenOptions::new();
        options.read(true).follow(FollowSymlinks::No);
        let attempt = begin(self.owner, MediaOperationRole::OpenExisting, 0);
        if let Some(error) = attempt.fail_before_error() {
            attempt.denied();
            return Err(ArtifactTreeFailure::io(
                ArtifactTreeFailureKind::DeniedBeforeEffect,
                &error,
            ));
        }
        match directory.open_with(&artifact.file_name, &options) {
            Ok(_) => {
                attempt.completed(0);
                Ok(true)
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                attempt.denied();
                Ok(false)
            }
            Err(error) => {
                attempt.denied();
                Err(ArtifactTreeFailure::io(
                    ArtifactTreeFailureKind::DeniedBeforeEffect,
                    &error,
                ))
            }
        }
    }

    pub fn create_directory(
        &self,
        directory: &ArtifactTreeDirectory,
    ) -> Result<(), ArtifactTreeFailure> {
        let _coordination = self
            .owner
            .begin_artifact_namespace_mutation(vec![directory.coordination_key()])
            .map_err(|_| {
                ArtifactTreeFailure::structural(ArtifactTreeFailureKind::DeniedBeforeEffect)
            })?;
        let (name, parent_components) = directory
            .components
            .split_last()
            .ok_or_else(|| ArtifactTreeFailure::structural(ArtifactTreeFailureKind::Damaged))?;
        if parent_components.is_empty() {
            create_directory(self.owner, self.root(directory.root), name)
        } else {
            let parent = self.open_components(directory.root, parent_components)?;
            create_directory(self.owner, &parent, name)
        }
    }

    pub fn write_new(
        &self,
        artifact: &ArtifactTreeFile,
        bytes: &[u8],
    ) -> Result<(), ArtifactTreeFailure> {
        let mut file = self.create_new_file(artifact)?;
        file.write_chunk(bytes).map_err(|failure| {
            if failure.kind() == ArtifactTreeFailureKind::DeniedBeforeEffect {
                ArtifactTreeFailure::structural(ArtifactTreeFailureKind::PartialWrite {
                    completed_bytes: 0,
                })
            } else {
                failure
            }
        })
    }

    pub fn synchronize_file(&self, artifact: &ArtifactTreeFile) -> Result<(), ArtifactTreeFailure> {
        let _coordination = self
            .owner
            .begin_artifact_mutation(vec![artifact.coordination_key()])
            .map_err(|_| {
                ArtifactTreeFailure::structural(ArtifactTreeFailureKind::DeniedBeforeEffect)
            })?;
        let directory = self.open_directory(&artifact.directory)?;
        let file = self.open_mutable_file(&directory, &artifact.file_name)?;
        let sequence_file = file
            .try_clone()
            .map(cap_std::fs::File::into_std)
            .map_err(|error| {
                ArtifactTreeFailure::io(ArtifactTreeFailureKind::DeniedBeforeEffect, &error)
            })?;
        let sequence = self
            .owner
            .mutation_sequence_for(&sequence_file)
            .map_err(|error| {
                ArtifactTreeFailure::io(ArtifactTreeFailureKind::DeniedBeforeEffect, &error)
            })?;
        let _sequence = sequence.lock();
        synchronize_file(self.owner, &file)
    }

    pub fn read_bounded(
        &self,
        artifact: &ArtifactTreeFile,
        limit: u32,
    ) -> Result<Vec<u8>, ArtifactTreeFailure> {
        let directory = self.open_directory(&artifact.directory)?;
        let mut options = OpenOptions::new();
        options.read(true).follow(FollowSymlinks::No);
        let open = begin(self.owner, MediaOperationRole::OpenExisting, 0);
        if let Some(error) = open.fail_before_error() {
            open.denied();
            return Err(ArtifactTreeFailure::io(
                ArtifactTreeFailureKind::DeniedBeforeEffect,
                &error,
            ));
        }
        let mut file = match directory.open_with(&artifact.file_name, &options) {
            Ok(file) => {
                open.completed(0);
                file
            }
            Err(error) => {
                open.denied();
                let kind = if error.kind() == std::io::ErrorKind::NotFound {
                    ArtifactTreeFailureKind::Absent
                } else {
                    ArtifactTreeFailureKind::DeniedBeforeEffect
                };
                return Err(ArtifactTreeFailure::io(kind, &error));
            }
        };
        let length = artifact_file_length(self.owner, &file)?;
        if length > u64::from(limit) || length > usize::MAX as u64 {
            return Err(ArtifactTreeFailure::structural(
                ArtifactTreeFailureKind::AccessLimitExceeded,
            ));
        }
        let read = begin(self.owner, MediaOperationRole::PositionedRead, length);
        if let Some(error) = read.fail_before_error() {
            read.denied();
            return Err(ArtifactTreeFailure::io(
                ArtifactTreeFailureKind::DeniedBeforeEffect,
                &error,
            ));
        }
        if read.transfer_limit(length) != length {
            read.denied();
            return Err(ArtifactTreeFailure::structural(
                ArtifactTreeFailureKind::AccessLimitExceeded,
            ));
        }
        let mut bytes = vec![0_u8; length as usize];
        match file.read_exact(&mut bytes) {
            Ok(()) => {
                read.completed(length);
                Ok(bytes)
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

    pub fn replace(
        &self,
        source: &ArtifactTreeFile,
        destination: &ArtifactTreeFile,
    ) -> Result<(), ArtifactTreeFailure> {
        let _coordination = self
            .owner
            .begin_artifact_namespace_mutation(vec![
                source.coordination_key(),
                destination.coordination_key(),
            ])
            .map_err(|_| {
                ArtifactTreeFailure::structural(ArtifactTreeFailureKind::DeniedBeforeEffect)
            })?;
        let source_directory = self.open_directory(&source.directory)?;
        let destination_directory = self.open_directory(&destination.directory)?;
        let source_file = self.open_mutable_file(&source_directory, &source.file_name)?;
        let source_sequence_file = source_file
            .try_clone()
            .map(cap_std::fs::File::into_std)
            .map_err(|error| {
                ArtifactTreeFailure::io(ArtifactTreeFailureKind::DeniedBeforeEffect, &error)
            })?;
        let source_sequence = self
            .owner
            .mutation_sequence_for(&source_sequence_file)
            .map_err(|error| {
                ArtifactTreeFailure::io(ArtifactTreeFailureKind::DeniedBeforeEffect, &error)
            })?;
        let destination_sequence =
            match self.open_mutable_file(&destination_directory, &destination.file_name) {
                Ok(file) => {
                    let sequence_file =
                        file.try_clone()
                            .map(cap_std::fs::File::into_std)
                            .map_err(|error| {
                                ArtifactTreeFailure::io(
                                    ArtifactTreeFailureKind::DeniedBeforeEffect,
                                    &error,
                                )
                            })?;
                    Some(
                        self.owner
                            .mutation_sequence_for(&sequence_file)
                            .map_err(|error| {
                                ArtifactTreeFailure::io(
                                    ArtifactTreeFailureKind::DeniedBeforeEffect,
                                    &error,
                                )
                            })?,
                    )
                }
                Err(failure) if failure.kind() == ArtifactTreeFailureKind::Absent => None,
                Err(failure) => return Err(failure),
            };
        let attempt = begin(self.owner, MediaOperationRole::AtomicReplace, 0);
        if let Some(error) = attempt.fail_before_error() {
            attempt.denied();
            return Err(ArtifactTreeFailure::io(
                ArtifactTreeFailureKind::DeniedBeforeEffect,
                &error,
            ));
        }
        let rename = || {
            source_directory.rename(
                &source.file_name,
                &destination_directory,
                &destination.file_name,
            )
        };
        match super::super::file_mutation_sequence::FileMutationSequence::with_ordered_pair(
            &source_sequence,
            destination_sequence.as_ref(),
            rename,
        ) {
            Ok(()) if attempt.effect_observation_is_indeterminate() => {
                attempt.indeterminate(0);
                Err(ArtifactTreeFailure::structural(
                    ArtifactTreeFailureKind::IndeterminateEffect,
                ))
            }
            Ok(()) => {
                self.owner.boundary().counters().replacement();
                attempt.completed(0);
                Ok(())
            }
            Err(error) => {
                attempt.indeterminate(0);
                Err(ArtifactTreeFailure::io(
                    ArtifactTreeFailureKind::IndeterminateEffect,
                    &error,
                ))
            }
        }
    }

    pub fn synchronize_directory(
        &self,
        directory: &ArtifactTreeDirectory,
    ) -> Result<(), ArtifactTreeFailure> {
        let _coordination = self
            .owner
            .begin_artifact_namespace_mutation(vec![directory.coordination_key()])
            .map_err(|_| {
                ArtifactTreeFailure::structural(ArtifactTreeFailureKind::DeniedBeforeEffect)
            })?;
        if directory.components.is_empty() {
            synchronize_directory(self.owner, self.root(directory.root))
        } else {
            let directory = self.open_directory(directory)?;
            synchronize_directory(self.owner, &directory)
        }
    }

    pub fn directory_has_entries(
        &self,
        directory: &ArtifactTreeDirectory,
    ) -> Result<bool, ArtifactTreeFailure> {
        let directory = self.open_directory(directory)?;
        directory_has_entries(self.owner, &directory)
    }

    pub fn directory_has_other_entry(
        &self,
        directory: &ArtifactTreeDirectory,
        selected: &str,
    ) -> Result<bool, ArtifactTreeFailure> {
        validate_component(selected)
            .map_err(|_| ArtifactTreeFailure::structural(ArtifactTreeFailureKind::Damaged))?;
        let directory = self.open_directory(directory)?;
        directory_has_other_entry(self.owner, &directory, selected)
    }
}
