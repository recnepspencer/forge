use worth_query_installation::facade::WorthQueryCanonicalWorkPhases;
use worth_relational::facade::history::CommitId;

use super::WorthQueryMutationPreconditionComparisonEvidence;

#[derive(Clone, Debug)]
pub struct WorthQueryApplicationCommitReceipt {
    provider_runtime_instance_id: u64,
    branch_id: worth_relational::facade::history::BranchId,
    commit_id: CommitId,
    changed_record_count: usize,
    emitted_effect_count: usize,
    precondition_comparison: WorthQueryMutationPreconditionComparisonEvidence,
    canonical_work: WorthQueryCanonicalWorkPhases,
    graph_work: Option<WorthQueryApplicationGraphWorkReceipt>,
}

impl PartialEq for WorthQueryApplicationCommitReceipt {
    fn eq(&self, other: &Self) -> bool {
        self.provider_runtime_instance_id == other.provider_runtime_instance_id
            && self.branch_id == other.branch_id
            && self.commit_id == other.commit_id
            && self.changed_record_count == other.changed_record_count
            && self.emitted_effect_count == other.emitted_effect_count
            && self.precondition_comparison == other.precondition_comparison
            && self.canonical_work == other.canonical_work
    }
}

impl Eq for WorthQueryApplicationCommitReceipt {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationGraphWorkReceipt {
    session_identity: worth_foundational::facade::CanonicalDigestId,
    provider_session_identity: String,
    plan_identity: worth_foundational::facade::CanonicalDigestId,
    obligation_identity:
        worth_query_installation::facade::WorthQueryInstalledGraphObligationSetIdentity,
    required_obligation_count: usize,
    branch_id: worth_relational::facade::history::BranchId,
    released_reservation_count: usize,
    basis_released: bool,
}

impl WorthQueryApplicationCommitReceipt {
    pub(in crate::domain_computation::primary_graph) const fn provider_runtime_instance_id(
        &self,
    ) -> u64 {
        self.provider_runtime_instance_id
    }

    pub const fn commit_id(&self) -> CommitId {
        self.commit_id
    }

    pub const fn branch_id(&self) -> &worth_relational::facade::history::BranchId {
        &self.branch_id
    }

    pub const fn changed_record_count(&self) -> usize {
        self.changed_record_count
    }

    pub const fn emitted_effect_count(&self) -> usize {
        self.emitted_effect_count
    }

    pub const fn precondition_comparison(
        &self,
    ) -> &WorthQueryMutationPreconditionComparisonEvidence {
        &self.precondition_comparison
    }

    pub const fn canonical_work(&self) -> WorthQueryCanonicalWorkPhases {
        self.canonical_work
    }

    pub const fn graph_work(&self) -> Option<&WorthQueryApplicationGraphWorkReceipt> {
        self.graph_work.as_ref()
    }
}

impl WorthQueryApplicationGraphWorkReceipt {
    pub const fn session_identity(&self) -> &worth_foundational::facade::CanonicalDigestId {
        &self.session_identity
    }

    pub fn provider_session_identity(&self) -> &str {
        &self.provider_session_identity
    }

    pub const fn plan_identity(&self) -> &worth_foundational::facade::CanonicalDigestId {
        &self.plan_identity
    }

    pub const fn obligation_identity(
        &self,
    ) -> &worth_query_installation::facade::WorthQueryInstalledGraphObligationSetIdentity {
        &self.obligation_identity
    }

    pub const fn required_obligation_count(&self) -> usize {
        self.required_obligation_count
    }

    pub const fn branch_id(&self) -> &worth_relational::facade::history::BranchId {
        &self.branch_id
    }

    pub const fn released_reservation_count(&self) -> usize {
        self.released_reservation_count
    }

    pub const fn basis_released(&self) -> bool {
        self.basis_released
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
}

impl WorthQueryApplicationCommitReceipt {
    pub(in crate::domain_computation::primary_graph) fn from_provider(
        provider: super::super::provider::WorthQueryPrimaryGraphCommittedApplication,
        precondition_comparison: WorthQueryMutationPreconditionComparisonEvidence,
        canonical_work: WorthQueryCanonicalWorkPhases,
    ) -> Self {
        Self {
            provider_runtime_instance_id: provider.runtime_instance_id(),
            branch_id: provider.branch_id().clone(),
            commit_id: provider.commit_id(),
            changed_record_count: provider.changed_record_count(),
            emitted_effect_count: provider.emitted_effect_count(),
            precondition_comparison,
            canonical_work,
            graph_work: None,
        }
    }

    fn attach_graph_work(
        &mut self,
        release: &crate::domain_computation::provider_session::WorthQueryGraphWorkSessionReleaseReceipt,
    ) {
        self.graph_work = Some(WorthQueryApplicationGraphWorkReceipt {
            session_identity: *release.session_identity(),
            provider_session_identity: release.provider_session_identity().to_owned(),
            plan_identity: *release.plan_identity(),
            obligation_identity: release.obligation_identity().clone(),
            required_obligation_count: release.required_obligation_count(),
            branch_id: release.branch_id().clone(),
            released_reservation_count: release.capacity().released_reservation_count(),
            basis_released: release.basis_released(),
        });
    }
}

impl WorthQueryApplicationCommitOutcome {
    pub(super) fn attach_graph_work(
        &mut self,
        release: &crate::domain_computation::provider_session::WorthQueryGraphWorkSessionReleaseReceipt,
    ) {
        match self {
            Self::Committed(receipt) | Self::AlreadyCommitted(receipt) => {
                receipt.attach_graph_work(release);
            }
            Self::Stale(_)
            | Self::Cancelled
            | Self::Denied(_)
            | Self::Aborted
            | Self::PartialEffect
            | Self::Indeterminate => {}
        }
    }
}

impl WorthQueryApplicationStaleAttempt {
    pub(super) const fn new(stale_fact_count: usize) -> Self {
        Self { stale_fact_count }
    }
}
