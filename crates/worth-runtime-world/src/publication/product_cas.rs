use std::sync::Arc;

use crate::branch::{
    ProductBranchHeadProtection, ProductBranchObservation, ProductBranchReferenceCell,
    ProductBranchReferencePublishFailure, ProductBranchReferenceSnapshot,
};
use crate::history::CompositeRuntimeWorldCommit;
use crate::identity::CompositePublicationAttemptIdentity;
use crate::recovery::{
    ProductUnpublishedCause, ProductUnpublishedOwnerEffectSummary, ProductUnpublishedOwnerEffects,
};
use crate::retention::PublicationRetentionObligation;

use super::{
    CompositeLateCancellationPosture, CompositeOwnerExecutionResults,
    CompositePublicationCostCounters, PerformedCompositePublication,
    RuntimeWorldPublicationOutcome,
};

const PRODUCT_UNPUBLISHED_LIVE_OBLIGATION_COUNT: usize = 3;

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
            cancellation,
            deadline,
            counters: mut cost_counters,
        } = self;
        assert!(
            commit.matches_owner_results(expected_head.basis(), &owner_results),
            "a ready publication carries the exact owner results embodied by its commit"
        );
        let cancellation_was_observed = cancellation_observed(cancellation, late_cancellation);
        if cancellation_was_observed {
            cost_counters.record_cancellation_observation();
        }
        let cancellation_head = cancellation_before_product_movement(
            cancellation,
            late_cancellation,
            cell,
            &expected_head,
            &mut cost_counters,
        );
        let successor_snapshot = derive_successor_snapshot(&expected_head, &commit);
        let protection = install_successor_protection(
            history,
            reserved_commit_capacity,
            publication_retention,
            &commit,
            successor_snapshot,
        );
        cost_counters.record_history_slot_installed();

        if let Some(observed_head) = cancellation_head {
            operation
                .begin_recovery()
                .expect("cancellation after owner movement enters retained recovery");
            return RuntimeWorldPublicationOutcome::ProductUnpublished(
                unpublished_from_protection(
                    observed_head,
                    protection,
                    attempt_identity,
                    product_unpublished_identity,
                    expected_head,
                    progress,
                    commit,
                    owner_results,
                    reserved_recovery_slot,
                    deadline,
                    ProductUnpublishedCause::CancellationAfterEffect,
                ),
            );
        }

        cost_counters.record_expected_head_recheck();
        cost_counters.record_product_cell_touch();
        cost_counters.record_cas_attempt();
        match cell.compare_and_publish(&expected_head, protection) {
            Ok(movement) => {
                cost_counters.record_cas_win();
                RuntimeWorldPublicationOutcome::Performed(
                    PerformedCompositePublication::owner_issued(
                        expected_head,
                        movement,
                        commit,
                        attempt_identity,
                        owner_results,
                        late_cancellation,
                        cost_counters,
                    ),
                )
            }
            Err(failure) => {
                cost_counters.record_cas_loss();
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
    cell: &ProductBranchReferenceCell,
    expected_head: &ProductBranchObservation,
    cost_counters: &mut CompositePublicationCostCounters,
) -> Option<ProductBranchReferenceSnapshot> {
    if matches!(
        attempt,
        super::CompositeAttemptCancellationPosture::CancellationObserved
    ) {
        let observed_head = cell.atomic_snapshot();
        cost_counters.record_product_cell_touch();
        return (observed_head == *expected_head.snapshot()).then_some(observed_head);
    }
    if !matches!(
        late,
        CompositeLateCancellationPosture::RequestedBeforeProductMovement
    ) {
        return None;
    }
    let observed_head = cell.atomic_snapshot();
    cost_counters.record_expected_head_recheck();
    cost_counters.record_product_cell_touch();
    // A frozen caller may report this posture while another ready attempt has
    // already moved the cell. Let the one CAS classify that observed race as
    // product-publication loss rather than hiding the loss as cancellation.
    (observed_head == *expected_head.snapshot()).then_some(observed_head)
}

fn derive_successor_snapshot(
    expected_head: &ProductBranchObservation,
    commit: &Arc<CompositeRuntimeWorldCommit>,
) -> ProductBranchReferenceSnapshot {
    ProductBranchReferenceSnapshot::owner_issued(
        expected_head.owner_identity(),
        expected_head.branch_identity().clone(),
        expected_head.lifecycle_incarnation(),
        expected_head
            .reference_generation()
            .advance()
            .expect("reference generation capacity was checked before owner effects"),
        Arc::clone(commit),
    )
    .expect("the ready commit and expected head share one owner and branch lineage")
}

fn install_successor_protection(
    history: crate::history::CompositeHistoryCatalog,
    reserved_commit_capacity: crate::history::ReservedCompositeCommitCapacity,
    publication_retention: PublicationRetentionObligation,
    commit: &Arc<CompositeRuntimeWorldCommit>,
    successor_snapshot: ProductBranchReferenceSnapshot,
) -> ProductBranchHeadProtection {
    let entry = reserved_commit_capacity
        .install(Arc::clone(commit))
        .expect("the ready commit matches its reserved history slot");
    let installed_rollback = history.arm_installed_commit_rollback(entry.identity());
    let product_history = history
        .protect_product_head(entry.commit())
        .expect("the installed ready commit admits product-head history protection");
    let transfer = publication_retention
        .into_product_head_transfer(commit.basis())
        .expect("ready publication retention is bound to the exact successor basis");
    let protection =
        ProductBranchHeadProtection::owner_issued(successor_snapshot, transfer, product_history)
            .expect("ready component and history custody match the successor image");
    installed_rollback.commit();
    protection
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
    unpublished_from_protection(
        observed_head,
        protection,
        attempt_identity,
        identity,
        expected_head,
        progress,
        commit,
        owner_results,
        recovery_slot,
        deadline,
        ProductUnpublishedCause::ProductPublicationLost,
    )
}

fn unpublished_from_protection(
    observed_head: ProductBranchReferenceSnapshot,
    protection: ProductBranchHeadProtection,
    attempt_identity: CompositePublicationAttemptIdentity,
    identity: crate::identity::ProductUnpublishedOwnerEffectsIdentity,
    expected_head: ProductBranchObservation,
    progress: super::CompositeAttemptProgress,
    commit: Arc<CompositeRuntimeWorldCommit>,
    owner_results: CompositeOwnerExecutionResults,
    recovery_slot: crate::recovery::ReservedProductUnpublishedSlot,
    deadline: Option<crate::lifecycle::RuntimeWorldInstant>,
    cause: ProductUnpublishedCause,
) -> ProductUnpublishedOwnerEffects {
    let (_snapshot, product_head, product_history, _receipt) = protection.into_parts();
    let retained = product_head.transition_to_retained_partial();
    let successor_history = product_history.transition_to_product_unpublished();
    let summary = ProductUnpublishedOwnerEffectSummary::from_progress(
        &progress,
        PRODUCT_UNPUBLISHED_LIVE_OBLIGATION_COUNT,
        0,
    );
    let next_actions = crate::recovery::next_actions_for_progress(&progress);
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
        cause,
        next_actions,
        deadline,
        0,
    )
}
