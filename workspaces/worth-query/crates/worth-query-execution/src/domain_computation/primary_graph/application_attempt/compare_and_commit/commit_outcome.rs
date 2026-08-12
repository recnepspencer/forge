//! Compare-and-commit outcome and denial taxonomy.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationStaleAttempt {
    stale_fact_count: usize,
}

impl WorthQueryApplicationStaleAttempt {
    pub const fn stale_fact_count(self) -> usize {
        self.stale_fact_count
    }

    pub(in super::super) const fn new(stale_fact_count: usize) -> Self {
        Self { stale_fact_count }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryApplicationCommitDenialKind {
    ProviderRejected,
    IdempotencyIntentDrift,
    ElevationTransitionRequired,
    ElevationRequestProgramMismatch,
    ElevationApprovalProgramMismatch,
    ElevationCloseProgramMismatch,
    MandatoryReviewProgramMismatch,
    DelegationActivationRequired,
    CapabilityRevocationRequired,
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
    DelegationTransition,
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

    pub(in super::super) const fn provider_rejected(
        stage: WorthQueryApplicationCommitDenialStage,
    ) -> Self {
        Self {
            kind: WorthQueryApplicationCommitDenialKind::ProviderRejected,
            stage,
        }
    }

    pub(in super::super) const fn idempotency_intent_drift() -> Self {
        Self {
            kind: WorthQueryApplicationCommitDenialKind::IdempotencyIntentDrift,
            stage: WorthQueryApplicationCommitDenialStage::Idempotency,
        }
    }

    pub(in super::super) const fn elevation_transition_required() -> Self {
        Self {
            kind: WorthQueryApplicationCommitDenialKind::ElevationTransitionRequired,
            stage: WorthQueryApplicationCommitDenialStage::ElevationTransition,
        }
    }

    pub(in super::super) const fn delegation_activation_required() -> Self {
        Self {
            kind: WorthQueryApplicationCommitDenialKind::DelegationActivationRequired,
            stage: WorthQueryApplicationCommitDenialStage::DelegationTransition,
        }
    }

    pub(in super::super) const fn capability_revocation_required() -> Self {
        Self {
            kind: WorthQueryApplicationCommitDenialKind::CapabilityRevocationRequired,
            stage: WorthQueryApplicationCommitDenialStage::DelegationTransition,
        }
    }

    pub(in super::super) const fn elevation_request_program_mismatch() -> Self {
        Self {
            kind: WorthQueryApplicationCommitDenialKind::ElevationRequestProgramMismatch,
            stage: WorthQueryApplicationCommitDenialStage::ElevationTransition,
        }
    }

    pub(in super::super) const fn elevation_approval_program_mismatch() -> Self {
        Self {
            kind: WorthQueryApplicationCommitDenialKind::ElevationApprovalProgramMismatch,
            stage: WorthQueryApplicationCommitDenialStage::ElevationTransition,
        }
    }

    pub(in super::super) const fn elevation_close_program_mismatch() -> Self {
        Self {
            kind: WorthQueryApplicationCommitDenialKind::ElevationCloseProgramMismatch,
            stage: WorthQueryApplicationCommitDenialStage::ElevationTransition,
        }
    }

    pub(in super::super) const fn mandatory_review_program_mismatch() -> Self {
        Self {
            kind: WorthQueryApplicationCommitDenialKind::MandatoryReviewProgramMismatch,
            stage: WorthQueryApplicationCommitDenialStage::ElevationTransition,
        }
    }
}

#[derive(Debug)]
pub enum WorthQueryApplicationCommitOutcome {
    Committed(super::WorthQueryApplicationCommitReceipt),
    AlreadyCommitted(super::WorthQueryApplicationCommitReceipt),
    Stale(WorthQueryApplicationStaleAttempt),
    Cancelled,
    Denied(WorthQueryApplicationCommitDenial),
    Aborted,
    PartialEffect(WorthQueryApplicationUnresolvedCommitEvidence),
    Indeterminate(WorthQueryApplicationUnresolvedCommitEvidence),
}

/// Correlation evidence retained when commit outcome is unresolved (R8.26 / C3).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationUnresolvedCommitEvidence {
    recovery: WorthQueryApplicationCommitRecoveryKind,
    denial_kind: crate::domain_computation::provider_session::WorthQueryProviderSessionDenialKind,
    stage: crate::domain_computation::provider_session::WorthQueryProviderSessionProtocolStage,
    detail: String,
}

/// Distinguishes commit-path vs abort-path recovery requirement (R8.26).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryApplicationCommitRecoveryKind {
    CommitRecoveryRequired,
    AbortRecoveryRequired,
}

impl WorthQueryApplicationUnresolvedCommitEvidence {
    pub(in crate::domain_computation::primary_graph) fn from_provider_session_failure(
        recovery: WorthQueryApplicationCommitRecoveryKind,
        failure: &crate::domain_computation::provider_session::WorthQueryProviderSessionFailure,
    ) -> Self {
        Self {
            recovery,
            denial_kind: failure.kind(),
            stage: failure.stage(),
            detail: failure.detail().to_owned(),
        }
    }

    pub const fn recovery(&self) -> WorthQueryApplicationCommitRecoveryKind {
        self.recovery
    }

    pub const fn denial_kind(
        &self,
    ) -> crate::domain_computation::provider_session::WorthQueryProviderSessionDenialKind {
        self.denial_kind
    }

    pub const fn stage(
        &self,
    ) -> crate::domain_computation::provider_session::WorthQueryProviderSessionProtocolStage {
        self.stage
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
