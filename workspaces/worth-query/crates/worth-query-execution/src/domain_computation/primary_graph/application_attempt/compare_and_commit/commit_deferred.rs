//! Application-owned deferred commit evidence.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationCommitDeferred {
    kind: WorthQueryApplicationCommitDeferredKind,
    stage: crate::domain_computation::provider_session::WorthQueryProviderSessionProtocolStage,
    detail: String,
    counters:
        crate::domain_computation::provider_session::WorthQueryProviderSessionProtocolCounters,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryApplicationCommitDeferredKind {
    RetentionCapacityExhausted,
    PatchPositionReservationContended,
    CandidateLifetimeExpired { maximum_lifetime_millis: u64 },
    CandidateCapacityExhausted { maximum_candidates: usize },
    PublishedSnapshotCapacityExhausted { maximum_handles: usize },
}

impl From<crate::domain_computation::provider_session::WorthQueryProviderSessionCommitDeferredKind>
    for WorthQueryApplicationCommitDeferredKind
{
    fn from(
        kind: crate::domain_computation::provider_session::WorthQueryProviderSessionCommitDeferredKind,
    ) -> Self {
        use crate::domain_computation::provider_session::WorthQueryProviderSessionCommitDeferredKind as Provider;
        match kind {
            Provider::RetentionCapacityExhausted => Self::RetentionCapacityExhausted,
            Provider::PatchPositionReservationContended => Self::PatchPositionReservationContended,
            Provider::CandidateLifetimeExpired {
                maximum_lifetime_millis,
            } => Self::CandidateLifetimeExpired {
                maximum_lifetime_millis,
            },
            Provider::CandidateCapacityExhausted { maximum_candidates } => {
                Self::CandidateCapacityExhausted { maximum_candidates }
            }
            Provider::PublishedSnapshotCapacityExhausted { maximum_handles } => {
                Self::PublishedSnapshotCapacityExhausted { maximum_handles }
            }
        }
    }
}

impl WorthQueryApplicationCommitDeferred {
    pub(in crate::domain_computation::primary_graph) fn from_provider_session(
        deferred: crate::domain_computation::provider_session::WorthQueryProviderSessionCommitDeferred,
    ) -> Self {
        Self {
            kind: deferred.kind().into(),
            stage: deferred.stage(),
            detail: deferred.detail().to_owned(),
            counters: deferred.counters(),
        }
    }

    pub const fn kind(&self) -> WorthQueryApplicationCommitDeferredKind {
        self.kind
    }

    pub const fn stage(
        &self,
    ) -> crate::domain_computation::provider_session::WorthQueryProviderSessionProtocolStage {
        self.stage
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub const fn counters(
        &self,
    ) -> crate::domain_computation::provider_session::WorthQueryProviderSessionProtocolCounters
    {
        self.counters
    }
}
