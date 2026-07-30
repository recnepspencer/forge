use worth_proof::TransitionOutcome;
use worth_store_security::{deny_drifted_store_security_scope, StoreSecurityScopePropagationSite};

use crate::{
    PartialPublicationBeforeWalReplayRead, PartialPublicationPersistedBytes,
    PartialPublicationReplayReadArtifact, PartialPublicationReplayReadDenial,
    PartialPublicationReplayReadRecord, RecoveryEntryAdmission, RecoveryEntryIdentity,
    RecoverySecurityScopePropagation, RecoverySecurityScopePropagationDenial,
};

pub type RecoveryReplayEntryGateOutcome<'runtime> =
    TransitionOutcome<RecoveryReplayEntryGate<'runtime>, RecoverySecurityScopePropagationDenial>;

#[derive(Debug)]
pub struct RecoveryReplayEntryGate<'runtime> {
    entry_identity: RecoveryEntryIdentity,
    security_scope: RecoverySecurityScopePropagation,
    memory_allocation: crate::RecoveryMemoryAllocation<'runtime>,
    partial_publication_before_wal_replay_read: Option<PartialPublicationBeforeWalReplayRead>,
    replay_planning_started: bool,
    source_precedence_chosen: bool,
}

impl<'runtime> RecoveryReplayEntryGate<'runtime> {
    pub fn before_source_precedence(
        admission: RecoveryEntryAdmission<'runtime>,
        security_scope: RecoverySecurityScopePropagation,
    ) -> RecoveryReplayEntryGateOutcome<'runtime> {
        if admission.entry_identity() != security_scope.entry_identity() {
            return TransitionOutcome::denied(
                RecoverySecurityScopePropagationDenial::from_store_denial(
                    deny_drifted_store_security_scope(
                        StoreSecurityScopePropagationSite::RecoveryAdmission,
                    ),
                ),
            );
        }

        let entry_identity = admission.entry_identity().clone();
        let partial_publication_before_wal_replay_read = admission
            .recovery_basis()
            .partial_publication_before_wal_replay_read()
            .cloned();
        let memory_allocation = admission.into_memory_allocation();

        TransitionOutcome::success(Self {
            entry_identity,
            memory_allocation,
            partial_publication_before_wal_replay_read,
            security_scope,
            replay_planning_started: false,
            source_precedence_chosen: false,
        })
    }

    pub const fn entry_identity(&self) -> &RecoveryEntryIdentity {
        &self.entry_identity
    }

    pub const fn replay_planning_started(&self) -> bool {
        self.replay_planning_started
    }

    pub const fn source_precedence_chosen(&self) -> bool {
        self.source_precedence_chosen
    }

    pub const fn security_scope(&self) -> &RecoverySecurityScopePropagation {
        &self.security_scope
    }

    pub const fn memory_allocation(&self) -> crate::RecoveryMemoryObservation {
        self.memory_allocation.observation()
    }

    pub fn read_partial_publication_checkpoint_cutover(
        &self,
        checkpoint_digest: impl Into<String>,
    ) -> Result<PartialPublicationReplayReadRecord, PartialPublicationReplayReadDenial> {
        let bytes = PartialPublicationPersistedBytes::during_checkpoint_cutover(checkpoint_digest);
        let artifact = PartialPublicationReplayReadArtifact::from_replay_entry_gate(self, bytes)?;
        Ok(PartialPublicationReplayReadRecord::from_replay_read_artifact(artifact))
    }

    pub fn read_partial_publication_before_wal_append(
        &self,
    ) -> Result<PartialPublicationReplayReadArtifact, PartialPublicationReplayReadDenial> {
        let Some(replay_read) = self.partial_publication_before_wal_replay_read.clone() else {
            return Err(PartialPublicationReplayReadDenial::NotBeforeWalAppend {
                actual_operation_digest: None,
            });
        };
        Ok(replay_read.into_replay_read_artifact(self.entry_identity().clone()))
    }
}
