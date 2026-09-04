//! Retained terminals for a composite publication that did not move the
//! product reference.
//!
//! Every terminal keeps the settled owner occurrence, the observed winner, and
//! the next actions derived from the attempt's own progress. The two custody
//! shapes differ only in how far the attempt got before it lost: an attempt
//! that lost at the expected-observation comparison never constructed
//! product-head authority over the branch cell, while an attempt that lost the
//! product CAS is handed its authority back by the cell.

use std::sync::Arc;

use crate::branch::{
    ProductBranchHeadProtection, ProductBranchObservation, ProductBranchReferencePublishFailure,
    ProductBranchReferenceSnapshot,
};
use crate::history::{
    CompositeHistoryCatalog, CompositeRuntimeWorldCommit,
    ProductUnpublishedHistoryProtectionObligation, ReservedCompositeCommitCapacity,
};
use crate::identity::{
    CompositePublicationAttemptIdentity, ProductUnpublishedOwnerEffectsIdentity,
};
use crate::recovery::{
    ProductUnpublishedCause, ProductUnpublishedOwnerEffectSummary, ProductUnpublishedOwnerEffects,
    ReservedProductUnpublishedSlot, RetainedAttemptFacts, RetainedRecordCharges,
    RetainedSuccessorEvidence,
};
use crate::retention::{PublicationRetentionObligation, RetainedPartialRetentionObligation};

use super::super::{
    CompositeAttemptProgress, CompositeOwnerExecutionResults, CompositePublicationCostCounters,
    RuntimeWorldPublicationOutcome,
};
use super::CompositePublicationReadyInputs;

/// The reserved recovery slot, the installed successor commit protection, and
/// the exact component pin pair are the three obligations a retained record
/// keeps live when it kept its successor installed. A record that released its
/// history slot instead keeps one fewer, which is why the count is derived from
/// the custody rather than restated at each terminal.
pub(super) const PRODUCT_UNPUBLISHED_LIVE_OBLIGATION_COUNT: usize = 3;

fn live_obligation_count(
    successor_history: Option<&ProductUnpublishedHistoryProtectionObligation>,
) -> usize {
    PRODUCT_UNPUBLISHED_LIVE_OBLIGATION_COUNT - usize::from(successor_history.is_none())
}

/// The parts of a ready attempt that outlive its product-CAS decision. Both
/// terminals take exactly these; nothing here is re-derived or re-observed.
pub(super) struct AttemptTerminal {
    pub(super) attempt_identity: CompositePublicationAttemptIdentity,
    pub(super) identity: ProductUnpublishedOwnerEffectsIdentity,
    pub(super) expected_head: ProductBranchObservation,
    pub(super) progress: CompositeAttemptProgress,
    pub(super) commit: Arc<CompositeRuntimeWorldCommit>,
    pub(super) owner_results: CompositeOwnerExecutionResults,
    pub(super) recovery_slot: ReservedProductUnpublishedSlot,
    pub(super) operation: crate::lifecycle::owner::RuntimeWorldOperationReservation,
    pub(super) deadline: Option<crate::lifecycle::RuntimeWorldInstant>,
    pub(super) counters: CompositePublicationCostCounters,
}

/// The reserved history slot and the successor-basis publication retention an
/// attempt still holds before it has materialized anything. Exactly one of
/// materialization and retained settlement consumes it.
pub(super) struct UnmaterializedSuccessor {
    pub(super) history: CompositeHistoryCatalog,
    pub(super) reserved_commit_capacity: ReservedCompositeCommitCapacity,
    pub(super) publication_retention: PublicationRetentionObligation,
}

/// The successor custody a retained terminal has to convert into recovery
/// custody.
pub(super) enum RetainedSuccessorCustody {
    /// The attempt lost before materializing its commit into the reserved
    /// history slot, so it still holds the reservation itself and has taken no
    /// product-head authority over the branch cell.
    Unmaterialized(UnmaterializedSuccessor),
    /// The attempt reached the product CAS and the cell handed its
    /// already-materialized product-head authority back.
    ProductHeadAuthority(ProductBranchHeadProtection),
}

impl RetainedSuccessorCustody {
    /// Take apart a lost product CAS: the cell names the winner it holds and
    /// hands this attempt's product-head authority back for retained custody.
    pub(super) fn from_cas_loss(
        failure: ProductBranchReferencePublishFailure,
    ) -> (ProductBranchReferenceSnapshot, Self) {
        let (winner_head, protection) = failure.into_recovery_parts();
        (winner_head, Self::ProductHeadAuthority(protection))
    }
}

impl AttemptTerminal {
    /// Split a ready attempt into the successor custody it may still
    /// materialize and the parts every terminal keeps.
    pub(super) fn split(ready: CompositePublicationReadyInputs) -> (UnmaterializedSuccessor, Self) {
        let CompositePublicationReadyInputs {
            identity,
            expected_head,
            commit,
            owner_results,
            progress,
            product_unpublished_identity,
            reserved_commit_capacity,
            reserved_recovery_slot,
            reserved_publication_capacity: _,
            history,
            operation,
            publication_retention,
            cancellation: _,
            deadline,
            counters,
        } = ready;
        (
            UnmaterializedSuccessor {
                history,
                reserved_commit_capacity,
                publication_retention,
            },
            Self {
                attempt_identity: identity,
                identity: product_unpublished_identity,
                expected_head,
                progress,
                commit,
                owner_results,
                recovery_slot: reserved_recovery_slot,
                operation,
                deadline,
                counters,
            },
        )
    }

    /// Terminate as retained owner effects that name the head this attempt
    /// observed, the cause it lost for, and the next actions derived from its
    /// own progress.
    pub(super) fn retain(
        mut self,
        observed_head: ProductBranchReferenceSnapshot,
        cause: ProductUnpublishedCause,
        custody: RetainedSuccessorCustody,
    ) -> RuntimeWorldPublicationOutcome {
        self.operation
            .begin_recovery()
            .expect("a publication that did not move the product reference enters recovery");
        let (retention, successor_history) = match custody {
            RetainedSuccessorCustody::Unmaterialized(successor) => {
                settle_unmaterialized(successor, &self.commit)
            }
            RetainedSuccessorCustody::ProductHeadAuthority(protection) => {
                settle_product_head_authority(protection)
            }
        };
        let summary = ProductUnpublishedOwnerEffectSummary::from_progress(
            &self.progress,
            live_obligation_count(successor_history.as_ref()),
            0,
        );
        RuntimeWorldPublicationOutcome::ProductUnpublished(
            ProductUnpublishedOwnerEffects::new_retained(
                RetainedAttemptFacts {
                    identity: self.identity,
                    attempt_identity: self.attempt_identity,
                    expected_head: self.expected_head,
                    last_observed_head: Some(observed_head),
                    progress: self.progress,
                    owner_results: self.owner_results,
                    // A product CAS moves the head of a branch that already
                    // exists. The loser reserved no occurrence, so it charged
                    // no custody and a cleanup has nothing to drain.
                    destination: None,
                },
                RetainedSuccessorEvidence {
                    basis: Some(self.commit.basis().clone()),
                    history_protection: successor_history,
                },
                retention,
                RetainedRecordCharges {
                    recovery_slot: self.recovery_slot,
                    summary,
                    cause,
                    deadline: self.deadline,
                },
            ),
        )
    }
}

/// Release the reserved history slot a pre-movement loser never used, and take
/// its component pins into retained custody.
///
/// This attempt lost before the product reference could move, so its successor
/// occurrence is never installed: spending a bounded history slot on a commit
/// no product head names and no caller can reach would charge the budget for
/// evidence the record does not need. The successor basis it retains is that
/// evidence, and the slot goes back to the catalog when the reservation drops.
///
/// The component pins are taken as a product-head transfer and relabelled by
/// `transition_to_retained_partial`. That is not authority over the product
/// reference: the attempt takes no `ProductBranchHeadProtection` over the cell
/// and advances no product reference generation.
fn settle_unmaterialized(
    successor: UnmaterializedSuccessor,
    commit: &Arc<CompositeRuntimeWorldCommit>,
) -> (
    RetainedPartialRetentionObligation,
    Option<ProductUnpublishedHistoryProtectionObligation>,
) {
    let UnmaterializedSuccessor {
        history,
        reserved_commit_capacity,
        publication_retention,
    } = successor;
    drop(reserved_commit_capacity);
    drop(history);
    let (product_head, _receipt) = publication_retention
        .into_product_head_transfer(commit.basis())
        .expect("ready publication retention is bound to the exact successor basis")
        .into_parts();
    (product_head.transition_to_retained_partial(), None)
}

/// Take back the authority the product cell returned after a lost CAS.
fn settle_product_head_authority(
    protection: ProductBranchHeadProtection,
) -> (
    RetainedPartialRetentionObligation,
    Option<ProductUnpublishedHistoryProtectionObligation>,
) {
    let (_snapshot, product_head, product_history, _receipt) = protection.into_parts();
    (
        product_head.transition_to_retained_partial(),
        Some(product_history.transition_to_product_unpublished()),
    )
}
