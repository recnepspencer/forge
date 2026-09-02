use std::sync::Arc;

use crate::branch::ProductBranchObservation;
use crate::history::{CompositeCommitParent, CompositeRuntimeWorldCommit, OrdinaryParent};
use crate::identity::CompositePublicationAttemptIdentity;
use crate::retention::{PublicationRetentionObligation, RetentionTransferReceipt};

use super::{
    CompositeAttemptProgress, CompositeLateCancellationPosture, CompositeOwnerExecutionResults,
    CompositePublicationCostCounters, NoEffectCause, NoEffectCompositePublication,
    PerformedCompositePublication, ReservedCompositePublicationAttempt,
};

/// Owner effects have been settled into exact progress, but product
/// publication has not yet crossed its final compare-and-publish point.
pub struct OwnerExecutionSettlement {
    attempt: ReservedCompositePublicationAttempt,
    progress: CompositeAttemptProgress,
}

impl std::fmt::Debug for OwnerExecutionSettlement {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("OwnerExecutionSettlement")
            .field("progress", &self.progress)
            .finish_non_exhaustive()
    }
}

impl OwnerExecutionSettlement {
    pub(crate) fn new(
        attempt: ReservedCompositePublicationAttempt,
        progress: CompositeAttemptProgress,
    ) -> Self {
        Self { attempt, progress }
    }

    pub fn progress(&self) -> &CompositeAttemptProgress {
        &self.progress
    }

    /// Consume settled owner work into the final pre-CAS token. The commit is
    /// accepted only when its owner and exact predecessor basis agree with
    /// the reserved attempt. Retention is represented by an opaque Phase 2
    /// handoff; this Phase 1 seam never mints or transfers one.
    pub(crate) fn ready(
        self,
        commit: Arc<CompositeRuntimeWorldCommit>,
    ) -> Result<CompositePublicationReady, NoEffectCompositePublication> {
        let (attempt, progress) = self.into_parts();
        let expected = attempt.expected_head().clone();
        let expected_parent = CompositeCommitParent::Ordinary(OrdinaryParent::new(
            expected.selected_commit().clone(),
        ));
        let (
            attempt_identity,
            expected_head,
            _predecessor_basis,
            _plan,
            reserved_commit_identity,
            _history_slot,
            _recovery_slot,
            _pin_slots,
            retention_obligation,
            _cancellation,
            _deadline,
            _order,
            _reserved_progress,
        ) = attempt.into_parts();
        if commit.identity() != &reserved_commit_identity
            || commit.identity().owner_identity() != expected.owner_identity()
            || commit.basis().owner_identity() != expected.owner_identity()
            || commit.parent() != &expected_parent
        {
            return Err(NoEffectCompositePublication::new(
                NoEffectCause::OwnerDeniedBeforeEffect,
                Some(expected),
            ));
        }
        Ok(CompositePublicationReady::new(
            attempt_identity,
            expected_head,
            commit,
            progress,
            retention_obligation,
        ))
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ReservedCompositePublicationAttempt,
        CompositeAttemptProgress,
    ) {
        (self.attempt, self.progress)
    }
}

/// Final pre-publication phase. The product reference has not moved yet.
pub struct CompositePublicationReady {
    attempt_identity: CompositePublicationAttemptIdentity,
    expected_head: ProductBranchObservation,
    commit: Arc<CompositeRuntimeWorldCommit>,
    progress: CompositeAttemptProgress,
    retention_obligation: PublicationRetentionObligation,
}

impl std::fmt::Debug for CompositePublicationReady {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CompositePublicationReady")
            .field("attempt_identity", &self.attempt_identity)
            .field("expected_head", &self.expected_head)
            .field("progress", &self.progress)
            .finish_non_exhaustive()
    }
}

impl CompositePublicationReady {
    pub(crate) fn new(
        attempt_identity: CompositePublicationAttemptIdentity,
        expected_head: ProductBranchObservation,
        commit: Arc<CompositeRuntimeWorldCommit>,
        progress: CompositeAttemptProgress,
        retention_obligation: PublicationRetentionObligation,
    ) -> Self {
        Self {
            attempt_identity,
            expected_head,
            commit,
            progress,
            retention_obligation,
        }
    }

    pub fn attempt_identity(&self) -> &CompositePublicationAttemptIdentity {
        &self.attempt_identity
    }

    pub fn expected_head(&self) -> &ProductBranchObservation {
        &self.expected_head
    }

    /// Consume the final pre-CAS token into the sole performed-publication
    /// authority after the owner has supplied one coherent new observation.
    /// Phase 1 consumes the reserved opaque obligation and the separately
    /// supplied opaque transfer receipt without proving their affinity. Phase
    /// 2 may bind them privately; Phase 3 must validate that relationship.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn publish(
        self,
        new_product_head: ProductBranchObservation,
        component_results: CompositeOwnerExecutionResults,
        late_cancellation: CompositeLateCancellationPosture,
        retention_transfer: RetentionTransferReceipt,
        cost_counters: CompositePublicationCostCounters,
    ) -> Result<PerformedCompositePublication, NoEffectCompositePublication> {
        let CompositePublicationReady {
            attempt_identity,
            expected_head,
            commit,
            progress: _,
            retention_obligation: _retention_obligation,
        } = self;
        let next_generation = expected_head.reference_generation().advance().ok();
        if new_product_head.owner_identity() != expected_head.owner_identity()
            || new_product_head.branch_identity() != expected_head.branch_identity()
            || new_product_head.lifecycle_incarnation() != expected_head.lifecycle_incarnation()
            || next_generation != Some(new_product_head.reference_generation())
            || new_product_head.selected_commit() != commit.identity()
            || !commit.matches_owner_results(expected_head.basis(), &component_results)
        {
            return Err(NoEffectCompositePublication::new(
                NoEffectCause::OwnerDeniedBeforeEffect,
                Some(expected_head),
            ));
        }
        // The retention lane issues this receipt and owns its transfer
        // semantics. Destructuring the reserved obligation above preserves
        // the linear handoff without creating a Phase 1 authority path.
        Ok(PerformedCompositePublication::owner_issued(
            expected_head,
            new_product_head,
            commit,
            attempt_identity,
            component_results,
            late_cancellation,
            retention_transfer,
            cost_counters,
        ))
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        CompositePublicationAttemptIdentity,
        ProductBranchObservation,
        Arc<CompositeRuntimeWorldCommit>,
        CompositeAttemptProgress,
        PublicationRetentionObligation,
    ) {
        (
            self.attempt_identity,
            self.expected_head,
            self.commit,
            self.progress,
            self.retention_obligation,
        )
    }
}
