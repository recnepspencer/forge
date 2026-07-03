use forge_proof::TransitionOutcome;
use forge_store_security::{deny_drifted_store_security_scope, StoreSecurityScopePropagationSite};

use crate::{
    RecoveryEntryAdmission, RecoveryEntryIdentity, RecoverySecurityScopePropagation,
    RecoverySecurityScopePropagationDenial,
};

pub type RecoveryReplayEntryGateOutcome =
    TransitionOutcome<RecoveryReplayEntryGate, RecoverySecurityScopePropagationDenial>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryReplayEntryGate {
    entry_identity: RecoveryEntryIdentity,
    security_scope: RecoverySecurityScopePropagation,
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
}
