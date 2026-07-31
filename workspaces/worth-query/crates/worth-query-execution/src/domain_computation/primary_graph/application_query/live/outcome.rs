use std::marker::PhantomData;

use worth_relational::facade::history::CommitId;

use super::super::{
    WorthQueryApplicationProjectionDenialKind, WorthQueryApplicationQueryAccessReceipt,
    WorthQueryApplicationQueryAdmissionDenialKind,
};
use crate::domain_computation::primary_graph::WorthQueryOperationAuthorizationDenialKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryApplicationLiveOpenDenialKind {
    LiveContractMissing,
    BindingMismatch,
    BufferCapacityExceedsInstalled,
    WorkLimitExceedsInstalled,
    Admission(WorthQueryApplicationQueryAdmissionDenialKind),
    AuthorizationDenied(WorthQueryOperationAuthorizationDenialKind),
    ScopeIdentityUnavailable,
    BasisReleaseFailed,
    ProviderVersionUnavailable,
    BridgeBasisRejected,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationLiveOpenDenial {
    kind: WorthQueryApplicationLiveOpenDenialKind,
    subject: String,
}

impl WorthQueryApplicationLiveOpenDenial {
    pub(super) fn new(
        kind: WorthQueryApplicationLiveOpenDenialKind,
        subject: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            subject: subject.into(),
        }
    }

    pub const fn kind(&self) -> WorthQueryApplicationLiveOpenDenialKind {
        self.kind
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
}

pub struct WorthQueryApplicationLiveUpdate<Query, QueryResult> {
    commit_id: CommitId,
    result: QueryResult,
    receipt: WorthQueryApplicationQueryAccessReceipt,
    _query: PhantomData<fn() -> Query>,
}

impl<Query, QueryResult> WorthQueryApplicationLiveUpdate<Query, QueryResult> {
    pub(super) fn new(
        commit_id: CommitId,
        result: QueryResult,
        receipt: WorthQueryApplicationQueryAccessReceipt,
    ) -> Self {
        Self {
            commit_id,
            result,
            receipt,
            _query: PhantomData,
        }
    }

    pub const fn commit_id(&self) -> CommitId {
        self.commit_id
    }

    pub const fn commit_ordinal(&self) -> u64 {
        self.commit_id.0
    }

    pub const fn result(&self) -> &QueryResult {
        &self.result
    }

    pub const fn receipt(&self) -> &WorthQueryApplicationQueryAccessReceipt {
        &self.receipt
    }

    pub fn into_parts(
        self,
    ) -> (
        CommitId,
        QueryResult,
        WorthQueryApplicationQueryAccessReceipt,
    ) {
        (self.commit_id, self.result, self.receipt)
    }

    pub fn into_admitted_disclosed(
        self,
    ) -> (
        CommitId,
        super::super::WorthQueryAdmittedDisclosedApplicationResult<Query, QueryResult>,
    ) {
        (
            self.commit_id,
            super::super::WorthQueryAdmittedDisclosedApplicationResult::new(
                vec![self.result],
                self.receipt,
            ),
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryApplicationLiveCauseDenialKind {
    TargetIdentityUnavailable,
    TargetOutsideScope,
    ResultShapeUnavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use]
pub enum WorthQueryApplicationLiveCloseOutcome {
    Completed,
    Unavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationLiveOverflow {
    missed_commit_count: u64,
}

impl WorthQueryApplicationLiveOverflow {
    pub(super) const fn new(missed_commit_count: u64) -> Self {
        Self {
            missed_commit_count,
        }
    }

    pub const fn missed_commit_batches(self) -> u64 {
        self.missed_commit_count
    }
}

pub enum WorthQueryApplicationLiveOutcome<Query, QueryResult> {
    Delivered(WorthQueryApplicationLiveUpdate<Query, QueryResult>),
    Pending,
    Overflow(WorthQueryApplicationLiveOverflow),
    AuthorizationDenied(WorthQueryOperationAuthorizationDenialKind),
    StalePrincipal,
    StaleScope,
    ProjectionDenied(WorthQueryApplicationProjectionDenialKind),
    CauseDenied(WorthQueryApplicationLiveCauseDenialKind),
    Cancelled,
    DeadlineExceeded,
    Closed,
    Unavailable,
}
