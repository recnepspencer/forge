use super::ForkedBranchRecoveryContext;

use crate::branch::ProductBranchReferenceSnapshot;
use crate::history::{
    ProductHeadHistoryProtectionObligation, ProductUnpublishedHistoryProtectionObligation,
};
use crate::recovery::{
    InstalledSuccessorEvidence, PendingRetentionCustody, ProductUnpublishedCause,
    ProductUnpublishedOwnerEffectSummary, ProductUnpublishedOwnerEffects, RetainedAttemptFacts,
    RetainedRecordCharges, RetainedSuccessorEvidence,
};
use crate::retention::{
    PublicationRetentionObligation, ReservedComponentPinPairCapacity,
    RetainedPartialRetentionObligation, RetentionObligationDenial,
};

/// How a settled fork that cannot install its product reference names what
/// stopped it: the cause its record carries and, when a moved source head is
/// that cause, the head that won. The route is reached either by losing an
/// owner-issued authority the fork already depended on, or by a source head
/// that moved, or was retired, between the fork's last recheck and its
/// installation under the source guard.
pub(super) struct RetainedForkNaming {
    pub(super) cause: ProductUnpublishedCause,
    pub(super) last_observed_head: Option<ProductBranchReferenceSnapshot>,
}

impl RetainedForkNaming {
    /// Losing an owner-issued authority the fork already depended on. No
    /// head displaced the fork, so there is no winner to name.
    pub(super) const fn owner_lost() -> Self {
        Self {
            cause: ProductUnpublishedCause::OwnerLost,
            last_observed_head: None,
        }
    }
}

pub(super) fn retain_forked_effects(
    context: ForkedBranchRecoveryContext,
    publication: PublicationRetentionObligation,
    product_history: ProductHeadHistoryProtectionObligation,
) -> ProductUnpublishedOwnerEffects {
    let transfer = publication
        .into_product_head_transfer(&context.successor_basis)
        .expect("retained branch publication matches the successor basis");
    let (product_head, _) = transfer.into_parts();
    let retained = product_head.transition_to_retained_partial();
    let successor_history = product_history.transition_to_product_unpublished();
    retained_effects(
        context,
        retained,
        successor_history,
        RetainedForkNaming::owner_lost(),
    )
}

pub(super) fn retain_from_protection(
    context: ForkedBranchRecoveryContext,
    protection: crate::branch::ProductBranchHeadProtection,
    naming: RetainedForkNaming,
) -> ProductUnpublishedOwnerEffects {
    let (_snapshot, product_head, product_history, _receipt) = protection.into_parts();
    let retained = product_head.transition_to_retained_partial();
    let successor_history = product_history.transition_to_product_unpublished();
    retained_effects(context, retained, successor_history, naming)
}

fn retained_effects(
    context: ForkedBranchRecoveryContext,
    retained: RetainedPartialRetentionObligation,
    successor_history: ProductUnpublishedHistoryProtectionObligation,
    naming: RetainedForkNaming,
) -> ProductUnpublishedOwnerEffects {
    let summary = retained_summary(&context);
    let ForkedBranchRecoveryContext {
        identity,
        attempt_identity,
        expected_head,
        progress,
        successor_basis,
        owner_results,
        recovery_slot,
        deadline,
        destination,
    } = context;
    let RetainedForkNaming {
        cause,
        last_observed_head,
    } = naming;
    #[cfg(test)]
    super::test_control::pause_before_forked_recovery_record(&identity);
    ProductUnpublishedOwnerEffects::new_retained(
        RetainedAttemptFacts {
            identity,
            attempt_identity,
            expected_head,
            last_observed_head,
            progress,
            owner_results,
            destination: Some(destination),
        },
        RetainedSuccessorEvidence {
            basis: Some(successor_basis),
            history_protection: Some(successor_history),
        },
        retained,
        RetainedRecordCharges {
            recovery_slot,
            summary,
            cause,
            deadline,
        },
    )
}

pub(super) fn product_unpublished_pending(
    context: ForkedBranchRecoveryContext,
    capacity: ReservedComponentPinPairCapacity,
    denial: RetentionObligationDenial,
    product_history: ProductHeadHistoryProtectionObligation,
) -> ProductUnpublishedOwnerEffects {
    let successor_history = product_history.transition_to_product_unpublished();
    let summary = retained_summary(&context);
    let ForkedBranchRecoveryContext {
        identity,
        attempt_identity,
        expected_head,
        progress,
        successor_basis,
        owner_results,
        recovery_slot,
        deadline,
        destination,
    } = context;
    #[cfg(test)]
    super::test_control::pause_before_forked_recovery_record(&identity);
    ProductUnpublishedOwnerEffects::new_reacquisition_pending(
        RetainedAttemptFacts {
            identity,
            attempt_identity,
            expected_head,
            last_observed_head: None,
            progress,
            owner_results,
            destination: Some(destination),
        },
        InstalledSuccessorEvidence {
            basis: successor_basis,
            history_protection: successor_history,
        },
        PendingRetentionCustody { capacity, denial },
        RetainedRecordCharges {
            recovery_slot,
            summary,
            cause: RetainedForkNaming::owner_lost().cause,
            deadline,
        },
    )
}

/// Every product-unpublished forked-branch record charges the same
/// record-shaped metadata as the publication retention route. Its live
/// obligations are not named here at all: the record counts them from the
/// custody it is installed with.
fn retained_summary(context: &ForkedBranchRecoveryContext) -> ProductUnpublishedOwnerEffectSummary {
    ProductUnpublishedOwnerEffectSummary::from_progress(
        &context.progress,
        ProductUnpublishedOwnerEffects::metadata_charge_hint(),
    )
}
