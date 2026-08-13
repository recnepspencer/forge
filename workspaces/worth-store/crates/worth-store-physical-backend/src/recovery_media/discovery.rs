use std::ffi::OsString;

use crate::filesystem_media::{
    ArtifactTreeDirectory, ArtifactTreeFailure, ArtifactTreeFailureKind, ArtifactTreeFile,
};
use worth_store_physical_format::{store_namespace::NamespaceEntryType, RecordArtifactFile};

use super::{AdmittedRecoveryFilesystemMedia, RecoveryFilesystemQualificationError};

mod addressed_payload;
mod addressed_range;
mod artifact;

pub(crate) use artifact::record_artifact;
pub use artifact::RecoveryDiscoveryArtifact;

pub struct BoundedRecoveryFilesystemDiscovery {
    parts: crate::filesystem_media::recovery_qualification::AdmittedRecoveryParts,
    remaining_entries: u64,
    remaining_bytes: u64,
    maximum_bytes: u64,
    counters: RecoveryDiscoveryCounters,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservedRecoveryArtifact {
    bytes: Option<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservedWalArtifact {
    name: OsString,
    entry_type: NamespaceEntryType,
    bytes: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RecoveryDiscoveryCounters {
    pub fixed_slots_read: u64,
    pub addressed_artifacts_read: u64,
    pub directory_entries_observed: u64,
    pub bytes_read: u64,
    pub wal_bytes_read: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryDiscoveryFailure {
    EntryLimitExceeded {
        observed: u64,
        admitted: u64,
    },
    ByteLimitExceeded {
        observed: u64,
        admitted: u64,
        scope: RecoveryDiscoveryByteLimitScope,
    },
    Media {
        artifact: RecoveryDiscoveryArtifact,
        failure: ArtifactTreeFailure,
    },
    InvalidAddress {
        artifact: RecoveryDiscoveryArtifact,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryDiscoveryByteLimitScope {
    Observation,
    Requested,
}

impl BoundedRecoveryFilesystemDiscovery {
    pub const fn store_identity(
        &self,
    ) -> worth_store_physical_format::store_namespace::StableStoreIdentity {
        self.parts.store_identity
    }

    pub(crate) fn new(
        parts: crate::filesystem_media::recovery_qualification::AdmittedRecoveryParts,
        maximum_entries: u64,
        maximum_bytes: u64,
    ) -> Result<Self, RecoveryFilesystemQualificationError> {
        if maximum_entries == 0 || maximum_bytes == 0 {
            return Err(RecoveryFilesystemQualificationError::InvalidDiscoveryLimit);
        }
        Ok(Self {
            parts,
            remaining_entries: maximum_entries,
            remaining_bytes: maximum_bytes,
            maximum_bytes,
            counters: RecoveryDiscoveryCounters::default(),
        })
    }

    pub fn read_current_selector(
        &mut self,
        byte_limit: u64,
    ) -> Result<ObservedRecoveryArtifact, RecoveryDiscoveryFailure> {
        self.read_fixed(RecordArtifactFile::CurrentRootSelector, byte_limit)
    }

    pub fn read_bootstrap_catalog(
        &mut self,
        byte_limit: u64,
    ) -> Result<ObservedRecoveryArtifact, RecoveryDiscoveryFailure> {
        self.read_fixed(RecordArtifactFile::BootstrapCatalog, byte_limit)
    }

    pub fn read_previous_selector(
        &mut self,
        byte_limit: u64,
    ) -> Result<ObservedRecoveryArtifact, RecoveryDiscoveryFailure> {
        self.read_fixed(RecordArtifactFile::PreviousRootSelector, byte_limit)
    }

    pub fn read_root_manifest(
        &mut self,
        generation: u64,
        byte_limit: u64,
    ) -> Result<ObservedRecoveryArtifact, RecoveryDiscoveryFailure> {
        self.read_addressed(RecordArtifactFile::RootManifest { generation }, byte_limit)
    }

    pub fn read_root_routing_block(
        &mut self,
        generation: u64,
        block: u64,
        byte_limit: u64,
    ) -> Result<ObservedRecoveryArtifact, RecoveryDiscoveryFailure> {
        self.read_addressed(
            RecordArtifactFile::RootRoutingBlock { generation, block },
            byte_limit,
        )
    }

    pub fn read_current_checkpoint(
        &mut self,
        byte_limit: u64,
    ) -> Result<ObservedRecoveryArtifact, RecoveryDiscoveryFailure> {
        let context = RecoveryDiscoveryArtifact::CurrentCheckpoint;
        let artifact = ArtifactTreeDirectory::families()
            .file("checkpoint.current")
            .map_err(|_| RecoveryDiscoveryFailure::invalid(context.clone()))?;
        self.read_artifact(artifact, context, byte_limit, false)
    }

    pub fn read_wal_artifacts(
        &mut self,
        maximum_segments: u64,
        byte_limit: u64,
    ) -> Result<Vec<ObservedWalArtifact>, RecoveryDiscoveryFailure> {
        let maximum_segments = usize::try_from(maximum_segments).map_err(|_| {
            RecoveryDiscoveryFailure::EntryLimitExceeded {
                observed: maximum_segments,
                admitted: usize::MAX as u64,
            }
        })?;
        if maximum_segments == 0 {
            return Err(RecoveryDiscoveryFailure::EntryLimitExceeded {
                observed: maximum_segments as u64,
                admitted: 0,
            });
        }
        let directory_context = RecoveryDiscoveryArtifact::WalDirectory;
        let directory = ArtifactTreeDirectory::families()
            .child("wal")
            .map_err(|_| RecoveryDiscoveryFailure::invalid(directory_context.clone()))?;
        if !self
            .parts
            .artifact_tree()
            .directory_exists(&directory)
            .map_err(|failure| map_media(failure, directory_context.clone()))?
        {
            return Ok(Vec::new());
        }
        let entries = self
            .parts
            .artifact_tree()
            .list_bounded(&directory, maximum_segments)
            .map_err(|failure| map_media(failure, directory_context))?;
        self.counters.directory_entries_observed += entries.len() as u64;
        let mut observed = Vec::with_capacity(entries.len());
        let mut remaining_wal_bytes = byte_limit;
        for entry in entries {
            let name = entry.name().to_owned();
            let context = RecoveryDiscoveryArtifact::WalArtifact(name.clone());
            let bytes = if entry.entry_type() == NamespaceEntryType::RegularFile {
                let file = directory
                    .file(
                        name.to_str()
                            .ok_or_else(|| RecoveryDiscoveryFailure::invalid(context.clone()))?,
                    )
                    .map_err(|_| RecoveryDiscoveryFailure::invalid(context.clone()))?;
                let bytes = match self.read_artifact(file, context, remaining_wal_bytes, false) {
                    Ok(artifact) => artifact.bytes,
                    Err(RecoveryDiscoveryFailure::ByteLimitExceeded {
                        observed,
                        admitted: _,
                        scope: RecoveryDiscoveryByteLimitScope::Requested,
                    }) => {
                        return Err(RecoveryDiscoveryFailure::ByteLimitExceeded {
                            observed: byte_limit
                                .saturating_sub(remaining_wal_bytes)
                                .saturating_add(observed),
                            admitted: byte_limit,
                            scope: RecoveryDiscoveryByteLimitScope::Requested,
                        })
                    }
                    Err(failure) => return Err(failure),
                };
                remaining_wal_bytes = remaining_wal_bytes
                    .checked_sub(bytes.as_ref().map_or(0, |bytes| bytes.len() as u64))
                    .ok_or(RecoveryDiscoveryFailure::ByteLimitExceeded {
                        observed: byte_limit,
                        admitted: remaining_wal_bytes,
                        scope: RecoveryDiscoveryByteLimitScope::Requested,
                    })?;
                self.counters.wal_bytes_read = self
                    .counters
                    .wal_bytes_read
                    .checked_add(bytes.as_ref().map_or(0, |bytes| bytes.len() as u64))
                    .ok_or(RecoveryDiscoveryFailure::ByteLimitExceeded {
                        observed: u64::MAX,
                        admitted: byte_limit,
                        scope: RecoveryDiscoveryByteLimitScope::Requested,
                    })?;
                bytes
            } else {
                None
            };
            observed.push(ObservedWalArtifact {
                name,
                entry_type: entry.entry_type(),
                bytes,
            });
        }
        Ok(observed)
    }

    pub const fn counters(&self) -> RecoveryDiscoveryCounters {
        self.counters
    }

    pub fn finish(self) -> AdmittedRecoveryFilesystemMedia {
        AdmittedRecoveryFilesystemMedia::from_parts(self.parts)
    }

    fn read_fixed(
        &mut self,
        artifact: RecordArtifactFile,
        byte_limit: u64,
    ) -> Result<ObservedRecoveryArtifact, RecoveryDiscoveryFailure> {
        let context = RecoveryDiscoveryArtifact::Record(artifact);
        let artifact = record_artifact(artifact)?;
        let result = self.read_artifact(artifact, context, byte_limit, true)?;
        self.counters.fixed_slots_read += 1;
        Ok(result)
    }

    fn read_addressed(
        &mut self,
        artifact: RecordArtifactFile,
        byte_limit: u64,
    ) -> Result<ObservedRecoveryArtifact, RecoveryDiscoveryFailure> {
        let context = RecoveryDiscoveryArtifact::Record(artifact);
        let artifact = record_artifact(artifact)?;
        self.read_artifact(artifact, context, byte_limit, false)
    }

    fn read_artifact(
        &mut self,
        artifact: ArtifactTreeFile,
        context: RecoveryDiscoveryArtifact,
        byte_limit: u64,
        fixed: bool,
    ) -> Result<ObservedRecoveryArtifact, RecoveryDiscoveryFailure> {
        let observation_is_tighter = self.remaining_bytes <= byte_limit;
        let effective_byte_limit = byte_limit.min(self.remaining_bytes);
        if self.remaining_entries == 0 {
            return Err(RecoveryDiscoveryFailure::EntryLimitExceeded {
                observed: 1,
                admitted: 0,
            });
        }
        self.remaining_entries -= 1;
        match self
            .parts
            .artifact_tree()
            .read_bounded(&artifact, effective_byte_limit)
        {
            Ok(bytes) => {
                self.counters.bytes_read = self
                    .counters
                    .bytes_read
                    .checked_add(bytes.len() as u64)
                    .ok_or(RecoveryDiscoveryFailure::ByteLimitExceeded {
                        observed: u64::MAX,
                        admitted: self.maximum_bytes,
                        scope: RecoveryDiscoveryByteLimitScope::Observation,
                    })?;
                self.remaining_bytes = self.remaining_bytes.checked_sub(bytes.len() as u64).ok_or(
                    RecoveryDiscoveryFailure::ByteLimitExceeded {
                        observed: bytes.len() as u64,
                        admitted: self.maximum_bytes,
                        scope: RecoveryDiscoveryByteLimitScope::Observation,
                    },
                )?;
                if !fixed {
                    self.counters.addressed_artifacts_read += 1;
                }
                Ok(ObservedRecoveryArtifact { bytes: Some(bytes) })
            }
            Err(failure) if failure.kind() == ArtifactTreeFailureKind::Absent => {
                Ok(ObservedRecoveryArtifact { bytes: None })
            }
            Err(failure) if failure.kind() == ArtifactTreeFailureKind::AccessLimitExceeded => {
                let limit = failure.access_limit().unwrap_or(
                    crate::filesystem_media::ArtifactTreeAccessLimit {
                        observed: effective_byte_limit.saturating_add(1),
                        admitted: effective_byte_limit,
                    },
                );
                if observation_is_tighter {
                    Err(RecoveryDiscoveryFailure::ByteLimitExceeded {
                        observed: self.counters.bytes_read.saturating_add(limit.observed),
                        admitted: self.maximum_bytes,
                        scope: RecoveryDiscoveryByteLimitScope::Observation,
                    })
                } else {
                    Err(RecoveryDiscoveryFailure::ByteLimitExceeded {
                        observed: limit.observed,
                        admitted: limit.admitted,
                        scope: RecoveryDiscoveryByteLimitScope::Requested,
                    })
                }
            }
            Err(failure) => Err(RecoveryDiscoveryFailure::Media {
                artifact: context,
                failure,
            }),
        }
    }
}

impl ObservedRecoveryArtifact {
    pub fn bytes(&self) -> Option<&[u8]> {
        self.bytes.as_deref()
    }

    pub fn into_bytes(self) -> Option<Vec<u8>> {
        self.bytes
    }
}

impl ObservedWalArtifact {
    pub fn name(&self) -> &std::ffi::OsStr {
        &self.name
    }

    pub const fn entry_type(&self) -> NamespaceEntryType {
        self.entry_type
    }

    pub fn bytes(&self) -> Option<&[u8]> {
        self.bytes.as_deref()
    }
}

fn map_media(
    failure: ArtifactTreeFailure,
    artifact: RecoveryDiscoveryArtifact,
) -> RecoveryDiscoveryFailure {
    if failure.kind() == ArtifactTreeFailureKind::AccessLimitExceeded {
        let limit =
            failure
                .access_limit()
                .unwrap_or(crate::filesystem_media::ArtifactTreeAccessLimit {
                    observed: 1,
                    admitted: 0,
                });
        RecoveryDiscoveryFailure::EntryLimitExceeded {
            observed: limit.observed,
            admitted: limit.admitted,
        }
    } else {
        RecoveryDiscoveryFailure::Media { artifact, failure }
    }
}

impl RecoveryDiscoveryFailure {
    fn invalid(artifact: RecoveryDiscoveryArtifact) -> Self {
        Self::InvalidAddress { artifact }
    }
}
