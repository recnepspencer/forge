use std::sync::Arc;

use crate::branch::{AdmittedRelationalBranchBasis, RelationalBranchBasisDescriptor};
use crate::history::data::{CanonicalCommitEnvelope, PositionedCanonicalCommit};

pub struct PublishRelationalCommit;

impl worth_proof::ActionMarker for PublishRelationalCommit {}

/// Posture after the owner reports performed publication. The branch root and
/// exact basis are current; history, patch, and replay inputs resolve from
/// that root immediately. Optional accelerators and diagnostic projections
/// may be refreshed later and never decide publication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationalPublicationProjectionPosture {
    CanonicalRootCurrentOptionalProjectionsDeferred,
}

/// Durability posture of the independently borrowable Phase 9 movement port.
/// The port proves in-process owner movement only; the ordinary commit facade
/// reports success only after its durability barrier acknowledges.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationalPublicationDurabilityPosture {
    OwnerAcknowledgementDeferred,
}

#[derive(Debug)]
struct PerformedRelationalCommitData {
    positioned_commit: Arc<PositionedCanonicalCommit>,
    next_basis: AdmittedRelationalBranchBasis,
}

/// Proof that one prepared candidate crossed its branch linearization point.
///
/// This value is deliberately non-cloneable. It carries the one canonical
/// commit artifact, owner-admitted basis, and the single-use work required to
/// settle durability and derived projections.
#[must_use = "performed publication must be settled by its owning runtime"]
pub struct PerformedRelationalCommit {
    performed: worth_proof::Performed<
        PublishRelationalCommit,
        crate::branch::RelationalBranchPublicationAuthorityMarker,
        PerformedRelationalCommitData,
    >,
    completion: crate::authority::commit::pipeline::PreparedCommitPublicationCompletion,
    settlement_retention: crate::history::retention::RelationalPerformedSettlementObligation,
    control: crate::runtime::RelationalOperationControl,
    late_interruption: Option<crate::runtime::RelationalInterruptionEvent>,
}

impl std::fmt::Debug for PerformedRelationalCommit {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PerformedRelationalCommit")
            .field(
                "commit_id",
                &self
                    .performed
                    .outcome()
                    .positioned_commit
                    .envelope()
                    .commit
                    .commit_id,
            )
            .field(
                "branch_id",
                self.performed.outcome().next_basis.identity().branch_id(),
            )
            .finish_non_exhaustive()
    }
}

impl PerformedRelationalCommit {
    pub(crate) fn record(
        positioned_commit: Arc<PositionedCanonicalCommit>,
        next_basis: AdmittedRelationalBranchBasis,
        completion: crate::authority::commit::pipeline::PreparedCommitPublicationCompletion,
        settlement_retention: crate::history::retention::RelationalPerformedSettlementObligation,
        control: crate::runtime::RelationalOperationControl,
    ) -> Self {
        Self {
            performed: worth_proof::Performed::record(
                &crate::branch::issue_relational_branch_publication_authority(),
                PerformedRelationalCommitData {
                    positioned_commit,
                    next_basis,
                },
            ),
            completion,
            settlement_retention,
            control,
            late_interruption: None,
        }
    }

    pub fn canonical_commit(&self) -> &CanonicalCommitEnvelope {
        self.performed.outcome().positioned_commit.envelope()
    }

    pub fn patch_position(&self) -> crate::publication::patch::data::PatchStreamPosition {
        self.performed.outcome().positioned_commit.position()
    }

    pub fn next_basis(&self) -> &AdmittedRelationalBranchBasis {
        &self.performed.outcome().next_basis
    }

    pub(crate) fn into_settlement_parts(
        self,
    ) -> (
        Arc<PositionedCanonicalCommit>,
        AdmittedRelationalBranchBasis,
        crate::authority::commit::pipeline::PreparedCommitPublicationCompletion,
        crate::history::retention::RelationalPerformedSettlementObligation,
        crate::runtime::RelationalOperationControl,
    ) {
        let outcome = self.performed.into_outcome();
        (
            outcome.positioned_commit,
            outcome.next_basis,
            self.completion,
            self.settlement_retention,
            self.control,
        )
    }

    pub const fn projection_posture(&self) -> RelationalPublicationProjectionPosture {
        RelationalPublicationProjectionPosture::CanonicalRootCurrentOptionalProjectionsDeferred
    }

    pub const fn durability_posture(&self) -> RelationalPublicationDurabilityPosture {
        RelationalPublicationDurabilityPosture::OwnerAcknowledgementDeferred
    }

    pub const fn late_interruption(&self) -> Option<crate::runtime::RelationalInterruptionEvent> {
        self.late_interruption
    }

    pub(crate) fn record_late_interruption(
        &mut self,
        interruption: crate::runtime::RelationalInterruptionEvent,
    ) {
        self.late_interruption = Some(interruption);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaleRelationalBranchObservation {
    expected: RelationalBranchBasisDescriptor,
    observed: RelationalBranchBasisDescriptor,
}

impl StaleRelationalBranchObservation {
    pub(crate) fn new(
        expected: RelationalBranchBasisDescriptor,
        observed: RelationalBranchBasisDescriptor,
    ) -> Self {
        Self { expected, observed }
    }

    pub fn expected(&self) -> &RelationalBranchBasisDescriptor {
        &self.expected
    }

    pub fn observed(&self) -> &RelationalBranchBasisDescriptor {
        &self.observed
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationalPublicationDenial {
    OwnerUnavailable {
        runtime_instance_id: u64,
    },
    ForeignRuntime {
        expected_runtime_instance_id: u64,
        actual_runtime_instance_id: u64,
    },
    OwnerMismatch,
    BranchUnavailable,
    Archived,
    Deleting,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationalPublicationDeferred {
    PatchPositionReservationContended,
    RetentionBackpressure,
    CandidateLifetimeExpired { maximum_lifetime_millis: u64 },
    CandidateCapacityExhausted { maximum_candidates: usize },
    PublishedSnapshotCapacityExhausted { maximum_handles: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationalPublicationFailureKind {
    SnapshotIdentityExhausted,
    CandidateIdentityExhausted,
    PreparedRootBudgetExhausted {
        maximum_bytes: u64,
        required_bytes: u64,
    },
    PreparedRootMismatch,
    PreparedBasisDescriptor(crate::branch::RelationalBranchBasisDenial),
    NextBasisAdmission(crate::branch::RelationalBranchBasisDenial),
    BranchObservation(crate::branch::RelationalBranchBasisDenial),
    PatchPositionCapacityExhausted,
    RetentionIdentityExhausted,
    RetentionOwner,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationalPublicationFailure {
    kind: RelationalPublicationFailureKind,
    detail: String,
}

impl RelationalPublicationFailure {
    pub(crate) fn new(kind: RelationalPublicationFailureKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> &RelationalPublicationFailureKind {
        &self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Debug)]
#[must_use = "publication outcomes carry performed work or a typed terminal posture"]
pub enum RelationalPublicationOutcome {
    Performed(PerformedRelationalCommit),
    Stale(StaleRelationalBranchObservation),
    Denied(RelationalPublicationDenial),
    Interrupted(crate::runtime::RelationalInterruptionEvent),
    Deferred(RelationalPublicationDeferred),
    Failed(RelationalPublicationFailure),
}

impl RelationalPublicationOutcome {
    pub(crate) fn performed(performed: PerformedRelationalCommit) -> Self {
        Self::Performed(performed)
    }

    pub(crate) fn stale(stale: StaleRelationalBranchObservation) -> Self {
        Self::Stale(stale)
    }

    pub(crate) fn denied(denial: RelationalPublicationDenial) -> Self {
        Self::Denied(denial)
    }

    pub(crate) fn interrupted(interruption: crate::runtime::RelationalInterruptionEvent) -> Self {
        Self::Interrupted(interruption)
    }

    pub(crate) fn deferred(deferred: RelationalPublicationDeferred) -> Self {
        Self::Deferred(deferred)
    }

    pub(crate) fn failed(failure: RelationalPublicationFailure) -> Self {
        Self::Failed(failure)
    }
}
