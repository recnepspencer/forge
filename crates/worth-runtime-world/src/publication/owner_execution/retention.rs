use std::sync::Arc;

use crate::basis::AdmittedCompositeRuntimeWorldBasis;
use crate::branch::{ProductBranchObservation, ProductBranchReferenceSnapshot};
use crate::history::{
    CompositeHistoryCatalog, CompositeRuntimeWorldCommit,
    ProductUnpublishedHistoryProtectionObligation, ReservedCompositeCommitCapacity,
};
use crate::identity::{
    CompositePublicationAttemptIdentity, ProductUnpublishedOwnerEffectsIdentity,
};
use crate::lifecycle::RuntimeWorldInstant;
use crate::recovery::{
    next_actions_for_progress, ProductUnpublishedCause, ProductUnpublishedOwnerEffectSummary,
    ProductUnpublishedOwnerEffects, ReservedProductUnpublishedSlot,
};
use crate::retention::{
    ReservedComponentPinPairCapacity, RetainedPartialRetentionObligation, RetentionObligationDenial,
};

use super::super::{
    CompositeAttemptProgress, CompositeOwnerExecutionResults, ReservedAttemptCapacities,
};
use super::RETENTION_PENDING_LIVE_OBLIGATION_COUNT;

/// The reserved resources one retained attempt still owns. Publication and
/// branch creation reach retention with the same bundle, so a retained record
/// has exactly one authority no matter which operation produced it.
pub(crate) struct RetainedAttemptInputs {
    pub(crate) attempt_identity: CompositePublicationAttemptIdentity,
    pub(crate) expected_head: ProductBranchObservation,
    pub(crate) capacities: ReservedAttemptCapacities,
    pub(crate) deadline: Option<RuntimeWorldInstant>,
}

/// What one retained attempt actually did, as distinct from the reserved
/// resources it still owns. Publication and creation retention are told the
/// same story, so neither can name a terminal the other could not.
pub(crate) struct RetainedOwnerEffectInputs {
    pub(crate) progress: CompositeAttemptProgress,
    pub(crate) successor_basis: AdmittedCompositeRuntimeWorldBasis,
    pub(crate) owner_results: CompositeOwnerExecutionResults,
    pub(crate) cause: ProductUnpublishedCause,
    pub(crate) last_observed_head: Option<ProductBranchReferenceSnapshot>,
}

/// Install one retained owner-effect image. The caller has already checked
/// that the owner results match the plan its own operation reserved; this body
/// owns the commit installation, history protection and pin binding.
pub(crate) fn retain_attempt_effects(
    inputs: RetainedAttemptInputs,
    effects: RetainedOwnerEffectInputs,
) -> ProductUnpublishedOwnerEffects {
    let RetainedAttemptInputs {
        attempt_identity,
        expected_head,
        capacities,
        deadline,
    } = inputs;
    let (
        reserved_commit_identity,
        product_unpublished_identity,
        reserved_commit_capacity,
        reserved_recovery_slot,
        reserved_component_pin_pair,
        _reserved_publication_capacity,
        history,
        mut operation,
    ) = capacities.into_parts();
    let commit = Arc::new(
        CompositeRuntimeWorldCommit::from_ordinary_publication(
            reserved_commit_identity,
            expected_head.snapshot().commit(),
            effects.successor_basis.clone(),
            attempt_identity.clone(),
            &effects.owner_results,
            None,
        )
        .expect("owner-issued pending evidence forms the retained commit occurrence"),
    );
    let successor_history = install_retained_commit(history, reserved_commit_capacity, &commit);
    let summary = ProductUnpublishedOwnerEffectSummary::from_progress(
        &effects.progress,
        RETENTION_PENDING_LIVE_OBLIGATION_COUNT,
        ProductUnpublishedOwnerEffects::metadata_charge_hint(),
    );
    operation
        .begin_recovery()
        .expect("a settled owner attempt enters retained recovery");
    let terminal = RetainedTerminal {
        product_unpublished_identity,
        attempt_identity,
        expected_head,
        successor_history,
        reserved_recovery_slot,
        summary,
        deadline,
        effects,
    };
    terminal.bind(reserved_component_pin_pair, &commit)
}

/// Install the retained successor occurrence and take the product-unpublished
/// protection it needs. The commit is real owner evidence, so it is installed
/// before any terminal is chosen, and the rollback is armed until it is.
pub(super) fn install_retained_commit(
    history: CompositeHistoryCatalog,
    capacity: ReservedCompositeCommitCapacity,
    commit: &Arc<CompositeRuntimeWorldCommit>,
) -> ProductUnpublishedHistoryProtectionObligation {
    let entry = capacity
        .install(Arc::clone(commit))
        .expect("the reserved commit installs into its exact history slot");
    let installed_rollback = history.arm_installed_commit_rollback(entry.identity());
    let successor_history = history
        .protect_product_head(entry.commit())
        .expect("the retained successor admits exact history protection")
        .transition_to_product_unpublished();
    installed_rollback.commit();
    successor_history
}

/// Everything the retained record needs once the commit is installed. Only the
/// component pin binding is still undecided, and both of its answers are
/// retained records: a pin denial here is post-effect, never a no-effect.
struct RetainedTerminal {
    product_unpublished_identity: ProductUnpublishedOwnerEffectsIdentity,
    attempt_identity: CompositePublicationAttemptIdentity,
    expected_head: ProductBranchObservation,
    successor_history: ProductUnpublishedHistoryProtectionObligation,
    reserved_recovery_slot: ReservedProductUnpublishedSlot,
    summary: ProductUnpublishedOwnerEffectSummary,
    deadline: Option<RuntimeWorldInstant>,
    effects: RetainedOwnerEffectInputs,
}

impl RetainedTerminal {
    fn bind(
        self,
        pins: ReservedComponentPinPairCapacity,
        commit: &CompositeRuntimeWorldCommit,
    ) -> ProductUnpublishedOwnerEffects {
        match pins.bind_publication(commit.basis()) {
            Ok(publication) => {
                let transfer = publication
                    .into_product_head_transfer(commit.basis())
                    .expect("the reserved publication pins match the retained basis");
                let (product_head, _) = transfer.into_parts();
                self.into_retained(product_head.transition_to_retained_partial())
            }
            Err((capacity, denial)) => self.into_reacquisition_pending(capacity, denial),
        }
    }

    /// The retained terminal that owns its component pins outright.
    fn into_retained(
        self,
        retention_obligation: RetainedPartialRetentionObligation,
    ) -> ProductUnpublishedOwnerEffects {
        let next_actions = next_actions_for_progress(&self.effects.progress);
        ProductUnpublishedOwnerEffects::new_retained(
            self.product_unpublished_identity,
            self.attempt_identity,
            self.expected_head,
            self.effects.last_observed_head,
            self.effects.progress,
            Some(self.effects.successor_basis),
            self.effects.owner_results,
            retention_obligation,
            self.successor_history,
            self.reserved_recovery_slot,
            self.summary,
            self.effects.cause,
            next_actions,
            self.deadline,
            0,
        )
    }

    /// The retained terminal whose pins could not be reacquired. The owner
    /// effects are just as real; the record carries the denial that names why.
    fn into_reacquisition_pending(
        self,
        capacity: ReservedComponentPinPairCapacity,
        denial: RetentionObligationDenial,
    ) -> ProductUnpublishedOwnerEffects {
        ProductUnpublishedOwnerEffects::new_reacquisition_pending(
            self.product_unpublished_identity,
            self.attempt_identity,
            self.expected_head,
            self.effects.last_observed_head,
            self.effects.progress,
            self.effects.successor_basis,
            self.effects.owner_results,
            capacity,
            denial,
            self.successor_history,
            self.reserved_recovery_slot,
            self.summary,
            self.effects.cause,
            self.deadline,
        )
    }
}
