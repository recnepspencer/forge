use crate::branch::ProductBranchObservation;
use crate::history::CompositeRuntimeWorldCommit;
use crate::identity::CompositePublicationAttemptIdentity;

use super::{CompositeAttemptProgress, ReservedCompositePublicationAttempt};

/// Owner effects have been settled into exact progress, but product
/// publication has not yet crossed its final compare-and-publish point.
pub struct OwnerExecutionSettlement {
    attempt: ReservedCompositePublicationAttempt,
    progress: CompositeAttemptProgress,
}

impl std::fmt::Debug for OwnerExecutionSettlement {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("OwnerExecutionSettlement")
            .field("progress", &self.progress)
            .finish_non_exhaustive()
    }
}

impl OwnerExecutionSettlement {
    pub(crate) fn new(
        attempt: ReservedCompositePublicationAttempt,
        progress: CompositeAttemptProgress,
    ) -> Self {
        Self { attempt, progress }
    }

    pub fn progress(&self) -> &CompositeAttemptProgress {
        &self.progress
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ReservedCompositePublicationAttempt,
        CompositeAttemptProgress,
    ) {
        (self.attempt, self.progress)
    }
}

/// Final pre-publication phase. The product reference has not moved yet.
pub struct CompositePublicationReady {
    attempt_identity: CompositePublicationAttemptIdentity,
    expected_head: ProductBranchObservation,
    commit: CompositeRuntimeWorldCommit,
    progress: CompositeAttemptProgress,
}

impl std::fmt::Debug for CompositePublicationReady {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CompositePublicationReady")
            .field("attempt_identity", &self.attempt_identity)
            .field("expected_head", &self.expected_head)
            .field("progress", &self.progress)
            .finish_non_exhaustive()
    }
}

impl CompositePublicationReady {
    pub(crate) fn new(
        attempt_identity: CompositePublicationAttemptIdentity,
        expected_head: ProductBranchObservation,
        commit: CompositeRuntimeWorldCommit,
        progress: CompositeAttemptProgress,
    ) -> Self {
        Self {
            attempt_identity,
            expected_head,
            commit,
            progress,
        }
    }

    pub fn attempt_identity(&self) -> &CompositePublicationAttemptIdentity {
        &self.attempt_identity
    }

    pub fn expected_head(&self) -> &ProductBranchObservation {
        &self.expected_head
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        CompositePublicationAttemptIdentity,
        ProductBranchObservation,
        CompositeRuntimeWorldCommit,
        CompositeAttemptProgress,
    ) {
        (
            self.attempt_identity,
            self.expected_head,
            self.commit,
            self.progress,
        )
    }
}
