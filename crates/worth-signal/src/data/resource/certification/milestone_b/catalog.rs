use super::super::family::ResourceCertificationFamily;
use crate::data::resource::CompletionDenialClass;
use serde::Deserialize;
use serde::Serialize;

pub const REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS: [ResourceMilestoneBScenarioId; 12] = [
    ResourceMilestoneBScenarioId::LifecycleReplayParity,
    ResourceMilestoneBScenarioId::OutOfOrderSupersession,
    ResourceMilestoneBScenarioId::RollbackObservationEquivalence,
    ResourceMilestoneBScenarioId::BranchRestoreReplayEquivalence,
    ResourceMilestoneBScenarioId::InflightBoundedness,
    ResourceMilestoneBScenarioId::LateCompletionAfterSupersessionRejected,
    ResourceMilestoneBScenarioId::LateCompletionAfterCancellationRejected,
    ResourceMilestoneBScenarioId::LateCompletionAfterTimeoutRejected,
    ResourceMilestoneBScenarioId::MalformedCompletionRejected,
    ResourceMilestoneBScenarioId::DuplicateCompletionRejected,
    ResourceMilestoneBScenarioId::ContradictoryCompletionRejected,
    ResourceMilestoneBScenarioId::UnknownRequestCompletionRejected,
];

pub const REQUIRED_RESOURCE_MILESTONE_B_HOSTILE_SCENARIOS: [ResourceMilestoneBScenarioId; 7] = [
    ResourceMilestoneBScenarioId::LateCompletionAfterSupersessionRejected,
    ResourceMilestoneBScenarioId::LateCompletionAfterCancellationRejected,
    ResourceMilestoneBScenarioId::LateCompletionAfterTimeoutRejected,
    ResourceMilestoneBScenarioId::MalformedCompletionRejected,
    ResourceMilestoneBScenarioId::DuplicateCompletionRejected,
    ResourceMilestoneBScenarioId::ContradictoryCompletionRejected,
    ResourceMilestoneBScenarioId::UnknownRequestCompletionRejected,
];

pub const REQUIRED_RESOURCE_MILESTONE_B_SCALAR_HOSTILE_SCENARIOS: [ResourceMilestoneBScenarioId;
    4] = [
    ResourceMilestoneBScenarioId::LateCompletionAfterSupersessionRejected,
    ResourceMilestoneBScenarioId::LateCompletionAfterCancellationRejected,
    ResourceMilestoneBScenarioId::LateCompletionAfterTimeoutRejected,
    ResourceMilestoneBScenarioId::MalformedCompletionRejected,
];

pub const REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS: [ResourceMilestoneBPerformanceClaimId;
    9] = [
    ResourceMilestoneBPerformanceClaimId::LifecycleReplayParityDebtBounded,
    ResourceMilestoneBPerformanceClaimId::OutOfOrderSupersessionAdmissionBounded,
    ResourceMilestoneBPerformanceClaimId::RollbackObservationRollbackBounded,
    ResourceMilestoneBPerformanceClaimId::BranchRestoreReplayRestoreBounded,
    ResourceMilestoneBPerformanceClaimId::InflightBoundednessAdmissionBounded,
    ResourceMilestoneBPerformanceClaimId::RuntimeSummaryReadZeroColdReconstruction,
    ResourceMilestoneBPerformanceClaimId::DiagnosticsExpansionBudgetedColdReconstruction,
    ResourceMilestoneBPerformanceClaimId::DiagnosticsExpansionBudgetDenial,
    ResourceMilestoneBPerformanceClaimId::HostileCompletionDenialsScalarBounded,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResourceMilestoneBScenarioId {
    LifecycleReplayParity,
    OutOfOrderSupersession,
    RollbackObservationEquivalence,
    BranchRestoreReplayEquivalence,
    InflightBoundedness,
    LateCompletionAfterSupersessionRejected,
    LateCompletionAfterCancellationRejected,
    LateCompletionAfterTimeoutRejected,
    MalformedCompletionRejected,
    DuplicateCompletionRejected,
    ContradictoryCompletionRejected,
    UnknownRequestCompletionRejected,
}

impl ResourceMilestoneBScenarioId {
    pub fn certification_family(self) -> Option<ResourceCertificationFamily> {
        match self {
            Self::LifecycleReplayParity => {
                Some(ResourceCertificationFamily::AsyncResourceLifecycleParity)
            }
            Self::OutOfOrderSupersession => {
                Some(ResourceCertificationFamily::OutOfOrderCompletionSupersession)
            }
            Self::RollbackObservationEquivalence => {
                Some(ResourceCertificationFamily::AsyncRollbackObservationEquivalence)
            }
            Self::BranchRestoreReplayEquivalence => {
                Some(ResourceCertificationFamily::AsyncBranchRestoreReplayEquivalence)
            }
            Self::InflightBoundedness => {
                Some(ResourceCertificationFamily::AsyncInflightBoundedness)
            }
            Self::LateCompletionAfterSupersessionRejected
            | Self::LateCompletionAfterCancellationRejected
            | Self::LateCompletionAfterTimeoutRejected
            | Self::MalformedCompletionRejected
            | Self::DuplicateCompletionRejected
            | Self::ContradictoryCompletionRejected
            | Self::UnknownRequestCompletionRejected => None,
        }
    }

    pub fn completion_denial_class(self) -> Option<CompletionDenialClass> {
        match self {
            Self::LateCompletionAfterSupersessionRejected => {
                Some(CompletionDenialClass::Superseded)
            }
            Self::LateCompletionAfterCancellationRejected => Some(CompletionDenialClass::Cancelled),
            Self::LateCompletionAfterTimeoutRejected => Some(CompletionDenialClass::TimedOut),
            Self::MalformedCompletionRejected => Some(CompletionDenialClass::Malformed),
            Self::DuplicateCompletionRejected => Some(CompletionDenialClass::Duplicate),
            Self::ContradictoryCompletionRejected => Some(CompletionDenialClass::Contradictory),
            Self::UnknownRequestCompletionRejected => Some(CompletionDenialClass::UnknownRequest),
            Self::LifecycleReplayParity
            | Self::OutOfOrderSupersession
            | Self::RollbackObservationEquivalence
            | Self::BranchRestoreReplayEquivalence
            | Self::InflightBoundedness => None,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::LifecycleReplayParity => "lifecycle-replay-parity",
            Self::OutOfOrderSupersession => "out-of-order-supersession",
            Self::RollbackObservationEquivalence => "rollback-observation-equivalence",
            Self::BranchRestoreReplayEquivalence => "branch-restore-replay-equivalence",
            Self::InflightBoundedness => "inflight-boundedness",
            Self::LateCompletionAfterSupersessionRejected => {
                "late-completion-after-supersession-rejected"
            }
            Self::LateCompletionAfterCancellationRejected => {
                "late-completion-after-cancellation-rejected"
            }
            Self::LateCompletionAfterTimeoutRejected => "late-completion-after-timeout-rejected",
            Self::MalformedCompletionRejected => "malformed-completion-rejected",
            Self::DuplicateCompletionRejected => "duplicate-completion-rejected",
            Self::ContradictoryCompletionRejected => "contradictory-completion-rejected",
            Self::UnknownRequestCompletionRejected => "unknown-request-completion-rejected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResourceMilestoneBScenarioEvidenceKind {
    CertificationFamily,
    HostileCompletionDenial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResourceMilestoneBPerformanceClaimId {
    LifecycleReplayParityDebtBounded,
    OutOfOrderSupersessionAdmissionBounded,
    RollbackObservationRollbackBounded,
    BranchRestoreReplayRestoreBounded,
    InflightBoundednessAdmissionBounded,
    RuntimeSummaryReadZeroColdReconstruction,
    DiagnosticsExpansionBudgetedColdReconstruction,
    DiagnosticsExpansionBudgetDenial,
    HostileCompletionDenialsScalarBounded,
}

impl ResourceMilestoneBPerformanceClaimId {
    pub fn label(self) -> &'static str {
        match self {
            Self::LifecycleReplayParityDebtBounded => "lifecycle-replay-parity-debt-bounded",
            Self::OutOfOrderSupersessionAdmissionBounded => {
                "out-of-order-supersession-admission-bounded"
            }
            Self::RollbackObservationRollbackBounded => "rollback-observation-rollback-bounded",
            Self::BranchRestoreReplayRestoreBounded => "branch-restore-replay-restore-bounded",
            Self::InflightBoundednessAdmissionBounded => "inflight-boundedness-admission-bounded",
            Self::RuntimeSummaryReadZeroColdReconstruction => {
                "runtime-summary-read-zero-cold-reconstruction"
            }
            Self::DiagnosticsExpansionBudgetedColdReconstruction => {
                "diagnostics-expansion-budgeted-cold-reconstruction"
            }
            Self::DiagnosticsExpansionBudgetDenial => "diagnostics-expansion-budget-denial",
            Self::HostileCompletionDenialsScalarBounded => {
                "hostile-completion-denials-scalar-bounded"
            }
        }
    }
}
