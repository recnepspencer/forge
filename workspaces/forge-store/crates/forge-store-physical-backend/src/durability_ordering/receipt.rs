use crate::{BackendTargetProfile, CapabilityEvidenceClass, WalDurabilityBarrierSet};

use super::{
    StoreDurabilityCounterSnapshot, StoreDurabilityDenial, StoreDurabilityDenialKind,
    StoreDurabilityExecutionProof, StoreDurabilityFileSyncKind, StoreDurabilityOperation,
    StoreDurabilityPublicationKind, StoreDurabilityRequirement, StoreDurabilityState,
};

#[derive(Clone, Debug, Eq, PartialEq)]
struct StoreDurabilityReceiptCore<S> {
    scope: S,
    profile: BackendTargetProfile,
    evidence_class: CapabilityEvidenceClass,
    requirement: StoreDurabilityRequirement,
    completed_barriers: WalDurabilityBarrierSet,
    directory_sync_completed: bool,
    rename_completed: bool,
    ordering_barrier_completed: bool,
    counters: StoreDurabilityCounterSnapshot,
    persisted_artifact: Option<StoreDurabilityPersistedArtifact>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoreDurabilityPersistedArtifact {
    path: std::path::PathBuf,
    bytes: u64,
}

impl<S> StoreDurabilityReceiptCore<S> {
    const fn new(
        scope: S,
        profile: BackendTargetProfile,
        evidence_class: CapabilityEvidenceClass,
        requirement: StoreDurabilityRequirement,
        completed_barriers: WalDurabilityBarrierSet,
        directory_sync_completed: bool,
        rename_completed: bool,
        ordering_barrier_completed: bool,
        counters: StoreDurabilityCounterSnapshot,
    ) -> Self {
        Self {
            scope,
            profile,
            evidence_class,
            requirement,
            completed_barriers,
            directory_sync_completed,
            rename_completed,
            ordering_barrier_completed,
            counters,
            persisted_artifact: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoreDurabilityWriteSubmitted<S> {
    core: StoreDurabilityReceiptCore<S>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoreDurabilityWriteAccepted<S> {
    core: StoreDurabilityReceiptCore<S>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoreDurabilityBoundaryReached<S> {
    core: StoreDurabilityReceiptCore<S>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoreDurabilityParentNamespaceDurable<S> {
    core: StoreDurabilityReceiptCore<S>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoreDurabilityRenameDurable<S> {
    core: StoreDurabilityReceiptCore<S>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoreDurabilityOrderingBarrierDurable<S> {
    core: StoreDurabilityReceiptCore<S>,
}

impl<S> StoreDurabilityWriteSubmitted<S> {
    pub(crate) const fn new(
        scope: S,
        profile: BackendTargetProfile,
        evidence_class: CapabilityEvidenceClass,
        requirement: StoreDurabilityRequirement,
        completed_barriers: WalDurabilityBarrierSet,
        counters: StoreDurabilityCounterSnapshot,
    ) -> Self {
        Self {
            core: StoreDurabilityReceiptCore::new(
                scope,
                profile,
                evidence_class,
                requirement,
                completed_barriers,
                false,
                false,
                false,
                counters,
            ),
        }
    }

    pub fn backend_accepted(self) -> StoreDurabilityWriteAccepted<S> {
        StoreDurabilityWriteAccepted {
            core: StoreDurabilityReceiptCore {
                counters: self.core.counters.with_write_accepted(),
                ..self.core
            },
        }
    }

    pub const fn state(&self) -> StoreDurabilityState {
        StoreDurabilityState::WriteSubmitted
    }
}

impl<S> StoreDurabilityWriteAccepted<S>
where
    S: Eq,
{
    pub fn reach_durability_boundary(
        self,
        execution: StoreDurabilityExecutionProof<S>,
    ) -> Result<StoreDurabilityBoundaryReached<S>, StoreDurabilityDenial> {
        if !execution.binds_accepted(&self) {
            return Err(StoreDurabilityDenial::new(
                StoreDurabilityDenialKind::ExecutionBindingMismatch,
                StoreDurabilityState::Denied,
                operation_for(self.core.requirement),
                self.core.profile,
                CapabilityEvidenceClass::CertifiedBackendProfile,
                self.core.evidence_class,
                self.core.counters.with_denied_claim(),
            ));
        }
        if execution.failed_syncs() > 0 {
            return Err(StoreDurabilityDenial::new(
                StoreDurabilityDenialKind::FailedSync,
                StoreDurabilityState::Denied,
                StoreDurabilityOperation::Flush,
                self.core.profile,
                CapabilityEvidenceClass::CertifiedBackendProfile,
                self.core.evidence_class,
                self.core
                    .counters
                    .with_failed_syncs(execution.failed_syncs()),
            ));
        }
        if !file_sync_satisfies(
            execution.file_sync(),
            self.core.requirement.required_file_sync(),
        ) {
            return Err(StoreDurabilityDenial::new(
                StoreDurabilityDenialKind::MissingMediaAssumption,
                StoreDurabilityState::Denied,
                operation_for(self.core.requirement),
                self.core.profile,
                CapabilityEvidenceClass::CertifiedBackendProfile,
                self.core.evidence_class,
                self.core.counters.with_denied_claim(),
            ));
        }
        let completed_barriers = execution.completed_barriers();
        if !completed_barriers.satisfies(self.core.requirement.required_barriers()) {
            let missing = completed_barriers
                .first_missing_from(self.core.requirement.required_barriers())
                .expect("required barriers are not satisfied");
            return Err(StoreDurabilityDenial::new(
                StoreDurabilityDenialKind::MissingRequiredBarrier,
                StoreDurabilityState::Denied,
                operation_for(self.core.requirement),
                self.core.profile,
                CapabilityEvidenceClass::CertifiedBackendProfile,
                self.core.evidence_class,
                self.core.counters.with_denied_claim(),
            )
            .with_missing_barrier(missing));
        }
        Ok(StoreDurabilityBoundaryReached {
            core: StoreDurabilityReceiptCore {
                completed_barriers,
                directory_sync_completed: execution.directory_sync_completed(),
                rename_completed: execution.rename_completed(),
                ordering_barrier_completed: execution.ordering_barrier_completed(),
                counters: execution.apply_boundary_counters(self.core.counters),
                persisted_artifact: Some(StoreDurabilityPersistedArtifact {
                    path: execution.persisted_path().to_path_buf(),
                    bytes: execution.persisted_bytes(),
                }),
                ..self.core
            },
        })
    }
}

impl<S> StoreDurabilityWriteAccepted<S> {
    pub const fn state(&self) -> StoreDurabilityState {
        StoreDurabilityState::WriteAcceptedByBackend
    }
}

const fn operation_for(requirement: StoreDurabilityRequirement) -> StoreDurabilityOperation {
    match requirement.publication() {
        StoreDurabilityPublicationKind::WalFrame => StoreDurabilityOperation::WalPublication,
        StoreDurabilityPublicationKind::Checkpoint => {
            StoreDurabilityOperation::CheckpointPublication
        }
        StoreDurabilityPublicationKind::Manifest => StoreDurabilityOperation::ManifestPublication,
    }
}

const fn file_sync_satisfies(
    actual: StoreDurabilityFileSyncKind,
    required: StoreDurabilityFileSyncKind,
) -> bool {
    matches!(
        (actual, required),
        (
            StoreDurabilityFileSyncKind::Fsync,
            StoreDurabilityFileSyncKind::Fsync
        ) | (
            StoreDurabilityFileSyncKind::Fsync,
            StoreDurabilityFileSyncKind::Fdatasync
        ) | (
            StoreDurabilityFileSyncKind::Fdatasync,
            StoreDurabilityFileSyncKind::Fdatasync
        )
    )
}

impl<S> StoreDurabilityBoundaryReached<S> {
    pub fn parent_namespace_durable(
        self,
    ) -> Result<StoreDurabilityParentNamespaceDurable<S>, StoreDurabilityDenial> {
        if !self.core.directory_sync_completed {
            return Err(missing_completed_step_denial(
                StoreDurabilityDenialKind::FailedSync,
                StoreDurabilityOperation::DirectorySync,
                &self.core,
            ));
        }
        Ok(StoreDurabilityParentNamespaceDurable {
            core: StoreDurabilityReceiptCore {
                counters: self.core.counters.with_directory_sync_completed(),
                ..self.core
            },
        })
    }

    pub fn ordering_barrier_durable(
        self,
    ) -> Result<StoreDurabilityOrderingBarrierDurable<S>, StoreDurabilityDenial> {
        if self.core.requirement.requires_parent_namespace_durable()
            || self.core.requirement.requires_rename_durable()
        {
            return Err(missing_completed_step_denial(
                StoreDurabilityDenialKind::MissingRequiredBarrier,
                StoreDurabilityOperation::DirectorySync,
                &self.core,
            ));
        }
        if !self.core.ordering_barrier_completed {
            return Err(missing_completed_step_denial(
                StoreDurabilityDenialKind::FailedSync,
                StoreDurabilityOperation::Flush,
                &self.core,
            ));
        }
        Ok(StoreDurabilityOrderingBarrierDurable {
            core: StoreDurabilityReceiptCore {
                counters: self.core.counters.with_ordering_barrier_completed(),
                ..self.core
            },
        })
    }

    pub const fn state(&self) -> StoreDurabilityState {
        StoreDurabilityState::WriteReachedDurabilityBoundary
    }
}

impl<S> StoreDurabilityParentNamespaceDurable<S> {
    pub fn rename_durable(self) -> Result<StoreDurabilityRenameDurable<S>, StoreDurabilityDenial> {
        if !self.core.rename_completed {
            return Err(missing_completed_step_denial(
                StoreDurabilityDenialKind::FailedSync,
                StoreDurabilityOperation::Rename,
                &self.core,
            ));
        }
        Ok(StoreDurabilityRenameDurable {
            core: StoreDurabilityReceiptCore {
                counters: self.core.counters.with_rename_completed(),
                ..self.core
            },
        })
    }

    pub fn ordering_barrier_durable(
        self,
    ) -> Result<StoreDurabilityOrderingBarrierDurable<S>, StoreDurabilityDenial> {
        if self.core.requirement.requires_rename_durable() {
            return Err(missing_completed_step_denial(
                StoreDurabilityDenialKind::MissingRequiredBarrier,
                StoreDurabilityOperation::Rename,
                &self.core,
            ));
        }
        if !self.core.ordering_barrier_completed {
            return Err(missing_completed_step_denial(
                StoreDurabilityDenialKind::FailedSync,
                StoreDurabilityOperation::Flush,
                &self.core,
            ));
        }
        Ok(StoreDurabilityOrderingBarrierDurable {
            core: StoreDurabilityReceiptCore {
                counters: self.core.counters.with_ordering_barrier_completed(),
                ..self.core
            },
        })
    }

    pub const fn state(&self) -> StoreDurabilityState {
        StoreDurabilityState::ParentNamespaceDurable
    }
}

impl<S> StoreDurabilityRenameDurable<S> {
    pub fn ordering_barrier_durable(
        self,
    ) -> Result<StoreDurabilityOrderingBarrierDurable<S>, StoreDurabilityDenial> {
        if !self.core.ordering_barrier_completed {
            return Err(missing_completed_step_denial(
                StoreDurabilityDenialKind::FailedSync,
                StoreDurabilityOperation::Flush,
                &self.core,
            ));
        }
        Ok(StoreDurabilityOrderingBarrierDurable {
            core: StoreDurabilityReceiptCore {
                counters: self.core.counters.with_ordering_barrier_completed(),
                ..self.core
            },
        })
    }

    pub const fn state(&self) -> StoreDurabilityState {
        StoreDurabilityState::RenameDurable
    }
}

fn missing_completed_step_denial<S>(
    kind: StoreDurabilityDenialKind,
    operation: StoreDurabilityOperation,
    core: &StoreDurabilityReceiptCore<S>,
) -> StoreDurabilityDenial {
    StoreDurabilityDenial::new(
        kind,
        StoreDurabilityState::Denied,
        operation,
        core.profile,
        CapabilityEvidenceClass::CertifiedBackendProfile,
        core.evidence_class,
        core.counters.with_denied_claim(),
    )
}

impl<S> StoreDurabilityOrderingBarrierDurable<S> {
    pub const fn state(&self) -> StoreDurabilityState {
        StoreDurabilityState::OrderingBarrierDurable
    }

    pub fn persisted_artifact(&self) -> &StoreDurabilityPersistedArtifact {
        self.core
            .persisted_artifact
            .as_ref()
            .expect("ordering-barrier durability retains the executed artifact")
    }
}

impl StoreDurabilityPersistedArtifact {
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    pub const fn bytes(&self) -> u64 {
        self.bytes
    }
}

#[path = "receipt_accessors.rs"]
mod accessors;
