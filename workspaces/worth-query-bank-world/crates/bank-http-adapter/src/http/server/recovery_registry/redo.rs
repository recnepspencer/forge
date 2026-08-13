use super::state::{BankHttpRedoAuthority, BankHttpRedoReplay, RecoveryState};
use super::BankHttpRecoveryRegistry;
use crate::http::protocol::{BankHttpCommitDescription, BankHttpCommitDisposition};
use crate::http::server::authenticated_owner::BankHttpAuthenticatedOwner;

impl BankHttpRecoveryRegistry {
    pub(in crate::http::server) fn take_redo(
        &mut self,
        owner: &BankHttpAuthenticatedOwner,
        token: &str,
    ) -> Option<BankHttpRedoAuthority> {
        let record = self.owned_record_mut(owner, token)?;
        match std::mem::replace(&mut record.state, RecoveryState::Terminal) {
            RecoveryState::RedoAvailable {
                recovery,
                undo_key,
                disposition,
                commit,
            } => Some(BankHttpRedoAuthority::new(
                recovery,
                undo_key,
                disposition,
                commit,
            )),
            state => {
                record.state = state;
                None
            }
        }
    }

    pub(in crate::http::server) fn restore_redo(
        &mut self,
        token: &str,
        authority: BankHttpRedoAuthority,
    ) {
        let (recovery, binding) = authority.into_parts();
        self.install_state(token, binding.into_state(recovery));
    }

    pub(in crate::http::server) fn install_redo_commit(
        &mut self,
        token: &str,
        disposition: BankHttpCommitDisposition,
        commit: BankHttpCommitDescription,
    ) {
        self.install_state(
            token,
            RecoveryState::RedoCommitted {
                disposition,
                commit,
            },
        );
    }

    pub(in crate::http::server) fn redo_replay(
        &mut self,
        owner: &BankHttpAuthenticatedOwner,
        token: &str,
    ) -> BankHttpRedoReplay {
        let Some(record) = self.owned_record_mut(owner, token) else {
            return BankHttpRedoReplay::Missing;
        };
        match &record.state {
            RecoveryState::RedoCommitted {
                disposition,
                commit,
            } => BankHttpRedoReplay::Applied {
                disposition: *disposition,
                commit: *commit,
            },
            _ => BankHttpRedoReplay::Missing,
        }
    }
}
