use std::sync::Arc;

use crate::basis::AdmittedCompositeRuntimeWorldBasis;
use crate::branch::{ProductBranchObservation, ProductBranchReferenceSnapshot};
use crate::history::ProductUnpublishedHistoryProtectionObligation;
use crate::identity::{
    CompositePublicationAttemptIdentity, ProductUnpublishedOwnerEffectsIdentity,
};
use crate::lifecycle::RuntimeWorldInstant;
use crate::publication::{CompositeAttemptProgress, CompositeOwnerExecutionResults};
use crate::retention::{
    ReservedComponentPinPairCapacity, RetainedPartialRetentionObligation, RetentionObligationDenial,
};

use super::{
    ProductUnpublishedNextAction, ProductUnpublishedOwnerEffectSummary,
    ReservedProductUnpublishedSlot,
};

#[path = "product_unpublished/custody.rs"]
mod custody;

/// Why at least one owner effect survived without a product-reference move.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProductUnpublishedCause {
    SiblingOwnerDenied,
    SettlementPending,
    OwnerSettlementComplete,
    CancellationAfterEffect,
    StaleProductHead,
    OwnerLost,
    ProductPublicationLost,
}

/// Whether recovery already owns exact successor pins or retains the reserved
/// pair capacity needed to retry an owner-denied acquisition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProductUnpublishedRetentionPosture {
    RetainedExact,
    ReacquisitionPending,
}

enum ProductUnpublishedRetentionCustody {
    Retained(RetainedPartialRetentionObligation),
    Pending {
        capacity: ReservedComponentPinPairCapacity,
        denial: RetentionObligationDenial,
    },
}

impl std::fmt::Debug for ProductUnpublishedRetentionCustody {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Retained(obligation) => {
                formatter.debug_tuple("Retained").field(obligation).finish()
            }
            Self::Pending { denial, .. } => formatter
                .debug_struct("Pending")
                .field("denial", denial)
                .finish_non_exhaustive(),
        }
    }
}

/// A non-authorizing handle that lets a caller retain or inspect a specific
/// recovery record without turning it into a product commit.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProductUnpublishedRecoveryHandle {
    identity: ProductUnpublishedOwnerEffectsIdentity,
    catalog_affinity: usize,
}

impl ProductUnpublishedRecoveryHandle {
    pub(crate) const fn new(
        identity: ProductUnpublishedOwnerEffectsIdentity,
        catalog_affinity: usize,
    ) -> Self {
        Self {
            identity,
            catalog_affinity,
        }
    }

    pub fn identity(&self) -> &ProductUnpublishedOwnerEffectsIdentity {
        &self.identity
    }

    pub(crate) const fn catalog_affinity(&self) -> usize {
        self.catalog_affinity
    }
}

/// Catalog-owned recovery custody. The public returned value below is only a
/// view over this allocation, so dropping the caller capability cannot drop
/// the component pins, history protection, or deferred owner route.
#[derive(Debug)]
pub(crate) struct ProductUnpublishedOwnerEffectsRecord {
    identity: ProductUnpublishedOwnerEffectsIdentity,
    attempt_identity: CompositePublicationAttemptIdentity,
    expected_head: ProductBranchObservation,
    last_observed_head: Option<ProductBranchReferenceSnapshot>,
    progress: CompositeAttemptProgress,
    successor_basis: Option<AdmittedCompositeRuntimeWorldBasis>,
    component_results: CompositeOwnerExecutionResults,
    retention: ProductUnpublishedRetentionCustody,
    successor_history_protection: ProductUnpublishedHistoryProtectionObligation,
    catalog_affinity: usize,
    live_obligations: usize,
    cause: ProductUnpublishedCause,
    next_actions: Vec<ProductUnpublishedNextAction>,
    deadline: Option<RuntimeWorldInstant>,
    age_ticks: u64,
    owner_effect_count: usize,
    metadata_bytes: usize,
}

/// Retained record for owner-local effects that were not represented by a
/// product-reference movement. It is not a commit, rollback token, or replay
/// artifact.
#[must_use = "product-unpublished owner effects must remain retained or be explicitly cleaned up"]
pub struct ProductUnpublishedOwnerEffects {
    record: Arc<ProductUnpublishedOwnerEffectsRecord>,
}

impl std::fmt::Debug for ProductUnpublishedOwnerEffects {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ProductUnpublishedOwnerEffects")
            .field("identity", &self.record.identity)
            .field("attempt_identity", &self.record.attempt_identity)
            .field("cause", &self.record.cause)
            .field("next_actions", &self.record.next_actions)
            .field("live_obligations", &self.record.live_obligations)
            .field("owner_effect_count", &self.record.owner_effect_count)
            .field("metadata_bytes", &self.record.metadata_bytes)
            .finish_non_exhaustive()
    }
}

impl ProductUnpublishedOwnerEffects {
    pub(crate) fn from_catalog_record(record: Arc<ProductUnpublishedOwnerEffectsRecord>) -> Self {
        Self { record }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new_retained(
        identity: ProductUnpublishedOwnerEffectsIdentity,
        attempt_identity: CompositePublicationAttemptIdentity,
        expected_head: ProductBranchObservation,
        last_observed_head: Option<ProductBranchReferenceSnapshot>,
        progress: CompositeAttemptProgress,
        successor_basis: Option<AdmittedCompositeRuntimeWorldBasis>,
        component_results: CompositeOwnerExecutionResults,
        retention_obligation: RetainedPartialRetentionObligation,
        successor_history_protection: ProductUnpublishedHistoryProtectionObligation,
        recovery_slot: ReservedProductUnpublishedSlot,
        summary: ProductUnpublishedOwnerEffectSummary,
        cause: ProductUnpublishedCause,
        next_actions: Vec<ProductUnpublishedNextAction>,
        deadline: Option<RuntimeWorldInstant>,
        age_ticks: u64,
    ) -> Self {
        let catalog_affinity = recovery_slot.catalog_affinity();
        let record = Arc::new(ProductUnpublishedOwnerEffectsRecord {
            identity: identity.clone(),
            attempt_identity,
            expected_head,
            last_observed_head,
            progress,
            successor_basis,
            component_results,
            retention: ProductUnpublishedRetentionCustody::Retained(retention_obligation),
            successor_history_protection,
            catalog_affinity,
            live_obligations: summary.live_obligation_count,
            cause,
            next_actions,
            deadline,
            age_ticks,
            owner_effect_count: summary.owner_effect_count,
            metadata_bytes: summary.metadata_bytes,
        });
        install_record(
            recovery_slot,
            identity,
            summary.metadata_bytes,
            Arc::clone(&record),
        );
        Self { record }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new_retention_pending(
        identity: ProductUnpublishedOwnerEffectsIdentity,
        attempt_identity: CompositePublicationAttemptIdentity,
        expected_head: ProductBranchObservation,
        progress: CompositeAttemptProgress,
        successor_basis: AdmittedCompositeRuntimeWorldBasis,
        component_results: CompositeOwnerExecutionResults,
        capacity: ReservedComponentPinPairCapacity,
        denial: RetentionObligationDenial,
        successor_history_protection: ProductUnpublishedHistoryProtectionObligation,
        recovery_slot: ReservedProductUnpublishedSlot,
        summary: ProductUnpublishedOwnerEffectSummary,
        deadline: Option<RuntimeWorldInstant>,
    ) -> Self {
        Self::new_reacquisition_pending(
            identity,
            attempt_identity,
            expected_head,
            progress,
            successor_basis,
            component_results,
            capacity,
            denial,
            successor_history_protection,
            recovery_slot,
            summary,
            ProductUnpublishedCause::OwnerLost,
            deadline,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new_reacquisition_pending(
        identity: ProductUnpublishedOwnerEffectsIdentity,
        attempt_identity: CompositePublicationAttemptIdentity,
        expected_head: ProductBranchObservation,
        progress: CompositeAttemptProgress,
        successor_basis: AdmittedCompositeRuntimeWorldBasis,
        component_results: CompositeOwnerExecutionResults,
        capacity: ReservedComponentPinPairCapacity,
        denial: RetentionObligationDenial,
        successor_history_protection: ProductUnpublishedHistoryProtectionObligation,
        recovery_slot: ReservedProductUnpublishedSlot,
        summary: ProductUnpublishedOwnerEffectSummary,
        cause: ProductUnpublishedCause,
        deadline: Option<RuntimeWorldInstant>,
    ) -> Self {
        let catalog_affinity = recovery_slot.catalog_affinity();
        let record = Arc::new(ProductUnpublishedOwnerEffectsRecord {
            identity: identity.clone(),
            attempt_identity,
            expected_head,
            last_observed_head: None,
            progress,
            successor_basis: Some(successor_basis),
            component_results,
            retention: ProductUnpublishedRetentionCustody::Pending { capacity, denial },
            successor_history_protection,
            catalog_affinity,
            live_obligations: summary.live_obligation_count,
            cause,
            next_actions: vec![
                ProductUnpublishedNextAction::SettleOwnerEffects,
                ProductUnpublishedNextAction::ReleaseObligations,
                ProductUnpublishedNextAction::Inspect,
            ],
            deadline,
            age_ticks: 0,
            owner_effect_count: summary.owner_effect_count,
            metadata_bytes: summary.metadata_bytes,
        });
        install_record(
            recovery_slot,
            identity,
            summary.metadata_bytes,
            Arc::clone(&record),
        );
        Self { record }
    }

    pub fn identity(&self) -> &ProductUnpublishedOwnerEffectsIdentity {
        &self.record.identity
    }

    pub fn attempt_identity(&self) -> &CompositePublicationAttemptIdentity {
        &self.record.attempt_identity
    }

    pub fn expected_head(&self) -> &ProductBranchObservation {
        &self.record.expected_head
    }

    pub fn last_observed_head(&self) -> Option<&ProductBranchReferenceSnapshot> {
        self.record.last_observed_head.as_ref()
    }

    pub fn progress(&self) -> &CompositeAttemptProgress {
        &self.record.progress
    }

    pub fn successor_basis(&self) -> Option<&AdmittedCompositeRuntimeWorldBasis> {
        self.record.successor_basis.as_ref()
    }

    pub fn component_results(&self) -> &CompositeOwnerExecutionResults {
        &self.record.component_results
    }

    /// Exact installed successor occurrence retained by this recovery record.
    /// The identity is evidence only; it grants no History or publication authority.
    pub fn successor_commit(&self) -> &crate::identity::CompositeCommitIdentity {
        self.record.successor_history_protection.commit_identity()
    }

    pub fn retention_posture(&self) -> ProductUnpublishedRetentionPosture {
        match &self.record.retention {
            ProductUnpublishedRetentionCustody::Retained(_) => {
                ProductUnpublishedRetentionPosture::RetainedExact
            }
            ProductUnpublishedRetentionCustody::Pending { .. } => {
                ProductUnpublishedRetentionPosture::ReacquisitionPending
            }
        }
    }

    pub(crate) fn retention_obligation(&self) -> Option<&RetainedPartialRetentionObligation> {
        match &self.record.retention {
            ProductUnpublishedRetentionCustody::Retained(obligation) => Some(obligation),
            ProductUnpublishedRetentionCustody::Pending { .. } => None,
        }
    }

    pub(crate) fn successor_history_protection(
        &self,
    ) -> &ProductUnpublishedHistoryProtectionObligation {
        &self.record.successor_history_protection
    }

    pub fn live_obligation_count(&self) -> usize {
        self.record.live_obligations
    }

    pub fn cause(&self) -> ProductUnpublishedCause {
        self.record.cause
    }

    pub fn next_actions(&self) -> &[ProductUnpublishedNextAction] {
        &self.record.next_actions
    }

    pub fn deadline(&self) -> Option<RuntimeWorldInstant> {
        self.record.deadline
    }

    pub fn age_ticks(&self) -> u64 {
        self.record.age_ticks
    }

    pub fn owner_effect_count(&self) -> usize {
        self.record.owner_effect_count
    }

    pub fn metadata_bytes(&self) -> usize {
        self.record.metadata_bytes
    }

    pub fn recovery_handle(&self) -> ProductUnpublishedRecoveryHandle {
        ProductUnpublishedRecoveryHandle::new(
            self.record.identity.clone(),
            self.record.catalog_affinity,
        )
    }
}

fn install_record(
    recovery_slot: ReservedProductUnpublishedSlot,
    identity: ProductUnpublishedOwnerEffectsIdentity,
    metadata_bytes: usize,
    record: Arc<ProductUnpublishedOwnerEffectsRecord>,
) {
    if let Err((slot, denial)) = recovery_slot.install_record(identity, metadata_bytes, record) {
        drop(slot);
        panic!("reserved recovery capacity could not install its record: {denial:?}");
    }
}
