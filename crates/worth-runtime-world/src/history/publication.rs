//! Canonical facts of a performed publication. The history entry owns this
//! allocation; a caller's linear delivery claim never becomes its truth source.

mod delivery;

use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::{Arc, OnceLock};

use crate::branch::{ProductBranchReferenceMovement, ProductBranchReferenceSnapshot};
use crate::identity::{CompositeCommitIdentity, CompositePublicationAttemptIdentity};
use crate::publication::{
    CompositeLateCancellationPosture, CompositeOwnerExecutionResults,
    CompositePublicationCostCounters,
};

pub(crate) use delivery::PublicationDeliveryClaim;

/// Preallocated before owner effects and charged with its exact history slot.
/// Facts become visible only at the successful branch-cell replacement.
#[derive(Debug)]
pub(crate) struct CanonicalPublicationEnvelope {
    commit_identity: CompositeCommitIdentity,
    attempt_identity: CompositePublicationAttemptIdentity,
    expected: ProductBranchReferenceSnapshot,
    facts: OnceLock<PerformedPublicationFacts>,
    committed: AtomicBool,
    delivery: AtomicU8,
}

#[derive(Debug)]
pub(crate) struct PerformedPublicationFacts {
    pub(crate) movement: ProductBranchReferenceMovement,
    pub(crate) component_results: CompositeOwnerExecutionResults,
    pub(crate) late_cancellation: CompositeLateCancellationPosture,
    pub(crate) cost_counters: CompositePublicationCostCounters,
}

/// Read-only owner evidence prepared before entering the branch critical
/// section. It cannot authorize a performed publication by itself.
pub(crate) struct PreparedPublicationRecord {
    envelope: Arc<CanonicalPublicationEnvelope>,
    commit: Arc<crate::history::CompositeRuntimeWorldCommit>,
    component_results: CompositeOwnerExecutionResults,
    late_cancellation: CompositeLateCancellationPosture,
    cost_counters: CompositePublicationCostCounters,
}

impl CanonicalPublicationEnvelope {
    pub(crate) fn reserve(
        commit_identity: CompositeCommitIdentity,
        attempt_identity: CompositePublicationAttemptIdentity,
        expected: ProductBranchReferenceSnapshot,
    ) -> Arc<Self> {
        Arc::new(Self {
            commit_identity,
            attempt_identity,
            expected,
            facts: OnceLock::new(),
            committed: AtomicBool::new(false),
            delivery: AtomicU8::new(delivery::AVAILABLE),
        })
    }

    pub(crate) fn prepare(
        self: &Arc<Self>,
        commit: &Arc<crate::history::CompositeRuntimeWorldCommit>,
        component_results: &CompositeOwnerExecutionResults,
        late_cancellation: CompositeLateCancellationPosture,
        cost_counters: CompositePublicationCostCounters,
    ) -> PreparedPublicationRecord {
        assert_eq!(commit.identity(), &self.commit_identity);
        assert_eq!(
            commit.provenance(),
            &crate::history::CompositeCommitProvenance::Publication(self.attempt_identity.clone())
        );
        assert!(commit.matches_owner_results(self.expected.basis(), component_results));
        PreparedPublicationRecord {
            envelope: Arc::clone(self),
            commit: Arc::clone(commit),
            component_results: component_results.evidence_image(),
            late_cancellation,
            cost_counters,
        }
    }

    pub(crate) fn commit_identity(&self) -> &CompositeCommitIdentity {
        &self.commit_identity
    }

    pub(crate) fn branch_name(&self) -> &crate::branch::ProductBranchName {
        self.expected.branch().name()
    }

    pub(crate) fn attempt_identity(&self) -> &CompositePublicationAttemptIdentity {
        &self.attempt_identity
    }

    pub(crate) fn facts(&self) -> Option<&PerformedPublicationFacts> {
        self.committed
            .load(Ordering::Acquire)
            .then(|| self.facts.get())
            .flatten()
    }
}

impl PreparedPublicationRecord {
    /// Validate all bindings and fill the preallocated record before the cell
    /// swaps. The caller must then swap and mark committed without a fallible
    /// operation, allocation, callback, or destructor between those steps.
    pub(crate) fn stage(
        mut self,
        movement: &ProductBranchReferenceMovement,
    ) -> Arc<CanonicalPublicationEnvelope> {
        assert_eq!(movement.before(), &self.envelope.expected);
        assert_eq!(
            movement.after().selected_commit(),
            &self.envelope.commit_identity
        );
        assert!(!self.envelope.committed.load(Ordering::Acquire));
        assert!(
            std::ptr::eq(movement.after().commit(), self.commit.as_ref()),
            "the movement installs the immutable commit whose full owner evidence was validated"
        );
        self.cost_counters.record_cas_win();
        let facts = PerformedPublicationFacts {
            movement: movement.clone(),
            component_results: self.component_results,
            late_cancellation: self.late_cancellation,
            cost_counters: self.cost_counters,
        };
        self.envelope
            .facts
            .set(facts)
            .expect("one reserved history entry records one publication movement");
        self.envelope
    }
}

impl CanonicalPublicationEnvelope {
    /// Called only by the cell immediately after its infallible image swap,
    /// while readers remain excluded and before the old protection drops.
    pub(crate) fn mark_committed(&self) {
        self.committed.store(true, Ordering::Release);
    }
}
