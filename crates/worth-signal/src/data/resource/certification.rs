use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::data::error::SignalError;
use crate::data::telemetry::ResourceTelemetry;

use super::completion::DeniedResourceCompletion;
use super::denial::CompletionDenialClass;
use super::diagnostics::{
    ResourceDiagnosticsExpansionBudget, ResourceDiagnosticsExpansionDenial,
    ResourceDiagnosticsExpansionDenialClass, ResourceDiagnosticsSummary,
};
use super::observation::ResourceObservationBatchReport;
use super::policy::{
    DeniedResourcePolicyRestoreCompatibility, ResourcePolicyRestoreCompatibilityDenialClass,
    ResourcePolicyRestoreCompatibilityProof,
};
use super::policy_registry::ResourcePolicyRegistryFreezeReport;
use super::replay_availability::ResourceReplayAvailabilityReport;
use super::retry::{DeniedResourceRetry, ResourceRetryDenialClass};
use super::summary::{
    ResourceBoundaryKind, ResourceBoundaryPerformanceEnvelope, ResourceBranchRestoreReport,
    ResourceCancellationReport, ResourceCompletionAdmissionReport,
    ResourceCompletionBatchAdmissionReport, ResourceCompletionRollbackReport,
    ResourceCostContractId, ResourceCostPosture, ResourceDensityStrategy,
    ResourceLifecycleRetentionCompactionReport, ResourceReplayReconstructionReport,
    ResourceRequestAdmissionReport, ResourceRetryScheduleReport, ResourceRevalidationReport,
    ResourceRuntimeSummary, ResourceRuntimeSummaryReadReport,
    ResourceTimeoutHeartbeatExtensionReport, ResourceTimeoutReport,
};
use super::supersession::{
    ResourceIntentEquivalenceCoalescing, ResourceOverlappingGenerationAdmission,
};
use super::timeout::ResourceTimeoutHeartbeatExtensionDenialClass;
use crate::facade::runtime::ObservationBoundaryOutcome;

pub const REQUIRED_RESOURCE_CERTIFICATION_FAMILIES: [ResourceCertificationFamily; 5] = [
    ResourceCertificationFamily::AsyncResourceLifecycleParity,
    ResourceCertificationFamily::OutOfOrderCompletionSupersession,
    ResourceCertificationFamily::AsyncRollbackObservationEquivalence,
    ResourceCertificationFamily::AsyncBranchRestoreReplayEquivalence,
    ResourceCertificationFamily::AsyncInflightBoundedness,
];

pub const RESOURCE_CERTIFICATION_BUNDLE_SCHEMA_VERSION: &str =
    "worth-signal-resource-certification-bundle-v1";
pub const RESOURCE_CERTIFICATION_BUNDLE_PARITY_SCHEMA_VERSION: &str =
    "worth-signal-resource-certification-bundle-parity-v1";
pub const RESOURCE_MILESTONE_B_HOSTILE_SCENARIO_EVIDENCE_SCHEMA_VERSION: &str =
    "worth-signal-resource-milestone-b-hostile-scenario-evidence-v1";
pub const RESOURCE_MILESTONE_B_SCENARIO_MATRIX_SCHEMA_VERSION: &str =
    "worth-signal-resource-milestone-b-scenario-matrix-v1";
pub const RESOURCE_MILESTONE_B_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION: &str =
    "worth-signal-resource-milestone-b-performance-closeout-v1";
pub const RESOURCE_MILESTONE_B_CERTIFICATION_RUN_SCHEMA_VERSION: &str =
    "worth-signal-resource-milestone-b-certification-run-v1";
pub const RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_BUNDLE_SCHEMA_VERSION: &str =
    "worth-signal-resource-milestone-c-policy-certification-bundle-v1";
pub const RESOURCE_MILESTONE_C_POLICY_SCENARIO_MATRIX_SCHEMA_VERSION: &str =
    "worth-signal-resource-milestone-c-policy-scenario-matrix-v1";
pub const RESOURCE_MILESTONE_C_POLICY_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION: &str =
    "worth-signal-resource-milestone-c-policy-performance-closeout-v1";
pub const RESOURCE_MILESTONE_C_CERTIFICATION_RUN_SCHEMA_VERSION: &str =
    "worth-signal-resource-milestone-c-certification-run-v1";

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
pub enum ResourceCertificationFamily {
    AsyncResourceLifecycleParity,
    OutOfOrderCompletionSupersession,
    AsyncRollbackObservationEquivalence,
    AsyncBranchRestoreReplayEquivalence,
    AsyncInflightBoundedness,
}

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneCPolicyCertificationRecord {
    family: ResourceMilestoneCPolicyCertificationFamily,
    evidence_digest: String,
    performance: ResourceBoundaryPerformanceEnvelope,
    passed: bool,
}

impl ResourceMilestoneCPolicyCertificationRecord {
    fn passing(
        family: ResourceMilestoneCPolicyCertificationFamily,
        evidence_digest: impl Into<String>,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Result<Self, SignalError> {
        let evidence_digest = evidence_digest.into();
        if evidence_digest.is_empty() {
            return Err(SignalError::invalid_input(format!(
                "invalid milestone C policy certification evidence for {}: evidence digest is empty",
                family.label()
            )));
        }
        Ok(Self {
            family,
            evidence_digest,
            performance,
            passed: true,
        })
    }

    pub fn family(&self) -> ResourceMilestoneCPolicyCertificationFamily {
        self.family
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }

    pub fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneCPolicyCertificationSummary {
    required_family_count: u32,
    provided_record_count: u32,
    certified_family_count: u32,
    failed_family_count: u32,
    missing_family_count: u32,
    duplicate_family_count: u32,
}

impl ResourceMilestoneCPolicyCertificationSummary {
    pub fn required_family_count(self) -> u32 {
        self.required_family_count
    }

    pub fn provided_record_count(self) -> u32 {
        self.provided_record_count
    }

    pub fn certified_family_count(self) -> u32 {
        self.certified_family_count
    }

    pub fn failed_family_count(self) -> u32 {
        self.failed_family_count
    }

    pub fn missing_family_count(self) -> u32 {
        self.missing_family_count
    }

    pub fn duplicate_family_count(self) -> u32 {
        self.duplicate_family_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneCPolicyCertificationBundle {
    schema_version: String,
    records: Vec<ResourceMilestoneCPolicyCertificationRecord>,
    summary: ResourceMilestoneCPolicyCertificationSummary,
    bundle_digest: String,
    passed: bool,
}

impl ResourceMilestoneCPolicyCertificationBundle {
    pub fn schema_version(&self) -> &str {
        &self.schema_version
    }

    pub fn records(&self) -> &[ResourceMilestoneCPolicyCertificationRecord] {
        &self.records
    }

    pub fn summary(&self) -> ResourceMilestoneCPolicyCertificationSummary {
        self.summary
    }

    pub fn bundle_digest(&self) -> &str {
        &self.bundle_digest
    }

    pub fn passed(&self) -> bool {
        self.passed
    }

    pub fn ensure_passed(&self) -> Result<(), SignalError> {
        if self.passed {
            Ok(())
        } else {
            Err(SignalError::invalid_input(format!(
                "resource milestone C policy certification bundle failed completeness checks"
            )))
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ResourceMilestoneCPolicyCertificationBuilder {
    async_resource_policy_family_certification: Option<ResourceMilestoneCPolicyCertificationRecord>,
    async_retry_budget_and_backoff_certification:
        Option<ResourceMilestoneCPolicyCertificationRecord>,
    async_timeout_deadline_certification: Option<ResourceMilestoneCPolicyCertificationRecord>,
    async_cancellation_supersession_policy_certification:
        Option<ResourceMilestoneCPolicyCertificationRecord>,
    async_revalidation_freshness_certification: Option<ResourceMilestoneCPolicyCertificationRecord>,
    async_observation_output_continuity_certification:
        Option<ResourceMilestoneCPolicyCertificationRecord>,
    async_retention_replay_policy_certification:
        Option<ResourceMilestoneCPolicyCertificationRecord>,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneCPolicyScenarioRow {
    id: ResourceMilestoneCPolicyScenarioId,
    evidence_kind: ResourceMilestoneCPolicyScenarioEvidenceKind,
    certification_family: Option<ResourceMilestoneCPolicyCertificationFamily>,
    policy_provenance_digest: Option<String>,
    retry_denial_class: Option<ResourceRetryDenialClass>,
    timeout_heartbeat_denial_class: Option<ResourceTimeoutHeartbeatExtensionDenialClass>,
    replay_restore_denial_class: Option<ResourcePolicyRestoreCompatibilityDenialClass>,
    evidence_digest: String,
    performance: ResourceBoundaryPerformanceEnvelope,
    passed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneCPolicyScenarioMatrixSummary {
    required_scenario_count: u32,
    certified_scenario_count: u32,
    failed_scenario_count: u32,
    bundle_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneCPolicyScenarioMatrix {
    schema_version: String,
    rows: Vec<ResourceMilestoneCPolicyScenarioRow>,
    summary: ResourceMilestoneCPolicyScenarioMatrixSummary,
    matrix_digest: String,
    passed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneCPolicyPerformanceCloseoutRow {
    id: ResourceMilestoneCPolicyPerformanceClaimId,
    evidence_digest: String,
    policy_provenance_digest: String,
    performance: ResourceBoundaryPerformanceEnvelope,
    passed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneCPolicyPerformanceCloseoutSummary {
    required_claim_count: u32,
    certified_claim_count: u32,
    failed_claim_count: u32,
    scenario_matrix_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneCPolicyPerformanceCloseout {
    schema_version: String,
    scenario_matrix_digest: String,
    rows: Vec<ResourceMilestoneCPolicyPerformanceCloseoutRow>,
    summary: ResourceMilestoneCPolicyPerformanceCloseoutSummary,
    closeout_digest: String,
    passed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneCCertificationRunSummary {
    required_family_count: u32,
    certified_family_count: u32,
    failed_family_count: u32,
    bundle_digest: String,
    required_scenario_count: u32,
    certified_scenario_count: u32,
    scenario_matrix_digest: String,
    required_performance_claim_count: u32,
    certified_performance_claim_count: u32,
    performance_closeout_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneCCertificationRun {
    schema_version: String,
    bundle: ResourceMilestoneCPolicyCertificationBundle,
    scenario_matrix: ResourceMilestoneCPolicyScenarioMatrix,
    performance_closeout: ResourceMilestoneCPolicyPerformanceCloseout,
    summary: ResourceMilestoneCCertificationRunSummary,
    run_digest: String,
    passed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceCertificationRecord {
    family: ResourceCertificationFamily,
    evidence_digest: String,
    performance: ResourceBoundaryPerformanceEnvelope,
    passed: bool,
}

impl ResourceCertificationRecord {
    pub(crate) fn passing(
        family: ResourceCertificationFamily,
        evidence_digest: impl Into<String>,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Result<Self, SignalError> {
        let evidence_digest = evidence_digest.into();
        if evidence_digest.is_empty() {
            return Err(invalid_resource_certification_evidence(
                family,
                "evidence digest is empty",
            ));
        }
        Ok(Self {
            family,
            evidence_digest,
            performance,
            passed: true,
        })
    }

    pub fn family(&self) -> ResourceCertificationFamily {
        self.family
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }

    pub fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Clone, Default)]
pub struct ResourceCertificationBuilder {
    async_resource_lifecycle_parity: Option<ResourceCertificationRecord>,
    out_of_order_completion_supersession: Option<ResourceCertificationRecord>,
    async_rollback_observation_equivalence: Option<ResourceCertificationRecord>,
    async_branch_restore_replay_equivalence: Option<ResourceCertificationRecord>,
    async_inflight_boundedness: Option<ResourceCertificationRecord>,
}

impl ResourceCertificationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_async_resource_lifecycle_parity(
        mut self,
        baseline: &ResourceReplayReconstructionReport,
        equivalent: &ResourceReplayReconstructionReport,
        baseline_diagnostics: &ResourceDiagnosticsSummary,
        equivalent_diagnostics: &ResourceDiagnosticsSummary,
    ) -> Result<Self, SignalError> {
        self.async_resource_lifecycle_parity = Some(Self::record(
            self.async_resource_lifecycle_parity.take(),
            ResourceCertificationFamily::AsyncResourceLifecycleParity,
            ResourceCertificationEvidence::lifecycle_parity(
                baseline,
                equivalent,
                baseline_diagnostics,
                equivalent_diagnostics,
            )?,
        )?);
        Ok(self)
    }

    pub fn with_out_of_order_completion_supersession(
        mut self,
        admission: ResourceRequestAdmissionReport,
    ) -> Result<Self, SignalError> {
        self.out_of_order_completion_supersession = Some(Self::record(
            self.out_of_order_completion_supersession.take(),
            ResourceCertificationFamily::OutOfOrderCompletionSupersession,
            ResourceCertificationEvidence::out_of_order_supersession(admission)?,
        )?);
        Ok(self)
    }

    pub fn with_async_rollback_observation_equivalence(
        mut self,
        rollback: ResourceCompletionRollbackReport,
        observation: ResourceObservationBatchReport,
        control_observation: ResourceObservationBatchReport,
        pre_rollback: &ResourceReplayReconstructionReport,
        post_rollback: &ResourceReplayReconstructionReport,
        diagnostics: &ResourceDiagnosticsSummary,
    ) -> Result<Self, SignalError> {
        self.async_rollback_observation_equivalence = Some(Self::record(
            self.async_rollback_observation_equivalence.take(),
            ResourceCertificationFamily::AsyncRollbackObservationEquivalence,
            ResourceCertificationEvidence::rollback_observation(
                rollback,
                observation,
                control_observation,
                pre_rollback,
                post_rollback,
                diagnostics,
            )?,
        )?);
        Ok(self)
    }

    pub fn with_async_branch_restore_replay_equivalence(
        mut self,
        restore: ResourceBranchRestoreReport,
        replay: &ResourceReplayReconstructionReport,
    ) -> Result<Self, SignalError> {
        self.async_branch_restore_replay_equivalence = Some(Self::record(
            self.async_branch_restore_replay_equivalence.take(),
            ResourceCertificationFamily::AsyncBranchRestoreReplayEquivalence,
            ResourceCertificationEvidence::branch_restore_replay(restore, replay),
        )?);
        Ok(self)
    }

    pub fn with_async_inflight_boundedness(
        mut self,
        summary: ResourceRuntimeSummary,
        replay: &ResourceReplayReconstructionReport,
        telemetry: ResourceTelemetry,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Result<Self, SignalError> {
        self.async_inflight_boundedness = Some(Self::record(
            self.async_inflight_boundedness.take(),
            ResourceCertificationFamily::AsyncInflightBoundedness,
            ResourceCertificationEvidence::inflight_boundedness(
                summary,
                replay,
                telemetry,
                performance,
            )?,
        )?);
        Ok(self)
    }

    pub fn build(self) -> Result<ResourceCertificationBundle, SignalError> {
        let records = [
            self.async_resource_lifecycle_parity,
            self.out_of_order_completion_supersession,
            self.async_rollback_observation_equivalence,
            self.async_branch_restore_replay_equivalence,
            self.async_inflight_boundedness,
        ];
        let mut complete = Vec::with_capacity(REQUIRED_RESOURCE_CERTIFICATION_FAMILIES.len());
        for (family, record) in REQUIRED_RESOURCE_CERTIFICATION_FAMILIES
            .into_iter()
            .zip(records)
        {
            let Some(record) = record else {
                return Err(invalid_resource_certification_evidence(
                    family,
                    "required certification family was not supplied",
                ));
            };
            complete.push(record);
        }

        let bundle = resource_certification_bundle(complete);
        bundle.ensure_passed()?;
        Ok(bundle)
    }

    fn record(
        existing: Option<ResourceCertificationRecord>,
        family: ResourceCertificationFamily,
        evidence: ResourceCertificationEvidence,
    ) -> Result<ResourceCertificationRecord, SignalError> {
        if existing.is_some() {
            return Err(invalid_resource_certification_evidence(
                family,
                "duplicate certification family evidence",
            ));
        }
        ResourceCertificationRecord::passing(family, evidence.digest, evidence.performance)
    }
}

impl ResourceMilestoneCPolicyCertificationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_async_resource_policy_family_certification(
        mut self,
        freeze_report: &ResourcePolicyRegistryFreezeReport,
    ) -> Result<Self, SignalError> {
        self.async_resource_policy_family_certification = Some(Self::record(
            self.async_resource_policy_family_certification.take(),
            ResourceMilestoneCPolicyCertificationFamily::AsyncResourcePolicyFamilyCertification,
            ResourceMilestoneCPolicyCertificationEvidence::resource_policy_family(freeze_report),
        )?);
        Ok(self)
    }

    pub fn with_async_retry_budget_and_backoff_certification(
        mut self,
        report: &ResourceRetryScheduleReport,
    ) -> Result<Self, SignalError> {
        self.async_retry_budget_and_backoff_certification = Some(Self::record(
            self.async_retry_budget_and_backoff_certification.take(),
            ResourceMilestoneCPolicyCertificationFamily::AsyncRetryBudgetAndBackoffCertification,
            ResourceMilestoneCPolicyCertificationEvidence::retry_budget_and_backoff(report)?,
        )?);
        Ok(self)
    }

    pub fn with_async_timeout_deadline_certification(
        mut self,
        timeout_report: &ResourceTimeoutReport,
        heartbeat_report: &ResourceTimeoutHeartbeatExtensionReport,
    ) -> Result<Self, SignalError> {
        self.async_timeout_deadline_certification = Some(Self::record(
            self.async_timeout_deadline_certification.take(),
            ResourceMilestoneCPolicyCertificationFamily::AsyncTimeoutDeadlineCertification,
            ResourceMilestoneCPolicyCertificationEvidence::timeout_deadline(
                timeout_report,
                heartbeat_report,
            )?,
        )?);
        Ok(self)
    }

    pub fn with_async_cancellation_supersession_policy_certification(
        mut self,
        cancellation_report: &ResourceCancellationReport,
        overlap_admission: &ResourceOverlappingGenerationAdmission,
        intent_coalescing: &ResourceIntentEquivalenceCoalescing,
    ) -> Result<Self, SignalError> {
        self.async_cancellation_supersession_policy_certification = Some(Self::record(
            self.async_cancellation_supersession_policy_certification.take(),
            ResourceMilestoneCPolicyCertificationFamily::AsyncCancellationSupersessionPolicyCertification,
            ResourceMilestoneCPolicyCertificationEvidence::cancellation_supersession(
                cancellation_report,
                overlap_admission,
                intent_coalescing,
            )?,
        )?);
        Ok(self)
    }

    pub fn with_async_revalidation_freshness_certification(
        mut self,
        report: &ResourceRevalidationReport,
    ) -> Result<Self, SignalError> {
        self.async_revalidation_freshness_certification = Some(Self::record(
            self.async_revalidation_freshness_certification.take(),
            ResourceMilestoneCPolicyCertificationFamily::AsyncRevalidationFreshnessCertification,
            ResourceMilestoneCPolicyCertificationEvidence::revalidation_freshness(report)?,
        )?);
        Ok(self)
    }

    pub fn with_async_observation_output_continuity_certification(
        mut self,
        report: &ResourceObservationBatchReport,
    ) -> Result<Self, SignalError> {
        self.async_observation_output_continuity_certification = Some(Self::record(
            self.async_observation_output_continuity_certification.take(),
            ResourceMilestoneCPolicyCertificationFamily::AsyncObservationOutputContinuityCertification,
            ResourceMilestoneCPolicyCertificationEvidence::observation_output_continuity(report)?,
        )?);
        Ok(self)
    }

    pub fn with_async_retention_replay_policy_certification(
        mut self,
        retention_report: &ResourceLifecycleRetentionCompactionReport,
        replay_availability: &ResourceReplayAvailabilityReport,
    ) -> Result<Self, SignalError> {
        self.async_retention_replay_policy_certification = Some(Self::record(
            self.async_retention_replay_policy_certification.take(),
            ResourceMilestoneCPolicyCertificationFamily::AsyncRetentionReplayPolicyCertification,
            ResourceMilestoneCPolicyCertificationEvidence::retention_replay(
                retention_report,
                replay_availability,
            )?,
        )?);
        Ok(self)
    }

    pub fn build(self) -> Result<ResourceMilestoneCPolicyCertificationBundle, SignalError> {
        let records = [
            self.async_resource_policy_family_certification,
            self.async_retry_budget_and_backoff_certification,
            self.async_timeout_deadline_certification,
            self.async_cancellation_supersession_policy_certification,
            self.async_revalidation_freshness_certification,
            self.async_observation_output_continuity_certification,
            self.async_retention_replay_policy_certification,
        ];
        let mut complete =
            Vec::with_capacity(REQUIRED_RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_FAMILIES.len());
        for (family, record) in REQUIRED_RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_FAMILIES
            .into_iter()
            .zip(records)
        {
            let Some(record) = record else {
                return Err(SignalError::invalid_input(format!(
                    "invalid milestone C policy certification evidence for {}: required certification family was not supplied",
                    family.label()
                )));
            };
            complete.push(record);
        }
        let bundle = resource_milestone_c_policy_certification_bundle(complete);
        bundle.ensure_passed()?;
        Ok(bundle)
    }

    fn record(
        existing: Option<ResourceMilestoneCPolicyCertificationRecord>,
        family: ResourceMilestoneCPolicyCertificationFamily,
        evidence: ResourceMilestoneCPolicyCertificationEvidence,
    ) -> Result<ResourceMilestoneCPolicyCertificationRecord, SignalError> {
        if existing.is_some() {
            return Err(SignalError::invalid_input(format!(
                "invalid milestone C policy certification evidence for {}: duplicate certification family evidence",
                family.label()
            )));
        }
        ResourceMilestoneCPolicyCertificationRecord::passing(
            family,
            evidence.digest,
            evidence.performance,
        )
    }
}

impl ResourceMilestoneCPolicyScenarioRow {
    fn from_registry_freeze(
        report: &ResourcePolicyRegistryFreezeReport,
    ) -> Result<Self, SignalError> {
        Ok(Self {
            id: ResourceMilestoneCPolicyScenarioId::RegistryOrderCanonicalization,
            evidence_kind: ResourceMilestoneCPolicyScenarioEvidenceKind::RegistryFreeze,
            certification_family: Some(
                ResourceMilestoneCPolicyCertificationFamily::AsyncResourcePolicyFamilyCertification,
            ),
            policy_provenance_digest: Some(report.registry_digest().as_str().to_owned()),
            retry_denial_class: None,
            timeout_heartbeat_denial_class: None,
            replay_restore_denial_class: None,
            evidence_digest: resource_canonical_digest(
                &ResourceMilestoneCPolicyRegistryFreezeEvidenceBasis {
                    descriptor_count: report.descriptor_count(),
                    id_index_width: report.id_index_width(),
                    kind_name_index_width: report.kind_name_index_width(),
                    registry_digest: report.registry_digest().as_str(),
                },
            ),
            performance: ResourceBoundaryPerformanceEnvelope::policy_compatibility(
                report.descriptor_count() as u32,
                0,
            ),
            passed: true,
        })
    }

    fn from_retry_denial(report: &ResourceRetryScheduleReport) -> Result<Self, SignalError> {
        let denied = report.denied_retry().ok_or_else(|| {
            SignalError::invalid_input(
                "resource milestone C policy scenario retry-budget-exhaustion-rejected requires denied retry evidence",
            )
        })?;
        if denied.class() != ResourceRetryDenialClass::RetryBudgetExhausted {
            return Err(SignalError::invalid_input(
                "resource milestone C policy scenario retry-budget-exhaustion-rejected requires RetryBudgetExhausted denial evidence",
            ));
        }
        Ok(Self {
            id: ResourceMilestoneCPolicyScenarioId::RetryBudgetExhaustionRejected,
            evidence_kind: ResourceMilestoneCPolicyScenarioEvidenceKind::RetryDenial,
            certification_family: Some(
                ResourceMilestoneCPolicyCertificationFamily::AsyncRetryBudgetAndBackoffCertification,
            ),
            policy_provenance_digest: Some(denied.policy_decision_digest().as_str().to_owned()),
            retry_denial_class: Some(denied.class()),
            timeout_heartbeat_denial_class: None,
            replay_restore_denial_class: None,
            evidence_digest: resource_canonical_digest(&ResourceMilestoneCRetryDenialEvidenceBasis {
                class: denied.class(),
                retry_budget_scope: denied.retry_budget_scope(),
                retry_budget_limit: denied.retry_budget_limit(),
                retry_budget_usage: denied.retry_budget_usage(),
                performance: report.performance(),
            }),
            performance: report.performance(),
            passed: true,
        })
    }

    fn from_timeout_heartbeat_denial(
        report: &ResourceTimeoutHeartbeatExtensionReport,
    ) -> Result<Self, SignalError> {
        let denied = report.denied_extension().ok_or_else(|| {
            SignalError::invalid_input(
                "resource milestone C policy scenario heartbeat-extension-terminal-denied requires denied heartbeat extension evidence",
            )
        })?;
        if denied.class() != ResourceTimeoutHeartbeatExtensionDenialClass::NonActiveRequest {
            return Err(SignalError::invalid_input(
                "resource milestone C policy scenario heartbeat-extension-terminal-denied requires NonActiveRequest denial evidence",
            ));
        }
        Ok(Self {
            id: ResourceMilestoneCPolicyScenarioId::HeartbeatExtensionTerminalDenied,
            evidence_kind: ResourceMilestoneCPolicyScenarioEvidenceKind::TimeoutHeartbeatDenial,
            certification_family: Some(
                ResourceMilestoneCPolicyCertificationFamily::AsyncTimeoutDeadlineCertification,
            ),
            policy_provenance_digest: None,
            retry_denial_class: None,
            timeout_heartbeat_denial_class: Some(denied.class()),
            replay_restore_denial_class: None,
            evidence_digest: resource_canonical_digest(
                &ResourceMilestoneCTimeoutHeartbeatDenialEvidenceBasis {
                    class: denied.class(),
                    performance: report.performance(),
                },
            ),
            performance: report.performance(),
            passed: true,
        })
    }

    fn from_retention_compaction(
        report: &ResourceLifecycleRetentionCompactionReport,
    ) -> Result<Self, SignalError> {
        if report.retained_history_unavailable_count() == 0
            && report.retained_denied_completion_pruned_count() == 0
            && report.retained_retry_lineage_pruned_count() == 0
        {
            return Err(SignalError::invalid_input(
                "resource milestone C policy scenario retention-compaction-reports-unavailable-history requires unavailable or pruned history evidence",
            ));
        }
        Ok(Self {
            id: ResourceMilestoneCPolicyScenarioId::RetentionCompactionReportsUnavailableHistory,
            evidence_kind: ResourceMilestoneCPolicyScenarioEvidenceKind::RetentionCompaction,
            certification_family: Some(
                ResourceMilestoneCPolicyCertificationFamily::AsyncRetentionReplayPolicyCertification,
            ),
            policy_provenance_digest: Some(report.policy_provenance_digest().to_owned()),
            retry_denial_class: None,
            timeout_heartbeat_denial_class: None,
            replay_restore_denial_class: None,
            evidence_digest: resource_canonical_digest(
                &ResourceMilestoneCRetentionCompactionEvidenceBasis {
                    retained_history_pruned_count: report.retained_history_pruned_count(),
                    retained_history_unavailable_count: report
                        .retained_history_unavailable_count(),
                    retained_denied_completion_pruned_count: report
                        .retained_denied_completion_pruned_count(),
                    retained_retry_lineage_pruned_count: report
                        .retained_retry_lineage_pruned_count(),
                    compacted_terminal_summary_count: report.compacted_terminal_summary_count(),
                    performance: report.performance(),
                },
            ),
            performance: report.performance(),
            passed: true,
        })
    }

    fn from_diagnostics_denial(
        denial: &ResourceDiagnosticsExpansionDenial,
    ) -> Result<Self, SignalError> {
        if denial.performance().boundary() != ResourceBoundaryKind::DiagnosticsExpansion {
            return Err(SignalError::invalid_input(
                "resource milestone C policy scenario diagnostics-expansion-budget-denied-zero-cold requires diagnostics expansion denial evidence",
            ));
        }
        Ok(Self {
            id: ResourceMilestoneCPolicyScenarioId::DiagnosticsExpansionBudgetDeniedZeroCold,
            evidence_kind: ResourceMilestoneCPolicyScenarioEvidenceKind::DiagnosticsExpansionDenial,
            certification_family: Some(
                ResourceMilestoneCPolicyCertificationFamily::AsyncRetentionReplayPolicyCertification,
            ),
            policy_provenance_digest: Some(denial.policy_decision_digest().as_str().to_owned()),
            retry_denial_class: None,
            timeout_heartbeat_denial_class: None,
            replay_restore_denial_class: None,
            evidence_digest: resource_canonical_digest(
                &ResourceMilestoneCDiagnosticsDenialEvidenceBasis {
                    class: denial.class(),
                    policy_decision_class: denial.policy_decision_class(),
                    replay_reconstruction_width: denial.replay_reconstruction_width(),
                    forensic_reconstruction_width: denial.forensic_reconstruction_width(),
                    performance: denial.performance(),
                    policy_decision_digest: denial.policy_decision_digest().as_str(),
                },
            ),
            performance: denial.performance(),
            passed: true,
        })
    }

    fn from_restore_proof(
        proof: &ResourcePolicyRestoreCompatibilityProof,
    ) -> Result<Self, SignalError> {
        Ok(Self {
            id: ResourceMilestoneCPolicyScenarioId::CompatibleDescriptorRestoreAdmitted,
            evidence_kind: ResourceMilestoneCPolicyScenarioEvidenceKind::ReplayCompatibilityProof,
            certification_family: Some(
                ResourceMilestoneCPolicyCertificationFamily::AsyncRetentionReplayPolicyCertification,
            ),
            policy_provenance_digest: Some(proof.replay_decision_digest().as_str().to_owned()),
            retry_denial_class: None,
            timeout_heartbeat_denial_class: None,
            replay_restore_denial_class: None,
            evidence_digest: resource_canonical_digest(
                &ResourceMilestoneCRestoreProofEvidenceBasis {
                    compatibility_digest: proof.compatibility_digest().as_str(),
                    replay_decision_digest: proof.replay_decision_digest().as_str(),
                    performance: proof.performance(),
                },
            ),
            performance: proof.performance(),
            passed: true,
        })
    }

    fn from_restore_denial(
        id: ResourceMilestoneCPolicyScenarioId,
        denial: &DeniedResourcePolicyRestoreCompatibility,
    ) -> Result<Self, SignalError> {
        let expected_class = match id {
            ResourceMilestoneCPolicyScenarioId::IncompatibleDescriptorRestoreDenied => {
                ResourcePolicyRestoreCompatibilityDenialClass::VersionIncompatible
            }
            ResourceMilestoneCPolicyScenarioId::MissingDescriptorRestoreDenied => {
                ResourcePolicyRestoreCompatibilityDenialClass::MissingDescriptor
            }
            _ => {
                return Err(SignalError::invalid_input(format!(
                    "resource milestone C policy scenario {} is not a restore-denial scenario",
                    id.label()
                )))
            }
        };
        if denial.class() != expected_class {
            return Err(SignalError::invalid_input(format!(
                "resource milestone C policy scenario {} requires {:?} denial evidence",
                id.label(),
                expected_class
            )));
        }
        Ok(Self {
            id,
            evidence_kind: ResourceMilestoneCPolicyScenarioEvidenceKind::ReplayCompatibilityDenial,
            certification_family: Some(
                ResourceMilestoneCPolicyCertificationFamily::AsyncRetentionReplayPolicyCertification,
            ),
            policy_provenance_digest: Some(denial.replay_decision_digest().as_str().to_owned()),
            retry_denial_class: None,
            timeout_heartbeat_denial_class: None,
            replay_restore_denial_class: Some(denial.class()),
            evidence_digest: resource_canonical_digest(
                &ResourceMilestoneCRestoreDenialEvidenceBasis {
                    class: denial.class(),
                    primary_incompatible_kind: denial.primary_incompatible_kind(),
                    compatibility_digest: denial.compatibility_digest().as_str(),
                    replay_decision_digest: denial.replay_decision_digest().as_str(),
                    performance: denial.performance(),
                },
            ),
            performance: denial.performance(),
            passed: true,
        })
    }

    pub fn id(&self) -> ResourceMilestoneCPolicyScenarioId {
        self.id
    }

    pub fn evidence_kind(&self) -> ResourceMilestoneCPolicyScenarioEvidenceKind {
        self.evidence_kind
    }

    pub fn certification_family(&self) -> Option<ResourceMilestoneCPolicyCertificationFamily> {
        self.certification_family
    }

    pub fn policy_provenance_digest(&self) -> Option<&str> {
        self.policy_provenance_digest.as_deref()
    }

    pub fn retry_denial_class(&self) -> Option<ResourceRetryDenialClass> {
        self.retry_denial_class
    }

    pub fn timeout_heartbeat_denial_class(
        &self,
    ) -> Option<ResourceTimeoutHeartbeatExtensionDenialClass> {
        self.timeout_heartbeat_denial_class
    }

    pub fn replay_restore_denial_class(
        &self,
    ) -> Option<ResourcePolicyRestoreCompatibilityDenialClass> {
        self.replay_restore_denial_class
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }

    pub fn passed(&self) -> bool {
        self.passed
    }
}

impl ResourceMilestoneCPolicyScenarioMatrix {
    pub fn schema_version(&self) -> &str {
        &self.schema_version
    }

    pub fn rows(&self) -> &[ResourceMilestoneCPolicyScenarioRow] {
        &self.rows
    }

    pub fn summary(&self) -> &ResourceMilestoneCPolicyScenarioMatrixSummary {
        &self.summary
    }

    pub fn matrix_digest(&self) -> &str {
        &self.matrix_digest
    }

    pub fn passed(&self) -> bool {
        self.passed
    }
}

impl ResourceMilestoneCPolicyScenarioMatrixSummary {
    pub fn required_scenario_count(&self) -> u32 {
        self.required_scenario_count
    }

    pub fn certified_scenario_count(&self) -> u32 {
        self.certified_scenario_count
    }

    pub fn failed_scenario_count(&self) -> u32 {
        self.failed_scenario_count
    }

    pub fn bundle_digest(&self) -> &str {
        &self.bundle_digest
    }
}

impl ResourceMilestoneCPolicyPerformanceCloseoutRow {
    fn scenario_row(
        id: ResourceMilestoneCPolicyPerformanceClaimId,
        scenario: ResourceMilestoneCPolicyScenarioId,
        matrix: &ResourceMilestoneCPolicyScenarioMatrix,
    ) -> Result<Self, SignalError> {
        let Some(row) = matrix.rows().iter().find(|row| row.id() == scenario) else {
            return Err(SignalError::invalid_input(format!(
                "resource milestone C policy performance claim {} is missing {} scenario evidence",
                id.label(),
                scenario.label()
            )));
        };
        if !row.passed() {
            return Err(SignalError::invalid_input(format!(
                "resource milestone C policy performance claim {} requires passing scenario evidence",
                id.label()
            )));
        }
        let policy_provenance_digest = row.policy_provenance_digest().ok_or_else(|| {
            SignalError::invalid_input(format!(
                "resource milestone C policy performance claim {} requires explicit policy provenance digest",
                id.label()
            ))
        })?;
        validate_milestone_c_policy_performance(id, row.performance())?;
        let evidence_digest =
            resource_canonical_digest(&ResourceMilestoneCPolicyPerformanceScenarioEvidenceBasis {
                claim: id,
                scenario,
                scenario_evidence_digest: row.evidence_digest(),
                policy_provenance_digest,
                performance: row.performance(),
            });
        Ok(Self {
            id,
            evidence_digest,
            policy_provenance_digest: policy_provenance_digest.to_owned(),
            performance: row.performance(),
            passed: true,
        })
    }

    fn replay_descriptor_bound(
        matrix: &ResourceMilestoneCPolicyScenarioMatrix,
    ) -> Result<Self, SignalError> {
        let id = ResourceMilestoneCPolicyPerformanceClaimId::ReplayCompatibilityDescriptorBounded;
        let scenarios = [
            ResourceMilestoneCPolicyScenarioId::CompatibleDescriptorRestoreAdmitted,
            ResourceMilestoneCPolicyScenarioId::IncompatibleDescriptorRestoreDenied,
            ResourceMilestoneCPolicyScenarioId::MissingDescriptorRestoreDenied,
        ];
        let mut row_digests = Vec::with_capacity(scenarios.len());
        let mut policy_provenance_rows = Vec::with_capacity(scenarios.len());
        let mut compared_width = 0_u32;
        let mut incompatible_width = 0_u32;
        for scenario in scenarios {
            let Some(row) = matrix.rows().iter().find(|row| row.id() == scenario) else {
                return Err(SignalError::invalid_input(format!(
                    "resource milestone C policy performance claim {} is missing {} scenario evidence",
                    id.label(),
                    scenario.label()
                )));
            };
            if !row.passed()
                || row.evidence_kind()
                    == ResourceMilestoneCPolicyScenarioEvidenceKind::RegistryFreeze
                || row.performance().boundary() != ResourceBoundaryKind::PolicyCompatibility
            {
                return Err(SignalError::invalid_input(format!(
                    "resource milestone C policy performance claim {} requires passing policy-compatibility replay rows",
                    id.label()
                )));
            }
            compared_width = compared_width.saturating_add(row.performance().input_width());
            incompatible_width =
                incompatible_width.saturating_add(row.performance().denied_count());
            row_digests.push((scenario, row.evidence_digest().to_owned()));
            policy_provenance_rows.push((
                scenario,
                row.policy_provenance_digest()
                    .ok_or_else(|| {
                        SignalError::invalid_input(format!(
                            "resource milestone C policy performance claim {} requires replay policy provenance for {}",
                            id.label(),
                            scenario.label()
                        ))
                    })?
                    .to_owned(),
            ));
        }
        let performance = ResourceBoundaryPerformanceEnvelope::policy_compatibility(
            compared_width,
            incompatible_width,
        );
        let policy_provenance_digest = resource_canonical_digest(
            &ResourceMilestoneCPolicyPerformanceReplayPolicyProvenanceBasis {
                claim: id,
                row_policy_provenance: &policy_provenance_rows,
            },
        );
        let evidence_digest = resource_canonical_digest(
            &ResourceMilestoneCPolicyPerformanceReplayCompatibilityBasis {
                claim: id,
                scenario_matrix_digest: matrix.matrix_digest(),
                row_digests: &row_digests,
                policy_provenance_digest: &policy_provenance_digest,
                performance,
            },
        );
        Ok(Self {
            id,
            evidence_digest,
            policy_provenance_digest,
            performance,
            passed: true,
        })
    }

    pub fn id(&self) -> ResourceMilestoneCPolicyPerformanceClaimId {
        self.id
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    pub fn policy_provenance_digest(&self) -> &str {
        &self.policy_provenance_digest
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }

    pub fn passed(&self) -> bool {
        self.passed
    }
}

impl ResourceMilestoneCPolicyPerformanceCloseoutSummary {
    pub fn required_claim_count(&self) -> u32 {
        self.required_claim_count
    }

    pub fn certified_claim_count(&self) -> u32 {
        self.certified_claim_count
    }

    pub fn failed_claim_count(&self) -> u32 {
        self.failed_claim_count
    }

    pub fn scenario_matrix_digest(&self) -> &str {
        &self.scenario_matrix_digest
    }
}

impl ResourceMilestoneCPolicyPerformanceCloseout {
    pub fn schema_version(&self) -> &str {
        &self.schema_version
    }

    pub fn scenario_matrix_digest(&self) -> &str {
        &self.scenario_matrix_digest
    }

    pub fn rows(&self) -> &[ResourceMilestoneCPolicyPerformanceCloseoutRow] {
        &self.rows
    }

    pub fn summary(&self) -> &ResourceMilestoneCPolicyPerformanceCloseoutSummary {
        &self.summary
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }

    pub fn passed(&self) -> bool {
        self.passed
    }
}

impl ResourceMilestoneCCertificationRunSummary {
    pub fn required_family_count(&self) -> u32 {
        self.required_family_count
    }

    pub fn certified_family_count(&self) -> u32 {
        self.certified_family_count
    }

    pub fn failed_family_count(&self) -> u32 {
        self.failed_family_count
    }

    pub fn bundle_digest(&self) -> &str {
        &self.bundle_digest
    }

    pub fn required_scenario_count(&self) -> u32 {
        self.required_scenario_count
    }

    pub fn certified_scenario_count(&self) -> u32 {
        self.certified_scenario_count
    }

    pub fn scenario_matrix_digest(&self) -> &str {
        &self.scenario_matrix_digest
    }

    pub fn required_performance_claim_count(&self) -> u32 {
        self.required_performance_claim_count
    }

    pub fn certified_performance_claim_count(&self) -> u32 {
        self.certified_performance_claim_count
    }

    pub fn performance_closeout_digest(&self) -> &str {
        &self.performance_closeout_digest
    }
}

impl ResourceMilestoneCCertificationRun {
    pub fn schema_version(&self) -> &str {
        &self.schema_version
    }

    pub fn bundle(&self) -> &ResourceMilestoneCPolicyCertificationBundle {
        &self.bundle
    }

    pub fn scenario_matrix(&self) -> &ResourceMilestoneCPolicyScenarioMatrix {
        &self.scenario_matrix
    }

    pub fn performance_closeout(&self) -> &ResourceMilestoneCPolicyPerformanceCloseout {
        &self.performance_closeout
    }

    pub fn summary(&self) -> &ResourceMilestoneCCertificationRunSummary {
        &self.summary
    }

    pub fn run_digest(&self) -> &str {
        &self.run_digest
    }

    pub fn passed(&self) -> bool {
        self.passed
    }
}

fn validate_milestone_c_policy_performance(
    id: ResourceMilestoneCPolicyPerformanceClaimId,
    performance: ResourceBoundaryPerformanceEnvelope,
) -> Result<(), SignalError> {
    match id {
        ResourceMilestoneCPolicyPerformanceClaimId::RegistryFreezeOrderBounded => {
            if performance.boundary() != ResourceBoundaryKind::PolicyCompatibility
                || performance.cost_posture() != ResourceCostPosture::Verified
                || performance.cost_contract() != ResourceCostContractId::new(18)
                || performance.denied_count() != 0
            {
                return Err(SignalError::invalid_input(format!(
                    "resource milestone C policy performance claim {} requires verified registry freeze compatibility evidence",
                    id.label()
                )));
            }
        }
        ResourceMilestoneCPolicyPerformanceClaimId::RetryBudgetDenialZeroWake => {
            if performance.boundary() != ResourceBoundaryKind::RetrySchedule
                || performance.cost_posture() != ResourceCostPosture::Verified
                || performance.cost_contract() != ResourceCostContractId::new(5)
                || performance.admitted_count() != 0
                || performance.denied_count() != 1
                || performance.temporal_wake_footprint() != 0
                || performance.retry_budget_scope_touch_count() == 0
            {
                return Err(SignalError::invalid_input(format!(
                    "resource milestone C policy performance claim {} requires zero-wake retry budget denial evidence",
                    id.label()
                )));
            }
        }
        ResourceMilestoneCPolicyPerformanceClaimId::RetentionCompactionAvailabilityBounded => {
            if performance.boundary() != ResourceBoundaryKind::LifecycleRetentionCompaction
                || performance.cost_posture() != ResourceCostPosture::Verified
                || performance.cost_contract() != ResourceCostContractId::new(17)
                || performance.denied_count() != 0
                || performance.retained_history_allocation_count() == 0
            {
                return Err(SignalError::invalid_input(format!(
                    "resource milestone C policy performance claim {} requires retention compaction availability evidence",
                    id.label()
                )));
            }
        }
        ResourceMilestoneCPolicyPerformanceClaimId::DiagnosticsBudgetDenialZeroCold => {
            if performance.boundary() != ResourceBoundaryKind::DiagnosticsExpansion
                || performance.cost_posture() != ResourceCostPosture::DeniedFallback
                || performance.cost_contract() != ResourceCostContractId::new(16)
                || performance.admitted_count() != 0
                || performance.denied_count() != 1
                || performance.diagnostics_allocation_count() != 0
            {
                return Err(SignalError::invalid_input(format!(
                    "resource milestone C policy performance claim {} requires zero-cold diagnostics denial evidence",
                    id.label()
                )));
            }
        }
        ResourceMilestoneCPolicyPerformanceClaimId::ReplayCompatibilityDescriptorBounded => {
            if performance.boundary() != ResourceBoundaryKind::PolicyCompatibility
                || performance.cost_posture() != ResourceCostPosture::Verified
                || performance.cost_contract() != ResourceCostContractId::new(18)
                || performance.input_width() < 3
                || performance.denied_count() < 2
            {
                return Err(SignalError::invalid_input(format!(
                    "resource milestone C policy performance claim {} requires descriptor-bounded replay compatibility evidence",
                    id.label()
                )));
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
struct ResourceMilestoneCPolicyCertificationEvidence {
    digest: String,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceMilestoneCPolicyCertificationEvidence {
    fn resource_policy_family(freeze_report: &ResourcePolicyRegistryFreezeReport) -> Self {
        let performance = ResourceBoundaryPerformanceEnvelope::policy_compatibility(
            freeze_report.descriptor_count() as u32,
            0,
        );
        Self {
            digest: resource_canonical_digest(&ResourceMilestoneCPolicyFamilyEvidenceBasis {
                descriptor_count: freeze_report.descriptor_count(),
                id_index_width: freeze_report.id_index_width(),
                kind_name_index_width: freeze_report.kind_name_index_width(),
                registry_digest: freeze_report.registry_digest().as_str(),
                performance,
            }),
            performance,
        }
    }

    fn retry_budget_and_backoff(report: &ResourceRetryScheduleReport) -> Result<Self, SignalError> {
        let performance = report.performance();
        if report.scheduled_retry().is_none() && report.denied_retry().is_none() {
            return Err(SignalError::invalid_input(
                "milestone C retry certification requires scheduled or denied retry evidence",
            ));
        }
        Ok(Self {
            digest: resource_canonical_digest(&ResourceMilestoneCRetryPolicyEvidenceBasis {
                scheduled_retry: report.scheduled_retry(),
                denied_retry: report.denied_retry(),
                performance,
            }),
            performance,
        })
    }

    fn timeout_deadline(
        timeout_report: &ResourceTimeoutReport,
        heartbeat_report: &ResourceTimeoutHeartbeatExtensionReport,
    ) -> Result<Self, SignalError> {
        let performance = timeout_report.performance();
        if timeout_report.timed_out_request().is_none() && timeout_report.denied_timeout().is_none()
        {
            return Err(SignalError::invalid_input(
                "milestone C timeout certification requires timeout evidence",
            ));
        }
        Ok(Self {
            digest: resource_canonical_digest(&ResourceMilestoneCTimeoutPolicyEvidenceBasis {
                timed_out_request: timeout_report.timed_out_request(),
                denied_timeout: timeout_report.denied_timeout(),
                heartbeat_extension: heartbeat_report.extended_heartbeat(),
                denied_heartbeat_extension: heartbeat_report.denied_extension(),
                timeout_performance: timeout_report.performance(),
                heartbeat_performance: heartbeat_report.performance(),
            }),
            performance,
        })
    }

    fn cancellation_supersession(
        cancellation_report: &ResourceCancellationReport,
        overlap_admission: &ResourceOverlappingGenerationAdmission,
        intent_coalescing: &ResourceIntentEquivalenceCoalescing,
    ) -> Result<Self, SignalError> {
        let performance = cancellation_report.performance();
        if cancellation_report.cancelled_request().is_none()
            && cancellation_report.denied_cancellation().is_none()
        {
            return Err(SignalError::invalid_input(
                "milestone C cancellation certification requires cancellation evidence",
            ));
        }
        Ok(Self {
            digest: resource_canonical_digest(
                &ResourceMilestoneCCancellationSupersessionEvidenceBasis {
                    cancelled_request: cancellation_report.cancelled_request(),
                    denied_cancellation: cancellation_report.denied_cancellation(),
                    dependent_propagation: cancellation_report.dependent_propagation(),
                    overlap_admission,
                    intent_coalescing,
                    performance,
                },
            ),
            performance,
        })
    }

    fn revalidation_freshness(report: &ResourceRevalidationReport) -> Result<Self, SignalError> {
        let performance = report.performance();
        if report.admitted_revalidation().is_none() && report.denied_revalidation().is_none() {
            return Err(SignalError::invalid_input(
                "milestone C revalidation certification requires revalidation evidence",
            ));
        }
        Ok(Self {
            digest: resource_canonical_digest(&ResourceMilestoneCRevalidationEvidenceBasis {
                admitted_revalidation: report.admitted_revalidation(),
                denied_revalidation: report.denied_revalidation(),
                lifecycle: report.lifecycle(),
                transition: report.transition(),
                performance,
            }),
            performance,
        })
    }

    fn observation_output_continuity(
        report: &ResourceObservationBatchReport,
    ) -> Result<Self, SignalError> {
        let performance = report.performance();
        if report.events().is_empty() {
            return Err(SignalError::invalid_input(
                "milestone C observation certification requires observation event evidence",
            ));
        }
        Ok(Self {
            digest: resource_canonical_digest(&ResourceMilestoneCObservationEvidenceBasis {
                events: report.events(),
                performance,
            }),
            performance,
        })
    }

    fn retention_replay(
        retention_report: &ResourceLifecycleRetentionCompactionReport,
        replay_availability: &ResourceReplayAvailabilityReport,
    ) -> Result<Self, SignalError> {
        let performance = replay_availability.performance();
        Ok(Self {
            digest: resource_canonical_digest(&ResourceMilestoneCRetentionReplayEvidenceBasis {
                retention_report,
                replay_class: replay_availability.class(),
                replay_denial_class: replay_availability.denial_class(),
                retained_history_unavailable_count: replay_availability
                    .retained_history_unavailable_count(),
                denied_completion_unavailable_count: replay_availability
                    .denied_completion_unavailable_count(),
                retry_lineage_unavailable_count: replay_availability
                    .retry_lineage_unavailable_count(),
                availability_digest: replay_availability.availability_digest(),
                performance,
            }),
            performance,
        })
    }
}

#[derive(Debug)]
struct ResourceCertificationEvidence {
    digest: String,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceCertificationEvidence {
    fn lifecycle_parity(
        baseline: &ResourceReplayReconstructionReport,
        equivalent: &ResourceReplayReconstructionReport,
        baseline_diagnostics: &ResourceDiagnosticsSummary,
        equivalent_diagnostics: &ResourceDiagnosticsSummary,
    ) -> Result<Self, SignalError> {
        if baseline.descriptor_digest() != equivalent.descriptor_digest()
            || baseline.lifecycle_digest() != equivalent.lifecycle_digest()
            || baseline.output_continuity_digest() != equivalent.output_continuity_digest()
            || baseline.denied_completion_digest() != equivalent.denied_completion_digest()
            || baseline.retry_lineage_digest() != equivalent.retry_lineage_digest()
            || baseline.in_flight_digest() != equivalent.in_flight_digest()
            || baseline.replay_digest() != equivalent.replay_digest()
            || baseline.retained_history_unavailable_count()
                != equivalent.retained_history_unavailable_count()
            || baseline.denied_completion_unavailable_count()
                != equivalent.denied_completion_unavailable_count()
            || baseline.retry_lineage_unavailable_count()
                != equivalent.retry_lineage_unavailable_count()
            || baseline_diagnostics.provenance_digest()
                != equivalent_diagnostics.provenance_digest()
        {
            return Err(invalid_resource_certification_evidence(
                ResourceCertificationFamily::AsyncResourceLifecycleParity,
                "requires equivalent replay and diagnostics truth across canonical async executions",
            ));
        }
        Ok(Self {
            digest: resource_canonical_digest(&ResourceLifecycleParityEvidenceBasis {
                descriptor_digest: baseline.descriptor_digest(),
                lifecycle_digest: baseline.lifecycle_digest(),
                output_continuity_digest: baseline.output_continuity_digest(),
                denied_completion_digest: baseline.denied_completion_digest(),
                retry_lineage_digest: baseline.retry_lineage_digest(),
                in_flight_digest: baseline.in_flight_digest(),
                replay_digest: baseline.replay_digest(),
                retained_history_unavailable_count: baseline.retained_history_unavailable_count(),
                denied_completion_unavailable_count: baseline.denied_completion_unavailable_count(),
                retry_lineage_unavailable_count: baseline.retry_lineage_unavailable_count(),
                diagnostics_provenance_digest: baseline_diagnostics.provenance_digest(),
                performance: baseline.performance(),
            }),
            performance: baseline.performance(),
        })
    }

    fn out_of_order_supersession(
        admission: ResourceRequestAdmissionReport,
    ) -> Result<Self, SignalError> {
        let Some(supersession) = admission.supersession_record() else {
            return Err(invalid_resource_certification_evidence(
                ResourceCertificationFamily::OutOfOrderCompletionSupersession,
                "requires request admission with supersession evidence",
            ));
        };
        let performance = admission.performance();
        Ok(Self {
            digest: resource_canonical_digest(&ResourceSupersessionEvidenceBasis {
                supersession,
                superseded_request: admission.superseded_request(),
                superseded_transition: admission.superseded_transition(),
                performance,
            }),
            performance,
        })
    }

    fn rollback_observation(
        rollback: ResourceCompletionRollbackReport,
        observation: ResourceObservationBatchReport,
        control_observation: ResourceObservationBatchReport,
        pre_rollback: &ResourceReplayReconstructionReport,
        post_rollback: &ResourceReplayReconstructionReport,
        diagnostics: &ResourceDiagnosticsSummary,
    ) -> Result<Self, SignalError> {
        if observation.events().is_empty() {
            return Err(invalid_resource_certification_evidence(
                ResourceCertificationFamily::AsyncRollbackObservationEquivalence,
                "requires rollback-suppressed observation evidence",
            ));
        }
        if !observation
            .events()
            .iter()
            .all(|event| event.outcome() == ObservationBoundaryOutcome::RollbackSuppressed)
        {
            return Err(invalid_resource_certification_evidence(
                ResourceCertificationFamily::AsyncRollbackObservationEquivalence,
                "requires only rollback-suppressed observation events",
            ));
        }
        if control_observation.events().is_empty() {
            return Err(invalid_resource_certification_evidence(
                ResourceCertificationFamily::AsyncRollbackObservationEquivalence,
                "requires a delivered control observation packet",
            ));
        }
        if !control_observation
            .events()
            .iter()
            .all(|event| event.outcome() == ObservationBoundaryOutcome::Delivered)
        {
            return Err(invalid_resource_certification_evidence(
                ResourceCertificationFamily::AsyncRollbackObservationEquivalence,
                "requires only delivered events on the no-failure control path",
            ));
        }
        if observation.events().len() != control_observation.events().len()
            || observation
                .events()
                .iter()
                .zip(control_observation.events())
                .any(|(suppressed, delivered)| {
                    suppressed.observer_id() != delivered.observer_id()
                        || suppressed.handle_id() != delivered.handle_id()
                        || suppressed.policy() != delivered.policy()
                        || suppressed.touched() != delivered.touched()
                        || suppressed.recomputed() != delivered.recomputed()
                        || suppressed.meaningful_change() != delivered.meaningful_change()
                        || suppressed.trigger_matched() != delivered.trigger_matched()
                        || suppressed
                            .matched_resource_nodes()
                            .iter()
                            .map(|node| node.node())
                            .collect::<Vec<_>>()
                            != delivered
                                .matched_resource_nodes()
                                .iter()
                                .map(|node| node.node())
                                .collect::<Vec<_>>()
                })
        {
            return Err(invalid_resource_certification_evidence(
                ResourceCertificationFamily::AsyncRollbackObservationEquivalence,
                "requires rollback-suppressed observation to match the no-failure control delivery exactly in packet shape apart from boundary outcome",
            ));
        }
        if pre_rollback.replay_digest() != post_rollback.replay_digest()
            || pre_rollback.lifecycle_digest() != post_rollback.lifecycle_digest()
            || pre_rollback.descriptor_digest() != post_rollback.descriptor_digest()
            || pre_rollback.output_continuity_digest() != post_rollback.output_continuity_digest()
            || pre_rollback.in_flight_digest() != post_rollback.in_flight_digest()
            || pre_rollback.denied_completion_digest() != post_rollback.denied_completion_digest()
            || pre_rollback.retry_lineage_digest() != post_rollback.retry_lineage_digest()
        {
            return Err(invalid_resource_certification_evidence(
                ResourceCertificationFamily::AsyncRollbackObservationEquivalence,
                "requires rollback lane to preserve canonical replay truth",
            ));
        }
        let performance = rollback.performance();
        let rolled_back = rollback.rolled_back_completion();
        Ok(Self {
            digest: resource_canonical_digest(&ResourceRollbackObservationEvidenceBasis {
                subject: rolled_back.subject(),
                observation,
                control_observation,
                pre_rollback_descriptor_digest: pre_rollback.descriptor_digest(),
                pre_rollback_lifecycle_digest: pre_rollback.lifecycle_digest(),
                pre_rollback_output_continuity_digest: pre_rollback.output_continuity_digest(),
                pre_rollback_denied_completion_digest: pre_rollback.denied_completion_digest(),
                pre_rollback_retry_lineage_digest: pre_rollback.retry_lineage_digest(),
                pre_rollback_in_flight_digest: pre_rollback.in_flight_digest(),
                pre_rollback_replay_digest: pre_rollback.replay_digest(),
                post_rollback_descriptor_digest: post_rollback.descriptor_digest(),
                post_rollback_lifecycle_digest: post_rollback.lifecycle_digest(),
                post_rollback_output_continuity_digest: post_rollback.output_continuity_digest(),
                post_rollback_denied_completion_digest: post_rollback.denied_completion_digest(),
                post_rollback_retry_lineage_digest: post_rollback.retry_lineage_digest(),
                post_rollback_in_flight_digest: post_rollback.in_flight_digest(),
                post_rollback_replay_digest: post_rollback.replay_digest(),
                diagnostics_provenance_digest: diagnostics.provenance_digest(),
                performance,
            }),
            performance,
        })
    }

    fn branch_restore_replay(
        restore: ResourceBranchRestoreReport,
        replay: &ResourceReplayReconstructionReport,
    ) -> Self {
        Self {
            digest: resource_canonical_digest(&ResourceBranchRestoreReplayEvidenceBasis {
                restore,
                descriptor_digest: replay.descriptor_digest(),
                lifecycle_digest: replay.lifecycle_digest(),
                denied_completion_digest: replay.denied_completion_digest(),
                in_flight_digest: replay.in_flight_digest(),
                replay_digest: replay.replay_digest(),
                replay_performance: replay.performance(),
            }),
            performance: restore.performance(),
        }
    }

    fn inflight_boundedness(
        summary: ResourceRuntimeSummary,
        replay: &ResourceReplayReconstructionReport,
        telemetry: ResourceTelemetry,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Result<Self, SignalError> {
        match performance.boundary() {
            ResourceBoundaryKind::RequestAdmission
            | ResourceBoundaryKind::Cancellation
            | ResourceBoundaryKind::TimeoutAdmission
            | ResourceBoundaryKind::RetryAdmission
            | ResourceBoundaryKind::RevalidationAdmission
            | ResourceBoundaryKind::CompletionAdmission
            | ResourceBoundaryKind::CompletionBatchAdmission
            | ResourceBoundaryKind::BranchRestore
            | ResourceBoundaryKind::ReplayReconstruction => {}
            _ => {
                return Err(invalid_resource_certification_evidence(
                    ResourceCertificationFamily::AsyncInflightBoundedness,
                    "requires an in-flight or replay resource boundary performance envelope",
                ));
            }
        }
        if summary.in_flight_request_count() != replay.in_flight_width() as u64 {
            return Err(invalid_resource_certification_evidence(
                ResourceCertificationFamily::AsyncInflightBoundedness,
                "requires runtime summary and replay reconstruction to agree on in-flight width",
            ));
        }
        if telemetry.resource_retry_admission_count == 0
            || telemetry.resource_branch_restore_count == 0
            || telemetry.resource_superseded_completion_denial_count == 0
            || telemetry.resource_duplicate_completion_denial_count == 0
            || telemetry.resource_contradictory_completion_denial_count == 0
            || telemetry.resource_unknown_request_completion_denial_count == 0
        {
            return Err(invalid_resource_certification_evidence(
                ResourceCertificationFamily::AsyncInflightBoundedness,
                "requires hostile async pressure evidence for retry, restore, supersession, duplicate, contradictory, and unknown completion lanes",
            ));
        }
        Ok(Self {
            digest: resource_canonical_digest(&ResourceInflightBoundednessEvidenceBasis {
                summary,
                replay_in_flight_width: replay.in_flight_width(),
                replay_digest: replay.replay_digest().to_string(),
                retry_admission_count: telemetry.resource_retry_admission_count,
                retry_duplicate_denial_count: telemetry
                    .resource_retry_already_scheduled_denial_count,
                branch_restore_count: telemetry.resource_branch_restore_count,
                branch_restore_broad_rebuild_denial_count: telemetry
                    .resource_branch_restore_broad_rebuild_denial_count,
                superseded_completion_denial_count: telemetry
                    .resource_superseded_completion_denial_count,
                duplicate_completion_denial_count: telemetry
                    .resource_duplicate_completion_denial_count,
                contradictory_completion_denial_count: telemetry
                    .resource_contradictory_completion_denial_count,
                unknown_request_completion_denial_count: telemetry
                    .resource_unknown_request_completion_denial_count,
                broad_scan_denial_count: telemetry.resource_broad_scan_denial_count,
                hot_in_flight_lookup_count: telemetry.resource_hot_in_flight_lookup_count,
                performance,
            }),
            performance,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResourceCertificationFailure {
    MissingRequiredFamily {
        family: ResourceCertificationFamily,
    },
    DuplicateFamily {
        family: ResourceCertificationFamily,
        count: u32,
    },
    FailedFamily {
        family: ResourceCertificationFamily,
    },
    EmptyEvidenceDigest {
        family: ResourceCertificationFamily,
    },
}

impl ResourceCertificationFailure {
    pub fn family(&self) -> ResourceCertificationFamily {
        match self {
            Self::MissingRequiredFamily { family }
            | Self::DuplicateFamily { family, .. }
            | Self::FailedFamily { family }
            | Self::EmptyEvidenceDigest { family } => *family,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceCertificationSummary {
    required_family_count: u32,
    provided_record_count: u32,
    passed_family_count: u32,
    failed_family_count: u32,
    missing_family_count: u32,
    duplicate_family_count: u32,
}

impl ResourceCertificationSummary {
    pub fn required_family_count(self) -> u32 {
        self.required_family_count
    }

    pub fn provided_record_count(self) -> u32 {
        self.provided_record_count
    }

    pub fn passed_family_count(self) -> u32 {
        self.passed_family_count
    }

    pub fn failed_family_count(self) -> u32 {
        self.failed_family_count
    }

    pub fn missing_family_count(self) -> u32 {
        self.missing_family_count
    }

    pub fn duplicate_family_count(self) -> u32 {
        self.duplicate_family_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceCertificationBundle {
    schema_version: String,
    records: Vec<ResourceCertificationRecord>,
    summary: ResourceCertificationSummary,
    bundle_digest: String,
    passed: bool,
    failures: Vec<ResourceCertificationFailure>,
}

impl ResourceCertificationBundle {
    pub fn schema_version(&self) -> &str {
        &self.schema_version
    }

    pub fn records(&self) -> &[ResourceCertificationRecord] {
        &self.records
    }

    pub fn summary(&self) -> ResourceCertificationSummary {
        self.summary
    }

    pub fn bundle_digest(&self) -> &str {
        &self.bundle_digest
    }

    pub fn passed(&self) -> bool {
        self.passed
    }

    pub fn failures(&self) -> &[ResourceCertificationFailure] {
        &self.failures
    }

    pub fn ensure_passed(&self) -> Result<(), SignalError> {
        if self.passed {
            return Ok(());
        }
        Err(SignalError::invalid_input(format!(
            "resource certification bundle failed with {} failure(s)",
            self.failures.len()
        )))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResourceCertificationBundleMismatchClass {
    SchemaVersionMismatch,
    BundleDigestMismatch,
    PassStatusMismatch,
    SummaryMismatch,
    FailureSetMismatch,
    RecordSetMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceCertificationBundleParityReport {
    proof_schema_version: String,
    expected: ResourceCertificationBundle,
    replayed: ResourceCertificationBundle,
    parity: bool,
    mismatch_classes: Vec<ResourceCertificationBundleMismatchClass>,
}

impl ResourceCertificationBundleParityReport {
    pub fn proof_schema_version(&self) -> &str {
        &self.proof_schema_version
    }

    pub fn expected(&self) -> &ResourceCertificationBundle {
        &self.expected
    }

    pub fn replayed(&self) -> &ResourceCertificationBundle {
        &self.replayed
    }

    pub fn parity(&self) -> bool {
        self.parity
    }

    pub fn mismatch_classes(&self) -> &[ResourceCertificationBundleMismatchClass] {
        &self.mismatch_classes
    }
}

pub fn resource_certification_builder() -> ResourceCertificationBuilder {
    ResourceCertificationBuilder::new()
}

pub fn resource_certification_bundle(
    records: impl IntoIterator<Item = ResourceCertificationRecord>,
) -> ResourceCertificationBundle {
    let mut records = records.into_iter().collect::<Vec<_>>();
    records.sort_by_key(|record| record.family);

    let mut by_family: BTreeMap<ResourceCertificationFamily, Vec<&ResourceCertificationRecord>> =
        BTreeMap::new();
    for record in &records {
        by_family.entry(record.family).or_default().push(record);
    }

    let mut failures = Vec::new();
    for family in REQUIRED_RESOURCE_CERTIFICATION_FAMILIES {
        match by_family.get(&family) {
            None => failures.push(ResourceCertificationFailure::MissingRequiredFamily { family }),
            Some(records_for_family) if records_for_family.len() > 1 => {
                failures.push(ResourceCertificationFailure::DuplicateFamily {
                    family,
                    count: records_for_family.len() as u32,
                });
            }
            Some(_) => {}
        }
    }

    for record in &records {
        if record.evidence_digest.is_empty() {
            failures.push(ResourceCertificationFailure::EmptyEvidenceDigest {
                family: record.family,
            });
        }
        if !record.passed {
            failures.push(ResourceCertificationFailure::FailedFamily {
                family: record.family,
            });
        }
    }

    let failed_families = failures
        .iter()
        .map(ResourceCertificationFailure::family)
        .collect::<BTreeSet<_>>();
    let passed_family_count = REQUIRED_RESOURCE_CERTIFICATION_FAMILIES
        .iter()
        .filter(|family| {
            by_family
                .get(family)
                .is_some_and(|records_for_family| records_for_family.len() == 1)
                && !failed_families.contains(family)
        })
        .count() as u32;
    let missing_family_count = REQUIRED_RESOURCE_CERTIFICATION_FAMILIES
        .iter()
        .filter(|family| !by_family.contains_key(family))
        .count() as u32;
    let duplicate_family_count = by_family
        .values()
        .filter(|records_for_family| records_for_family.len() > 1)
        .count() as u32;
    let failed_family_count = failed_families.len() as u32;
    let summary = ResourceCertificationSummary {
        required_family_count: REQUIRED_RESOURCE_CERTIFICATION_FAMILIES.len() as u32,
        provided_record_count: records.len() as u32,
        passed_family_count,
        failed_family_count,
        missing_family_count,
        duplicate_family_count,
    };
    let bundle_digest = resource_canonical_digest(&ResourceCertificationBundleDigestBasis {
        schema_version: RESOURCE_CERTIFICATION_BUNDLE_SCHEMA_VERSION,
        records: &records,
    });
    let passed = failures.is_empty();
    ResourceCertificationBundle {
        schema_version: RESOURCE_CERTIFICATION_BUNDLE_SCHEMA_VERSION.to_owned(),
        records,
        summary,
        bundle_digest,
        passed,
        failures,
    }
}

pub fn resource_certification_bundle_parity_report(
    expected: &ResourceCertificationBundle,
    replayed: &ResourceCertificationBundle,
) -> ResourceCertificationBundleParityReport {
    let mut mismatch_classes = Vec::new();
    if expected.schema_version != replayed.schema_version {
        mismatch_classes.push(ResourceCertificationBundleMismatchClass::SchemaVersionMismatch);
    }
    if expected.bundle_digest != replayed.bundle_digest {
        mismatch_classes.push(ResourceCertificationBundleMismatchClass::BundleDigestMismatch);
    }
    if expected.passed != replayed.passed {
        mismatch_classes.push(ResourceCertificationBundleMismatchClass::PassStatusMismatch);
    }
    if expected.summary != replayed.summary {
        mismatch_classes.push(ResourceCertificationBundleMismatchClass::SummaryMismatch);
    }
    if expected.failures != replayed.failures {
        mismatch_classes.push(ResourceCertificationBundleMismatchClass::FailureSetMismatch);
    }
    if expected.records != replayed.records {
        mismatch_classes.push(ResourceCertificationBundleMismatchClass::RecordSetMismatch);
    }
    ResourceCertificationBundleParityReport {
        proof_schema_version: RESOURCE_CERTIFICATION_BUNDLE_PARITY_SCHEMA_VERSION.to_owned(),
        expected: expected.clone(),
        replayed: replayed.clone(),
        parity: mismatch_classes.is_empty(),
        mismatch_classes,
    }
}

pub fn resource_milestone_c_policy_certification_builder(
) -> ResourceMilestoneCPolicyCertificationBuilder {
    ResourceMilestoneCPolicyCertificationBuilder::new()
}

pub fn resource_milestone_c_policy_certification_bundle(
    records: impl IntoIterator<Item = ResourceMilestoneCPolicyCertificationRecord>,
) -> ResourceMilestoneCPolicyCertificationBundle {
    let mut records = records.into_iter().collect::<Vec<_>>();
    records.sort_by_key(|record| record.family);

    let mut by_family: BTreeMap<
        ResourceMilestoneCPolicyCertificationFamily,
        Vec<&ResourceMilestoneCPolicyCertificationRecord>,
    > = BTreeMap::new();
    for record in &records {
        by_family.entry(record.family).or_default().push(record);
    }

    let certified_family_count = REQUIRED_RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_FAMILIES
        .iter()
        .filter(|family| {
            by_family.get(family).is_some_and(|records_for_family| {
                records_for_family.len() == 1 && records_for_family[0].passed()
            })
        })
        .count() as u32;
    let missing_family_count = REQUIRED_RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_FAMILIES
        .iter()
        .filter(|family| !by_family.contains_key(family))
        .count() as u32;
    let duplicate_family_count = by_family
        .values()
        .filter(|records_for_family| records_for_family.len() > 1)
        .count() as u32;
    let failed_family_count = records.iter().filter(|record| !record.passed()).count() as u32
        + missing_family_count
        + duplicate_family_count;
    let summary = ResourceMilestoneCPolicyCertificationSummary {
        required_family_count: REQUIRED_RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_FAMILIES.len()
            as u32,
        provided_record_count: records.len() as u32,
        certified_family_count,
        failed_family_count,
        missing_family_count,
        duplicate_family_count,
    };
    let bundle_digest =
        resource_canonical_digest(&ResourceMilestoneCPolicyCertificationBundleDigestBasis {
            schema_version: RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_BUNDLE_SCHEMA_VERSION,
            records: &records,
        });
    ResourceMilestoneCPolicyCertificationBundle {
        schema_version: RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_BUNDLE_SCHEMA_VERSION.to_owned(),
        records,
        summary,
        bundle_digest,
        passed: failed_family_count == 0,
    }
}

pub fn resource_milestone_c_policy_scenario_matrix(
    bundle: &ResourceMilestoneCPolicyCertificationBundle,
    freeze_report: &ResourcePolicyRegistryFreezeReport,
    retry_schedule_report: &ResourceRetryScheduleReport,
    timeout_heartbeat_report: &ResourceTimeoutHeartbeatExtensionReport,
    retention_compaction_report: &ResourceLifecycleRetentionCompactionReport,
    diagnostics_denial: &ResourceDiagnosticsExpansionDenial,
    compatible_restore: &ResourcePolicyRestoreCompatibilityProof,
    incompatible_restore: &DeniedResourcePolicyRestoreCompatibility,
    missing_restore: &DeniedResourcePolicyRestoreCompatibility,
) -> Result<ResourceMilestoneCPolicyScenarioMatrix, SignalError> {
    bundle.ensure_passed()?;

    let mut rows = vec![
        ResourceMilestoneCPolicyScenarioRow::from_registry_freeze(freeze_report)?,
        ResourceMilestoneCPolicyScenarioRow::from_retry_denial(retry_schedule_report)?,
        ResourceMilestoneCPolicyScenarioRow::from_timeout_heartbeat_denial(
            timeout_heartbeat_report,
        )?,
        ResourceMilestoneCPolicyScenarioRow::from_retention_compaction(
            retention_compaction_report,
        )?,
        ResourceMilestoneCPolicyScenarioRow::from_diagnostics_denial(diagnostics_denial)?,
        ResourceMilestoneCPolicyScenarioRow::from_restore_proof(compatible_restore)?,
        ResourceMilestoneCPolicyScenarioRow::from_restore_denial(
            ResourceMilestoneCPolicyScenarioId::IncompatibleDescriptorRestoreDenied,
            incompatible_restore,
        )?,
        ResourceMilestoneCPolicyScenarioRow::from_restore_denial(
            ResourceMilestoneCPolicyScenarioId::MissingDescriptorRestoreDenied,
            missing_restore,
        )?,
    ];
    rows.sort_by_key(|row| row.id);

    let mut row_counts: BTreeMap<ResourceMilestoneCPolicyScenarioId, u32> = BTreeMap::new();
    for row in &rows {
        *row_counts.entry(row.id()).or_default() += 1;
    }
    for scenario in REQUIRED_RESOURCE_MILESTONE_C_POLICY_SCENARIOS {
        let count = row_counts.get(&scenario).copied().unwrap_or(0);
        if count != 1 {
            return Err(SignalError::invalid_input(format!(
                "resource milestone C policy scenario matrix requires exactly one row for {}",
                scenario.label()
            )));
        }
    }

    let summary = ResourceMilestoneCPolicyScenarioMatrixSummary {
        required_scenario_count: REQUIRED_RESOURCE_MILESTONE_C_POLICY_SCENARIOS.len() as u32,
        certified_scenario_count: rows.iter().filter(|row| row.passed()).count() as u32,
        failed_scenario_count: rows.iter().filter(|row| !row.passed()).count() as u32,
        bundle_digest: bundle.bundle_digest().to_owned(),
    };
    let matrix_digest =
        resource_canonical_digest(&ResourceMilestoneCPolicyScenarioMatrixDigestBasis {
            schema_version: RESOURCE_MILESTONE_C_POLICY_SCENARIO_MATRIX_SCHEMA_VERSION,
            required_scenarios: &REQUIRED_RESOURCE_MILESTONE_C_POLICY_SCENARIOS,
            bundle_digest: bundle.bundle_digest(),
            summary: &summary,
            rows: &rows,
        });
    Ok(ResourceMilestoneCPolicyScenarioMatrix {
        schema_version: RESOURCE_MILESTONE_C_POLICY_SCENARIO_MATRIX_SCHEMA_VERSION.to_owned(),
        rows,
        summary,
        matrix_digest,
        passed: true,
    })
}

pub fn resource_milestone_c_policy_performance_closeout(
    scenario_matrix: &ResourceMilestoneCPolicyScenarioMatrix,
) -> Result<ResourceMilestoneCPolicyPerformanceCloseout, SignalError> {
    if !scenario_matrix.passed() {
        return Err(SignalError::invalid_input(
            "resource milestone C policy performance closeout requires a passing scenario matrix",
        ));
    }
    let rows = vec![
        ResourceMilestoneCPolicyPerformanceCloseoutRow::scenario_row(
            ResourceMilestoneCPolicyPerformanceClaimId::RegistryFreezeOrderBounded,
            ResourceMilestoneCPolicyScenarioId::RegistryOrderCanonicalization,
            scenario_matrix,
        )?,
        ResourceMilestoneCPolicyPerformanceCloseoutRow::scenario_row(
            ResourceMilestoneCPolicyPerformanceClaimId::RetryBudgetDenialZeroWake,
            ResourceMilestoneCPolicyScenarioId::RetryBudgetExhaustionRejected,
            scenario_matrix,
        )?,
        ResourceMilestoneCPolicyPerformanceCloseoutRow::scenario_row(
            ResourceMilestoneCPolicyPerformanceClaimId::RetentionCompactionAvailabilityBounded,
            ResourceMilestoneCPolicyScenarioId::RetentionCompactionReportsUnavailableHistory,
            scenario_matrix,
        )?,
        ResourceMilestoneCPolicyPerformanceCloseoutRow::scenario_row(
            ResourceMilestoneCPolicyPerformanceClaimId::DiagnosticsBudgetDenialZeroCold,
            ResourceMilestoneCPolicyScenarioId::DiagnosticsExpansionBudgetDeniedZeroCold,
            scenario_matrix,
        )?,
        ResourceMilestoneCPolicyPerformanceCloseoutRow::replay_descriptor_bound(scenario_matrix)?,
    ];
    let row_ids = rows.iter().map(|row| row.id()).collect::<Vec<_>>();
    if row_ids != REQUIRED_RESOURCE_MILESTONE_C_POLICY_PERFORMANCE_CLAIMS {
        return Err(SignalError::invalid_input(
            "resource milestone C policy performance closeout rows do not match required claims",
        ));
    }
    let summary = ResourceMilestoneCPolicyPerformanceCloseoutSummary {
        required_claim_count: REQUIRED_RESOURCE_MILESTONE_C_POLICY_PERFORMANCE_CLAIMS.len() as u32,
        certified_claim_count: rows.len() as u32,
        failed_claim_count: rows.iter().filter(|row| !row.passed()).count() as u32,
        scenario_matrix_digest: scenario_matrix.matrix_digest().to_owned(),
    };
    if summary.certified_claim_count != summary.required_claim_count
        || summary.failed_claim_count != 0
    {
        return Err(SignalError::invalid_input(
            "resource milestone C policy performance closeout did not cover every required claim",
        ));
    }
    let closeout_digest =
        resource_canonical_digest(&ResourceMilestoneCPolicyPerformanceCloseoutDigestBasis {
            schema_version: RESOURCE_MILESTONE_C_POLICY_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION,
            required_claims: &REQUIRED_RESOURCE_MILESTONE_C_POLICY_PERFORMANCE_CLAIMS,
            scenario_matrix_digest: scenario_matrix.matrix_digest(),
            summary: &summary,
            rows: &rows,
        });
    Ok(ResourceMilestoneCPolicyPerformanceCloseout {
        schema_version: RESOURCE_MILESTONE_C_POLICY_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION.to_owned(),
        scenario_matrix_digest: scenario_matrix.matrix_digest().to_owned(),
        rows,
        summary,
        closeout_digest,
        passed: true,
    })
}

pub fn resource_milestone_c_certification_run(
    bundle: ResourceMilestoneCPolicyCertificationBundle,
    scenario_matrix: ResourceMilestoneCPolicyScenarioMatrix,
    performance_closeout: ResourceMilestoneCPolicyPerformanceCloseout,
) -> Result<ResourceMilestoneCCertificationRun, SignalError> {
    bundle.ensure_passed()?;
    if !scenario_matrix.passed() {
        return Err(SignalError::invalid_input(
            "resource milestone C certification run requires a passing scenario matrix",
        ));
    }
    if !performance_closeout.passed() {
        return Err(SignalError::invalid_input(
            "resource milestone C certification run requires a passing performance closeout",
        ));
    }
    if scenario_matrix.summary().bundle_digest() != bundle.bundle_digest() {
        return Err(SignalError::invalid_input(
            "resource milestone C certification run requires scenario matrix evidence from the same bundle",
        ));
    }
    if performance_closeout.scenario_matrix_digest() != scenario_matrix.matrix_digest() {
        return Err(SignalError::invalid_input(
            "resource milestone C certification run requires performance closeout evidence from the same scenario matrix",
        ));
    }
    if scenario_matrix.rows().len() != REQUIRED_RESOURCE_MILESTONE_C_POLICY_SCENARIOS.len()
        || scenario_matrix.summary().required_scenario_count()
            != REQUIRED_RESOURCE_MILESTONE_C_POLICY_SCENARIOS.len() as u32
        || scenario_matrix.summary().certified_scenario_count()
            != REQUIRED_RESOURCE_MILESTONE_C_POLICY_SCENARIOS.len() as u32
        || scenario_matrix.summary().failed_scenario_count() != 0
    {
        return Err(SignalError::invalid_input(
            "resource milestone C certification run requires one passing row for every required scenario",
        ));
    }
    if performance_closeout.rows().len()
        != REQUIRED_RESOURCE_MILESTONE_C_POLICY_PERFORMANCE_CLAIMS.len()
        || performance_closeout.summary().required_claim_count()
            != REQUIRED_RESOURCE_MILESTONE_C_POLICY_PERFORMANCE_CLAIMS.len() as u32
        || performance_closeout.summary().certified_claim_count()
            != REQUIRED_RESOURCE_MILESTONE_C_POLICY_PERFORMANCE_CLAIMS.len() as u32
        || performance_closeout.summary().failed_claim_count() != 0
    {
        return Err(SignalError::invalid_input(
            "resource milestone C certification run requires one passing row for every required performance claim",
        ));
    }
    let bundle_summary = bundle.summary();
    let required_family_count =
        REQUIRED_RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_FAMILIES.len() as u32;
    if bundle.records().len() != REQUIRED_RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_FAMILIES.len()
        || bundle_summary.required_family_count() != required_family_count
        || bundle_summary.certified_family_count() != required_family_count
        || bundle_summary.failed_family_count() != 0
        || bundle_summary.missing_family_count() != 0
        || bundle_summary.duplicate_family_count() != 0
    {
        return Err(SignalError::invalid_input(
            "resource milestone C certification run requires one passing record for every required family",
        ));
    }

    let summary = ResourceMilestoneCCertificationRunSummary {
        required_family_count,
        certified_family_count: bundle_summary.certified_family_count(),
        failed_family_count: bundle_summary.failed_family_count(),
        bundle_digest: bundle.bundle_digest().to_owned(),
        required_scenario_count: scenario_matrix.summary().required_scenario_count(),
        certified_scenario_count: scenario_matrix.summary().certified_scenario_count(),
        scenario_matrix_digest: scenario_matrix.matrix_digest().to_owned(),
        required_performance_claim_count: performance_closeout.summary().required_claim_count(),
        certified_performance_claim_count: performance_closeout.summary().certified_claim_count(),
        performance_closeout_digest: performance_closeout.closeout_digest().to_owned(),
    };
    let run_digest = resource_canonical_digest(&ResourceMilestoneCCertificationRunDigestBasis {
        schema_version: RESOURCE_MILESTONE_C_CERTIFICATION_RUN_SCHEMA_VERSION,
        required_families: &REQUIRED_RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_FAMILIES,
        required_scenarios: &REQUIRED_RESOURCE_MILESTONE_C_POLICY_SCENARIOS,
        required_performance_claims: &REQUIRED_RESOURCE_MILESTONE_C_POLICY_PERFORMANCE_CLAIMS,
        summary: &summary,
        bundle_digest: bundle.bundle_digest(),
        scenario_matrix_digest: scenario_matrix.matrix_digest(),
        performance_closeout_digest: performance_closeout.closeout_digest(),
        record_digests: bundle
            .records()
            .iter()
            .map(|record| (record.family(), record.evidence_digest()))
            .collect::<Vec<_>>(),
        scenario_digests: scenario_matrix
            .rows()
            .iter()
            .map(|row| (row.id(), row.evidence_digest()))
            .collect::<Vec<_>>(),
        performance_claim_digests: performance_closeout
            .rows()
            .iter()
            .map(|row| (row.id(), row.evidence_digest()))
            .collect::<Vec<_>>(),
    });

    Ok(ResourceMilestoneCCertificationRun {
        schema_version: RESOURCE_MILESTONE_C_CERTIFICATION_RUN_SCHEMA_VERSION.to_owned(),
        bundle,
        scenario_matrix,
        performance_closeout,
        summary,
        run_digest,
        passed: true,
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneBCertificationRunSummary {
    required_family_count: u32,
    certified_family_count: u32,
    failed_family_count: u32,
    bundle_digest: String,
    required_scenario_count: u32,
    certified_scenario_count: u32,
    scenario_matrix_digest: String,
    required_performance_claim_count: u32,
    certified_performance_claim_count: u32,
    performance_closeout_digest: String,
}

impl ResourceMilestoneBCertificationRunSummary {
    pub fn required_family_count(&self) -> u32 {
        self.required_family_count
    }

    pub fn certified_family_count(&self) -> u32 {
        self.certified_family_count
    }

    pub fn failed_family_count(&self) -> u32 {
        self.failed_family_count
    }

    pub fn bundle_digest(&self) -> &str {
        &self.bundle_digest
    }

    pub fn required_scenario_count(&self) -> u32 {
        self.required_scenario_count
    }

    pub fn certified_scenario_count(&self) -> u32 {
        self.certified_scenario_count
    }

    pub fn scenario_matrix_digest(&self) -> &str {
        &self.scenario_matrix_digest
    }

    pub fn required_performance_claim_count(&self) -> u32 {
        self.required_performance_claim_count
    }

    pub fn certified_performance_claim_count(&self) -> u32 {
        self.certified_performance_claim_count
    }

    pub fn performance_closeout_digest(&self) -> &str {
        &self.performance_closeout_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneBHostileScenarioEvidenceRow {
    id: ResourceMilestoneBScenarioId,
    expected_denial_class: CompletionDenialClass,
    denied_completion: DeniedResourceCompletion,
    performance: ResourceBoundaryPerformanceEnvelope,
    evidence_digest: String,
}

impl ResourceMilestoneBHostileScenarioEvidenceRow {
    fn from_completion_denial_report(
        id: ResourceMilestoneBScenarioId,
        report: ResourceCompletionAdmissionReport,
    ) -> Result<Self, SignalError> {
        let Some(expected_denial_class) = id.completion_denial_class() else {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} is not a hostile completion denial scenario",
                id.label()
            )));
        };
        if report.admitted_completion().is_some() {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} requires denied completion evidence",
                id.label()
            )));
        }
        let Some(denied_completion) = report.denied_completion() else {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} is missing denied completion evidence",
                id.label()
            )));
        };
        if denied_completion.class() != expected_denial_class {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} requires {expected_denial_class:?} denial evidence, got {:?}",
                id.label(),
                denied_completion.class()
            )));
        }
        let performance = report.performance();
        if performance.boundary() != ResourceBoundaryKind::CompletionAdmission {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} requires completion admission performance evidence",
                id.label()
            )));
        }
        let evidence_digest =
            resource_canonical_digest(&ResourceMilestoneBHostileScenarioEvidenceRowDigestBasis {
                id,
                expected_denial_class,
                denied_completion,
                performance,
            });
        Ok(Self {
            id,
            expected_denial_class,
            denied_completion,
            performance,
            evidence_digest,
        })
    }

    fn from_completion_batch_denial_report(
        id: ResourceMilestoneBScenarioId,
        report: &ResourceCompletionBatchAdmissionReport,
    ) -> Result<Self, SignalError> {
        let Some(expected_denial_class) = id.completion_denial_class() else {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} is not a hostile completion denial scenario",
                id.label()
            )));
        };
        let mut matches = report
            .denied_completions()
            .iter()
            .copied()
            .filter(|denied| denied.class() == expected_denial_class);
        let Some(denied_completion) = matches.next() else {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} is missing {:?} denial evidence in completion batch",
                id.label(),
                expected_denial_class
            )));
        };
        if matches.next().is_some() {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} requires exactly one {:?} denial entry in completion batch evidence",
                id.label(),
                expected_denial_class
            )));
        }
        let performance = report.performance();
        if performance.boundary() != ResourceBoundaryKind::CompletionBatchAdmission {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} requires completion batch admission performance evidence",
                id.label()
            )));
        }
        if performance.input_width() != 4
            || performance.admitted_count() != 1
            || performance.denied_count() != 3
            || performance.lifecycle_transition_count() != 1
            || performance.operational_allocation_count() != 3
            || performance.retained_history_allocation_count() != 0
            || performance.diagnostics_allocation_count() != 4
            || performance.facade_report_allocation_count() != 1
            || performance.density_strategy() != ResourceDensityStrategy::BurstySortedDeduplicated
        {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} requires hostile mixed batch denial evidence rather than an arbitrary completion batch",
                id.label()
            )));
        }
        let evidence_digest =
            resource_canonical_digest(&ResourceMilestoneBHostileScenarioEvidenceRowDigestBasis {
                id,
                expected_denial_class,
                denied_completion,
                performance,
            });
        Ok(Self {
            id,
            expected_denial_class,
            denied_completion,
            performance,
            evidence_digest,
        })
    }

    pub fn id(&self) -> ResourceMilestoneBScenarioId {
        self.id
    }

    pub fn expected_denial_class(&self) -> CompletionDenialClass {
        self.expected_denial_class
    }

    pub fn denied_completion(&self) -> DeniedResourceCompletion {
        self.denied_completion
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneBHostileScenarioEvidence {
    schema_version: String,
    rows: Vec<ResourceMilestoneBHostileScenarioEvidenceRow>,
    evidence_digest: String,
}

impl ResourceMilestoneBHostileScenarioEvidence {
    pub fn schema_version(&self) -> &str {
        &self.schema_version
    }

    pub fn rows(&self) -> &[ResourceMilestoneBHostileScenarioEvidenceRow] {
        &self.rows
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    fn row_for(
        &self,
        id: ResourceMilestoneBScenarioId,
    ) -> Option<&ResourceMilestoneBHostileScenarioEvidenceRow> {
        self.rows.iter().find(|row| row.id() == id)
    }
}

pub fn resource_milestone_b_hostile_scenario_evidence(
    late_superseded_completion: ResourceCompletionAdmissionReport,
    late_cancelled_completion: ResourceCompletionAdmissionReport,
    late_timed_out_completion: ResourceCompletionAdmissionReport,
    malformed_completion: ResourceCompletionAdmissionReport,
    completion_pressure_batch: &ResourceCompletionBatchAdmissionReport,
) -> Result<ResourceMilestoneBHostileScenarioEvidence, SignalError> {
    let rows = vec![
        ResourceMilestoneBHostileScenarioEvidenceRow::from_completion_denial_report(
            ResourceMilestoneBScenarioId::LateCompletionAfterSupersessionRejected,
            late_superseded_completion,
        )?,
        ResourceMilestoneBHostileScenarioEvidenceRow::from_completion_denial_report(
            ResourceMilestoneBScenarioId::LateCompletionAfterCancellationRejected,
            late_cancelled_completion,
        )?,
        ResourceMilestoneBHostileScenarioEvidenceRow::from_completion_denial_report(
            ResourceMilestoneBScenarioId::LateCompletionAfterTimeoutRejected,
            late_timed_out_completion,
        )?,
        ResourceMilestoneBHostileScenarioEvidenceRow::from_completion_denial_report(
            ResourceMilestoneBScenarioId::MalformedCompletionRejected,
            malformed_completion,
        )?,
        ResourceMilestoneBHostileScenarioEvidenceRow::from_completion_batch_denial_report(
            ResourceMilestoneBScenarioId::DuplicateCompletionRejected,
            completion_pressure_batch,
        )?,
        ResourceMilestoneBHostileScenarioEvidenceRow::from_completion_batch_denial_report(
            ResourceMilestoneBScenarioId::ContradictoryCompletionRejected,
            completion_pressure_batch,
        )?,
        ResourceMilestoneBHostileScenarioEvidenceRow::from_completion_batch_denial_report(
            ResourceMilestoneBScenarioId::UnknownRequestCompletionRejected,
            completion_pressure_batch,
        )?,
    ];
    let evidence_digest =
        resource_canonical_digest(&ResourceMilestoneBHostileScenarioEvidenceDigestBasis {
            schema_version: RESOURCE_MILESTONE_B_HOSTILE_SCENARIO_EVIDENCE_SCHEMA_VERSION,
            required_scenarios: &REQUIRED_RESOURCE_MILESTONE_B_HOSTILE_SCENARIOS,
            rows: &rows,
        });
    Ok(ResourceMilestoneBHostileScenarioEvidence {
        schema_version: RESOURCE_MILESTONE_B_HOSTILE_SCENARIO_EVIDENCE_SCHEMA_VERSION.to_owned(),
        rows,
        evidence_digest,
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneBScenarioRow {
    id: ResourceMilestoneBScenarioId,
    evidence_kind: ResourceMilestoneBScenarioEvidenceKind,
    certification_family: Option<ResourceCertificationFamily>,
    completion_denial_class: Option<CompletionDenialClass>,
    evidence_digest: String,
    performance: ResourceBoundaryPerformanceEnvelope,
    passed: bool,
}

impl ResourceMilestoneBScenarioRow {
    fn from_record(
        id: ResourceMilestoneBScenarioId,
        record: &ResourceCertificationRecord,
    ) -> Result<Self, SignalError> {
        let Some(expected_family) = id.certification_family() else {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} is not a certification-family scenario",
                id.label()
            )));
        };
        if record.family() != expected_family {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} requires {expected_family:?} evidence, got {:?}",
                id.label(),
                record.family()
            )));
        }
        if !record.passed() {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} requires passing certification evidence",
                id.label()
            )));
        }
        Ok(Self {
            id,
            evidence_kind: ResourceMilestoneBScenarioEvidenceKind::CertificationFamily,
            certification_family: Some(expected_family),
            completion_denial_class: None,
            evidence_digest: record.evidence_digest().to_owned(),
            performance: record.performance(),
            passed: true,
        })
    }

    fn from_hostile_completion_denial(
        id: ResourceMilestoneBScenarioId,
        evidence: &ResourceMilestoneBHostileScenarioEvidenceRow,
    ) -> Result<Self, SignalError> {
        let Some(expected_denial_class) = id.completion_denial_class() else {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} is not a hostile completion denial scenario",
                id.label()
            )));
        };
        if evidence.id() != id {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} cannot use {:?} hostile evidence",
                id.label(),
                evidence.id()
            )));
        }
        if evidence.expected_denial_class() != expected_denial_class {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} requires {expected_denial_class:?} hostile evidence",
                id.label()
            )));
        }
        Ok(Self {
            id,
            evidence_kind: ResourceMilestoneBScenarioEvidenceKind::HostileCompletionDenial,
            certification_family: None,
            completion_denial_class: Some(expected_denial_class),
            evidence_digest: evidence.evidence_digest().to_owned(),
            performance: evidence.performance(),
            passed: true,
        })
    }

    pub fn id(&self) -> ResourceMilestoneBScenarioId {
        self.id
    }

    pub fn evidence_kind(&self) -> ResourceMilestoneBScenarioEvidenceKind {
        self.evidence_kind
    }

    pub fn certification_family(&self) -> Option<ResourceCertificationFamily> {
        self.certification_family
    }

    pub fn completion_denial_class(&self) -> Option<CompletionDenialClass> {
        self.completion_denial_class
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }

    pub fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneBScenarioMatrixSummary {
    required_scenario_count: u32,
    certified_scenario_count: u32,
    failed_scenario_count: u32,
    bundle_digest: String,
}

impl ResourceMilestoneBScenarioMatrixSummary {
    pub fn required_scenario_count(&self) -> u32 {
        self.required_scenario_count
    }

    pub fn certified_scenario_count(&self) -> u32 {
        self.certified_scenario_count
    }

    pub fn failed_scenario_count(&self) -> u32 {
        self.failed_scenario_count
    }

    pub fn bundle_digest(&self) -> &str {
        &self.bundle_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneBScenarioMatrix {
    schema_version: String,
    bundle_digest: String,
    rows: Vec<ResourceMilestoneBScenarioRow>,
    summary: ResourceMilestoneBScenarioMatrixSummary,
    matrix_digest: String,
    passed: bool,
}

impl ResourceMilestoneBScenarioMatrix {
    pub fn schema_version(&self) -> &str {
        &self.schema_version
    }

    pub fn bundle_digest(&self) -> &str {
        &self.bundle_digest
    }

    pub fn rows(&self) -> &[ResourceMilestoneBScenarioRow] {
        &self.rows
    }

    pub fn summary(&self) -> &ResourceMilestoneBScenarioMatrixSummary {
        &self.summary
    }

    pub fn matrix_digest(&self) -> &str {
        &self.matrix_digest
    }

    pub fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneBPerformanceCloseoutRow {
    id: ResourceMilestoneBPerformanceClaimId,
    evidence_digest: String,
    performance: ResourceBoundaryPerformanceEnvelope,
    passed: bool,
}

impl ResourceMilestoneBPerformanceCloseoutRow {
    fn scenario_family(
        id: ResourceMilestoneBPerformanceClaimId,
        scenario: ResourceMilestoneBScenarioId,
        scenario_matrix: &ResourceMilestoneBScenarioMatrix,
    ) -> Result<Self, SignalError> {
        let Some(row) = scenario_matrix
            .rows()
            .iter()
            .find(|row| row.id() == scenario)
        else {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B performance claim {} is missing {} scenario evidence",
                id.label(),
                scenario.label()
            )));
        };
        if row.evidence_kind() != ResourceMilestoneBScenarioEvidenceKind::CertificationFamily
            || !row.passed()
        {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B performance claim {} requires passing certification-family scenario evidence",
                id.label()
            )));
        }
        validate_certification_family_performance(id, scenario, row.performance())?;
        let evidence_digest =
            resource_canonical_digest(&ResourceMilestoneBPerformanceScenarioEvidenceBasis {
                claim: id,
                scenario,
                scenario_evidence_digest: row.evidence_digest(),
                performance: row.performance(),
            });
        Ok(Self {
            id,
            evidence_digest,
            performance: row.performance(),
            passed: true,
        })
    }

    fn summary_read(report: ResourceRuntimeSummaryReadReport) -> Result<Self, SignalError> {
        let performance = report.performance();
        let id = ResourceMilestoneBPerformanceClaimId::RuntimeSummaryReadZeroColdReconstruction;
        require_performance(
            id,
            performance,
            ResourceBoundaryKind::SummaryRead,
            ResourceCostPosture::Verified,
        )?;
        if performance.input_width() != 1
            || performance.admitted_count() != 1
            || performance.denied_count() != 0
            || performance.lifecycle_transition_count() != 0
            || performance.operational_allocation_count() != 0
            || performance.retained_history_allocation_count() != 0
            || performance.diagnostics_allocation_count() != 0
            || performance.facade_report_allocation_count() != 1
        {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B performance claim {} requires zero-cold summary read evidence",
                id.label()
            )));
        }
        let evidence_digest =
            resource_canonical_digest(&ResourceMilestoneBPerformanceSummaryReadEvidenceBasis {
                summary: report.summary(),
                performance,
            });
        Ok(Self {
            id,
            evidence_digest,
            performance,
            passed: true,
        })
    }

    fn diagnostics_summary(summary: &ResourceDiagnosticsSummary) -> Result<Self, SignalError> {
        let performance = summary.performance();
        let id =
            ResourceMilestoneBPerformanceClaimId::DiagnosticsExpansionBudgetedColdReconstruction;
        require_performance(
            id,
            performance,
            ResourceBoundaryKind::DiagnosticsExpansion,
            ResourceCostPosture::Debt,
        )?;
        let replay_width = summary.replay_reconstruction().performance().input_width();
        if !summary.expansion_budget().admits_replay_width(replay_width)
            || summary.replay_reconstruction().performance().boundary()
                != ResourceBoundaryKind::ReplayReconstruction
            || summary.replay_reconstruction().performance().cost_posture()
                != ResourceCostPosture::Debt
            || performance.diagnostics_allocation_count() != replay_width
            || performance.denied_count() != 0
        {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B performance claim {} requires budgeted diagnostics expansion evidence",
                id.label()
            )));
        }
        Ok(Self {
            id,
            evidence_digest: summary.provenance_digest().to_owned(),
            performance,
            passed: true,
        })
    }

    fn diagnostics_denial(denial: ResourceDiagnosticsExpansionDenial) -> Result<Self, SignalError> {
        let performance = denial.performance();
        let id = ResourceMilestoneBPerformanceClaimId::DiagnosticsExpansionBudgetDenial;
        require_performance(
            id,
            performance,
            ResourceBoundaryKind::DiagnosticsExpansion,
            ResourceCostPosture::DeniedFallback,
        )?;
        if denial.budget().denial_class(
            denial.replay_reconstruction_width(),
            denial.forensic_reconstruction_width(),
        ) != Some(denial.class())
            || performance.admitted_count() != 0
            || performance.denied_count() != 1
            || performance.diagnostics_allocation_count() != 0
        {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B performance claim {} requires budget denial evidence",
                id.label()
            )));
        }
        let evidence_digest =
            resource_canonical_digest(&ResourceMilestoneBPerformanceDiagnosticsDenialBasis {
                class: denial.class(),
                budget: denial.budget(),
                replay_reconstruction_width: denial.replay_reconstruction_width(),
                performance,
            });
        Ok(Self {
            id,
            evidence_digest,
            performance,
            passed: true,
        })
    }

    fn hostile_completion_denials(
        scenario_matrix: &ResourceMilestoneBScenarioMatrix,
    ) -> Result<Self, SignalError> {
        let id = ResourceMilestoneBPerformanceClaimId::HostileCompletionDenialsScalarBounded;
        let mut hostile_digests =
            Vec::with_capacity(REQUIRED_RESOURCE_MILESTONE_B_SCALAR_HOSTILE_SCENARIOS.len());
        let mut total_denied = 0_u32;
        for scenario in REQUIRED_RESOURCE_MILESTONE_B_SCALAR_HOSTILE_SCENARIOS {
            let Some(row) = scenario_matrix
                .rows()
                .iter()
                .find(|row| row.id() == scenario)
            else {
                return Err(SignalError::invalid_input(format!(
                    "resource milestone B performance claim {} is missing {} evidence",
                    id.label(),
                    scenario.label()
                )));
            };
            let performance = row.performance();
            require_performance(
                id,
                performance,
                ResourceBoundaryKind::CompletionAdmission,
                ResourceCostPosture::Verified,
            )?;
            if row.evidence_kind()
                != ResourceMilestoneBScenarioEvidenceKind::HostileCompletionDenial
                || !row.passed()
                || performance.input_width() != 1
                || performance.admitted_count() != 0
                || performance.denied_count() != 1
                || performance.lifecycle_transition_count() != 0
                || performance.operational_allocation_count() != 0
                || performance.diagnostics_allocation_count() != 0
                || performance.facade_report_allocation_count() != 1
            {
                return Err(SignalError::invalid_input(format!(
                    "resource milestone B performance claim {} requires scalar hostile completion denial evidence",
                    id.label()
                )));
            }
            total_denied = total_denied.saturating_add(performance.denied_count());
            hostile_digests.push((scenario, row.evidence_digest().to_owned()));
        }
        let performance =
            ResourceBoundaryPerformanceEnvelope::completion_admission(0, total_denied, 0);
        let evidence_digest =
            resource_canonical_digest(&ResourceMilestoneBPerformanceHostileDenialBasis {
                scenario_matrix_digest: scenario_matrix.matrix_digest(),
                hostile_digests: &hostile_digests,
                performance,
            });
        Ok(Self {
            id,
            evidence_digest,
            performance,
            passed: true,
        })
    }

    pub fn id(&self) -> ResourceMilestoneBPerformanceClaimId {
        self.id
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }

    pub fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneBPerformanceCloseoutSummary {
    required_claim_count: u32,
    certified_claim_count: u32,
    failed_claim_count: u32,
    scenario_matrix_digest: String,
}

impl ResourceMilestoneBPerformanceCloseoutSummary {
    pub fn required_claim_count(&self) -> u32 {
        self.required_claim_count
    }

    pub fn certified_claim_count(&self) -> u32 {
        self.certified_claim_count
    }

    pub fn failed_claim_count(&self) -> u32 {
        self.failed_claim_count
    }

    pub fn scenario_matrix_digest(&self) -> &str {
        &self.scenario_matrix_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneBPerformanceCloseout {
    schema_version: String,
    scenario_matrix_digest: String,
    rows: Vec<ResourceMilestoneBPerformanceCloseoutRow>,
    summary: ResourceMilestoneBPerformanceCloseoutSummary,
    closeout_digest: String,
    passed: bool,
}

impl ResourceMilestoneBPerformanceCloseout {
    pub fn schema_version(&self) -> &str {
        &self.schema_version
    }

    pub fn scenario_matrix_digest(&self) -> &str {
        &self.scenario_matrix_digest
    }

    pub fn rows(&self) -> &[ResourceMilestoneBPerformanceCloseoutRow] {
        &self.rows
    }

    pub fn summary(&self) -> &ResourceMilestoneBPerformanceCloseoutSummary {
        &self.summary
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }

    pub fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneBCertificationRun {
    schema_version: String,
    bundle: ResourceCertificationBundle,
    scenario_matrix: ResourceMilestoneBScenarioMatrix,
    performance_closeout: ResourceMilestoneBPerformanceCloseout,
    summary: ResourceMilestoneBCertificationRunSummary,
    run_digest: String,
    passed: bool,
}

impl ResourceMilestoneBCertificationRun {
    pub fn schema_version(&self) -> &str {
        &self.schema_version
    }

    pub fn bundle(&self) -> &ResourceCertificationBundle {
        &self.bundle
    }

    pub fn scenario_matrix(&self) -> &ResourceMilestoneBScenarioMatrix {
        &self.scenario_matrix
    }

    pub fn performance_closeout(&self) -> &ResourceMilestoneBPerformanceCloseout {
        &self.performance_closeout
    }

    pub fn summary(&self) -> &ResourceMilestoneBCertificationRunSummary {
        &self.summary
    }

    pub fn run_digest(&self) -> &str {
        &self.run_digest
    }

    pub fn passed(&self) -> bool {
        self.passed
    }
}

pub fn resource_milestone_b_scenario_matrix(
    bundle: &ResourceCertificationBundle,
    hostile_evidence: &ResourceMilestoneBHostileScenarioEvidence,
) -> Result<ResourceMilestoneBScenarioMatrix, SignalError> {
    bundle.ensure_passed()?;
    let bundle_summary = bundle.summary();
    let required_family_count = REQUIRED_RESOURCE_CERTIFICATION_FAMILIES.len() as u32;
    if bundle.records().len() != REQUIRED_RESOURCE_CERTIFICATION_FAMILIES.len()
        || bundle_summary.required_family_count() != required_family_count
        || bundle_summary.passed_family_count() != required_family_count
        || bundle_summary.failed_family_count() != 0
        || bundle_summary.missing_family_count() != 0
        || bundle_summary.duplicate_family_count() != 0
        || !bundle.failures().is_empty()
    {
        return Err(SignalError::invalid_input(
            "resource milestone B scenario matrix requires one passing record for every required family",
        ));
    }

    let records_by_family = bundle
        .records()
        .iter()
        .map(|record| (record.family(), record))
        .collect::<BTreeMap<_, _>>();
    let mut seen = BTreeSet::new();
    let mut rows = Vec::with_capacity(REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS.len());
    for scenario in REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS {
        if !seen.insert(scenario) {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} is duplicated",
                scenario.label()
            )));
        }
        if let Some(family) = scenario.certification_family() {
            let Some(record) = records_by_family.get(&family) else {
                return Err(SignalError::invalid_input(format!(
                    "resource milestone B scenario {} is missing {family:?} evidence",
                    scenario.label()
                )));
            };
            rows.push(ResourceMilestoneBScenarioRow::from_record(
                scenario, record,
            )?);
            continue;
        }
        let Some(hostile_row) = hostile_evidence.row_for(scenario) else {
            return Err(SignalError::invalid_input(format!(
                "resource milestone B scenario {} is missing hostile completion evidence",
                scenario.label()
            )));
        };
        rows.push(
            ResourceMilestoneBScenarioRow::from_hostile_completion_denial(scenario, hostile_row)?,
        );
    }

    let summary = ResourceMilestoneBScenarioMatrixSummary {
        required_scenario_count: REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS.len() as u32,
        certified_scenario_count: rows.len() as u32,
        failed_scenario_count: rows.iter().filter(|row| !row.passed()).count() as u32,
        bundle_digest: bundle.bundle_digest().to_owned(),
    };
    if summary.certified_scenario_count != summary.required_scenario_count
        || summary.failed_scenario_count != 0
    {
        return Err(SignalError::invalid_input(
            "resource milestone B scenario matrix did not cover every required scenario",
        ));
    }
    let matrix_digest = resource_canonical_digest(&ResourceMilestoneBScenarioMatrixDigestBasis {
        schema_version: RESOURCE_MILESTONE_B_SCENARIO_MATRIX_SCHEMA_VERSION,
        required_scenarios: &REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS,
        bundle_digest: bundle.bundle_digest(),
        summary: &summary,
        rows: &rows,
    });

    Ok(ResourceMilestoneBScenarioMatrix {
        schema_version: RESOURCE_MILESTONE_B_SCENARIO_MATRIX_SCHEMA_VERSION.to_owned(),
        bundle_digest: bundle.bundle_digest().to_owned(),
        rows,
        summary,
        matrix_digest,
        passed: true,
    })
}

pub fn resource_milestone_b_performance_closeout(
    scenario_matrix: &ResourceMilestoneBScenarioMatrix,
    summary_read: ResourceRuntimeSummaryReadReport,
    diagnostics_summary: ResourceDiagnosticsSummary,
    diagnostics_denial: ResourceDiagnosticsExpansionDenial,
) -> Result<ResourceMilestoneBPerformanceCloseout, SignalError> {
    if !scenario_matrix.passed() {
        return Err(SignalError::invalid_input(
            "resource milestone B performance closeout requires a passing scenario matrix",
        ));
    }
    let rows = vec![
        ResourceMilestoneBPerformanceCloseoutRow::scenario_family(
            ResourceMilestoneBPerformanceClaimId::LifecycleReplayParityDebtBounded,
            ResourceMilestoneBScenarioId::LifecycleReplayParity,
            scenario_matrix,
        )?,
        ResourceMilestoneBPerformanceCloseoutRow::scenario_family(
            ResourceMilestoneBPerformanceClaimId::OutOfOrderSupersessionAdmissionBounded,
            ResourceMilestoneBScenarioId::OutOfOrderSupersession,
            scenario_matrix,
        )?,
        ResourceMilestoneBPerformanceCloseoutRow::scenario_family(
            ResourceMilestoneBPerformanceClaimId::RollbackObservationRollbackBounded,
            ResourceMilestoneBScenarioId::RollbackObservationEquivalence,
            scenario_matrix,
        )?,
        ResourceMilestoneBPerformanceCloseoutRow::scenario_family(
            ResourceMilestoneBPerformanceClaimId::BranchRestoreReplayRestoreBounded,
            ResourceMilestoneBScenarioId::BranchRestoreReplayEquivalence,
            scenario_matrix,
        )?,
        ResourceMilestoneBPerformanceCloseoutRow::scenario_family(
            ResourceMilestoneBPerformanceClaimId::InflightBoundednessAdmissionBounded,
            ResourceMilestoneBScenarioId::InflightBoundedness,
            scenario_matrix,
        )?,
        ResourceMilestoneBPerformanceCloseoutRow::summary_read(summary_read)?,
        ResourceMilestoneBPerformanceCloseoutRow::diagnostics_summary(&diagnostics_summary)?,
        ResourceMilestoneBPerformanceCloseoutRow::diagnostics_denial(diagnostics_denial)?,
        ResourceMilestoneBPerformanceCloseoutRow::hostile_completion_denials(scenario_matrix)?,
    ];
    let row_ids = rows.iter().map(|row| row.id()).collect::<Vec<_>>();
    if row_ids != REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS {
        return Err(SignalError::invalid_input(
            "resource milestone B performance closeout rows do not match required claims",
        ));
    }
    let summary = ResourceMilestoneBPerformanceCloseoutSummary {
        required_claim_count: REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS.len() as u32,
        certified_claim_count: rows.len() as u32,
        failed_claim_count: rows.iter().filter(|row| !row.passed()).count() as u32,
        scenario_matrix_digest: scenario_matrix.matrix_digest().to_owned(),
    };
    if summary.certified_claim_count != summary.required_claim_count
        || summary.failed_claim_count != 0
    {
        return Err(SignalError::invalid_input(
            "resource milestone B performance closeout did not cover every required claim",
        ));
    }
    let closeout_digest =
        resource_canonical_digest(&ResourceMilestoneBPerformanceCloseoutDigestBasis {
            schema_version: RESOURCE_MILESTONE_B_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION,
            required_claims: &REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS,
            scenario_matrix_digest: scenario_matrix.matrix_digest(),
            summary: &summary,
            rows: &rows,
        });
    Ok(ResourceMilestoneBPerformanceCloseout {
        schema_version: RESOURCE_MILESTONE_B_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION.to_owned(),
        scenario_matrix_digest: scenario_matrix.matrix_digest().to_owned(),
        rows,
        summary,
        closeout_digest,
        passed: true,
    })
}

pub fn resource_milestone_b_certification_run(
    bundle: ResourceCertificationBundle,
    scenario_matrix: ResourceMilestoneBScenarioMatrix,
    performance_closeout: ResourceMilestoneBPerformanceCloseout,
) -> Result<ResourceMilestoneBCertificationRun, SignalError> {
    bundle.ensure_passed()?;
    if !scenario_matrix.passed() {
        return Err(SignalError::invalid_input(
            "resource milestone B certification run requires a passing scenario matrix",
        ));
    }
    if !performance_closeout.passed() {
        return Err(SignalError::invalid_input(
            "resource milestone B certification run requires a passing performance closeout",
        ));
    }
    if scenario_matrix.bundle_digest() != bundle.bundle_digest() {
        return Err(SignalError::invalid_input(
            "resource milestone B certification run requires scenario matrix evidence from the same bundle",
        ));
    }
    if performance_closeout.scenario_matrix_digest() != scenario_matrix.matrix_digest() {
        return Err(SignalError::invalid_input(
            "resource milestone B certification run requires performance closeout evidence from the same scenario matrix",
        ));
    }
    if scenario_matrix.rows().len() != REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS.len()
        || scenario_matrix.summary().required_scenario_count()
            != REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS.len() as u32
        || scenario_matrix.summary().certified_scenario_count()
            != REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS.len() as u32
        || scenario_matrix.summary().failed_scenario_count() != 0
    {
        return Err(SignalError::invalid_input(
            "resource milestone B certification run requires one passing row for every required scenario",
        ));
    }
    if performance_closeout.rows().len() != REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS.len()
        || performance_closeout.summary().required_claim_count()
            != REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS.len() as u32
        || performance_closeout.summary().certified_claim_count()
            != REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS.len() as u32
        || performance_closeout.summary().failed_claim_count() != 0
    {
        return Err(SignalError::invalid_input(
            "resource milestone B certification run requires one passing row for every required performance claim",
        ));
    }
    let bundle_summary = bundle.summary();
    let required_family_count = REQUIRED_RESOURCE_CERTIFICATION_FAMILIES.len() as u32;
    if bundle.records().len() != REQUIRED_RESOURCE_CERTIFICATION_FAMILIES.len()
        || bundle_summary.required_family_count() != required_family_count
        || bundle_summary.passed_family_count() != required_family_count
        || bundle_summary.failed_family_count() != 0
        || bundle_summary.missing_family_count() != 0
        || bundle_summary.duplicate_family_count() != 0
        || !bundle.failures().is_empty()
    {
        return Err(SignalError::invalid_input(
            "resource milestone B certification run requires one passing record for every required family",
        ));
    }

    let summary = ResourceMilestoneBCertificationRunSummary {
        required_family_count,
        certified_family_count: bundle_summary.passed_family_count(),
        failed_family_count: bundle_summary.failed_family_count(),
        bundle_digest: bundle.bundle_digest().to_owned(),
        required_scenario_count: scenario_matrix.summary().required_scenario_count(),
        certified_scenario_count: scenario_matrix.summary().certified_scenario_count(),
        scenario_matrix_digest: scenario_matrix.matrix_digest().to_owned(),
        required_performance_claim_count: performance_closeout.summary().required_claim_count(),
        certified_performance_claim_count: performance_closeout.summary().certified_claim_count(),
        performance_closeout_digest: performance_closeout.closeout_digest().to_owned(),
    };
    let run_digest = resource_canonical_digest(&ResourceMilestoneBCertificationRunDigestBasis {
        schema_version: RESOURCE_MILESTONE_B_CERTIFICATION_RUN_SCHEMA_VERSION,
        required_families: &REQUIRED_RESOURCE_CERTIFICATION_FAMILIES,
        required_scenarios: &REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS,
        required_performance_claims: &REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS,
        summary: &summary,
        bundle_digest: bundle.bundle_digest(),
        scenario_matrix_digest: scenario_matrix.matrix_digest(),
        performance_closeout_digest: performance_closeout.closeout_digest(),
        record_digests: bundle
            .records()
            .iter()
            .map(|record| (record.family(), record.evidence_digest()))
            .collect::<Vec<_>>(),
        scenario_digests: scenario_matrix
            .rows()
            .iter()
            .map(|row| (row.id(), row.evidence_digest()))
            .collect::<Vec<_>>(),
        performance_claim_digests: performance_closeout
            .rows()
            .iter()
            .map(|row| (row.id(), row.evidence_digest()))
            .collect::<Vec<_>>(),
    });

    Ok(ResourceMilestoneBCertificationRun {
        schema_version: RESOURCE_MILESTONE_B_CERTIFICATION_RUN_SCHEMA_VERSION.to_owned(),
        bundle,
        scenario_matrix,
        performance_closeout,
        summary,
        run_digest,
        passed: true,
    })
}

fn require_performance(
    id: ResourceMilestoneBPerformanceClaimId,
    performance: ResourceBoundaryPerformanceEnvelope,
    expected_boundary: ResourceBoundaryKind,
    expected_cost_posture: ResourceCostPosture,
) -> Result<(), SignalError> {
    if performance.boundary() != expected_boundary {
        return Err(SignalError::invalid_input(format!(
            "resource milestone B performance claim {} requires {expected_boundary:?} boundary evidence, got {:?}",
            id.label(),
            performance.boundary()
        )));
    }
    if performance.cost_posture() != expected_cost_posture {
        return Err(SignalError::invalid_input(format!(
            "resource milestone B performance claim {} requires {expected_cost_posture:?} cost posture, got {:?}",
            id.label(),
            performance.cost_posture()
        )));
    }
    Ok(())
}

fn validate_certification_family_performance(
    id: ResourceMilestoneBPerformanceClaimId,
    scenario: ResourceMilestoneBScenarioId,
    performance: ResourceBoundaryPerformanceEnvelope,
) -> Result<(), SignalError> {
    match scenario {
        ResourceMilestoneBScenarioId::LifecycleReplayParity => {
            require_performance(
                id,
                performance,
                ResourceBoundaryKind::ReplayReconstruction,
                ResourceCostPosture::Debt,
            )?;
            if performance.operational_allocation_count() != 0
                || performance.retained_history_allocation_count() != 0
                || performance.diagnostics_allocation_count() != performance.input_width()
                || performance.facade_report_allocation_count() != 1
                || performance.density_strategy() != ResourceDensityStrategy::NotApplicable
            {
                return Err(performance_claim_error(
                    id,
                    "replay parity must expose diagnostics-only cold reconstruction debt",
                ));
            }
        }
        ResourceMilestoneBScenarioId::OutOfOrderSupersession => {
            require_performance(
                id,
                performance,
                ResourceBoundaryKind::RequestAdmission,
                ResourceCostPosture::Verified,
            )?;
            if performance.input_width() != 1
                || performance.admitted_count() != 1
                || performance.denied_count() != 0
                || performance.lifecycle_transition_count() != 2
                || performance.operational_allocation_count() != 1
                || performance.retained_history_allocation_count() != 2
                || performance.diagnostics_allocation_count() != 0
                || performance.facade_report_allocation_count() != 1
                || performance.density_strategy()
                    != ResourceDensityStrategy::BurstySortedDeduplicated
            {
                return Err(performance_claim_error(
                    id,
                    "supersession admission must stay one admitted request with explicit two-transition lineage",
                ));
            }
        }
        ResourceMilestoneBScenarioId::RollbackObservationEquivalence => {
            require_performance(
                id,
                performance,
                ResourceBoundaryKind::CompletionRollback,
                ResourceCostPosture::Verified,
            )?;
            if performance.input_width() != 1
                || performance.admitted_count() != 1
                || performance.denied_count() != 0
                || performance.lifecycle_transition_count() != 0
                || performance.operational_allocation_count() != 0
                || performance.retained_history_allocation_count() != 0
                || performance.diagnostics_allocation_count() != 0
                || performance.facade_report_allocation_count() != 1
                || performance.density_strategy() != ResourceDensityStrategy::NotApplicable
            {
                return Err(performance_claim_error(
                    id,
                    "rollback observation proof must not perform lifecycle or retained-history work",
                ));
            }
        }
        ResourceMilestoneBScenarioId::BranchRestoreReplayEquivalence => {
            require_performance(
                id,
                performance,
                ResourceBoundaryKind::BranchRestore,
                ResourceCostPosture::Verified,
            )?;
            if performance.denied_count() != 0
                || performance.broad_scan_denial_count() == 0
                || performance.operational_allocation_count() != performance.admitted_count()
                || performance.retained_history_allocation_count()
                    != performance
                        .input_width()
                        .saturating_sub(performance.admitted_count())
                || performance.diagnostics_allocation_count() != 0
                || performance.facade_report_allocation_count() != 1
                || performance.density_strategy() != ResourceDensityStrategy::NotApplicable
            {
                return Err(performance_claim_error(
                    id,
                    "branch restore must bind retained summaries and broad rebuild denial without diagnostics work",
                ));
            }
        }
        ResourceMilestoneBScenarioId::InflightBoundedness => {
            require_performance(
                id,
                performance,
                ResourceBoundaryKind::CompletionBatchAdmission,
                ResourceCostPosture::Verified,
            )?;
            if performance.input_width() != 4
                || performance.admitted_count() != 1
                || performance.denied_count() != 3
                || performance.lifecycle_transition_count() != 1
                || performance.operational_allocation_count() != 3
                || performance.retained_history_allocation_count() != 0
                || performance.diagnostics_allocation_count() != 4
                || performance.facade_report_allocation_count() != 1
                || performance.density_strategy()
                    != ResourceDensityStrategy::BurstySortedDeduplicated
            {
                return Err(performance_claim_error(
                    id,
                    "inflight boundedness must stay a bursty inflight-local completion boundary with explicit mixed denial pressure and attributable per-envelope diagnostics",
                ));
            }
        }
        ResourceMilestoneBScenarioId::LateCompletionAfterSupersessionRejected
        | ResourceMilestoneBScenarioId::LateCompletionAfterCancellationRejected
        | ResourceMilestoneBScenarioId::LateCompletionAfterTimeoutRejected
        | ResourceMilestoneBScenarioId::MalformedCompletionRejected
        | ResourceMilestoneBScenarioId::DuplicateCompletionRejected
        | ResourceMilestoneBScenarioId::ContradictoryCompletionRejected
        | ResourceMilestoneBScenarioId::UnknownRequestCompletionRejected => {
            return Err(performance_claim_error(
                id,
                "hostile completion scenarios are certified by the hostile closeout claim",
            ));
        }
    }
    Ok(())
}

fn performance_claim_error(
    id: ResourceMilestoneBPerformanceClaimId,
    reason: &'static str,
) -> SignalError {
    SignalError::invalid_input(format!(
        "resource milestone B performance claim {} failed: {reason}",
        id.label()
    ))
}

#[derive(Debug, Serialize)]
struct ResourceCertificationBundleDigestBasis<'a> {
    schema_version: &'static str,
    records: &'a [ResourceCertificationRecord],
}

#[derive(Debug, Serialize)]
struct ResourceMilestoneCPolicyCertificationBundleDigestBasis<'a> {
    schema_version: &'static str,
    records: &'a [ResourceMilestoneCPolicyCertificationRecord],
}

#[derive(Debug, Serialize)]
struct ResourceMilestoneCPolicyScenarioMatrixDigestBasis<'a> {
    schema_version: &'static str,
    required_scenarios: &'a [ResourceMilestoneCPolicyScenarioId],
    bundle_digest: &'a str,
    summary: &'a ResourceMilestoneCPolicyScenarioMatrixSummary,
    rows: &'a [ResourceMilestoneCPolicyScenarioRow],
}

#[derive(Debug, Serialize)]
struct ResourceMilestoneCPolicyFamilyEvidenceBasis<'a> {
    descriptor_count: usize,
    id_index_width: usize,
    kind_name_index_width: usize,
    registry_digest: &'a str,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
struct ResourceMilestoneCRetryPolicyEvidenceBasis<'a> {
    scheduled_retry: Option<&'a super::retry::ScheduledResourceRetry>,
    denied_retry: Option<DeniedResourceRetry>,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
struct ResourceMilestoneCTimeoutPolicyEvidenceBasis<'a> {
    timed_out_request: Option<&'a super::timeout::TimedOutResourceRequest>,
    denied_timeout: Option<super::timeout::DeniedResourceTimeout>,
    heartbeat_extension: Option<&'a super::timeout::ExtendedResourceTimeoutHeartbeat>,
    denied_heartbeat_extension: Option<super::timeout::DeniedResourceTimeoutHeartbeatExtension>,
    timeout_performance: ResourceBoundaryPerformanceEnvelope,
    heartbeat_performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
struct ResourceMilestoneCCancellationSupersessionEvidenceBasis<'a> {
    cancelled_request: Option<super::cancellation::CancelledResourceRequest>,
    denied_cancellation: Option<super::cancellation::DeniedResourceCancellation>,
    dependent_propagation: Option<super::cancellation::ResourceDependentCancellationPropagation>,
    overlap_admission: &'a ResourceOverlappingGenerationAdmission,
    intent_coalescing: &'a ResourceIntentEquivalenceCoalescing,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
struct ResourceMilestoneCRevalidationEvidenceBasis {
    admitted_revalidation: Option<super::revalidation::AdmittedResourceRevalidation>,
    denied_revalidation: Option<super::revalidation::DeniedResourceRevalidation>,
    lifecycle: Option<super::summary::ResourceLifecycleSummary>,
    transition: Option<super::lifecycle::ResourceLifecycleTransition>,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
struct ResourceMilestoneCObservationEvidenceBasis<'a> {
    events: &'a [super::observation::ResourceObservationEvent],
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
struct ResourceMilestoneCRetentionReplayEvidenceBasis<'a> {
    retention_report: &'a ResourceLifecycleRetentionCompactionReport,
    replay_class: super::replay_availability::ResourceReplayAvailabilityClass,
    replay_denial_class: Option<super::replay_availability::ResourceReplayAvailabilityDenialClass>,
    retained_history_unavailable_count: u32,
    denied_completion_unavailable_count: u32,
    retry_lineage_unavailable_count: u32,
    availability_digest: &'a str,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
struct ResourceMilestoneCPolicyRegistryFreezeEvidenceBasis<'a> {
    descriptor_count: usize,
    id_index_width: usize,
    kind_name_index_width: usize,
    registry_digest: &'a str,
}

#[derive(Debug, Serialize)]
struct ResourceMilestoneCRetryDenialEvidenceBasis {
    class: ResourceRetryDenialClass,
    retry_budget_scope: Option<super::policy::ResourceRetryBudgetScope>,
    retry_budget_limit: Option<u32>,
    retry_budget_usage: Option<u32>,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
struct ResourceMilestoneCTimeoutHeartbeatDenialEvidenceBasis {
    class: ResourceTimeoutHeartbeatExtensionDenialClass,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
struct ResourceMilestoneCRetentionCompactionEvidenceBasis {
    retained_history_pruned_count: u32,
    retained_history_unavailable_count: u32,
    retained_denied_completion_pruned_count: u32,
    retained_retry_lineage_pruned_count: u32,
    compacted_terminal_summary_count: u32,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
struct ResourceMilestoneCDiagnosticsDenialEvidenceBasis<'a> {
    class: ResourceDiagnosticsExpansionDenialClass,
    policy_decision_class: super::policy::ResourceDiagnosticsDecisionClass,
    replay_reconstruction_width: u32,
    forensic_reconstruction_width: u32,
    performance: ResourceBoundaryPerformanceEnvelope,
    policy_decision_digest: &'a str,
}

#[derive(Debug, Serialize)]
struct ResourceMilestoneCRestoreProofEvidenceBasis<'a> {
    compatibility_digest: &'a str,
    replay_decision_digest: &'a str,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
struct ResourceMilestoneCRestoreDenialEvidenceBasis<'a> {
    class: ResourcePolicyRestoreCompatibilityDenialClass,
    primary_incompatible_kind: Option<super::policy_registry::ResourcePolicyKind>,
    compatibility_digest: &'a str,
    replay_decision_digest: &'a str,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
struct ResourceMilestoneCPolicyPerformanceCloseoutDigestBasis<'a> {
    schema_version: &'static str,
    required_claims: &'a [ResourceMilestoneCPolicyPerformanceClaimId],
    scenario_matrix_digest: &'a str,
    summary: &'a ResourceMilestoneCPolicyPerformanceCloseoutSummary,
    rows: &'a [ResourceMilestoneCPolicyPerformanceCloseoutRow],
}

#[derive(Debug, Serialize)]
struct ResourceMilestoneCPolicyPerformanceScenarioEvidenceBasis<'a> {
    claim: ResourceMilestoneCPolicyPerformanceClaimId,
    scenario: ResourceMilestoneCPolicyScenarioId,
    scenario_evidence_digest: &'a str,
    policy_provenance_digest: &'a str,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
struct ResourceMilestoneCPolicyPerformanceReplayCompatibilityBasis<'a> {
    claim: ResourceMilestoneCPolicyPerformanceClaimId,
    scenario_matrix_digest: &'a str,
    row_digests: &'a [(ResourceMilestoneCPolicyScenarioId, String)],
    policy_provenance_digest: &'a str,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
struct ResourceMilestoneCPolicyPerformanceReplayPolicyProvenanceBasis<'a> {
    claim: ResourceMilestoneCPolicyPerformanceClaimId,
    row_policy_provenance: &'a [(ResourceMilestoneCPolicyScenarioId, String)],
}

#[derive(Debug, Serialize)]
struct ResourceMilestoneCCertificationRunDigestBasis<'a> {
    schema_version: &'static str,
    required_families: &'a [ResourceMilestoneCPolicyCertificationFamily],
    required_scenarios: &'a [ResourceMilestoneCPolicyScenarioId],
    required_performance_claims: &'a [ResourceMilestoneCPolicyPerformanceClaimId],
    summary: &'a ResourceMilestoneCCertificationRunSummary,
    bundle_digest: &'a str,
    scenario_matrix_digest: &'a str,
    performance_closeout_digest: &'a str,
    record_digests: Vec<(ResourceMilestoneCPolicyCertificationFamily, &'a str)>,
    scenario_digests: Vec<(ResourceMilestoneCPolicyScenarioId, &'a str)>,
    performance_claim_digests: Vec<(ResourceMilestoneCPolicyPerformanceClaimId, &'a str)>,
}

#[derive(Debug, Serialize)]
struct ResourceMilestoneBScenarioMatrixDigestBasis<'a> {
    schema_version: &'static str,
    required_scenarios: &'a [ResourceMilestoneBScenarioId],
    bundle_digest: &'a str,
    summary: &'a ResourceMilestoneBScenarioMatrixSummary,
    rows: &'a [ResourceMilestoneBScenarioRow],
}

#[derive(Debug, Serialize)]
struct ResourceMilestoneBHostileScenarioEvidenceDigestBasis<'a> {
    schema_version: &'static str,
    required_scenarios: &'a [ResourceMilestoneBScenarioId],
    rows: &'a [ResourceMilestoneBHostileScenarioEvidenceRow],
}

#[derive(Debug, Serialize)]
struct ResourceMilestoneBHostileScenarioEvidenceRowDigestBasis {
    id: ResourceMilestoneBScenarioId,
    expected_denial_class: CompletionDenialClass,
    denied_completion: DeniedResourceCompletion,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
struct ResourceMilestoneBPerformanceCloseoutDigestBasis<'a> {
    schema_version: &'static str,
    required_claims: &'a [ResourceMilestoneBPerformanceClaimId],
    scenario_matrix_digest: &'a str,
    summary: &'a ResourceMilestoneBPerformanceCloseoutSummary,
    rows: &'a [ResourceMilestoneBPerformanceCloseoutRow],
}

#[derive(Debug, Serialize)]
struct ResourceMilestoneBPerformanceScenarioEvidenceBasis<'a> {
    claim: ResourceMilestoneBPerformanceClaimId,
    scenario: ResourceMilestoneBScenarioId,
    scenario_evidence_digest: &'a str,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
struct ResourceMilestoneBPerformanceSummaryReadEvidenceBasis {
    summary: ResourceRuntimeSummary,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
struct ResourceMilestoneBPerformanceDiagnosticsDenialBasis {
    class: ResourceDiagnosticsExpansionDenialClass,
    budget: ResourceDiagnosticsExpansionBudget,
    replay_reconstruction_width: u32,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
struct ResourceMilestoneBPerformanceHostileDenialBasis<'a> {
    scenario_matrix_digest: &'a str,
    hostile_digests: &'a [(ResourceMilestoneBScenarioId, String)],
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
struct ResourceMilestoneBCertificationRunDigestBasis<'a> {
    schema_version: &'static str,
    required_families: &'a [ResourceCertificationFamily],
    required_scenarios: &'a [ResourceMilestoneBScenarioId],
    required_performance_claims: &'a [ResourceMilestoneBPerformanceClaimId],
    summary: &'a ResourceMilestoneBCertificationRunSummary,
    bundle_digest: &'a str,
    scenario_matrix_digest: &'a str,
    performance_closeout_digest: &'a str,
    record_digests: Vec<(ResourceCertificationFamily, &'a str)>,
    scenario_digests: Vec<(ResourceMilestoneBScenarioId, &'a str)>,
    performance_claim_digests: Vec<(ResourceMilestoneBPerformanceClaimId, &'a str)>,
}

#[derive(Debug, Serialize)]
struct ResourceLifecycleParityEvidenceBasis<'a> {
    descriptor_digest: &'a str,
    lifecycle_digest: &'a str,
    output_continuity_digest: &'a str,
    denied_completion_digest: &'a str,
    retry_lineage_digest: &'a str,
    in_flight_digest: &'a str,
    replay_digest: &'a str,
    retained_history_unavailable_count: u32,
    denied_completion_unavailable_count: u32,
    retry_lineage_unavailable_count: u32,
    diagnostics_provenance_digest: &'a str,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
struct ResourceSupersessionEvidenceBasis {
    supersession: super::ResourceSupersessionRecord,
    superseded_request: Option<super::ResourceRequestHandle>,
    superseded_transition: Option<super::ResourceLifecycleTransition>,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
struct ResourceRollbackObservationEvidenceBasis<'a> {
    subject: super::ResourceCompletionRollbackSubject,
    observation: ResourceObservationBatchReport,
    control_observation: ResourceObservationBatchReport,
    pre_rollback_descriptor_digest: &'a str,
    pre_rollback_lifecycle_digest: &'a str,
    pre_rollback_output_continuity_digest: &'a str,
    pre_rollback_denied_completion_digest: &'a str,
    pre_rollback_retry_lineage_digest: &'a str,
    pre_rollback_in_flight_digest: &'a str,
    pre_rollback_replay_digest: &'a str,
    post_rollback_descriptor_digest: &'a str,
    post_rollback_lifecycle_digest: &'a str,
    post_rollback_output_continuity_digest: &'a str,
    post_rollback_denied_completion_digest: &'a str,
    post_rollback_retry_lineage_digest: &'a str,
    post_rollback_in_flight_digest: &'a str,
    post_rollback_replay_digest: &'a str,
    diagnostics_provenance_digest: &'a str,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
struct ResourceBranchRestoreReplayEvidenceBasis<'a> {
    restore: ResourceBranchRestoreReport,
    descriptor_digest: &'a str,
    lifecycle_digest: &'a str,
    denied_completion_digest: &'a str,
    in_flight_digest: &'a str,
    replay_digest: &'a str,
    replay_performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
struct ResourceInflightBoundednessEvidenceBasis {
    summary: ResourceRuntimeSummary,
    replay_in_flight_width: u32,
    replay_digest: String,
    retry_admission_count: u64,
    retry_duplicate_denial_count: u64,
    branch_restore_count: u64,
    branch_restore_broad_rebuild_denial_count: u64,
    superseded_completion_denial_count: u64,
    duplicate_completion_denial_count: u64,
    contradictory_completion_denial_count: u64,
    unknown_request_completion_denial_count: u64,
    broad_scan_denial_count: u64,
    hot_in_flight_lookup_count: u64,
    performance: ResourceBoundaryPerformanceEnvelope,
}

fn resource_canonical_digest<T: Serialize>(value: &T) -> String {
    let bytes = serde_json::to_vec(value).expect("resource certification serialization");
    let digest = Sha256::digest(bytes);
    format!("{digest:x}")
}

fn invalid_resource_certification_evidence(
    family: ResourceCertificationFamily,
    reason: &'static str,
) -> SignalError {
    SignalError::invalid_input(format!(
        "invalid resource certification evidence for {family:?}: {reason}"
    ))
}
