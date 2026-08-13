use std::collections::HashMap;
use std::time::{Duration, Instant};

use bank_domain::estate::EstateAction;
use bank_server::BankCommitRecoveryHandle;
use rand::distributions::{Alphanumeric, DistString};
use rand::rngs::OsRng;

use super::authenticated_owner::BankHttpAuthenticatedOwner;

mod commit;
mod redo;
mod state;
mod undo;

pub(super) use state::{
    BankHttpCommitReplay, BankHttpRecoveryAuthority, BankHttpRecoveryRegistration,
    BankHttpRedoBinding, BankHttpRedoReplay, BankHttpUndoAuthority, BankHttpUndoReplay,
};
use state::{CommitReplayKey, RecoveryOrigin, RecoveryRecord, RecoveryState};

const TOKEN_PREFIX: &str = "bank-recovery-v1_";

pub(super) struct BankHttpRecoveryRegistry {
    records: HashMap<String, RecoveryRecord>,
    replay_tokens: HashMap<CommitReplayKey, String>,
    capacity: usize,
    lifetime: Duration,
}

pub(super) struct BankHttpRecoveryInspection<'registry> {
    handle: &'registry BankCommitRecoveryHandle,
    action: EstateAction,
}

impl BankHttpRecoveryInspection<'_> {
    pub(super) const fn handle(&self) -> &BankCommitRecoveryHandle {
        self.handle
    }

    pub(super) const fn action(&self) -> EstateAction {
        self.action
    }
}

impl BankHttpRecoveryRegistry {
    pub(super) fn recognizes_token(token: &str) -> bool {
        token.strip_prefix(TOKEN_PREFIX).is_some_and(|random| {
            random.len() == 40 && random.bytes().all(|byte| byte.is_ascii_alphanumeric())
        })
    }

    pub(super) fn new(capacity: usize, lifetime: Duration) -> Self {
        Self {
            records: HashMap::with_capacity(capacity),
            replay_tokens: HashMap::with_capacity(capacity),
            capacity,
            lifetime,
        }
    }

    pub(super) fn recovery(
        &mut self,
        owner: &BankHttpAuthenticatedOwner,
        token: &str,
    ) -> Option<BankHttpRecoveryInspection<'_>> {
        self.purge_expired();
        let record = self.records.get(token)?;
        (&record.owner == owner).then_some(())?;
        match &record.state {
            RecoveryState::Recovery(handle) => Some(BankHttpRecoveryInspection {
                handle,
                action: record.action,
            }),
            _ => None,
        }
    }

    pub(super) fn take_recovery(
        &mut self,
        owner: &BankHttpAuthenticatedOwner,
        token: &str,
    ) -> Option<BankHttpRecoveryAuthority> {
        let record = self.owned_record_mut(owner, token)?;
        match std::mem::replace(&mut record.state, RecoveryState::Terminal) {
            RecoveryState::Recovery(handle) => Some(match record.origin {
                RecoveryOrigin::Notification => BankHttpRecoveryAuthority::Notification(handle),
                RecoveryOrigin::Disbursement => BankHttpRecoveryAuthority::Disbursement(handle),
            }),
            state => {
                record.state = state;
                None
            }
        }
    }

    pub(super) fn restore_recovery(&mut self, token: &str, authority: BankHttpRecoveryAuthority) {
        let handle = match authority {
            BankHttpRecoveryAuthority::Notification(handle)
            | BankHttpRecoveryAuthority::Disbursement(handle) => handle,
        };
        self.install_state(token, RecoveryState::Recovery(handle));
    }

    fn owned_record_mut(
        &mut self,
        owner: &BankHttpAuthenticatedOwner,
        token: &str,
    ) -> Option<&mut RecoveryRecord> {
        self.purge_expired();
        let record = self.records.get_mut(token)?;
        (&record.owner == owner).then_some(record)
    }

    fn install_state(&mut self, token: &str, state: RecoveryState) {
        if let Some(record) = self.records.get_mut(token) {
            record.state = state;
            record.expires_at = Instant::now() + self.lifetime;
        }
    }

    fn new_token(&self) -> String {
        loop {
            let token = format!(
                "{TOKEN_PREFIX}{}",
                Alphanumeric.sample_string(&mut OsRng, 40)
            );
            if !self.records.contains_key(&token) {
                return token;
            }
        }
    }

    fn purge_expired(&mut self) {
        let now = Instant::now();
        self.records.retain(|_, record| record.expires_at > now);
        self.replay_tokens
            .retain(|_, token| self.records.contains_key(token));
    }
}
