use forge_proof::TransitionOutcome;
use forge_store_security::{deny_drifted_store_security_scope, StoreSecurityScopePropagationSite};

use crate::{
    PartialPublicationBeforeWalReplayRead, PartialPublicationPersistedBytes,
    PartialPublicationReplayReadArtifact, PartialPublicationReplayReadDenial,
    PartialPublicationReplayReadRecord, RecoveryEntryAdmission, RecoveryEntryIdentity,
    RecoverySecurityScopePropagation, RecoverySecurityScopePropagationDenial,
};

pub type RecoveryReplayEntryGateOutcome =
    TransitionOutcome<RecoveryReplayEntryGate, RecoverySecurityScopePropagationDenial>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryReplayEntryGate {
    entry_identity: RecoveryEntryIdentity,
    security_scope: RecoverySecurityScopePropagation,
    partial_publication_before_wal_replay_read: Option<PartialPublicationBeforeWalReplayRead>,
    replay_planning_started: bool,
    source_precedence_chosen: bool,
}

impl RecoveryReplayEntryGate {
    pub fn before_source_precedence(
        admission: RecoveryEntryAdmission,
        security_scope: RecoverySecurityScopePropagation,
    ) -> RecoveryReplayEntryGateOutcome {
        if admission.entry_identity() != security_scope.entry_identity() {
            return TransitionOutcome::denied(
                RecoverySecurityScopePropagationDenial::from_store_denial(
                    deny_drifted_store_security_scope(
                        StoreSecurityScopePropagationSite::RecoveryAdmission,
                    ),
                ),
            );
        }

        TransitionOutcome::success(Self {
            entry_identity: admission.entry_identity().clone(),
            partial_publication_before_wal_replay_read: admission
                .recovery_basis()
                .partial_publication_before_wal_replay_read()
                .cloned(),
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

    pub fn read_partial_publication_checkpoint_cutover(
        &self,
        checkpoint_digest: impl Into<String>,
    ) -> PartialPublicationReplayReadRecord {
        let bytes = PartialPublicationPersistedBytes::during_checkpoint_cutover(checkpoint_digest);
        let artifact = PartialPublicationReplayReadArtifact::from_replay_entry_gate(self, bytes);
        PartialPublicationReplayReadRecord::from_replay_read_artifact(artifact)
    }

    pub fn read_partial_publication_before_wal_append(
        &self,
    ) -> Result<PartialPublicationReplayReadArtifact, PartialPublicationReplayReadDenial> {
        let Some(replay_read) = self.partial_publication_before_wal_replay_read.clone() else {
            return Err(PartialPublicationReplayReadDenial::NotBeforeWalAppend {
                actual_operation_digest: None,
            });
        };
        Ok(
            PartialPublicationReplayReadArtifact::from_replay_entry_gate(
                self,
                replay_read.into_persisted_bytes(),
            ),
        )
    }
}
