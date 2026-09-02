use std::sync::Arc;

use crate::branch::{
    ProductBranchHeadProtection, ProductBranchHeadProtectionDenial, ProductBranchObservation,
    ProductBranchReferenceCell, ProductBranchReferencePublishFailure,
};
use crate::history::{CompositeRuntimeWorldCommit, ProductHeadHistoryProtectionObligation};
use crate::identity::CompositePublicationAttemptIdentity;
use crate::recovery::{
    ProductUnpublishedCause, ProductUnpublishedNextAction, ProductUnpublishedOwnerEffectSummary,
    ProductUnpublishedOwnerEffects,
};
use crate::retention::{
    ProductHeadRetentionObligation, ProductHeadRetentionTransfer, PublicationRetentionObligation,
    RetentionTransferDenial,
};

use super::{
    CompositeLateCancellationPosture, CompositeOwnerExecutionResults,
    CompositePublicationCostCounters, NoEffectCause, NoEffectCompositePublication,
    PerformedCompositePublication, RuntimeWorldPublicationOutcome,
};

const PRODUCT_CAS_LOSS_LIVE_OBLIGATION_COUNT: usize = 3;

#[derive(Debug)]
pub(crate) enum CompositePublicationReadyFailure {
    NoEffect(NoEffectCompositePublication),
    ProductHeadTransfer {
        obligation: PublicationRetentionObligation,
        denial: RetentionTransferDenial,
    },
    ProductHeadTransition {
        obligation: ProductHeadRetentionObligation,
        history: ProductHeadHistoryProtectionObligation,
        denial: RetentionTransferDenial,
    },
    ProductHeadProtection {
        transfer: ProductHeadRetentionTransfer,
        denial: ProductBranchHeadProtectionDenial,
    },
}

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
        new_product_head: ProductBranchObservation,
        late_cancellation: CompositeLateCancellationPosture,
        cost_counters: CompositePublicationCostCounters,
    ) -> Result<RuntimeWorldPublicationOutcome, CompositePublicationReadyFailure> {
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
            operation: _operation,
            publication_retention,
            cancellation: _cancellation,
            deadline,
            order: _order,
        } = self;
        if !valid_successor(
            &expected_head,
            &new_product_head,
            commit.as_ref(),
            &owner_results,
        ) {
            return Err(CompositePublicationReadyFailure::NoEffect(
                NoEffectCompositePublication::new(
                    NoEffectCause::OwnerDeniedBeforeEffect,
                    Some(expected_head),
                ),
            ));
        }
        let entry = match reserved_commit_capacity.install(Arc::clone(&commit)) {
            Ok(entry) => entry,
            Err(_) => {
                return Err(CompositePublicationReadyFailure::NoEffect(
                    NoEffectCompositePublication::new(
                        NoEffectCause::CapacityExhausted,
                        Some(expected_head),
                    ),
                ))
            }
        };
        let installed_rollback = history.arm_installed_commit_rollback(entry.identity());
        let product_history = match history.protect_product_head(entry.commit()) {
            Ok(protection) => protection,
            Err(_) => {
                return Err(CompositePublicationReadyFailure::NoEffect(
                    NoEffectCompositePublication::new(
                        NoEffectCause::CapacityExhausted,
                        Some(expected_head),
                    ),
                ))
            }
        };
        let transfer = match publication_retention.into_product_head_transfer(commit.basis()) {
            Ok(transfer) => transfer,
            Err((obligation, denial)) => {
                return Err(CompositePublicationReadyFailure::ProductHeadTransfer {
                    obligation,
                    denial,
                })
            }
        };
        let protection = match ProductBranchHeadProtection::owner_issued(
            new_product_head.snapshot().clone(),
            transfer,
            product_history,
        ) {
            Ok(protection) => protection,
            Err(failure) => {
                let denial = failure.denial();
                let (_snapshot, product_head, product_history, receipt) =
                    failure.into_protection().into_parts();
                drop(product_history);
                let receipt = receipt.expect("owner-issued protection carries transfer evidence");
                return Err(CompositePublicationReadyFailure::ProductHeadProtection {
                    transfer: ProductHeadRetentionTransfer::new(product_head, receipt),
                    denial,
                });
            }
        };
        installed_rollback.commit();
        match cell.compare_and_publish(&expected_head, protection) {
            Ok(movement) => Ok(RuntimeWorldPublicationOutcome::Performed(
                PerformedCompositePublication::owner_issued(
                    expected_head,
                    new_product_head,
                    commit,
                    attempt_identity,
                    owner_results,
                    late_cancellation,
                    movement.retention_transfer().clone(),
                    cost_counters,
                ),
            )),
            Err(failure) => Ok(RuntimeWorldPublicationOutcome::ProductUnpublished(
                unpublished_from_cas_loss(
                    failure,
                    attempt_identity,
                    product_unpublished_identity,
                    expected_head,
                    progress,
                    commit,
                    reserved_recovery_slot,
                    deadline,
                )?,
            )),
        }
    }
}

fn valid_successor(
    expected: &ProductBranchObservation,
    successor: &ProductBranchObservation,
    commit: &CompositeRuntimeWorldCommit,
    owner_results: &CompositeOwnerExecutionResults,
) -> bool {
    let next_generation = expected.reference_generation().advance().ok();
    successor.owner_identity() == expected.owner_identity()
        && successor.branch_identity() == expected.branch_identity()
        && successor.lifecycle_incarnation() == expected.lifecycle_incarnation()
        && next_generation == Some(successor.reference_generation())
        && successor.selected_commit() == commit.identity()
        && commit.matches_owner_results(expected.basis(), owner_results)
}

fn unpublished_from_cas_loss(
    failure: ProductBranchReferencePublishFailure,
    attempt_identity: CompositePublicationAttemptIdentity,
    identity: crate::identity::ProductUnpublishedOwnerEffectsIdentity,
    expected_head: ProductBranchObservation,
    progress: super::CompositeAttemptProgress,
    commit: Arc<CompositeRuntimeWorldCommit>,
    recovery_slot: crate::recovery::ReservedProductUnpublishedSlot,
    deadline: Option<crate::lifecycle::RuntimeWorldInstant>,
) -> Result<ProductUnpublishedOwnerEffects, CompositePublicationReadyFailure> {
    let (observed_head, protection) = failure.into_recovery_parts();
    let (_snapshot, product_head, product_history, _receipt) = protection.into_parts();
    let retained = match product_head.try_transition_to_retained_partial() {
        Ok(retained) => retained,
        Err((obligation, denial)) => {
            return Err(CompositePublicationReadyFailure::ProductHeadTransition {
                obligation,
                history: product_history,
                denial,
            })
        }
    };
    let successor_history = product_history.transition_to_product_unpublished();
    let summary = ProductUnpublishedOwnerEffectSummary::from_progress(
        &progress,
        PRODUCT_CAS_LOSS_LIVE_OBLIGATION_COUNT,
        0,
    );
    Ok(ProductUnpublishedOwnerEffects::new(
        identity,
        attempt_identity,
        expected_head,
        Some(observed_head),
        progress,
        Some(commit.basis().clone()),
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
    ))
}

#[cfg(test)]
#[path = "product_cas_tests.rs"]
mod tests;
