use crate::{RecoveryEntryAdmission, RecoveryEntryIdentity};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryReplayEntryGate {
    entry_identity: RecoveryEntryIdentity,
    replay_planning_started: bool,
    source_precedence_chosen: bool,
}

impl RecoveryReplayEntryGate {
    pub fn before_source_precedence(admission: RecoveryEntryAdmission) -> Self {
        Self {
            entry_identity: admission.entry_identity().clone(),
            replay_planning_started: false,
            source_precedence_chosen: false,
        }
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
}
