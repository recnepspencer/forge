use std::ffi::OsString;

use worth_store_physical_format::store_namespace::NamespaceEntryType;

use super::{
    FilesystemMediaOwner, MediaCausalBoundary, MediaOperationFailure, MediaOperationFailureKind,
    MediaOperationRole, MediaPathRole, NamespaceDirectoryHandle,
};

pub const MAX_DIRECTORY_BATCH_ENTRIES: usize = 4_096;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceEntry {
    name: OsString,
    entry_type: NamespaceEntryType,
}

impl NamespaceEntry {
    pub fn name(&self) -> &std::ffi::OsStr {
        &self.name
    }

    pub const fn entry_type(&self) -> NamespaceEntryType {
        self.entry_type
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceEntryBatch {
    entries: Vec<NamespaceEntry>,
    exhausted: bool,
}

impl NamespaceEntryBatch {
    pub fn entries(&self) -> &[NamespaceEntry] {
        &self.entries
    }

    pub const fn exhausted(&self) -> bool {
        self.exhausted
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NamespaceEntryBatchResult {
    Observed(NamespaceEntryBatch),
    Partial {
        entries: Vec<NamespaceEntry>,
        failure: MediaOperationFailure,
    },
    Failed(MediaOperationFailure),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceEntryBatchOutcome {
    operation: super::MediaOperationIdentity,
    result: NamespaceEntryBatchResult,
}

impl NamespaceEntryBatchOutcome {
    pub const fn operation(&self) -> super::MediaOperationIdentity {
        self.operation
    }

    pub const fn result(&self) -> &NamespaceEntryBatchResult {
        &self.result
    }
}

#[derive(Debug)]
pub enum NamespaceDirectoryListingResult<'owner> {
    Opened(Box<NamespaceDirectoryListing<'owner>>),
    Failed(MediaOperationFailure),
}

#[derive(Debug)]
pub struct NamespaceDirectoryListing<'owner> {
    owner: &'owner FilesystemMediaOwner,
    handle: super::MediaHandleIdentity,
    role: MediaPathRole,
    entries: cap_std::fs::ReadDir,
    exhausted: bool,
}

impl FilesystemMediaOwner {
    pub fn begin_family_listing(&self) -> NamespaceDirectoryListingResult<'_> {
        self.begin_directory_listing(self.families().handle())
    }

    pub fn begin_staging_listing(&self) -> NamespaceDirectoryListingResult<'_> {
        self.begin_directory_listing(self.staging().handle())
    }

    pub fn begin_directory_listing<'owner>(
        &'owner self,
        directory: &NamespaceDirectoryHandle,
    ) -> NamespaceDirectoryListingResult<'owner> {
        let operation = self
            .issue_operation_identity()
            .expect("media operation identity exhausted");
        let attempt = self.boundary().begin_operation(
            MediaOperationRole::ListDirectory,
            0,
            super::MediaOperationCoordinates::for_path(
                operation,
                directory.role(),
                Some(directory.identity()),
            ),
        );
        if let Some(error) = attempt.fail_before_error() {
            attempt.denied();
            return NamespaceDirectoryListingResult::Failed(
                super::failure_context::operation_failure(
                    operation,
                    MediaOperationRole::ListDirectory,
                    directory.role(),
                    Some(directory.identity()),
                    MediaOperationFailureKind::DeniedBeforeEffect,
                    Some(&error),
                    MediaCausalBoundary::BeforeOsCall,
                ),
            );
        }
        if self.require_owned_directory(directory).is_err() {
            attempt.confinement_denied();
            return NamespaceDirectoryListingResult::Failed(
                super::failure_context::operation_failure(
                    operation,
                    MediaOperationRole::ListDirectory,
                    directory.role(),
                    Some(directory.identity()),
                    MediaOperationFailureKind::DeniedBeforeEffect,
                    None,
                    MediaCausalBoundary::BeforeOsCall,
                ),
            );
        }
        match directory.directory().entries() {
            Ok(entries) => {
                attempt.completed(0);
                NamespaceDirectoryListingResult::Opened(Box::new(NamespaceDirectoryListing {
                    owner: self,
                    handle: directory.identity(),
                    role: directory.role(),
                    entries,
                    exhausted: false,
                }))
            }
            Err(error) => {
                attempt.denied();
                NamespaceDirectoryListingResult::Failed(super::failure_context::operation_failure(
                    operation,
                    MediaOperationRole::ListDirectory,
                    directory.role(),
                    Some(directory.identity()),
                    MediaOperationFailureKind::DeniedBeforeEffect,
                    Some(&error),
                    MediaCausalBoundary::OsCallReturned,
                ))
            }
        }
    }
}

impl NamespaceDirectoryListing<'_> {
    pub fn next_batch(&mut self, limit: usize) -> NamespaceEntryBatchOutcome {
        let operation = self
            .owner
            .issue_operation_identity()
            .expect("media operation identity exhausted");
        let attempt = self.owner.boundary().begin_operation(
            MediaOperationRole::ListDirectory,
            0,
            super::MediaOperationCoordinates::for_path(operation, self.role, Some(self.handle)),
        );
        if let Some(error) = attempt.fail_before_error() {
            attempt.denied();
            return self.failure(operation, Vec::new(), Some(&error));
        }
        if limit == 0 || limit > MAX_DIRECTORY_BATCH_ENTRIES {
            attempt.denied();
            return self.failure(operation, Vec::new(), None);
        }
        if self.exhausted {
            self.owner.boundary().counters().listing_batch(0);
            attempt.completed(0);
            return NamespaceEntryBatchOutcome {
                operation,
                result: NamespaceEntryBatchResult::Observed(NamespaceEntryBatch {
                    entries: Vec::new(),
                    exhausted: true,
                }),
            };
        }

        let mut observed = Vec::with_capacity(limit);
        self.owner
            .boundary()
            .counters()
            .explicit_heap_allocation(limit.saturating_mul(std::mem::size_of::<NamespaceEntry>()));
        while observed.len() < limit {
            let Some(entry) = self.entries.next() else {
                self.exhausted = true;
                break;
            };
            let entry = match entry {
                Ok(entry) => entry,
                Err(error) => {
                    terminalize_listing_failure(
                        attempt,
                        self.owner.boundary().counters(),
                        observed.len(),
                    );
                    return self.failure(operation, observed, Some(&error));
                }
            };
            let entry_type = match entry.file_type() {
                Ok(file_type) => classify_entry_type(file_type),
                Err(error) => {
                    terminalize_listing_failure(
                        attempt,
                        self.owner.boundary().counters(),
                        observed.len(),
                    );
                    return self.failure(operation, observed, Some(&error));
                }
            };
            observed.push(NamespaceEntry {
                name: entry.file_name(),
                entry_type,
            });
        }
        self.owner
            .boundary()
            .counters()
            .listing_batch(observed.len());
        attempt.completed(0);
        NamespaceEntryBatchOutcome {
            operation,
            result: NamespaceEntryBatchResult::Observed(NamespaceEntryBatch {
                entries: observed,
                exhausted: self.exhausted,
            }),
        }
    }

    fn failure(
        &self,
        operation: super::MediaOperationIdentity,
        entries: Vec<NamespaceEntry>,
        error: Option<&std::io::Error>,
    ) -> NamespaceEntryBatchOutcome {
        let failure = super::failure_context::operation_failure(
            operation,
            MediaOperationRole::ListDirectory,
            self.role,
            Some(self.handle),
            MediaOperationFailureKind::DeniedBeforeEffect,
            error,
            if error.is_some() {
                MediaCausalBoundary::OsCallReturned
            } else {
                MediaCausalBoundary::BeforeOsCall
            },
        );
        NamespaceEntryBatchOutcome {
            operation,
            result: if entries.is_empty() {
                NamespaceEntryBatchResult::Failed(failure)
            } else {
                NamespaceEntryBatchResult::Partial { entries, failure }
            },
        }
    }
}

fn terminalize_listing_failure(
    attempt: super::fault_interposition::MediaBoundaryAttempt<'_>,
    counters: &super::operation_counters::MediaCounterCells,
    observed_entries: usize,
) {
    if observed_entries == 0 {
        attempt.denied();
    } else {
        counters.listing_batch(observed_entries);
        attempt.partial(0);
    }
}

fn classify_entry_type(file_type: cap_std::fs::FileType) -> NamespaceEntryType {
    if file_type.is_file() {
        NamespaceEntryType::RegularFile
    } else if file_type.is_dir() {
        NamespaceEntryType::Directory
    } else if file_type.is_symlink() {
        NamespaceEntryType::LinkLike
    } else {
        NamespaceEntryType::Other
    }
}
