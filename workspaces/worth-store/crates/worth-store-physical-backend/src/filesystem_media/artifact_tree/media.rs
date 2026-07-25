use std::io::Read;

use cap_fs_ext::{FollowSymlinks, OpenOptionsFollowExt};
use cap_std::fs::OpenOptions;

use super::super::artifact_tree_effects::{
    artifact_file_length, begin, create_directory, directory_has_entries,
    directory_has_other_entry, synchronize_directory,
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
        discard_publication_effect(self.synchronize_file_observed(artifact))
    }

    /// Removes one confined artifact and synchronizes its parent namespace.
    ///
    /// Success is the durable terminal boundary for short-lived recovery
    /// obligation files; an indeterminate delete remains an inspection case.
    pub fn remove_file_durably(
        &self,
        artifact: &ArtifactTreeFile,
    ) -> Result<(), ArtifactTreeFailure> {
        let _coordination = self
            .owner
            .begin_artifact_namespace_mutation(vec![artifact.coordination_key()])
            .map_err(|_| {
                ArtifactTreeFailure::structural(ArtifactTreeFailureKind::DeniedBeforeEffect)
            })?;
        let directory = self.open_directory(&artifact.directory)?;
        let attempt = begin(self.owner, MediaOperationRole::Delete, 0);
        if let Some(error) = attempt.fail_before_error() {
            attempt.denied();
            return Err(ArtifactTreeFailure::io(
                ArtifactTreeFailureKind::DeniedBeforeEffect,
                &error,
            ));
        }
        match directory.remove_file(&artifact.file_name) {
            Ok(()) if attempt.effect_observation_is_indeterminate() => {
                attempt.indeterminate(0);
                Err(ArtifactTreeFailure::structural(
                    ArtifactTreeFailureKind::IndeterminateEffect,
                ))
            }
            Ok(()) => {
                self.owner.boundary().counters().deletion();
                attempt.completed(0);
                synchronize_directory(self.owner, &directory)
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
        discard_publication_effect(self.replace_observed(source, destination))
    }

    pub fn synchronize_directory(
        &self,
        directory: &ArtifactTreeDirectory,
    ) -> Result<(), ArtifactTreeFailure> {
        discard_publication_effect(self.synchronize_directory_observed(directory))
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

fn discard_publication_effect(
    outcome: super::ArtifactTreePublicationEffectOutcome,
) -> Result<(), ArtifactTreeFailure> {
    match outcome {
        super::ArtifactTreePublicationEffectOutcome::Completed(_) => Ok(()),
        super::ArtifactTreePublicationEffectOutcome::DeniedBeforeEffect(failure) => Err(failure),
        super::ArtifactTreePublicationEffectOutcome::Indeterminate(failure) => {
            Err(failure.failure())
        }
    }
}
