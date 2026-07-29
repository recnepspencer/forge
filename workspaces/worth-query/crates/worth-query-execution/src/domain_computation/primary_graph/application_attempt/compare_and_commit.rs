use worth_relational::facade::history::CommitId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationCommitReceipt {
    commit_id: CommitId,
    changed_record_count: usize,
    emitted_effect_count: usize,
}

impl WorthQueryApplicationCommitReceipt {
    pub const fn commit_id(&self) -> CommitId {
        self.commit_id
    }

    pub const fn changed_record_count(&self) -> usize {
        self.changed_record_count
    }

    pub const fn emitted_effect_count(&self) -> usize {
        self.emitted_effect_count
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
    pub(in crate::domain_computation::primary_graph) const fn new(
        commit_id: CommitId,
        changed_record_count: usize,
        emitted_effect_count: usize,
    ) -> Self {
        Self {
            commit_id,
            changed_record_count,
            emitted_effect_count,
        }
    }
}

impl WorthQueryApplicationStaleAttempt {
    pub(super) const fn new(stale_fact_count: usize) -> Self {
        Self { stale_fact_count }
    }
}
