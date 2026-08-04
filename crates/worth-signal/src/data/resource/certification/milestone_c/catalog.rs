use serde::Deserialize;
use serde::Serialize;

pub const REQUIRED_RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_FAMILIES:
    [ResourceMilestoneCPolicyCertificationFamily; 7] = [
    ResourceMilestoneCPolicyCertificationFamily::AsyncResourcePolicyFamilyCertification,
    ResourceMilestoneCPolicyCertificationFamily::AsyncRetryBudgetAndBackoffCertification,
    ResourceMilestoneCPolicyCertificationFamily::AsyncTimeoutDeadlineCertification,
    ResourceMilestoneCPolicyCertificationFamily::AsyncCancellationSupersessionPolicyCertification,
    ResourceMilestoneCPolicyCertificationFamily::AsyncRevalidationFreshnessCertification,
    ResourceMilestoneCPolicyCertificationFamily::AsyncObservationOutputContinuityCertification,
    ResourceMilestoneCPolicyCertificationFamily::AsyncRetentionReplayPolicyCertification,
];

pub const REQUIRED_RESOURCE_MILESTONE_C_POLICY_SCENARIOS: [ResourceMilestoneCPolicyScenarioId; 8] = [
    ResourceMilestoneCPolicyScenarioId::RegistryOrderCanonicalization,
    ResourceMilestoneCPolicyScenarioId::RetryBudgetExhaustionRejected,
    ResourceMilestoneCPolicyScenarioId::HeartbeatExtensionTerminalDenied,
    ResourceMilestoneCPolicyScenarioId::RetentionCompactionReportsUnavailableHistory,
    ResourceMilestoneCPolicyScenarioId::DiagnosticsExpansionBudgetDeniedZeroCold,
    ResourceMilestoneCPolicyScenarioId::CompatibleDescriptorRestoreAdmitted,
    ResourceMilestoneCPolicyScenarioId::IncompatibleDescriptorRestoreDenied,
    ResourceMilestoneCPolicyScenarioId::MissingDescriptorRestoreDenied,
];

pub const REQUIRED_RESOURCE_MILESTONE_C_POLICY_PERFORMANCE_CLAIMS:
    [ResourceMilestoneCPolicyPerformanceClaimId; 5] = [
    ResourceMilestoneCPolicyPerformanceClaimId::RegistryFreezeOrderBounded,
    ResourceMilestoneCPolicyPerformanceClaimId::RetryBudgetDenialZeroWake,
    ResourceMilestoneCPolicyPerformanceClaimId::RetentionCompactionAvailabilityBounded,
    ResourceMilestoneCPolicyPerformanceClaimId::DiagnosticsBudgetDenialZeroCold,
    ResourceMilestoneCPolicyPerformanceClaimId::ReplayCompatibilityDescriptorBounded,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResourceMilestoneCPolicyCertificationFamily {
    AsyncResourcePolicyFamilyCertification,
    AsyncRetryBudgetAndBackoffCertification,
    AsyncTimeoutDeadlineCertification,
    AsyncCancellationSupersessionPolicyCertification,
    AsyncRevalidationFreshnessCertification,
    AsyncObservationOutputContinuityCertification,
    AsyncRetentionReplayPolicyCertification,
}

impl ResourceMilestoneCPolicyCertificationFamily {
    pub fn label(self) -> &'static str {
        match self {
            Self::AsyncResourcePolicyFamilyCertification => {
                "async-resource-policy-family-certification"
            }
            Self::AsyncRetryBudgetAndBackoffCertification => {
                "async-retry-budget-and-backoff-certification"
            }
            Self::AsyncTimeoutDeadlineCertification => "async-timeout-deadline-certification",
            Self::AsyncCancellationSupersessionPolicyCertification => {
                "async-cancellation-supersession-policy-certification"
            }
            Self::AsyncRevalidationFreshnessCertification => {
                "async-revalidation-freshness-certification"
            }
            Self::AsyncObservationOutputContinuityCertification => {
                "async-observation-output-continuity-certification"
            }
            Self::AsyncRetentionReplayPolicyCertification => {
                "async-retention-replay-policy-certification"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResourceMilestoneCPolicyScenarioId {
    RegistryOrderCanonicalization,
    RetryBudgetExhaustionRejected,
    HeartbeatExtensionTerminalDenied,
    RetentionCompactionReportsUnavailableHistory,
    DiagnosticsExpansionBudgetDeniedZeroCold,
    CompatibleDescriptorRestoreAdmitted,
    IncompatibleDescriptorRestoreDenied,
    MissingDescriptorRestoreDenied,
}

impl ResourceMilestoneCPolicyScenarioId {
    pub fn label(self) -> &'static str {
        match self {
            Self::RegistryOrderCanonicalization => "registry-order-canonicalization",
            Self::RetryBudgetExhaustionRejected => "retry-budget-exhaustion-rejected",
            Self::HeartbeatExtensionTerminalDenied => "heartbeat-extension-terminal-denied",
            Self::RetentionCompactionReportsUnavailableHistory => {
                "retention-compaction-reports-unavailable-history"
            }
            Self::DiagnosticsExpansionBudgetDeniedZeroCold => {
                "diagnostics-expansion-budget-denied-zero-cold"
            }
            Self::CompatibleDescriptorRestoreAdmitted => "compatible-descriptor-restore-admitted",
            Self::IncompatibleDescriptorRestoreDenied => "incompatible-descriptor-restore-denied",
            Self::MissingDescriptorRestoreDenied => "missing-descriptor-restore-denied",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResourceMilestoneCPolicyScenarioEvidenceKind {
    RegistryFreeze,
    RetryDenial,
    TimeoutHeartbeatDenial,
    RetentionCompaction,
    DiagnosticsExpansionDenial,
    ReplayCompatibilityProof,
    ReplayCompatibilityDenial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResourceMilestoneCPolicyPerformanceClaimId {
    RegistryFreezeOrderBounded,
    RetryBudgetDenialZeroWake,
    RetentionCompactionAvailabilityBounded,
    DiagnosticsBudgetDenialZeroCold,
    ReplayCompatibilityDescriptorBounded,
}

impl ResourceMilestoneCPolicyPerformanceClaimId {
    pub fn label(self) -> &'static str {
        match self {
            Self::RegistryFreezeOrderBounded => "registry-freeze-order-bounded",
            Self::RetryBudgetDenialZeroWake => "retry-budget-denial-zero-wake",
            Self::RetentionCompactionAvailabilityBounded => {
                "retention-compaction-availability-bounded"
            }
            Self::DiagnosticsBudgetDenialZeroCold => "diagnostics-budget-denial-zero-cold",
            Self::ReplayCompatibilityDescriptorBounded => "replay-compatibility-descriptor-bounded",
        }
    }
}
