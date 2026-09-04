use std::sync::Arc;

use crate::branch::{
    ProductBranchObservation, ProductBranchReferenceCell, ProductBranchReferenceSnapshot,
};
use crate::history::CompositeRuntimeWorldCommit;
use crate::identity::CompositePublicationAttemptIdentity;
use crate::recovery::ProductUnpublishedCause;
use crate::retention::PublicationRetentionObligation;

use super::{
    CompositeLateCancellationPosture, CompositeOwnerExecutionResults,
    CompositePublicationCostCounters, RuntimeWorldPublicationOutcome,
};

#[path = "product_cas/movement.rs"]
mod movement;
#[path = "product_cas/retained.rs"]
mod retained;

use movement::attempt_product_movement;
use retained::{AttemptTerminal, RetainedSuccessorCustody};

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
    counters: CompositePublicationCostCounters,
}

/// The exact reserved contents a ready publication owns. Cost counters travel
/// with the attempt that accrued them, so no caller can hand the product CAS a
/// separate counter image.
pub(crate) struct CompositePublicationReadyInputs {
    pub(crate) identity: CompositePublicationAttemptIdentity,
    pub(crate) expected_head: ProductBranchObservation,
    pub(crate) commit: Arc<CompositeRuntimeWorldCommit>,
    pub(crate) owner_results: CompositeOwnerExecutionResults,
    pub(crate) progress: super::CompositeAttemptProgress,
    pub(crate) product_unpublished_identity:
        crate::identity::ProductUnpublishedOwnerEffectsIdentity,
    pub(crate) reserved_commit_capacity: crate::history::ReservedCompositeCommitCapacity,
    pub(crate) reserved_recovery_slot: crate::recovery::ReservedProductUnpublishedSlot,
    pub(crate) reserved_publication_capacity:
        crate::lifecycle::owner::ReservedPublicationAttemptCapacity,
    pub(crate) history: crate::history::CompositeHistoryCatalog,
    pub(crate) operation: crate::lifecycle::owner::RuntimeWorldOperationReservation,
    pub(crate) publication_retention: PublicationRetentionObligation,
    pub(crate) cancellation: super::CompositeAttemptCancellationPosture,
    pub(crate) deadline: Option<crate::lifecycle::RuntimeWorldInstant>,
    pub(crate) counters: CompositePublicationCostCounters,
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
    pub(crate) fn new(inputs: CompositePublicationReadyInputs) -> Self {
        let CompositePublicationReadyInputs {
            identity,
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
            counters,
        } = inputs;
        Self {
            attempt_identity: identity,
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
            counters,
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

    /// Hand the exact reserved contents back as one bundle. Nothing is
    /// re-derived; this is the same linear custody the constructor took.
    fn into_inputs(self) -> CompositePublicationReadyInputs {
        CompositePublicationReadyInputs {
            identity: self.attempt_identity,
            expected_head: self.expected_head,
            commit: self.commit,
            owner_results: self.owner_results,
            progress: self.progress,
            product_unpublished_identity: self.product_unpublished_identity,
            reserved_commit_capacity: self.reserved_commit_capacity,
            reserved_recovery_slot: self.reserved_recovery_slot,
            reserved_publication_capacity: self.reserved_publication_capacity,
            history: self.history,
            operation: self.operation,
            publication_retention: self.publication_retention,
            cancellation: self.cancellation,
            deadline: self.deadline,
            counters: self.counters,
        }
    }

    /// Compare the expected product observation, then materialize the reserved
    /// commit and perform the product CAS.
    ///
    /// SPEC-P4-016: the expected-`ProductBranchObservation` comparison precedes
    /// materializing the commit into the reserved history slot, so an attempt
    /// that has already lost the product reference never constructs
    /// product-head authority over the cell and never advances the product
    /// reference generation. The movement record is still written before the
    /// swap, inside the cell's own write lock.
    pub(crate) fn publish(
        self,
        cell: &ProductBranchReferenceCell,
        late_cancellation: CompositeLateCancellationPosture,
    ) -> RuntimeWorldPublicationOutcome {
        let mut ready = self.into_inputs();
        assert!(
            ready
                .commit
                .matches_owner_results(ready.expected_head.basis(), &ready.owner_results),
            "a ready publication carries the exact owner results embodied by its commit"
        );
        if cancellation_observed(ready.cancellation, late_cancellation) {
            ready.counters.record_cancellation_observation();
        }
        let observed_head = cell.atomic_snapshot();
        ready.counters.record_expected_head_recheck();
        ready.counters.record_product_cell_touch();
        match loss_before_product_movement(&ready, late_cancellation, &observed_head) {
            Some(cause) => retain_before_product_movement(ready, observed_head, cause),
            None => attempt_product_movement(ready, cell, late_cancellation),
        }
    }
}

/// Terminate an attempt that has already lost, before it materializes anything.
/// It still owns its reserved history slot, so the retained record installs the
/// successor for recovery custody alone and takes no product-head authority.
fn retain_before_product_movement(
    ready: CompositePublicationReadyInputs,
    observed_head: ProductBranchReferenceSnapshot,
    cause: ProductUnpublishedCause,
) -> RuntimeWorldPublicationOutcome {
    let (successor, terminal) = AttemptTerminal::split(ready);
    terminal.retain(
        observed_head,
        cause,
        RetainedSuccessorCustody::Unmaterialized(successor),
    )
}

fn cancellation_observed(
    attempt: super::CompositeAttemptCancellationPosture,
    late: CompositeLateCancellationPosture,
) -> bool {
    matches!(
        attempt,
        super::CompositeAttemptCancellationPosture::CancellationObserved
    ) || !matches!(late, CompositeLateCancellationPosture::NotRequested)
}

fn cancellation_before_product_movement(
    attempt: super::CompositeAttemptCancellationPosture,
    late: CompositeLateCancellationPosture,
) -> bool {
    matches!(
        attempt,
        super::CompositeAttemptCancellationPosture::CancellationObserved
    ) || matches!(
        late,
        CompositeLateCancellationPosture::RequestedBeforeProductMovement
    )
}

/// The one comparison that decides whether this attempt may still move the
/// product reference. `None` means the attempt still holds the exact head it
/// admitted against and observed no cancellation before product movement.
fn loss_before_product_movement(
    ready: &CompositePublicationReadyInputs,
    late: CompositeLateCancellationPosture,
    observed_head: &ProductBranchReferenceSnapshot,
) -> Option<ProductUnpublishedCause> {
    if ready
        .expected_head
        .mismatch_against_snapshot(observed_head)
        .is_some()
    {
        // Another ready attempt already moved the cell. A frozen caller may
        // report cancellation in the same window; classify the observed race as
        // product-publication loss rather than hiding the loss as cancellation.
        return Some(ProductUnpublishedCause::ProductPublicationLost);
    }
    cancellation_before_product_movement(ready.cancellation, late)
        .then_some(ProductUnpublishedCause::CancellationAfterEffect)
}
