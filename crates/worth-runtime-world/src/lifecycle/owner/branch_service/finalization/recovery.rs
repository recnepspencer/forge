use super::ForkedBranchRecoveryContext;

use crate::history::{
    ProductHeadHistoryProtectionObligation, ProductUnpublishedHistoryProtectionObligation,
};
use crate::recovery::{
    ProductUnpublishedCause, ProductUnpublishedNextAction, ProductUnpublishedOwnerEffectSummary,
    ProductUnpublishedOwnerEffects,
};
use crate::retention::{
    PublicationRetentionObligation, ReservedComponentPinPairCapacity,
    RetainedPartialRetentionObligation, RetentionObligationDenial,
};

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
    let summary = ProductUnpublishedOwnerEffectSummary::from_progress(&context.progress, 3, 0);
    retained_effects(context, retained, successor_history, summary)
}

pub(super) fn retain_from_protection(
    context: ForkedBranchRecoveryContext,
    protection: crate::branch::ProductBranchHeadProtection,
) -> ProductUnpublishedOwnerEffects {
    let (_snapshot, product_head, product_history, _receipt) = protection.into_parts();
    let retained = product_head.transition_to_retained_partial();
    let successor_history = product_history.transition_to_product_unpublished();
    let summary = ProductUnpublishedOwnerEffectSummary::from_progress(&context.progress, 3, 0);
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
    } = context;
    ProductUnpublishedOwnerEffects::new_retained(
        identity,
        attempt_identity,
        expected_head,
        None,
        progress,
        Some(successor_basis),
        owner_results,
        retained,
        successor_history,
        recovery_slot,
        summary,
        ProductUnpublishedCause::OwnerLost,
        vec![
            ProductUnpublishedNextAction::ReleaseObligations,
            ProductUnpublishedNextAction::Inspect,
        ],
        deadline,
        0,
    )
}

pub(super) fn product_unpublished_pending(
    context: ForkedBranchRecoveryContext,
    capacity: ReservedComponentPinPairCapacity,
    denial: RetentionObligationDenial,
    product_history: ProductHeadHistoryProtectionObligation,
) -> ProductUnpublishedOwnerEffects {
    let successor_history = product_history.transition_to_product_unpublished();
    let summary = ProductUnpublishedOwnerEffectSummary::from_progress(
        &context.progress,
        3,
        ProductUnpublishedOwnerEffects::metadata_charge_hint(),
    );
    let ForkedBranchRecoveryContext {
        identity,
        attempt_identity,
        expected_head,
        progress,
        successor_basis,
        owner_results,
        recovery_slot,
        deadline,
    } = context;
    ProductUnpublishedOwnerEffects::new_reacquisition_pending(
        identity,
        attempt_identity,
        expected_head,
        None,
        progress,
        successor_basis,
        owner_results,
        capacity,
        denial,
        successor_history,
        recovery_slot,
        summary,
        ProductUnpublishedCause::OwnerLost,
        deadline,
    )
}
