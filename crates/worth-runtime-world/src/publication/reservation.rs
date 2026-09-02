use crate::basis::AdmittedCompositeRuntimeWorldBasis;
use crate::branch::ProductBranchObservation;
use crate::budget::RuntimeWorldBudgetLimit;
use crate::identity::{CompositeCommitIdentity, CompositePublicationAttemptIdentity};
use crate::lifecycle::RuntimeWorldInstant;
use crate::retention::PublicationRetentionObligation;

use super::{
    CompositeAttemptProgress, LoweredOwnerComponentPlan, NoEffectCause,
    NoEffectCompositePublication, OwnerExecutionSettlement,
};

/// Component calls have one fixed order. There is no cross-owner lock held
/// across those calls.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompositePublicationOrder {
    RelationalThenSignal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompositeAttemptCancellationPosture {
    Open,
    CancellationObserved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ReservedHistorySlot {
    pub(crate) limit: RuntimeWorldBudgetLimit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ReservedRecoverySlot {
    pub(crate) limit: RuntimeWorldBudgetLimit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ReservedPinAcquisitionSlots {
    pub(crate) limit: RuntimeWorldBudgetLimit,
}

/// Reserved, attempt-affine state between lowering and owner execution.
///
/// This type deliberately has no public constructor, `Clone`, `Copy`, or
/// serialization implementation. Its fields name every bounded resource and
/// the exact expected head carried through execution.
#[must_use = "a reserved publication attempt must be executed or cleaned up"]
pub struct ReservedCompositePublicationAttempt {
    identity: CompositePublicationAttemptIdentity,
    expected_head: ProductBranchObservation,
    predecessor_basis: AdmittedCompositeRuntimeWorldBasis,
    plan: LoweredOwnerComponentPlan,
    reserved_commit_identity: CompositeCommitIdentity,
    reserved_history_slot: ReservedHistorySlot,
    reserved_recovery_slot: ReservedRecoverySlot,
    reserved_pin_acquisition_slots: ReservedPinAcquisitionSlots,
    /// Opaque Phase 2 retention handoff for the prospective successor basis.
    /// Phase 1 does not claim a component lease for either basis.
    retention_obligation: PublicationRetentionObligation,
    cancellation: CompositeAttemptCancellationPosture,
    deadline: Option<RuntimeWorldInstant>,
    order: CompositePublicationOrder,
    progress: CompositeAttemptProgress,
}

impl std::fmt::Debug for ReservedCompositePublicationAttempt {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ReservedCompositePublicationAttempt")
            .field("identity", &self.identity)
            .field("expected_head", &self.expected_head)
            .field("reserved_commit_identity", &self.reserved_commit_identity)
            .field("cancellation", &self.cancellation)
            .field("deadline", &self.deadline)
            .field("order", &self.order)
            .field("progress", &self.progress)
            .finish_non_exhaustive()
    }
}

impl ReservedCompositePublicationAttempt {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        identity: CompositePublicationAttemptIdentity,
        expected_head: ProductBranchObservation,
        predecessor_basis: AdmittedCompositeRuntimeWorldBasis,
        plan: LoweredOwnerComponentPlan,
        reserved_commit_identity: CompositeCommitIdentity,
        reserved_history_slot: ReservedHistorySlot,
        reserved_recovery_slot: ReservedRecoverySlot,
        reserved_pin_acquisition_slots: ReservedPinAcquisitionSlots,
        retention_obligation: PublicationRetentionObligation,
        deadline: Option<RuntimeWorldInstant>,
    ) -> Self {
        Self {
            identity,
            expected_head,
            predecessor_basis,
            plan,
            reserved_commit_identity,
            reserved_history_slot,
            reserved_recovery_slot,
            reserved_pin_acquisition_slots,
            retention_obligation,
            cancellation: CompositeAttemptCancellationPosture::Open,
            deadline,
            order: CompositePublicationOrder::RelationalThenSignal,
            progress: CompositeAttemptProgress::untouched(),
        }
    }

    pub fn identity(&self) -> &CompositePublicationAttemptIdentity {
        &self.identity
    }

    pub fn expected_head(&self) -> &ProductBranchObservation {
        &self.expected_head
    }

    pub fn predecessor_basis(&self) -> &AdmittedCompositeRuntimeWorldBasis {
        &self.predecessor_basis
    }

    pub fn order(&self) -> CompositePublicationOrder {
        self.order
    }

    pub fn progress(&self) -> &CompositeAttemptProgress {
        &self.progress
    }

    pub fn cancellation_posture(&self) -> CompositeAttemptCancellationPosture {
        self.cancellation
    }

    pub(crate) fn retention_obligation(&self) -> &PublicationRetentionObligation {
        &self.retention_obligation
    }

    /// Consume a still-pre-effect reservation into the only no-effect
    /// cancellation terminal. Owner execution cannot be reached afterward.
    pub fn cancel(self) -> NoEffectCompositePublication {
        NoEffectCompositePublication::new(
            NoEffectCause::CancelledBeforeEffect,
            Some(self.expected_head),
        )
    }

    pub(crate) fn observe_cancellation(&mut self) {
        self.cancellation = CompositeAttemptCancellationPosture::CancellationObserved;
    }

    pub(crate) fn settle(self, progress: CompositeAttemptProgress) -> OwnerExecutionSettlement {
        OwnerExecutionSettlement::new(self, progress)
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        CompositePublicationAttemptIdentity,
        ProductBranchObservation,
        AdmittedCompositeRuntimeWorldBasis,
        LoweredOwnerComponentPlan,
        CompositeCommitIdentity,
        ReservedHistorySlot,
        ReservedRecoverySlot,
        ReservedPinAcquisitionSlots,
        PublicationRetentionObligation,
        CompositeAttemptCancellationPosture,
        Option<RuntimeWorldInstant>,
        CompositePublicationOrder,
        CompositeAttemptProgress,
    ) {
        (
            self.identity,
            self.expected_head,
            self.predecessor_basis,
            self.plan,
            self.reserved_commit_identity,
            self.reserved_history_slot,
            self.reserved_recovery_slot,
            self.reserved_pin_acquisition_slots,
            self.retention_obligation,
            self.cancellation,
            self.deadline,
            self.order,
            self.progress,
        )
    }
}
