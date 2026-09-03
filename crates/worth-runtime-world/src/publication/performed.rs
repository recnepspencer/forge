use std::sync::Arc;

use worth_proof::AuthorityWitness;

use crate::branch::{
    ProductBranchObservation, ProductBranchReferenceMovement, ProductBranchReferenceSnapshot,
};
use crate::history::CompositeRuntimeWorldCommit;
use crate::identity::CompositePublicationAttemptIdentity;
use crate::retention::RetentionTransferReceipt;

use super::{CompositeOwnerExecutionResults, CompositePublicationCostCounters};

worth_proof::authority_marker!(pub(crate) CompositePublicationAuthorityMarker);

/// Cancellation observed after the branch reference moved is retained as
/// evidence; it cannot turn a performed movement back into no-effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompositeLateCancellationPosture {
    NotRequested,
    RequestedBeforeProductMovement,
    RequestedAfterProductMovement,
}

/// Linear proof that one immutable composite commit won the exact product
/// compare-and-publish transition.
#[must_use = "a performed publication must be handed to the product owner"]
pub struct PerformedCompositePublication {
    old_product_head: ProductBranchObservation,
    reference_movement: ProductBranchReferenceMovement,
    commit: Arc<CompositeRuntimeWorldCommit>,
    attempt_identity: CompositePublicationAttemptIdentity,
    component_results: CompositeOwnerExecutionResults,
    late_cancellation: CompositeLateCancellationPosture,
    cost_counters: CompositePublicationCostCounters,
    _authority: AuthorityWitness<CompositePublicationAuthorityMarker>,
}

impl std::fmt::Debug for PerformedCompositePublication {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PerformedCompositePublication")
            .field("commit", &self.commit.identity())
            .field("old_product_head", &self.old_product_head)
            .field("new_product_head", self.reference_movement.after())
            .field("attempt_identity", &self.attempt_identity)
            .field("component_results", &self.component_results)
            .field("late_cancellation", &self.late_cancellation)
            .field("cost_counters", &self.cost_counters)
            .finish_non_exhaustive()
    }
}

impl PerformedCompositePublication {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn owner_issued(
        old_product_head: ProductBranchObservation,
        reference_movement: ProductBranchReferenceMovement,
        commit: Arc<CompositeRuntimeWorldCommit>,
        attempt_identity: CompositePublicationAttemptIdentity,
        component_results: CompositeOwnerExecutionResults,
        late_cancellation: CompositeLateCancellationPosture,
        cost_counters: CompositePublicationCostCounters,
    ) -> Self {
        assert_eq!(reference_movement.before(), old_product_head.snapshot());
        assert_eq!(
            reference_movement.after().selected_commit(),
            commit.identity()
        );
        Self {
            old_product_head,
            reference_movement,
            commit,
            attempt_identity,
            component_results,
            late_cancellation,
            cost_counters,
            _authority: AuthorityWitness::from_authority_marker(
                CompositePublicationAuthorityMarker::seal(),
            ),
        }
    }

    pub fn old_product_head(&self) -> &ProductBranchObservation {
        &self.old_product_head
    }

    pub fn new_product_head(&self) -> &ProductBranchReferenceSnapshot {
        self.reference_movement.after()
    }

    pub fn commit(&self) -> &CompositeRuntimeWorldCommit {
        &self.commit
    }

    pub fn product_head(&self) -> &ProductBranchReferenceSnapshot {
        self.new_product_head()
    }

    pub fn attempt_identity(&self) -> &CompositePublicationAttemptIdentity {
        &self.attempt_identity
    }

    pub fn component_results(&self) -> &CompositeOwnerExecutionResults {
        &self.component_results
    }

    pub const fn late_cancellation(&self) -> CompositeLateCancellationPosture {
        self.late_cancellation
    }

    pub fn cost_counters(&self) -> CompositePublicationCostCounters {
        self.cost_counters
    }

    pub(crate) fn retention_transfer(&self) -> &RetentionTransferReceipt {
        self.reference_movement.retention_transfer()
    }
}
