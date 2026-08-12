use std::collections::BTreeSet;
use std::sync::{Mutex, OnceLock};

use worth_proof::{LinearResource, TerminalReceipt, TerminalState};

use super::counters;

worth_proof::authority_marker!(pub RecoverySessionAuthority);

static LIVE_SESSIONS: OnceLock<Mutex<BTreeSet<[u8; 16]>>> = OnceLock::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalRecoverySessionIdentity([u8; 16]);

impl PhysicalRecoverySessionIdentity {
    pub(crate) const fn bytes(self) -> [u8; 16] {
        self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
enum RecoverySessionTerminal {
    Refused,
    Blocked,
    PublicationIndeterminate,
    Recovered,
}

impl TerminalState for RecoverySessionTerminal {
    fn label(&self) -> &'static str {
        match self {
            Self::Refused => "refused",
            Self::Blocked => "blocked",
            Self::PublicationIndeterminate => "publication-indeterminate",
            Self::Recovered => "recovered",
        }
    }
}

type SessionResource = LinearResource<
    PhysicalRecoverySessionIdentity,
    RecoverySessionTerminal,
    RecoverySessionAuthority,
>;

type SessionTerminalReceipt = TerminalReceipt<
    PhysicalRecoverySessionIdentity,
    RecoverySessionTerminal,
    RecoverySessionAuthority,
>;

pub(crate) struct RecoveredRecoverySessionReceipt {
    receipt: SessionTerminalReceipt,
}

impl RecoveredRecoverySessionReceipt {
    pub(crate) fn identity(&self) -> PhysicalRecoverySessionIdentity {
        *self.receipt.id()
    }
}

pub(crate) struct RecoverySession {
    identity: PhysicalRecoverySessionIdentity,
    resource: Option<SessionResource>,
}

impl RecoverySession {
    pub(crate) fn issue(store_issued_identity: [u8; 16]) -> Result<Self, ()> {
        if store_issued_identity == [0; 16] {
            return Err(());
        }
        let identity = PhysicalRecoverySessionIdentity(store_issued_identity);
        let mut live = live_sessions().lock().map_err(|_| ())?;
        if !live.insert(store_issued_identity) {
            return Err(());
        }
        drop(live);
        counters::record_session_issued();
        Ok(Self {
            identity,
            resource: Some(LinearResource::mint(
                identity,
                &RecoverySessionAuthority::witness(),
            )),
        })
    }

    pub(crate) const fn identity(&self) -> PhysicalRecoverySessionIdentity {
        self.identity
    }

    pub(crate) fn refuse(mut self) {
        let resource = self.resource.take().expect("live recovery session");
        let _receipt = resource.terminate(RecoverySessionTerminal::Refused);
        remove_live(self.identity);
        counters::record_session_refused();
    }

    pub(crate) fn block(mut self) {
        let resource = self.resource.take().expect("live recovery session");
        let _receipt = resource.terminate(RecoverySessionTerminal::Blocked);
        remove_live(self.identity);
        counters::record_session_blocked();
    }

    pub(crate) fn publication_indeterminate(mut self) {
        let resource = self.resource.take().expect("live recovery session");
        let _receipt = resource.terminate(RecoverySessionTerminal::PublicationIndeterminate);
        remove_live(self.identity);
        counters::record_session_publication_indeterminate();
    }

    pub(crate) fn recovered(mut self) -> RecoveredRecoverySessionReceipt {
        let resource = self.resource.take().expect("live recovery session");
        let receipt = resource.terminate(RecoverySessionTerminal::Recovered);
        remove_live(self.identity);
        counters::record_session_recovered();
        RecoveredRecoverySessionReceipt { receipt }
    }
}

impl Drop for RecoverySession {
    fn drop(&mut self) {
        if self.resource.is_some() {
            remove_live(self.identity);
            counters::record_non_terminal_drop();
        }
    }
}

fn live_sessions() -> &'static Mutex<BTreeSet<[u8; 16]>> {
    LIVE_SESSIONS.get_or_init(|| Mutex::new(BTreeSet::new()))
}

fn remove_live(identity: PhysicalRecoverySessionIdentity) {
    if let Ok(mut live) = live_sessions().lock() {
        live.remove(&identity.bytes());
    }
}
