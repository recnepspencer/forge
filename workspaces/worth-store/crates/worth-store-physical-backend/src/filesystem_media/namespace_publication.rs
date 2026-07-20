use super::{
    DirectoryPublicationSynchronizationOutcome, FilesystemMediaOwner, MediaAttemptedEffect,
    MediaCausalBoundary, MediaEstablishedBoundary, MediaOperationFailure,
    MediaOperationFailureKind, NamespaceFileOpenResult, NamespaceRelativePath,
};

pub use super::namespace_publication_state::{
    AtomicReplacementOutcome, CompletedAtomicReplacement, CompletedStagedNamespaceWrite,
    DurableNamespacePublicationOutcome, DurablyPublishedNamespaceFile,
    IndeterminateNamespacePublication, NamespacePublicationStage, StagedNamespaceFile,
    StagedNamespaceFileOutcome, StagedNamespaceSynchronizationOutcome, StagedNamespaceWriteOutcome,
    SynchronizedStagedNamespaceFile,
};

impl<'owner> StagedNamespaceFile<'owner> {
    pub fn create(
        owner: &'owner FilesystemMediaOwner,
        staged_path: super::StagedNamespacePath,
    ) -> StagedNamespaceFileOutcome<'owner> {
        let path = staged_path.into_relative();
        let outcome = owner.create_new(&path);
        let create_operation = outcome.operation();
        match outcome.into_result() {
            NamespaceFileOpenResult::Opened { handle, .. } => {
                StagedNamespaceFileOutcome::Created(Self {
                    owner,
                    path,
                    handle,
                    create_operation,
                })
            }
            NamespaceFileOpenResult::Failed(failure) => StagedNamespaceFileOutcome::Failed(failure),
        }
    }
}

impl<'owner> CompletedStagedNamespaceWrite<'owner> {
    pub const fn bytes(&self) -> u64 {
        self.write.bytes()
    }

    pub fn synchronize(self) -> StagedNamespaceSynchronizationOutcome<'owner> {
        match self.staged.handle.synchronize_state() {
            super::FileStateSynchronizationOutcome::Synchronized(synchronization) => {
                StagedNamespaceSynchronizationOutcome::Synchronized(
                    SynchronizedStagedNamespaceFile {
                        completed: self,
                        synchronization,
                    },
                )
            }
            super::FileStateSynchronizationOutcome::Failed(failure) => {
                StagedNamespaceSynchronizationOutcome::Failed {
                    completed: self,
                    failure,
                }
            }
        }
    }
}

impl<'owner> SynchronizedStagedNamespaceFile<'owner> {
    pub const fn synchronization(&self) -> super::FileStateSynchronization {
        self.synchronization
    }

    pub fn replace(
        self,
        target: super::NamespacePublicationTarget,
    ) -> AtomicReplacementOutcome<'owner> {
        let destination = target.into_relative();
        let owner = self.completed.staged.owner;
        let operation = owner
            .issue_operation_identity()
            .expect("media operation identity exhausted");
        let attempt = owner.boundary().begin_operation(
            super::MediaOperationRole::AtomicReplace,
            0,
            super::MediaOperationCoordinates::for_path(
                operation,
                destination.role(),
                Some(self.completed.staged.handle.identity()),
            )
            .at_publication_stage(NamespacePublicationStage::AtomicReplacement),
        );
        if let Some(error) = attempt.fail_before_error() {
            attempt.denied();
            return AtomicReplacementOutcome::Denied(super::failure_context::operation_failure(
                operation,
                super::MediaOperationRole::AtomicReplace,
                destination.role(),
                Some(self.completed.staged.handle.identity()),
                MediaOperationFailureKind::DeniedBeforeEffect,
                Some(&error),
                MediaCausalBoundary::BeforeOsCall,
            ));
        }
        let source_parent = owner.directory_for_path(&self.completed.staged.path);
        let destination_parent = owner.directory_for_path(&destination);
        let same_parent = source_parent
            .zip(destination_parent)
            .is_some_and(|(source, destination)| source.identity() == destination.identity());
        if !same_parent {
            attempt.confinement_denied();
            return AtomicReplacementOutcome::Denied(super::failure_context::operation_failure(
                operation,
                super::MediaOperationRole::AtomicReplace,
                destination.role(),
                Some(self.completed.staged.handle.identity()),
                MediaOperationFailureKind::DeniedBeforeEffect,
                None,
                MediaCausalBoundary::BeforeOsCall,
            ));
        }
        let namespace_authority = match owner.begin_namespace_mutation() {
            Ok(authority) => authority,
            Err(_) => {
                attempt.denied();
                return AtomicReplacementOutcome::Denied(
                    super::failure_context::operation_failure(
                        operation,
                        super::MediaOperationRole::AtomicReplace,
                        destination.role(),
                        Some(self.completed.staged.handle.identity()),
                        MediaOperationFailureKind::DeniedBeforeEffect,
                        None,
                        MediaCausalBoundary::BeforeOsCall,
                    ),
                );
            }
        };
        let parent = source_parent.expect("same-parent validation established a parent");
        match super::named_file_identity::validates_coordinated_name(
            parent,
            self.completed.staged.path.file_name(),
            self.completed.staged.handle.stable_file(),
        ) {
            Ok(true) => {}
            Ok(false) => {
                attempt.stale_handle_denied();
                return AtomicReplacementOutcome::Denied(
                    super::failure_context::operation_failure(
                        operation,
                        super::MediaOperationRole::AtomicReplace,
                        destination.role(),
                        Some(self.completed.staged.handle.identity()),
                        MediaOperationFailureKind::DeniedBeforeEffect,
                        None,
                        MediaCausalBoundary::OsCallReturned,
                    ),
                );
            }
            Err(error) => {
                attempt.denied();
                return AtomicReplacementOutcome::Denied(
                    super::failure_context::operation_failure(
                        operation,
                        super::MediaOperationRole::AtomicReplace,
                        destination.role(),
                        Some(self.completed.staged.handle.identity()),
                        MediaOperationFailureKind::DeniedBeforeEffect,
                        Some(&error),
                        MediaCausalBoundary::OsCallReturned,
                    ),
                );
            }
        }
        match parent.directory().rename(
            self.completed.staged.path.file_name(),
            parent.directory(),
            destination.file_name(),
        ) {
            Ok(()) if attempt.effect_observation_is_indeterminate() => {
                owner.boundary().counters().replacement();
                attempt.indeterminate(0);
                AtomicReplacementOutcome::Indeterminate(IndeterminateNamespacePublication::new(
                    owner,
                    NamespacePublicationStage::AtomicReplacement,
                    super::failure_context::operation_failure(
                        operation,
                        super::MediaOperationRole::AtomicReplace,
                        destination.role(),
                        Some(self.completed.staged.handle.identity()),
                        MediaOperationFailureKind::IndeterminateEffect {
                            attempted: MediaAttemptedEffect::AtomicReplacement,
                            last_established: MediaEstablishedBoundary::AtomicReplacementIssued,
                        },
                        None,
                        MediaCausalBoundary::CompletionUnconfirmed,
                    ),
                ))
            }
            Ok(()) => {
                owner.boundary().counters().replacement();
                attempt.completed(0);
                AtomicReplacementOutcome::Replaced(CompletedAtomicReplacement {
                    owner,
                    destination,
                    create_operation: self.completed.staged.create_operation,
                    write: self.completed.write,
                    file_state_synchronization: self.synchronization,
                    rename_operation: operation,
                    _namespace_authority: namespace_authority,
                })
            }
            Err(error) => {
                attempt.indeterminate(0);
                AtomicReplacementOutcome::Indeterminate(IndeterminateNamespacePublication::new(
                    owner,
                    NamespacePublicationStage::AtomicReplacement,
                    super::failure_context::operation_failure(
                        operation,
                        super::MediaOperationRole::AtomicReplace,
                        destination.role(),
                        Some(self.completed.staged.handle.identity()),
                        MediaOperationFailureKind::IndeterminateEffect {
                            attempted: MediaAttemptedEffect::AtomicReplacement,
                            last_established: MediaEstablishedBoundary::AtomicReplacementIssued,
                        },
                        Some(&error),
                        MediaCausalBoundary::CompletionUnconfirmed,
                    ),
                ))
            }
        }
    }
}

impl<'owner> CompletedAtomicReplacement<'owner> {
    pub fn synchronize_publication(self) -> DurableNamespacePublicationOutcome<'owner> {
        let directory_synchronization =
            match self.owner.synchronize_coordinated_directory_publication(
                self.owner.namespace_directory(),
                &self._namespace_authority,
            ) {
                DirectoryPublicationSynchronizationOutcome::Synchronized(synchronization) => {
                    synchronization
                }
                DirectoryPublicationSynchronizationOutcome::Failed(failure) => {
                    return DurableNamespacePublicationOutcome::Indeterminate(
                        IndeterminateNamespacePublication::new(
                            self.owner,
                            NamespacePublicationStage::DirectoryPublicationSynchronization,
                            failure,
                        ),
                    );
                }
            };
        let store_root_synchronization = match self
            .owner
            .synchronize_coordinated_store_root_publication(&self._namespace_authority)
        {
            super::StoreRootPublicationSynchronizationOutcome::NotRequired => None,
            super::StoreRootPublicationSynchronizationOutcome::Synchronized(synchronization) => {
                Some(synchronization)
            }
            super::StoreRootPublicationSynchronizationOutcome::Failed(failure) => {
                return DurableNamespacePublicationOutcome::Indeterminate(
                    IndeterminateNamespacePublication::new(
                        self.owner,
                        NamespacePublicationStage::StoreRootPublicationSynchronization,
                        failure,
                    ),
                );
            }
        };
        let root_parent_synchronization = match self
            .owner
            .synchronize_coordinated_created_root_parent(&self._namespace_authority)
        {
            super::RootParentPublicationSynchronizationOutcome::NotRequired => None,
            super::RootParentPublicationSynchronizationOutcome::Synchronized(parent) => {
                Some(parent)
            }
            super::RootParentPublicationSynchronizationOutcome::Failed(failure) => {
                return DurableNamespacePublicationOutcome::Indeterminate(
                    IndeterminateNamespacePublication::new(
                        self.owner,
                        NamespacePublicationStage::RootParentPublicationSynchronization,
                        failure,
                    ),
                );
            }
        };
        DurableNamespacePublicationOutcome::Published(DurablyPublishedNamespaceFile {
            destination: self.destination,
            summary: super::NamespacePublicationSummary::new(
                self.create_operation,
                self.write,
                self.file_state_synchronization,
                self.rename_operation,
                directory_synchronization,
                store_root_synchronization,
                root_parent_synchronization,
            ),
        })
    }
}

impl DurablyPublishedNamespaceFile {
    pub const fn destination(&self) -> &NamespaceRelativePath {
        &self.destination
    }

    pub const fn rename_operation(&self) -> super::MediaOperationIdentity {
        self.summary.rename_operation()
    }

    pub const fn directory_synchronization(&self) -> super::DirectoryPublicationSynchronization {
        self.summary.namespace_directory_synchronization()
    }

    pub const fn store_root_synchronization(
        &self,
    ) -> Option<super::StoreRootPublicationSynchronization> {
        self.summary.store_root_synchronization()
    }

    pub const fn root_parent_synchronization(
        &self,
    ) -> Option<super::RootParentPublicationSynchronization> {
        self.summary.root_parent_synchronization()
    }

    pub const fn summary(&self) -> super::NamespacePublicationSummary {
        self.summary
    }
}

impl IndeterminateNamespacePublication<'_> {
    pub const fn stage(&self) -> NamespacePublicationStage {
        self.stage
    }

    pub const fn failure(&self) -> MediaOperationFailure {
        self.failure
    }

    pub const fn owner_identity(&self) -> super::MediaOwnerIdentity {
        self.owner.identity()
    }
}
