use std::sync::Arc;

use crate::basis::AdmittedCompositeRuntimeWorldBasis;
use crate::history::CompositeRuntimeWorldCommit;
use crate::recovery::{ProductUnpublishedOwnerEffectSummary, ProductUnpublishedOwnerEffects};

use super::product_cas::CompositePublicationReady;
use super::{CompositeAttemptProgress, ReservedCompositePublicationAttempt};

const RETENTION_PENDING_LIVE_OBLIGATION_COUNT: usize = 3;

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

    /// Consume exact settled owner evidence into one successor commit and bind
    /// its reserved publication pins. A post-effect pin denial becomes the
    /// retained recovery terminal; it can never be reclassified as no-effect.
    pub(crate) fn ready(
        self,
        successor_basis: AdmittedCompositeRuntimeWorldBasis,
    ) -> Result<CompositePublicationReady, ProductUnpublishedOwnerEffects> {
        let (attempt, progress) = self.into_parts();
        let (progress, owner_results) = progress
            .into_ready_results()
            .expect("an owner-execution settlement carries complete performed evidence");
        let (
            attempt_identity,
            expected_head,
            _predecessor_basis,
            plan,
            reserved_commit_identity,
            product_unpublished_identity,
            reserved_commit_capacity,
            reserved_recovery_slot,
            reserved_component_pin_pair,
            reserved_publication_capacity,
            history,
            mut operation,
            cancellation,
            deadline,
            order,
            _reserved_progress,
        ) = attempt.into_parts();
        assert!(
            owner_results.matches_plan(&plan),
            "settled owner results must match the reserved component plan"
        );
        let commit = Arc::new(
            CompositeRuntimeWorldCommit::from_ordinary_publication(
                reserved_commit_identity,
                expected_head.snapshot().commit(),
                successor_basis.clone(),
                attempt_identity.clone(),
                &owner_results,
                None,
            )
            .expect("owner-issued results and admitted successor form the reserved commit"),
        );
        let publication_retention = match reserved_component_pin_pair
            .bind_publication(commit.basis())
        {
            Ok(retention) => retention,
            Err((capacity, denial)) => {
                let entry = reserved_commit_capacity
                    .install(Arc::clone(&commit))
                    .expect("the reserved commit installs into its exact history slot");
                let installed_rollback = history.arm_installed_commit_rollback(entry.identity());
                let successor_history = history
                    .protect_product_head(entry.commit())
                    .expect("the installed successor admits exact history protection")
                    .transition_to_product_unpublished();
                installed_rollback.commit();
                operation
                    .begin_recovery()
                    .expect("a publishing attempt enters retained recovery");
                let summary = ProductUnpublishedOwnerEffectSummary::from_progress(
                    &progress,
                    RETENTION_PENDING_LIVE_OBLIGATION_COUNT,
                    0,
                );
                return Err(ProductUnpublishedOwnerEffects::new_retention_pending(
                    product_unpublished_identity,
                    attempt_identity,
                    expected_head,
                    progress,
                    successor_basis,
                    owner_results,
                    capacity,
                    denial,
                    successor_history,
                    reserved_recovery_slot,
                    summary,
                    deadline,
                ));
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
