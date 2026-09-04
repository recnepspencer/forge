use crate::history::{CompositeHistoryCatalog, ReservedCompositeCommitCapacity};
use crate::identity::{CompositeCommitIdentity, ProductUnpublishedOwnerEffectsIdentity};
use crate::lifecycle::owner::{
    ReservedPublicationAttemptCapacity, RuntimeWorldOperationReservation,
};
use crate::recovery::ReservedProductUnpublishedSlot;
use crate::retention::ReservedComponentPinPairCapacity;

/// The bounded reservations every terminal-governed attempt acquires before
/// its first owner effect. Publication and branch creation reserve the same
/// resources under the same rules, so they share one bundle rather than two
/// parallel field lists.
#[derive(Debug)]
#[must_use = "reserved attempt capacities are consumed by a terminal or released on drop"]
pub(crate) struct ReservedAttemptCapacities {
    reserved_commit_identity: CompositeCommitIdentity,
    product_unpublished_identity: ProductUnpublishedOwnerEffectsIdentity,
    reserved_commit_capacity: ReservedCompositeCommitCapacity,
    reserved_recovery_slot: ReservedProductUnpublishedSlot,
    reserved_component_pin_pair: ReservedComponentPinPairCapacity,
    reserved_publication_capacity: ReservedPublicationAttemptCapacity,
    history: CompositeHistoryCatalog,
    operation: RuntimeWorldOperationReservation,
}

pub(crate) struct ReservedAttemptCapacityInputs {
    pub(crate) reserved_commit_identity: CompositeCommitIdentity,
    pub(crate) product_unpublished_identity: ProductUnpublishedOwnerEffectsIdentity,
    pub(crate) reserved_commit_capacity: ReservedCompositeCommitCapacity,
    pub(crate) reserved_recovery_slot: ReservedProductUnpublishedSlot,
    pub(crate) reserved_component_pin_pair: ReservedComponentPinPairCapacity,
    pub(crate) reserved_publication_capacity: ReservedPublicationAttemptCapacity,
    pub(crate) history: CompositeHistoryCatalog,
    pub(crate) operation: RuntimeWorldOperationReservation,
}

impl ReservedAttemptCapacities {
    pub(crate) fn new(inputs: ReservedAttemptCapacityInputs) -> Self {
        let ReservedAttemptCapacityInputs {
            reserved_commit_identity,
            product_unpublished_identity,
            reserved_commit_capacity,
            reserved_recovery_slot,
            reserved_component_pin_pair,
            reserved_publication_capacity,
            history,
            operation,
        } = inputs;
        Self {
            reserved_commit_identity,
            product_unpublished_identity,
            reserved_commit_capacity,
            reserved_recovery_slot,
            reserved_component_pin_pair,
            reserved_publication_capacity,
            history,
            operation,
        }
    }

    pub(crate) const fn reserved_commit_identity(&self) -> &CompositeCommitIdentity {
        &self.reserved_commit_identity
    }

    pub(crate) const fn product_unpublished_identity(
        &self,
    ) -> &ProductUnpublishedOwnerEffectsIdentity {
        &self.product_unpublished_identity
    }

    pub(crate) fn begin_owner_execution(&mut self) {
        self.operation
            .begin_owner_execution()
            .expect("a reserved attempt begins owner execution exactly once");
    }

    pub(crate) fn begin_publication(&mut self) {
        self.operation
            .begin_publication()
            .expect("settled owner execution advances into publication exactly once");
    }

    pub(crate) fn begin_recovery(&mut self) {
        self.operation
            .begin_recovery()
            .expect("a publishing attempt enters recovery exactly once");
    }

    #[allow(clippy::type_complexity)]
    pub(crate) fn into_parts(
        self,
    ) -> (
        CompositeCommitIdentity,
        ProductUnpublishedOwnerEffectsIdentity,
        ReservedCompositeCommitCapacity,
        ReservedProductUnpublishedSlot,
        ReservedComponentPinPairCapacity,
        ReservedPublicationAttemptCapacity,
        CompositeHistoryCatalog,
        RuntimeWorldOperationReservation,
    ) {
        (
            self.reserved_commit_identity,
            self.product_unpublished_identity,
            self.reserved_commit_capacity,
            self.reserved_recovery_slot,
            self.reserved_component_pin_pair,
            self.reserved_publication_capacity,
            self.history,
            self.operation,
        )
    }
}
