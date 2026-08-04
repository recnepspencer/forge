use worth_query_installation::facade::WorthQueryCanonicalWorkPhases;
use worth_relational::facade::history::CommitId;

use super::{
    WorthQueryApplicationCommitTerminalEvidence, WorthQueryMutationPreconditionComparisonEvidence,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryApplicationCommitReceipt {
    provider_runtime_instance_id: u64,
    commit_id: CommitId,
    changed_record_count: usize,
    emitted_effect_count: usize,
    mutation_work: Option<super::super::provider::WorthQueryPrimaryMutationWorkEvidence>,
    precondition_comparison: WorthQueryMutationPreconditionComparisonEvidence,
    canonical_work: WorthQueryCanonicalWorkPhases,
    terminal: WorthQueryApplicationCommitTerminalEvidence,
}

impl Eq for WorthQueryApplicationCommitReceipt {}

pub(in crate::domain_computation::primary_graph) struct WorthQueryPendingApplicationCommitReceipt {
    provider: super::super::provider::WorthQueryPrimaryGraphCommittedApplication,
    precondition_comparison: WorthQueryMutationPreconditionComparisonEvidence,
    canonical_work: WorthQueryCanonicalWorkPhases,
}

impl WorthQueryApplicationCommitReceipt {
    pub const fn provider_runtime_instance_id(&self) -> u64 {
        self.provider_runtime_instance_id
    }

    pub const fn commit_id(&self) -> CommitId {
        self.commit_id
    }

    pub const fn changed_record_count(&self) -> usize {
        self.changed_record_count
    }

    pub const fn emitted_effect_count(&self) -> usize {
        self.emitted_effect_count
    }

    pub const fn mutation_work(
        &self,
    ) -> Option<super::super::provider::WorthQueryPrimaryMutationWorkEvidence> {
        self.mutation_work
    }

    pub const fn precondition_comparison(
        &self,
    ) -> &WorthQueryMutationPreconditionComparisonEvidence {
        &self.precondition_comparison
    }

    pub const fn canonical_work(&self) -> WorthQueryCanonicalWorkPhases {
        self.canonical_work
    }

    pub const fn terminal(&self) -> &WorthQueryApplicationCommitTerminalEvidence {
        &self.terminal
    }

    pub fn is_same_authoritative_commit(&self, other: &Self) -> bool {
        self.provider_runtime_instance_id == other.provider_runtime_instance_id
            && self.terminal.branch() == other.terminal.branch()
            && self.commit_id == other.commit_id
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationStaleAttempt {
    stale_fact_count: usize,
}

impl WorthQueryApplicationStaleAttempt {
    pub const fn stale_fact_count(self) -> usize {
        self.stale_fact_count
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryApplicationCommitDenialKind {
    ProviderRejected,
    IdempotencyIntentDrift,
    ElevationTransitionRequired,
    ElevationRequestProgramMismatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryApplicationCommitDenialStage {
    ProposalBinding,
    BridgePlanning,
    BasisAdmission,
    ResourceAdmission,
    ManagedRunAdmission,
    ProviderPlan,
    Idempotency,
    DecisionReadSet,
    EffectLowering,
    ElevationTransition,
    ProvisionalState,
    InvariantExecution,
    ProviderCommit,
}

#[derive(Debug)]
pub struct WorthQueryApplicationCommitDenial {
    kind: WorthQueryApplicationCommitDenialKind,
    stage: WorthQueryApplicationCommitDenialStage,
}

impl WorthQueryApplicationCommitDenial {
    pub const fn kind(&self) -> WorthQueryApplicationCommitDenialKind {
        self.kind
    }

    pub const fn stage(&self) -> WorthQueryApplicationCommitDenialStage {
        self.stage
    }
}

#[derive(Debug)]
pub enum WorthQueryApplicationCommitOutcome {
    Committed(WorthQueryApplicationCommitReceipt),
    AlreadyCommitted(WorthQueryApplicationCommitReceipt),
    Stale(WorthQueryApplicationStaleAttempt),
    Cancelled,
    Denied(WorthQueryApplicationCommitDenial),
    Aborted,
    PartialEffect,
    Indeterminate,
}

impl WorthQueryApplicationCommitDenial {
    pub(super) const fn provider_rejected(stage: WorthQueryApplicationCommitDenialStage) -> Self {
        Self {
            kind: WorthQueryApplicationCommitDenialKind::ProviderRejected,
            stage,
        }
    }

    pub(super) const fn idempotency_intent_drift() -> Self {
        Self {
            kind: WorthQueryApplicationCommitDenialKind::IdempotencyIntentDrift,
            stage: WorthQueryApplicationCommitDenialStage::Idempotency,
        }
    }

    pub(super) const fn elevation_transition_required() -> Self {
        Self {
            kind: WorthQueryApplicationCommitDenialKind::ElevationTransitionRequired,
            stage: WorthQueryApplicationCommitDenialStage::ElevationTransition,
        }
    }

    pub(super) const fn elevation_request_program_mismatch() -> Self {
        Self {
            kind: WorthQueryApplicationCommitDenialKind::ElevationRequestProgramMismatch,
            stage: WorthQueryApplicationCommitDenialStage::ElevationTransition,
        }
    }
}

impl WorthQueryApplicationCommitReceipt {
    pub(in crate::domain_computation::primary_graph) fn from_recovered_provider(
        provider: super::super::provider::WorthQueryPrimaryGraphCommittedApplication,
        precondition_comparison: WorthQueryMutationPreconditionComparisonEvidence,
        canonical_work: WorthQueryCanonicalWorkPhases,
    ) -> Self {
        let terminal =
            WorthQueryApplicationCommitTerminalEvidence::recovered(provider.branch().clone());
        Self {
            provider_runtime_instance_id: provider.runtime_instance_id(),
            commit_id: provider.commit_id(),
            changed_record_count: provider.changed_record_count(),
            emitted_effect_count: provider.emitted_effect_count(),
            mutation_work: provider.mutation_work(),
            precondition_comparison,
            canonical_work,
            terminal,
        }
    }

    pub(in crate::domain_computation::primary_graph) fn with_retry_inspection(
        mut self,
        completion: crate::domain_computation::provider_session::WorthQueryMutationGraphWorkCompletion,
    ) -> Option<Self> {
        self.terminal = self.terminal.with_retry_inspection(completion)?;
        Some(self)
    }
}

impl WorthQueryPendingApplicationCommitReceipt {
    pub(in crate::domain_computation::primary_graph) const fn from_provider(
        provider: super::super::provider::WorthQueryPrimaryGraphCommittedApplication,
        precondition_comparison: WorthQueryMutationPreconditionComparisonEvidence,
        canonical_work: WorthQueryCanonicalWorkPhases,
    ) -> Self {
        Self {
            provider,
            precondition_comparison,
            canonical_work,
        }
    }

    pub(in crate::domain_computation::primary_graph) fn complete(
        self,
        completion: crate::domain_computation::provider_session::WorthQueryMutationGraphWorkCompletion,
    ) -> Option<WorthQueryApplicationCommitReceipt> {
        if self.provider.branch() != completion.relational_branch() {
            return None;
        }
        let mutation_work = self.provider.mutation_work()?;
        Some(WorthQueryApplicationCommitReceipt {
            provider_runtime_instance_id: self.provider.runtime_instance_id(),
            commit_id: self.provider.commit_id(),
            changed_record_count: self.provider.changed_record_count(),
            emitted_effect_count: self.provider.emitted_effect_count(),
            mutation_work: Some(mutation_work),
            precondition_comparison: self.precondition_comparison,
            canonical_work: self.canonical_work,
            terminal: WorthQueryApplicationCommitTerminalEvidence::executed(completion),
        })
    }
}

impl WorthQueryApplicationStaleAttempt {
    pub(super) const fn new(stale_fact_count: usize) -> Self {
        Self { stale_fact_count }
    }
}
