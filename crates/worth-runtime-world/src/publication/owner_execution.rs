use std::sync::Arc;

use crate::history::{CompositeCommitParent, CompositeRuntimeWorldCommit, OrdinaryParent};

use super::product_cas::CompositePublicationReady;
use super::{
    CompositeAttemptProgress, CompositeOwnerExecutionResults, NoEffectCause,
    NoEffectCompositePublication, ReservedCompositePublicationAttempt,
};

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

    /// Bind settled owner work to the exact immutable successor basis. The
    /// retention owner performs the only publication-pair admission here.
    pub(crate) fn ready(
        self,
        commit: Arc<CompositeRuntimeWorldCommit>,
        owner_results: CompositeOwnerExecutionResults,
    ) -> Result<CompositePublicationReady, NoEffectCompositePublication> {
        let (attempt, progress) = self.into_parts();
        let expected = attempt.expected_head().clone();
        let expected_parent = CompositeCommitParent::Ordinary(OrdinaryParent::new(
            expected.selected_commit().clone(),
        ));
        let (
            attempt_identity,
            expected_head,
            _predecessor_basis,
            _plan,
            reserved_commit_identity,
            product_unpublished_identity,
            reserved_commit_capacity,
            reserved_recovery_slot,
            reserved_component_pin_pair,
            reserved_publication_capacity,
            history,
            operation,
            cancellation,
            deadline,
            order,
            _reserved_progress,
        ) = attempt.into_parts();
        if commit.identity() != &reserved_commit_identity
            || commit.identity().owner_identity() != expected.owner_identity()
            || commit.basis().owner_identity() != expected.owner_identity()
            || commit.parent() != &expected_parent
        {
            return Err(NoEffectCompositePublication::new(
                NoEffectCause::OwnerDeniedBeforeEffect,
                Some(expected),
            ));
        }
        let publication_retention =
            match reserved_component_pin_pair.bind_publication(commit.basis()) {
                Ok(retention) => retention,
                Err((_capacity, _denial)) => {
                    return Err(NoEffectCompositePublication::new(
                        NoEffectCause::CapacityExhausted,
                        Some(expected),
                    ))
                }
            };
        Ok(CompositePublicationReady::new(
            attempt_identity,
            expected_head,
            commit,
            owner_results,
            progress,
            product_unpublished_identity,
            reserved_commit_capacity,
            reserved_recovery_slot,
            reserved_publication_capacity,
            history,
            operation,
            publication_retention,
            cancellation,
            deadline,
            order,
        ))
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
