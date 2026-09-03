use crate::branch::ProductBranchObservation;
use crate::history::{CompositeHistoryCatalog, ReservedCompositeCommitCapacity};
use crate::identity::{
    CompositeCommitIdentity, CompositePublicationAttemptIdentity,
    ProductUnpublishedOwnerEffectsIdentity,
};
use crate::lifecycle::owner::{
    ReservedPublicationAttemptCapacity, RuntimeWorldOperationReservation,
};
use crate::lifecycle::RuntimeWorldInstant;
use crate::recovery::ReservedProductUnpublishedSlot;
use crate::retention::ReservedComponentPinPairCapacity;

use super::{
    CompositeAttemptProgress, LoweredOwnerComponentPlan, NoEffectCompositePublication,
    OwnerExecutionSettlement,
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

/// Reserved, attempt-affine state between lowering and owner execution. All
/// capacity fields are live owner-issued reservations; no copied limit can
/// authorize the later phases.
#[must_use = "a reserved publication attempt must be executed or cleaned up"]
pub struct ReservedCompositePublicationAttempt {
    identity: CompositePublicationAttemptIdentity,
    expected_head: ProductBranchObservation,
    predecessor_basis: crate::basis::AdmittedCompositeRuntimeWorldBasis,
    plan: LoweredOwnerComponentPlan,
    reserved_commit_identity: CompositeCommitIdentity,
    product_unpublished_identity: ProductUnpublishedOwnerEffectsIdentity,
    reserved_commit_capacity: ReservedCompositeCommitCapacity,
    reserved_recovery_slot: ReservedProductUnpublishedSlot,
    reserved_component_pin_pair: ReservedComponentPinPairCapacity,
    reserved_publication_capacity: ReservedPublicationAttemptCapacity,
    history: CompositeHistoryCatalog,
    operation: RuntimeWorldOperationReservation,
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
            .field(
                "product_unpublished_identity",
                &self.product_unpublished_identity,
            )
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
        predecessor_basis: crate::basis::AdmittedCompositeRuntimeWorldBasis,
        plan: LoweredOwnerComponentPlan,
        reserved_commit_identity: CompositeCommitIdentity,
        product_unpublished_identity: ProductUnpublishedOwnerEffectsIdentity,
        reserved_commit_capacity: ReservedCompositeCommitCapacity,
        reserved_recovery_slot: ReservedProductUnpublishedSlot,
        reserved_component_pin_pair: ReservedComponentPinPairCapacity,
        reserved_publication_capacity: ReservedPublicationAttemptCapacity,
        history: CompositeHistoryCatalog,
        deadline: Option<RuntimeWorldInstant>,
        operation: RuntimeWorldOperationReservation,
    ) -> Self {
        Self {
            identity,
            expected_head,
            predecessor_basis,
            plan,
            reserved_commit_identity,
            product_unpublished_identity,
            reserved_commit_capacity,
            reserved_recovery_slot,
            reserved_component_pin_pair,
            reserved_publication_capacity,
            history,
            operation,
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

    pub fn predecessor_basis(&self) -> &crate::basis::AdmittedCompositeRuntimeWorldBasis {
        &self.predecessor_basis
    }

    pub fn plan(&self) -> &LoweredOwnerComponentPlan {
        &self.plan
    }

    pub fn deadline(&self) -> Option<RuntimeWorldInstant> {
        self.deadline
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

    pub(crate) fn observe_cancellation(&mut self) {
        self.cancellation = CompositeAttemptCancellationPosture::CancellationObserved;
    }

    pub(crate) fn begin_owner_execution(&mut self) {
        self.operation
            .begin_owner_execution()
            .expect("a reserved attempt begins owner execution exactly once");
    }

    pub(crate) fn begin_recovery(&mut self) {
        self.operation
            .begin_recovery()
            .expect("a publishing attempt enters recovery exactly once");
    }

    pub(crate) fn take_relational_candidate(
        &mut self,
    ) -> Option<worth_relational::facade::mvcc::PreparedRelationalCommitCandidate> {
        self.plan.take_relational_candidate()
    }

    /// Consume a still-pre-effect reservation into the only no-effect
    /// cancellation terminal. Dropping the attempt releases every capacity.
    pub fn cancel(self) -> NoEffectCompositePublication {
        NoEffectCompositePublication::new(
            super::NoEffectCause::CancelledBeforeEffect,
            Some(self.expected_head),
        )
    }

    pub(crate) fn settle(mut self, progress: CompositeAttemptProgress) -> OwnerExecutionSettlement {
        self.operation
            .begin_publication()
            .expect("settled owner execution advances into publication exactly once");
        OwnerExecutionSettlement::new(self, progress)
    }

    pub(crate) fn settle_with_successor_basis(
        mut self,
        progress: CompositeAttemptProgress,
        successor_basis: crate::basis::AdmittedCompositeRuntimeWorldBasis,
    ) -> OwnerExecutionSettlement {
        self.operation
            .begin_publication()
            .expect("settled owner execution advances into publication exactly once");
        OwnerExecutionSettlement::with_successor_basis(self, progress, successor_basis)
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        CompositePublicationAttemptIdentity,
        ProductBranchObservation,
        crate::basis::AdmittedCompositeRuntimeWorldBasis,
        LoweredOwnerComponentPlan,
        CompositeCommitIdentity,
        ProductUnpublishedOwnerEffectsIdentity,
        ReservedCompositeCommitCapacity,
        ReservedProductUnpublishedSlot,
        ReservedComponentPinPairCapacity,
        ReservedPublicationAttemptCapacity,
        CompositeHistoryCatalog,
        RuntimeWorldOperationReservation,
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
            self.product_unpublished_identity,
            self.reserved_commit_capacity,
            self.reserved_recovery_slot,
            self.reserved_component_pin_pair,
            self.reserved_publication_capacity,
            self.history,
            self.operation,
            self.cancellation,
            self.deadline,
            self.order,
            self.progress,
        )
    }
}
