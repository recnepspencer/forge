use super::ForkedBranchRecoveryContext;

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

/// Why a settled fork that cannot install its product reference is retained.
/// This route is reached only by losing an owner-issued authority the fork
/// already depended on, so the record's cause and the continuation derived
/// from it are named once here rather than restated at each terminal.
const FORKED_RECOVERY_CAUSE: ProductUnpublishedCause = ProductUnpublishedCause::OwnerLost;

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
    let summary = retained_summary(&context);
    retained_effects(context, retained, successor_history, summary)
}

pub(super) fn retain_from_protection(
    context: ForkedBranchRecoveryContext,
    protection: crate::branch::ProductBranchHeadProtection,
) -> ProductUnpublishedOwnerEffects {
    let (_snapshot, product_head, product_history, _receipt) = protection.into_parts();
    let retained = product_head.transition_to_retained_partial();
    let successor_history = product_history.transition_to_product_unpublished();
    let summary = retained_summary(&context);
    retained_effects(context, retained, successor_history, summary)
}

fn retained_effects(
    context: ForkedBranchRecoveryContext,
    retained: RetainedPartialRetentionObligation,
    successor_history: ProductUnpublishedHistoryProtectionObligation,
    summary: ProductUnpublishedOwnerEffectSummary,
) -> ProductUnpublishedOwnerEffects {
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
    ProductUnpublishedOwnerEffects::new_retained(
        RetainedAttemptFacts {
            identity,
            attempt_identity,
            expected_head,
            last_observed_head: None,
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
            cause: FORKED_RECOVERY_CAUSE,
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
            cause: FORKED_RECOVERY_CAUSE,
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
