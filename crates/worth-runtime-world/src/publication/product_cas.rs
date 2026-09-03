use std::sync::Arc;

use crate::branch::{
    ProductBranchHeadProtection, ProductBranchObservation, ProductBranchReferenceCell,
    ProductBranchReferencePublishFailure, ProductBranchReferenceSnapshot,
};
use crate::history::CompositeRuntimeWorldCommit;
use crate::identity::CompositePublicationAttemptIdentity;
use crate::recovery::{
    ProductUnpublishedCause, ProductUnpublishedNextAction, ProductUnpublishedOwnerEffectSummary,
    ProductUnpublishedOwnerEffects,
};
use crate::retention::PublicationRetentionObligation;

use super::{
    CompositeLateCancellationPosture, CompositeOwnerExecutionResults,
    CompositePublicationCostCounters, PerformedCompositePublication,
    RuntimeWorldPublicationOutcome,
};

const PRODUCT_CAS_LOSS_LIVE_OBLIGATION_COUNT: usize = 3;

/// Final pre-publication phase. It owns all real reservations and the
/// successor-basis publication retention until the product CAS resolves.
pub struct CompositePublicationReady {
    attempt_identity: CompositePublicationAttemptIdentity,
    expected_head: ProductBranchObservation,
    commit: Arc<CompositeRuntimeWorldCommit>,
    owner_results: CompositeOwnerExecutionResults,
    progress: super::CompositeAttemptProgress,
    product_unpublished_identity: crate::identity::ProductUnpublishedOwnerEffectsIdentity,
    reserved_commit_capacity: crate::history::ReservedCompositeCommitCapacity,
    reserved_recovery_slot: crate::recovery::ReservedProductUnpublishedSlot,
    reserved_publication_capacity: crate::lifecycle::owner::ReservedPublicationAttemptCapacity,
    history: crate::history::CompositeHistoryCatalog,
    operation: crate::lifecycle::owner::RuntimeWorldOperationReservation,
    publication_retention: PublicationRetentionObligation,
    cancellation: super::CompositeAttemptCancellationPosture,
    deadline: Option<crate::lifecycle::RuntimeWorldInstant>,
    order: super::CompositePublicationOrder,
}

impl std::fmt::Debug for CompositePublicationReady {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CompositePublicationReady")
            .field("attempt_identity", &self.attempt_identity)
            .field("expected_head", &self.expected_head)
            .field("commit", &self.commit.identity())
            .field("progress", &self.progress)
            .finish_non_exhaustive()
    }
}

impl CompositePublicationReady {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        attempt_identity: CompositePublicationAttemptIdentity,
        expected_head: ProductBranchObservation,
        commit: Arc<CompositeRuntimeWorldCommit>,
        owner_results: CompositeOwnerExecutionResults,
        progress: super::CompositeAttemptProgress,
        product_unpublished_identity: crate::identity::ProductUnpublishedOwnerEffectsIdentity,
        reserved_commit_capacity: crate::history::ReservedCompositeCommitCapacity,
        reserved_recovery_slot: crate::recovery::ReservedProductUnpublishedSlot,
        reserved_publication_capacity: crate::lifecycle::owner::ReservedPublicationAttemptCapacity,
        history: crate::history::CompositeHistoryCatalog,
        operation: crate::lifecycle::owner::RuntimeWorldOperationReservation,
        publication_retention: PublicationRetentionObligation,
        cancellation: super::CompositeAttemptCancellationPosture,
        deadline: Option<crate::lifecycle::RuntimeWorldInstant>,
        order: super::CompositePublicationOrder,
    ) -> Self {
        Self {
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
        }
    }

    pub fn attempt_identity(&self) -> &CompositePublicationAttemptIdentity {
        &self.attempt_identity
    }

    pub fn expected_head(&self) -> &ProductBranchObservation {
        &self.expected_head
    }

    pub fn progress(&self) -> &super::CompositeAttemptProgress {
        &self.progress
    }

    pub(crate) fn successor_basis(&self) -> &crate::basis::AdmittedCompositeRuntimeWorldBasis {
        self.commit.basis()
    }

    /// Install the reserved commit, construct the exact product protection,
    /// and perform the complete expected-observation CAS. No receipt is
    /// supplied by the caller; this method transfers its own obligation.
    pub(crate) fn publish(
        self,
        cell: &ProductBranchReferenceCell,
        late_cancellation: CompositeLateCancellationPosture,
        cost_counters: CompositePublicationCostCounters,
    ) -> RuntimeWorldPublicationOutcome {
        let Self {
            attempt_identity,
            expected_head,
            commit,
            owner_results,
            progress,
            product_unpublished_identity,
            reserved_commit_capacity,
            reserved_recovery_slot,
            reserved_publication_capacity: _reserved_publication_capacity,
            history,
            mut operation,
            publication_retention,
            cancellation: _cancellation,
            deadline,
            order: _order,
        } = self;
        assert!(
            commit.matches_owner_results(expected_head.basis(), &owner_results),
            "a ready publication carries the exact owner results embodied by its commit"
        );
        let successor_snapshot = ProductBranchReferenceSnapshot::owner_issued(
            expected_head.owner_identity(),
            expected_head.branch_identity().clone(),
            expected_head.lifecycle_incarnation(),
            expected_head
                .reference_generation()
                .advance()
                .expect("reference generation capacity was checked before owner effects"),
            Arc::clone(&commit),
        )
        .expect("the ready commit and expected head share one owner and branch lineage");
        let entry = reserved_commit_capacity
            .install(Arc::clone(&commit))
            .expect("the ready commit matches its reserved history slot");
        let installed_rollback = history.arm_installed_commit_rollback(entry.identity());
        let product_history = history
            .protect_product_head(entry.commit())
            .expect("the installed ready commit admits product-head history protection");
        let transfer = publication_retention
            .into_product_head_transfer(commit.basis())
            .expect("ready publication retention is bound to the exact successor basis");
        let protection = ProductBranchHeadProtection::owner_issued(
            successor_snapshot,
            transfer,
            product_history,
        )
        .expect("ready component and history custody match the successor image");
        installed_rollback.commit();
        match cell.compare_and_publish(&expected_head, protection) {
            Ok(movement) => RuntimeWorldPublicationOutcome::Performed(
                PerformedCompositePublication::owner_issued(
                    expected_head,
                    movement,
                    commit,
                    attempt_identity,
                    owner_results,
                    late_cancellation,
                    cost_counters,
                ),
            ),
            Err(failure) => {
                operation
                    .begin_recovery()
                    .expect("a lost product CAS enters retained recovery");
                RuntimeWorldPublicationOutcome::ProductUnpublished(unpublished_from_cas_loss(
                    failure,
                    attempt_identity,
                    product_unpublished_identity,
                    expected_head,
                    progress,
                    commit,
                    owner_results,
                    reserved_recovery_slot,
                    deadline,
                ))
            }
        }
    }
}

fn unpublished_from_cas_loss(
    failure: ProductBranchReferencePublishFailure,
    attempt_identity: CompositePublicationAttemptIdentity,
    identity: crate::identity::ProductUnpublishedOwnerEffectsIdentity,
    expected_head: ProductBranchObservation,
    progress: super::CompositeAttemptProgress,
    commit: Arc<CompositeRuntimeWorldCommit>,
    owner_results: CompositeOwnerExecutionResults,
    recovery_slot: crate::recovery::ReservedProductUnpublishedSlot,
    deadline: Option<crate::lifecycle::RuntimeWorldInstant>,
) -> ProductUnpublishedOwnerEffects {
    let (observed_head, protection) = failure.into_recovery_parts();
    let (_snapshot, product_head, product_history, _receipt) = protection.into_parts();
    let retained = product_head.transition_to_retained_partial();
    let successor_history = product_history.transition_to_product_unpublished();
    let summary = ProductUnpublishedOwnerEffectSummary::from_progress(
        &progress,
        PRODUCT_CAS_LOSS_LIVE_OBLIGATION_COUNT,
        0,
    );
    ProductUnpublishedOwnerEffects::new_retained(
        identity,
        attempt_identity,
        expected_head,
        Some(observed_head),
        progress,
        Some(commit.basis().clone()),
        owner_results,
        retained,
        successor_history,
        recovery_slot,
        summary,
        ProductUnpublishedCause::ProductPublicationLost,
        vec![
            ProductUnpublishedNextAction::Inspect,
            ProductUnpublishedNextAction::ReleaseObligations,
        ],
        deadline,
        0,
    )
}
