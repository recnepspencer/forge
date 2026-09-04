use crate::branch::{LoweredBranchCreationPlan, ProductBranchObservation, ReservedCustodySlot};
use crate::identity::CompositePublicationAttemptIdentity;
use crate::lifecycle::RuntimeWorldInstant;

use super::{
    CompositeAttemptProgress, CompositePublicationCostCounters, LoweredOwnerComponentPlan,
    NoEffectCompositePublication, OwnerExecutionSettlement,
};

#[path = "reservation/capacities.rs"]
mod capacities;

pub(crate) use capacities::{ReservedAttemptCapacities, ReservedAttemptCapacityInputs};

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
    capacities: ReservedAttemptCapacities,
    cancellation: CompositeAttemptCancellationPosture,
    deadline: Option<RuntimeWorldInstant>,
    order: CompositePublicationOrder,
    progress: CompositeAttemptProgress,
    counters: CompositePublicationCostCounters,
}

impl std::fmt::Debug for ReservedCompositePublicationAttempt {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ReservedCompositePublicationAttempt")
            .field("identity", &self.identity)
            .field("expected_head", &self.expected_head)
            .field("cancellation", &self.cancellation)
            .field("deadline", &self.deadline)
            .field("order", &self.order)
            .field("progress", &self.progress)
            .field("counters", &self.counters)
            .finish_non_exhaustive()
    }
}

impl ReservedCompositePublicationAttempt {
    pub(crate) fn new(
        identity: CompositePublicationAttemptIdentity,
        expected_head: ProductBranchObservation,
        predecessor_basis: crate::basis::AdmittedCompositeRuntimeWorldBasis,
        plan: LoweredOwnerComponentPlan,
        capacities: ReservedAttemptCapacities,
        deadline: Option<RuntimeWorldInstant>,
    ) -> Self {
        Self {
            identity,
            expected_head,
            predecessor_basis,
            plan,
            capacities,
            cancellation: CompositeAttemptCancellationPosture::Open,
            deadline,
            order: CompositePublicationOrder::RelationalThenSignal,
            progress: CompositeAttemptProgress::untouched(),
            counters: CompositePublicationCostCounters::zero(),
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

    /// Structural counters are initialized when the attempt is reserved, before
    /// the first owner effect, so a caller can read an honest zeroed scope.
    pub fn counters(&self) -> &CompositePublicationCostCounters {
        &self.counters
    }

    pub(crate) fn counters_mut(&mut self) -> &mut CompositePublicationCostCounters {
        &mut self.counters
    }

    pub fn cancellation_posture(&self) -> CompositeAttemptCancellationPosture {
        self.cancellation
    }

    pub(crate) fn observe_cancellation(&mut self) {
        self.cancellation = CompositeAttemptCancellationPosture::CancellationObserved;
    }

    pub(crate) fn begin_owner_execution(&mut self) {
        self.capacities.begin_owner_execution();
    }

    pub(crate) fn begin_recovery(&mut self) {
        self.capacities.begin_recovery();
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
        self.capacities.begin_publication();
        OwnerExecutionSettlement::new(self, progress)
    }

    pub(crate) fn settle_with_successor_basis(
        mut self,
        progress: CompositeAttemptProgress,
        successor_basis: crate::basis::AdmittedCompositeRuntimeWorldBasis,
    ) -> OwnerExecutionSettlement {
        self.capacities.begin_publication();
        OwnerExecutionSettlement::with_successor_basis(self, progress, successor_basis)
    }

    pub(crate) fn into_parts(self) -> ReservedPublicationAttemptParts {
        ReservedPublicationAttemptParts {
            identity: self.identity,
            expected_head: self.expected_head,
            plan: self.plan,
            capacities: self.capacities,
            cancellation: self.cancellation,
            deadline: self.deadline,
            counters: self.counters,
        }
    }
}

/// The exact linear contents of a consumed publication attempt. Every field is
/// a live owner-issued reservation or the evidence bound to one.
pub(crate) struct ReservedPublicationAttemptParts {
    pub(crate) identity: CompositePublicationAttemptIdentity,
    pub(crate) expected_head: ProductBranchObservation,
    pub(crate) plan: LoweredOwnerComponentPlan,
    pub(crate) capacities: ReservedAttemptCapacities,
    pub(crate) cancellation: CompositeAttemptCancellationPosture,
    pub(crate) deadline: Option<RuntimeWorldInstant>,
    pub(crate) counters: CompositePublicationCostCounters,
}

/// Branch creation reserves the same resources under the same rules and
/// consumes into a creation terminal. Custody slots for every owner fork this
/// creation will perform are charged here, before the first owner effect.
#[must_use = "a reserved branch-creation attempt must be executed or cleaned up"]
pub(crate) struct ReservedBranchCreationAttempt {
    identity: CompositePublicationAttemptIdentity,
    source: ProductBranchObservation,
    plan: LoweredBranchCreationPlan,
    capacities: ReservedAttemptCapacities,
    relational_custody: Option<ReservedCustodySlot>,
    signal_custody: Option<ReservedCustodySlot>,
    cancellation: CompositeAttemptCancellationPosture,
    deadline: Option<RuntimeWorldInstant>,
    order: CompositePublicationOrder,
    progress: CompositeAttemptProgress,
    counters: CompositePublicationCostCounters,
}

impl std::fmt::Debug for ReservedBranchCreationAttempt {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ReservedBranchCreationAttempt")
            .field("identity", &self.identity)
            .field("source", &self.source)
            .field("plan", &self.plan)
            .field("cancellation", &self.cancellation)
            .field("deadline", &self.deadline)
            .field("order", &self.order)
            .field("progress", &self.progress)
            .field("counters", &self.counters)
            .finish_non_exhaustive()
    }
}

pub(crate) struct ReservedBranchCreationInputs {
    pub(crate) identity: CompositePublicationAttemptIdentity,
    pub(crate) source: ProductBranchObservation,
    pub(crate) plan: LoweredBranchCreationPlan,
    pub(crate) capacities: ReservedAttemptCapacities,
    pub(crate) relational_custody: Option<ReservedCustodySlot>,
    pub(crate) signal_custody: Option<ReservedCustodySlot>,
    pub(crate) deadline: Option<RuntimeWorldInstant>,
}

impl ReservedBranchCreationAttempt {
    pub(crate) fn new(inputs: ReservedBranchCreationInputs) -> Self {
        let ReservedBranchCreationInputs {
            identity,
            source,
            plan,
            capacities,
            relational_custody,
            signal_custody,
            deadline,
        } = inputs;
        Self {
            identity,
            source,
            plan,
            capacities,
            relational_custody,
            signal_custody,
            cancellation: CompositeAttemptCancellationPosture::Open,
            deadline,
            order: CompositePublicationOrder::RelationalThenSignal,
            progress: CompositeAttemptProgress::untouched(),
            counters: CompositePublicationCostCounters::zero(),
        }
    }

    pub(crate) const fn identity(&self) -> &CompositePublicationAttemptIdentity {
        &self.identity
    }

    pub(crate) const fn source(&self) -> &ProductBranchObservation {
        &self.source
    }

    pub(crate) const fn plan(&self) -> &LoweredBranchCreationPlan {
        &self.plan
    }

    pub(crate) const fn order(&self) -> CompositePublicationOrder {
        self.order
    }

    /// The reservation-time deadline, if the caller set one. Creation checks it
    /// before each owner effect, exactly as publication does.
    pub(crate) const fn deadline(&self) -> Option<RuntimeWorldInstant> {
        self.deadline
    }

    /// Structural counters are initialized when the attempt is reserved, before
    /// the first owner effect.
    pub(crate) const fn counters(&self) -> &CompositePublicationCostCounters {
        &self.counters
    }

    pub(crate) fn take_relational_custody(&mut self) -> Option<ReservedCustodySlot> {
        self.relational_custody.take()
    }

    pub(crate) fn take_signal_custody(&mut self) -> Option<ReservedCustodySlot> {
        self.signal_custody.take()
    }

    pub(crate) fn begin_owner_execution(&mut self) {
        self.capacities.begin_owner_execution();
    }

    /// A creation leaves its owner-effect phase when the last fork it will ever
    /// attempt has returned. Installing the product reference, and retaining
    /// the forks when installation cannot happen, are both publication-phase
    /// work, so they share the publication posture with an ordinary attempt.
    pub(crate) fn begin_publication(&mut self) {
        self.capacities.begin_publication();
    }

    pub(crate) fn cancel(self) -> NoEffectCompositePublication {
        NoEffectCompositePublication::new(
            super::NoEffectCause::CancelledBeforeEffect,
            Some(self.source),
        )
    }

    pub(crate) fn into_parts(self) -> ReservedBranchCreationParts {
        ReservedBranchCreationParts {
            identity: self.identity,
            source: self.source,
            plan: self.plan,
            capacities: self.capacities,
            cancellation: self.cancellation,
            deadline: self.deadline,
            progress: self.progress,
            counters: self.counters,
        }
    }
}

/// The exact linear contents of a consumed branch-creation attempt.
pub(crate) struct ReservedBranchCreationParts {
    pub(crate) identity: CompositePublicationAttemptIdentity,
    pub(crate) source: ProductBranchObservation,
    pub(crate) plan: LoweredBranchCreationPlan,
    pub(crate) capacities: ReservedAttemptCapacities,
    pub(crate) cancellation: CompositeAttemptCancellationPosture,
    pub(crate) deadline: Option<RuntimeWorldInstant>,
    pub(crate) progress: CompositeAttemptProgress,
    pub(crate) counters: CompositePublicationCostCounters,
}
