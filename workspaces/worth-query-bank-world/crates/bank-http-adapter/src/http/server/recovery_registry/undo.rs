use bank_domain::proposals::BankIdempotencyKey;
use bank_server::{
    BankCompensationUndoAdmission, BankRecordedInverseUndoAdmission, BankRedoRecovery,
};

use super::state::{BankHttpUndoAuthority, BankHttpUndoReplay, RecoveryState};
use super::BankHttpRecoveryRegistry;
use crate::http::protocol::{
    BankHttpCommitDescription, BankHttpCommitDisposition, BankHttpUndoCorrection,
};
use crate::http::server::authenticated_owner::BankHttpAuthenticatedOwner;

impl BankHttpRecoveryRegistry {
    pub(in crate::http::server) fn undo_admission_replay(
        &mut self,
        owner: &BankHttpAuthenticatedOwner,
        token: &str,
    ) -> Option<BankHttpUndoCorrection> {
        let record = self.owned_record_mut(owner, token)?;
        match &record.state {
            RecoveryState::RecordedInverseUndo { correction, .. } => Some(*correction),
            RecoveryState::CompensationUndo(_) => Some(BankHttpUndoCorrection::Compensation),
            _ => None,
        }
    }

    pub(in crate::http::server) fn install_recorded_inverse_undo(
        &mut self,
        token: &str,
        admission: BankRecordedInverseUndoAdmission,
        correction: BankHttpUndoCorrection,
    ) {
        self.install_state(
            token,
            RecoveryState::RecordedInverseUndo {
                _admission: admission,
                correction,
            },
        );
    }

    pub(in crate::http::server) fn install_compensation_undo(
        &mut self,
        token: &str,
        admission: BankCompensationUndoAdmission,
    ) {
        self.install_state(token, RecoveryState::CompensationUndo(admission));
    }

    pub(in crate::http::server) fn take_undo(
        &mut self,
        owner: &BankHttpAuthenticatedOwner,
        token: &str,
    ) -> Option<BankHttpUndoAuthority> {
        let record = self.owned_record_mut(owner, token)?;
        match std::mem::replace(&mut record.state, RecoveryState::Terminal) {
            RecoveryState::CompensationUndo(admission) => {
                Some(BankHttpUndoAuthority::Compensation(admission))
            }
            RecoveryState::RecordedInverseUndo {
                _admission: admission,
                correction,
            } => Some(BankHttpUndoAuthority::RecordedInverse {
                admission,
                correction,
            }),
            state => {
                record.state = state;
                None
            }
        }
    }

    pub(in crate::http::server) fn restore_undo(
        &mut self,
        token: &str,
        authority: BankHttpUndoAuthority,
    ) {
        let state = match authority {
            BankHttpUndoAuthority::Compensation(admission) => {
                RecoveryState::CompensationUndo(admission)
            }
            BankHttpUndoAuthority::RecordedInverse {
                admission,
                correction,
            } => RecoveryState::RecordedInverseUndo {
                _admission: admission,
                correction,
            },
        };
        self.install_state(token, state);
    }

    pub(in crate::http::server) fn undo_replay(
        &mut self,
        owner: &BankHttpAuthenticatedOwner,
        token: &str,
        key: &BankIdempotencyKey,
    ) -> BankHttpUndoReplay {
        let Some(record) = self.owned_record_mut(owner, token) else {
            return BankHttpUndoReplay::Missing;
        };
        match &record.state {
            RecoveryState::RedoAvailable {
                undo_key,
                disposition,
                commit,
                ..
            } if undo_key == key => BankHttpUndoReplay::Applied {
                disposition: *disposition,
                commit: *commit,
                redo: token.to_owned(),
            },
            RecoveryState::Reconciled { undo_key } if undo_key == key => {
                BankHttpUndoReplay::Reconciled
            }
            RecoveryState::Reconciled { .. } | RecoveryState::RedoAvailable { .. } => {
                BankHttpUndoReplay::Conflicting
            }
            _ => BankHttpUndoReplay::Missing,
        }
    }

    pub(in crate::http::server) fn install_reconciled(
        &mut self,
        token: &str,
        key: BankIdempotencyKey,
    ) {
        self.install_state(token, RecoveryState::Reconciled { undo_key: key });
    }

    pub(in crate::http::server) fn install_redo(
        &mut self,
        token: &str,
        key: BankIdempotencyKey,
        disposition: BankHttpCommitDisposition,
        commit: BankHttpCommitDescription,
        recovery: BankRedoRecovery,
    ) {
        self.install_state(
            token,
            RecoveryState::RedoAvailable {
                recovery,
                undo_key: key,
                disposition,
                commit,
            },
        );
    }
}
