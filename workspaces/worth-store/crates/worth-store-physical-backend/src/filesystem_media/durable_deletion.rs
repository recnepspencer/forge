use super::{
    DirectoryPublicationSynchronizationOutcome, FilesystemMediaOwner, MediaAttemptedEffect,
    MediaCausalBoundary, MediaEstablishedBoundary, MediaOperationFailure,
    MediaOperationFailureKind, NamespaceDirectoryHandle, NamespaceRelativePath,
};

#[derive(Debug)]
pub struct VisibleNamespaceDeletion<'owner> {
    owner: &'owner FilesystemMediaOwner,
    path: NamespaceRelativePath,
    deletion_operation: super::MediaOperationIdentity,
    parent: &'owner NamespaceDirectoryHandle,
    _namespace_authority: super::mutation_ownership::CoordinatedNamespaceMutation<'owner>,
}

#[derive(Debug)]
pub struct IndeterminateNamespaceDeletion {
    path: NamespaceRelativePath,
    failure: MediaOperationFailure,
}

#[derive(Debug)]
pub enum NamespaceDeletionOutcome<'owner> {
    Removed(VisibleNamespaceDeletion<'owner>),
    Failed(MediaOperationFailure),
    Indeterminate(IndeterminateNamespaceDeletion),
}

#[derive(Debug)]
pub struct DurableDeletion {
    path: NamespaceRelativePath,
    deletion_operation: super::MediaOperationIdentity,
    directory_synchronization: super::DirectoryPublicationSynchronization,
}

#[derive(Debug)]
pub enum DurableDeletionOutcome {
    Durable(DurableDeletion),
    Indeterminate(IndeterminateNamespaceDeletion),
}

impl FilesystemMediaOwner {
    pub fn delete_namespace_file(
        &self,
        handle: super::NamespaceFileHandle<'_>,
    ) -> NamespaceDeletionOutcome<'_> {
        let operation = self
            .issue_operation_identity()
            .expect("media operation identity exhausted");
        let (handle_owner, path, handle_identity, stable_file, open_file) =
            handle.into_deletion_parts();
        let attempt = self.boundary().begin_operation(
            super::MediaOperationRole::Delete,
            0,
            super::MediaOperationCoordinates::for_path(
                operation,
                path.role(),
                Some(handle_identity),
            ),
        );
        if let Some(error) = attempt.fail_before_error() {
            attempt.denied();
            return NamespaceDeletionOutcome::Failed(deletion_failure(
                operation,
                &path,
                Some(&error),
                Some(handle_identity),
                MediaOperationFailureKind::DeniedBeforeEffect,
                MediaCausalBoundary::BeforeOsCall,
            ));
        }
        if !std::ptr::eq(handle_owner, self) {
            attempt.confinement_denied();
            return NamespaceDeletionOutcome::Failed(deletion_failure(
                operation,
                &path,
                None,
                Some(handle_identity),
                MediaOperationFailureKind::DeniedBeforeEffect,
                MediaCausalBoundary::BeforeOsCall,
            ));
        }
        let Some(parent) = self.directory_for_path(&path) else {
            attempt.confinement_denied();
            return NamespaceDeletionOutcome::Failed(deletion_failure(
                operation,
                &path,
                None,
                Some(handle_identity),
                MediaOperationFailureKind::DeniedBeforeEffect,
                MediaCausalBoundary::BeforeOsCall,
            ));
        };
        let namespace_authority = match self.begin_namespace_mutation() {
            Ok(authority) => authority,
            Err(_) => {
                attempt.denied();
                return NamespaceDeletionOutcome::Failed(deletion_failure(
                    operation,
                    &path,
                    None,
                    Some(handle_identity),
                    MediaOperationFailureKind::DeniedBeforeEffect,
                    MediaCausalBoundary::BeforeOsCall,
                ));
            }
        };
        match super::named_file_identity::validates_coordinated_name(
            parent,
            path.file_name(),
            &stable_file,
        ) {
            Ok(true) => {}
            Ok(false) => {
                attempt.stale_handle_denied();
                return NamespaceDeletionOutcome::Failed(deletion_failure(
                    operation,
                    &path,
                    None,
                    Some(handle_identity),
                    MediaOperationFailureKind::DeniedBeforeEffect,
                    MediaCausalBoundary::OsCallReturned,
                ));
            }
            Err(error) => {
                attempt.denied();
                return NamespaceDeletionOutcome::Failed(deletion_failure(
                    operation,
                    &path,
                    Some(&error),
                    Some(handle_identity),
                    MediaOperationFailureKind::DeniedBeforeEffect,
                    MediaCausalBoundary::OsCallReturned,
                ));
            }
        }
        match parent.directory().remove_file(path.file_name()) {
            Ok(()) => {
                drop(open_file);
                self.boundary().counters().deletion();
                if attempt.effect_observation_is_indeterminate() {
                    attempt.indeterminate(0);
                    return NamespaceDeletionOutcome::Indeterminate(
                        IndeterminateNamespaceDeletion {
                            path: path.clone(),
                            failure: deletion_failure(
                                operation,
                                &path,
                                None,
                                Some(handle_identity),
                                MediaOperationFailureKind::IndeterminateEffect {
                                    attempted: MediaAttemptedEffect::NamespaceEntryDeletion,
                                    last_established:
                                        MediaEstablishedBoundary::NamespaceEntryDeletionIssued,
                                },
                                MediaCausalBoundary::CompletionUnconfirmed,
                            ),
                        },
                    );
                }
                attempt.completed(0);
                NamespaceDeletionOutcome::Removed(VisibleNamespaceDeletion {
                    owner: self,
                    path,
                    deletion_operation: operation,
                    parent,
                    _namespace_authority: namespace_authority,
                })
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                attempt.denied();
                NamespaceDeletionOutcome::Failed(deletion_failure(
                    operation,
                    &path,
                    Some(&error),
                    Some(handle_identity),
                    MediaOperationFailureKind::DeniedBeforeEffect,
                    MediaCausalBoundary::OsCallReturned,
                ))
            }
            Err(error) => {
                attempt.indeterminate(0);
                NamespaceDeletionOutcome::Indeterminate(IndeterminateNamespaceDeletion {
                    path: path.clone(),
                    failure: deletion_failure(
                        operation,
                        &path,
                        Some(&error),
                        Some(handle_identity),
                        MediaOperationFailureKind::IndeterminateEffect {
                            attempted: MediaAttemptedEffect::NamespaceEntryDeletion,
                            last_established:
                                MediaEstablishedBoundary::NamespaceEntryDeletionIssued,
                        },
                        MediaCausalBoundary::CompletionUnconfirmed,
                    ),
                })
            }
        }
    }
}

impl VisibleNamespaceDeletion<'_> {
    pub fn synchronize_removal(self) -> DurableDeletionOutcome {
        match self
            .owner
            .synchronize_coordinated_directory_publication(self.parent, &self._namespace_authority)
        {
            DirectoryPublicationSynchronizationOutcome::Synchronized(synchronization) => {
                DurableDeletionOutcome::Durable(DurableDeletion {
                    path: self.path,
                    deletion_operation: self.deletion_operation,
                    directory_synchronization: synchronization,
                })
            }
            DirectoryPublicationSynchronizationOutcome::Failed(failure) => {
                DurableDeletionOutcome::Indeterminate(IndeterminateNamespaceDeletion {
                    path: self.path,
                    failure,
                })
            }
        }
    }
}

impl DurableDeletion {
    pub const fn path(&self) -> &NamespaceRelativePath {
        &self.path
    }

    pub const fn deletion_operation(&self) -> super::MediaOperationIdentity {
        self.deletion_operation
    }

    pub const fn directory_synchronization(&self) -> super::DirectoryPublicationSynchronization {
        self.directory_synchronization
    }
}

impl IndeterminateNamespaceDeletion {
    pub const fn path(&self) -> &NamespaceRelativePath {
        &self.path
    }

    pub const fn failure(&self) -> MediaOperationFailure {
        self.failure
    }
}

fn deletion_failure(
    operation: super::MediaOperationIdentity,
    path: &NamespaceRelativePath,
    error: Option<&std::io::Error>,
    handle: Option<super::MediaHandleIdentity>,
    kind: MediaOperationFailureKind,
    boundary: MediaCausalBoundary,
) -> MediaOperationFailure {
    super::failure_context::operation_failure(
        operation,
        super::MediaOperationRole::Delete,
        path.role(),
        handle,
        kind,
        error,
        boundary,
    )
}
