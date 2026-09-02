use crate::basis::AdmittedCompositeRuntimeWorldBasis;
use crate::branch::ProductBranchObservation;
use crate::identity::{
    CompositePublicationAttemptIdentity, ProductUnpublishedOwnerEffectsIdentity,
};
use crate::lifecycle::RuntimeWorldInstant;
use crate::publication::CompositeAttemptProgress;
use crate::retention::RetainedPartialRetentionObligation;

use super::{
    ProductUnpublishedNextAction, ProductUnpublishedOwnerEffectSummary,
    ReservedProductUnpublishedSlot,
};

/// Why at least one owner effect survived without a product-reference move.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProductUnpublishedCause {
    SiblingOwnerDenied,
    SettlementPending,
    CancellationAfterEffect,
    StaleProductHead,
    OwnerLost,
    ProductPublicationLost,
}

/// A non-authorizing handle that lets a caller retain or inspect a specific
/// recovery record without turning it into a product commit.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProductUnpublishedRecoveryHandle {
    identity: ProductUnpublishedOwnerEffectsIdentity,
}

impl ProductUnpublishedRecoveryHandle {
    pub(crate) const fn new(identity: ProductUnpublishedOwnerEffectsIdentity) -> Self {
        Self { identity }
    }

    pub fn identity(&self) -> &ProductUnpublishedOwnerEffectsIdentity {
        &self.identity
    }
}

/// Retained record for owner-local effects that were not represented by a
/// product-reference movement. It is not a commit, rollback token, or replay
/// artifact.
#[must_use = "product-unpublished owner effects must remain retained or be explicitly cleaned up"]
pub struct ProductUnpublishedOwnerEffects {
    identity: ProductUnpublishedOwnerEffectsIdentity,
    attempt_identity: CompositePublicationAttemptIdentity,
    expected_head: ProductBranchObservation,
    last_observed_head: Option<ProductBranchObservation>,
    progress: CompositeAttemptProgress,
    successor_basis: Option<AdmittedCompositeRuntimeWorldBasis>,
    retention_obligation: RetainedPartialRetentionObligation,
    recovery_slot: ReservedProductUnpublishedSlot,
    live_obligations: usize,
    cause: ProductUnpublishedCause,
    next_actions: Vec<ProductUnpublishedNextAction>,
    deadline: Option<RuntimeWorldInstant>,
    age_ticks: u64,
    owner_effect_count: usize,
    metadata_bytes: usize,
}

impl std::fmt::Debug for ProductUnpublishedOwnerEffects {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ProductUnpublishedOwnerEffects")
            .field("identity", &self.identity)
            .field("attempt_identity", &self.attempt_identity)
            .field("cause", &self.cause)
            .field("next_actions", &self.next_actions)
            .field("live_obligations", &self.live_obligations)
            .field("owner_effect_count", &self.owner_effect_count)
            .field("metadata_bytes", &self.metadata_bytes)
            .finish_non_exhaustive()
    }
}

impl ProductUnpublishedOwnerEffects {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        identity: ProductUnpublishedOwnerEffectsIdentity,
        attempt_identity: CompositePublicationAttemptIdentity,
        expected_head: ProductBranchObservation,
        last_observed_head: Option<ProductBranchObservation>,
        progress: CompositeAttemptProgress,
        successor_basis: Option<AdmittedCompositeRuntimeWorldBasis>,
        retention_obligation: RetainedPartialRetentionObligation,
        recovery_slot: ReservedProductUnpublishedSlot,
        summary: ProductUnpublishedOwnerEffectSummary,
        cause: ProductUnpublishedCause,
        next_actions: Vec<ProductUnpublishedNextAction>,
        deadline: Option<RuntimeWorldInstant>,
        age_ticks: u64,
    ) -> Self {
        Self {
            identity,
            attempt_identity,
            expected_head,
            last_observed_head,
            progress,
            successor_basis,
            retention_obligation,
            recovery_slot,
            live_obligations: summary.live_obligation_count,
            cause,
            next_actions,
            deadline,
            age_ticks,
            owner_effect_count: summary.owner_effect_count,
            metadata_bytes: summary.metadata_bytes,
        }
    }

    pub fn identity(&self) -> &ProductUnpublishedOwnerEffectsIdentity {
        &self.identity
    }

    pub fn attempt_identity(&self) -> &CompositePublicationAttemptIdentity {
        &self.attempt_identity
    }

    pub fn expected_head(&self) -> &ProductBranchObservation {
        &self.expected_head
    }

    pub fn last_observed_head(&self) -> Option<&ProductBranchObservation> {
        self.last_observed_head.as_ref()
    }

    pub fn progress(&self) -> &CompositeAttemptProgress {
        &self.progress
    }

    pub fn successor_basis(&self) -> Option<&AdmittedCompositeRuntimeWorldBasis> {
        self.successor_basis.as_ref()
    }

    pub(crate) fn retention_obligation(&self) -> &RetainedPartialRetentionObligation {
        &self.retention_obligation
    }

    pub fn live_obligation_count(&self) -> usize {
        self.live_obligations
    }

    pub const fn cause(&self) -> ProductUnpublishedCause {
        self.cause
    }

    pub fn next_actions(&self) -> &[ProductUnpublishedNextAction] {
        &self.next_actions
    }

    pub fn deadline(&self) -> Option<RuntimeWorldInstant> {
        self.deadline
    }

    pub const fn age_ticks(&self) -> u64 {
        self.age_ticks
    }

    pub const fn owner_effect_count(&self) -> usize {
        self.owner_effect_count
    }

    pub const fn metadata_bytes(&self) -> usize {
        self.metadata_bytes
    }

    pub fn recovery_handle(&self) -> ProductUnpublishedRecoveryHandle {
        ProductUnpublishedRecoveryHandle::new(self.identity.clone())
    }
}
