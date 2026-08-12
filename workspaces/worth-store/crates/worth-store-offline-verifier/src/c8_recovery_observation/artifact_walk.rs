use std::path::{Path, PathBuf};

use super::artifact_observation::observe_file;
use super::{
    RecoveryObserverCounters, RecoveryObserverLimits, RecoveryObserverObservationDenial,
    RecoveryObserverObservationFailure,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ObservedRecoveryArtifact {
    pub(super) path: Box<str>,
    pub(super) byte_length: u64,
    pub(super) digest: [u8; 32],
}

pub(super) struct RecoveryObserverWalk {
    artifacts: Vec<ObservedRecoveryArtifact>,
    counters: RecoveryObserverCounters,
}

pub(super) fn walk(
    store_root: &Path,
    limits: RecoveryObserverLimits,
) -> Result<RecoveryObserverWalk, RecoveryObserverObservationFailure> {
    let mut counters = RecoveryObserverCounters::with_root_admitted();
    let root = store_root.canonicalize().map_err(|error| {
        RecoveryObserverObservationFailure::at_path(
            RecoveryObserverObservationDenial::Media(error.kind()),
            counters,
            store_root,
        )
    })?;
    let mut pending = vec![root.clone()];
    let mut artifacts = Vec::new();
    while let Some(directory) = pending.pop() {
        let mut entries = admitted_directory_entries(&directory, limits, &mut counters)?;
        entries.sort_by_key(std::fs::DirEntry::file_name);
        for entry in entries.into_iter().rev() {
            admit_entry(
                &root,
                entry,
                limits,
                &mut pending,
                &mut artifacts,
                &mut counters,
            )?;
        }
    }
    artifacts.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(RecoveryObserverWalk {
        artifacts,
        counters,
    })
}

fn admitted_directory_entries(
    directory: &Path,
    limits: RecoveryObserverLimits,
    counters: &mut RecoveryObserverCounters,
) -> Result<Vec<std::fs::DirEntry>, RecoveryObserverObservationFailure> {
    let entries = std::fs::read_dir(directory).map_err(|error| {
        RecoveryObserverObservationFailure::at_path(
            RecoveryObserverObservationDenial::Media(error.kind()),
            *counters,
            directory,
        )
    })?;
    counters.record_directory_opened().ok_or_else(|| {
        RecoveryObserverObservationFailure::at_path(
            RecoveryObserverObservationDenial::ArtifactChanged,
            *counters,
            directory,
        )
    })?;
    let remaining = limits
        .maximum_directory_entries()
        .saturating_sub(counters.directory_entries_observed());
    let capacity = usize::try_from(remaining.min(256)).unwrap_or(256);
    let mut admitted = Vec::with_capacity(capacity);
    for entry in entries {
        let entry = entry.map_err(|error| {
            RecoveryObserverObservationFailure::at_path(
                RecoveryObserverObservationDenial::Media(error.kind()),
                *counters,
                directory,
            )
        })?;
        let observed = counters.record_directory_entry().ok_or_else(|| {
            RecoveryObserverObservationFailure::at_path(
                RecoveryObserverObservationDenial::ArtifactChanged,
                *counters,
                directory,
            )
        })?;
        if observed > limits.maximum_directory_entries() {
            return Err(RecoveryObserverObservationFailure::at_path(
                RecoveryObserverObservationDenial::DirectoryEntryLimit {
                    observed,
                    admitted: limits.maximum_directory_entries(),
                },
                *counters,
                &entry.path(),
            ));
        }
        admitted.push(entry);
    }
    Ok(admitted)
}

fn admit_entry(
    root: &Path,
    entry: std::fs::DirEntry,
    limits: RecoveryObserverLimits,
    pending: &mut Vec<PathBuf>,
    artifacts: &mut Vec<ObservedRecoveryArtifact>,
    counters: &mut RecoveryObserverCounters,
) -> Result<(), RecoveryObserverObservationFailure> {
    let path = entry.path();
    let file_type = entry.file_type().map_err(|error| {
        RecoveryObserverObservationFailure::at_path(
            RecoveryObserverObservationDenial::Media(error.kind()),
            *counters,
            &path,
        )
    })?;
    if file_type.is_symlink() {
        return Err(RecoveryObserverObservationFailure::at_path(
            RecoveryObserverObservationDenial::SymbolicLink,
            *counters,
            &path,
        ));
    }
    if file_type.is_dir() {
        let observed = counters.record_directory_admitted().ok_or_else(|| {
            RecoveryObserverObservationFailure::at_path(
                RecoveryObserverObservationDenial::ArtifactChanged,
                *counters,
                &path,
            )
        })?;
        if observed > limits.maximum_directories() {
            return Err(RecoveryObserverObservationFailure::at_path(
                RecoveryObserverObservationDenial::DirectoryLimit {
                    observed,
                    admitted: limits.maximum_directories(),
                },
                *counters,
                &path,
            ));
        }
        pending.push(path);
        return Ok(());
    }
    if file_type.is_file() {
        let observed = counters.record_artifact_admitted().ok_or_else(|| {
            RecoveryObserverObservationFailure::at_path(
                RecoveryObserverObservationDenial::ArtifactChanged,
                *counters,
                &path,
            )
        })?;
        if observed > limits.maximum_artifacts() {
            return Err(RecoveryObserverObservationFailure::at_path(
                RecoveryObserverObservationDenial::ArtifactLimit {
                    observed,
                    admitted: limits.maximum_artifacts(),
                },
                *counters,
                &path,
            ));
        }
        artifacts.push(observe_file(root, path, limits, counters)?);
        return Ok(());
    }
    Err(RecoveryObserverObservationFailure::at_path(
        RecoveryObserverObservationDenial::UnsupportedFileType,
        *counters,
        &path,
    ))
}

impl RecoveryObserverWalk {
    pub(super) fn artifacts(&self) -> &[ObservedRecoveryArtifact] {
        &self.artifacts
    }

    pub(super) const fn counters(&self) -> RecoveryObserverCounters {
        self.counters
    }
}

impl ObservedRecoveryArtifact {
    pub(super) fn path(&self) -> &str {
        &self.path
    }

    pub(super) const fn byte_length(&self) -> u64 {
        self.byte_length
    }

    pub(super) const fn digest(&self) -> [u8; 32] {
        self.digest
    }
}
