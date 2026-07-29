use worth_relational::facade::history::CommitId;

use super::super::WorthQueryOperationAuthorizationDenialKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryLiveDeliveryOpenDenialKind {
    Authorization,
    UninstalledEffect,
    UnavailableProviderVersion,
    InvalidInstalledStrategySet,
    BufferCapacityExceedsInstalled,
    BridgeBasisRejected,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryLiveDeliveryOpenDenial {
    kind: WorthQueryLiveDeliveryOpenDenialKind,
    subject: String,
}

impl WorthQueryLiveDeliveryOpenDenial {
    pub(super) fn new(
        kind: WorthQueryLiveDeliveryOpenDenialKind,
        subject: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            subject: subject.into(),
        }
    }

    pub const fn kind(&self) -> WorthQueryLiveDeliveryOpenDenialKind {
        self.kind
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryLiveDeliveryOverflow {
    missed_commit_batches: u64,
}

impl WorthQueryLiveDeliveryOverflow {
    pub(super) const fn new(missed_commit_batches: u64) -> Self {
        Self {
            missed_commit_batches,
        }
    }

    pub const fn missed_commit_batches(self) -> u64 {
        self.missed_commit_batches
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryLiveCommitCause<Payload> {
    commit_id: CommitId,
    payload: Payload,
}

impl<Payload> WorthQueryLiveCommitCause<Payload> {
    pub(super) const fn new(commit_id: CommitId, payload: Payload) -> Self {
        Self { commit_id, payload }
    }

    pub const fn commit_id(&self) -> CommitId {
        self.commit_id
    }

    pub const fn commit_ordinal(&self) -> u64 {
        self.commit_id.0
    }

    pub const fn payload(&self) -> &Payload {
        &self.payload
    }

    pub fn into_payload(self) -> Payload {
        self.payload
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryLiveDeliveryOutcome<Payload> {
    Delivered(WorthQueryLiveCommitCause<Payload>),
    Pending,
    Overflow(WorthQueryLiveDeliveryOverflow),
    AuthorizationDenied(WorthQueryOperationAuthorizationDenialKind),
    ScopeMismatch,
    Cancelled,
    DeadlineExceeded,
    Closed,
    Unavailable,
}
