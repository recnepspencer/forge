use super::{
    MediaCausalBoundary, MediaOperationFailure, MediaOperationFailureKind, MediaOperationRole,
    NamespaceFileHandle,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaFileType {
    RegularFile,
    Directory,
    LinkLike,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaAllocatedBytes {
    Observed(u64),
    PlatformUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MediaMetadata {
    file_type: MediaFileType,
    logical_length: u64,
    allocated_bytes: MediaAllocatedBytes,
    readonly: bool,
}

impl MediaMetadata {
    pub const fn file_type(self) -> MediaFileType {
        self.file_type
    }

    pub const fn logical_length(self) -> u64 {
        self.logical_length
    }

    pub const fn allocated_bytes(self) -> MediaAllocatedBytes {
        self.allocated_bytes
    }

    pub const fn readonly(self) -> bool {
        self.readonly
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaMetadataResult {
    Observed(MediaMetadata),
    Failed(MediaOperationFailure),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MediaMetadataOutcome {
    operation: super::MediaOperationIdentity,
    result: MediaMetadataResult,
}

impl MediaMetadataOutcome {
    pub const fn operation(self) -> super::MediaOperationIdentity {
        self.operation
    }

    pub const fn result(self) -> MediaMetadataResult {
        self.result
    }
}

impl<Access> NamespaceFileHandle<'_, Access> {
    pub fn metadata(&self) -> MediaMetadataOutcome {
        let operation = self
            .owner()
            .issue_operation_identity()
            .expect("media operation identity exhausted");
        let attempt = self.owner().boundary().begin_operation(
            MediaOperationRole::ReadMetadata,
            0,
            super::MediaOperationCoordinates::for_path(
                operation,
                self.role(),
                Some(self.identity()),
            ),
        );
        if let Some(error) = attempt.fail_before_error() {
            attempt.denied();
            return MediaMetadataOutcome {
                operation,
                result: MediaMetadataResult::Failed(super::failure_context::operation_failure(
                    operation,
                    MediaOperationRole::ReadMetadata,
                    self.role(),
                    Some(self.identity()),
                    MediaOperationFailureKind::DeniedBeforeEffect,
                    Some(&error),
                    MediaCausalBoundary::BeforeOsCall,
                )),
            };
        }
        match self.file().metadata() {
            Ok(metadata) => {
                attempt.completed(0);
                MediaMetadataOutcome {
                    operation,
                    result: MediaMetadataResult::Observed(MediaMetadata {
                        file_type: classify_file_type(metadata.file_type()),
                        logical_length: metadata.len(),
                        allocated_bytes: allocated_bytes(&metadata),
                        readonly: metadata.permissions().readonly(),
                    }),
                }
            }
            Err(error) => {
                attempt.denied();
                MediaMetadataOutcome {
                    operation,
                    result: MediaMetadataResult::Failed(super::failure_context::operation_failure(
                        operation,
                        MediaOperationRole::ReadMetadata,
                        self.role(),
                        Some(self.identity()),
                        MediaOperationFailureKind::DeniedBeforeEffect,
                        Some(&error),
                        MediaCausalBoundary::OsCallReturned,
                    )),
                }
            }
        }
    }
}

fn classify_file_type(file_type: std::fs::FileType) -> MediaFileType {
    if file_type.is_file() {
        MediaFileType::RegularFile
    } else if file_type.is_dir() {
        MediaFileType::Directory
    } else if file_type.is_symlink() {
        MediaFileType::LinkLike
    } else {
        MediaFileType::Other
    }
}

#[cfg(unix)]
fn allocated_bytes(metadata: &std::fs::Metadata) -> MediaAllocatedBytes {
    use std::os::unix::fs::MetadataExt;

    MediaAllocatedBytes::Observed(metadata.blocks().saturating_mul(512))
}

#[cfg(not(unix))]
fn allocated_bytes(_metadata: &std::fs::Metadata) -> MediaAllocatedBytes {
    MediaAllocatedBytes::PlatformUnavailable
}
