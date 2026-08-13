use std::time::{Duration, Instant};

use bank_domain::estate::EstateAction;
use bank_domain::proposals::BankIdempotencyKey;
use bank_server::{
    BankCommitRecoveryHandle, BankCompensationUndoAdmission, BankRecordedInverseUndoAdmission,
    BankRedoRecovery,
};

use super::super::super::protocol::{
    BankHttpCommitDescription, BankHttpCommitDisposition, BankHttpUndoCorrection,
};
use super::super::authenticated_owner::BankHttpAuthenticatedOwner;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub(super) enum RecoveryOrigin {
    Notification,
    Disbursement,
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub(super) struct CommitReplayKey {
    pub(super) owner: BankHttpAuthenticatedOwner,
    pub(super) origin: RecoveryOrigin,
    pub(super) idempotency_key: BankIdempotencyKey,
}

pub(super) struct RecoveryRecord {
    pub(super) owner: BankHttpAuthenticatedOwner,
    pub(super) origin: RecoveryOrigin,
    pub(super) action: EstateAction,
    pub(super) commit: BankHttpCommitDescription,
    pub(super) expires_at: Instant,
    pub(super) state: RecoveryState,
}

impl RecoveryRecord {
    pub(super) fn new(
        replay: CommitReplayKey,
        action: EstateAction,
        registration: BankHttpRecoveryRegistration,
        lifetime: Duration,
    ) -> Self {
        Self {
            owner: replay.owner,
            origin: replay.origin,
            action,
            commit: registration.commit,
            expires_at: Instant::now() + lifetime,
            state: RecoveryState::Recovery(registration.handle),
        }
    }
}

pub(super) enum RecoveryState {
    Recovery(BankCommitRecoveryHandle),
    RecordedInverseUndo {
        _admission: BankRecordedInverseUndoAdmission,
        correction: BankHttpUndoCorrection,
    },
    CompensationUndo(BankCompensationUndoAdmission),
    RedoAvailable {
        recovery: BankRedoRecovery,
        undo_key: BankIdempotencyKey,
        disposition: BankHttpCommitDisposition,
        commit: BankHttpCommitDescription,
    },
    Reconciled {
        undo_key: BankIdempotencyKey,
    },
    RedoCommitted {
        disposition: BankHttpCommitDisposition,
        commit: BankHttpCommitDescription,
    },
    Terminal,
}

pub(in crate::http::server) enum BankHttpCommitReplay {
    Missing,
    Applied {
        commit: BankHttpCommitDescription,
        recovery: String,
    },
    Conflicting,
}

pub(in crate::http::server) enum BankHttpRecoveryAuthority {
    Notification(BankCommitRecoveryHandle),
    Disbursement(BankCommitRecoveryHandle),
}

pub(in crate::http::server) enum BankHttpUndoReplay {
    Missing,
    Applied {
        disposition: BankHttpCommitDisposition,
        commit: BankHttpCommitDescription,
        redo: String,
    },
    Reconciled,
    Conflicting,
}

pub(in crate::http::server) enum BankHttpUndoAuthority {
    Compensation(BankCompensationUndoAdmission),
    RecordedInverse {
        admission: BankRecordedInverseUndoAdmission,
        correction: BankHttpUndoCorrection,
    },
}

pub(in crate::http::server) enum BankHttpRedoReplay {
    Missing,
    Applied {
        disposition: BankHttpCommitDisposition,
        commit: BankHttpCommitDescription,
    },
}

pub(in crate::http::server) struct BankHttpRedoAuthority {
    recovery: BankRedoRecovery,
    binding: BankHttpRedoBinding,
}

pub(in crate::http::server) struct BankHttpRedoBinding {
    undo_key: BankIdempotencyKey,
    disposition: BankHttpCommitDisposition,
    commit: BankHttpCommitDescription,
}

impl BankHttpRedoAuthority {
    pub(super) fn new(
        recovery: BankRedoRecovery,
        undo_key: BankIdempotencyKey,
        disposition: BankHttpCommitDisposition,
        commit: BankHttpCommitDescription,
    ) -> Self {
        Self {
            recovery,
            binding: BankHttpRedoBinding {
                undo_key,
                disposition,
                commit,
            },
        }
    }

    pub(in crate::http::server) fn into_parts(self) -> (BankRedoRecovery, BankHttpRedoBinding) {
        (self.recovery, self.binding)
    }
}

impl BankHttpRedoBinding {
    pub(in crate::http::server) fn bind(self, recovery: BankRedoRecovery) -> BankHttpRedoAuthority {
        BankHttpRedoAuthority {
            recovery,
            binding: self,
        }
    }

    pub(super) fn into_state(self, recovery: BankRedoRecovery) -> RecoveryState {
        RecoveryState::RedoAvailable {
            recovery,
            undo_key: self.undo_key,
            disposition: self.disposition,
            commit: self.commit,
        }
    }
}

pub(in crate::http::server) struct BankHttpRecoveryRegistration {
    pub(in crate::http::server) commit: BankHttpCommitDescription,
    pub(in crate::http::server) handle: BankCommitRecoveryHandle,
}
