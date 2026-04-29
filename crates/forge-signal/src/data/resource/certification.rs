use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::data::error::SignalError;

use super::completion::DeniedResourceCompletion;
use super::denial::CompletionDenialClass;
use super::diagnostics::{
    ResourceDiagnosticsExpansionBudget, ResourceDiagnosticsExpansionDenial,
    ResourceDiagnosticsExpansionDenialClass, ResourceDiagnosticsSummary,
};
use super::summary::{
    ResourceBoundaryKind, ResourceBoundaryPerformanceEnvelope, ResourceBranchRestoreReport,
    ResourceCompletionAdmissionReport, ResourceCompletionRollbackReport, ResourceCostPosture,
    ResourceDensityStrategy, ResourceReplayReconstructionReport, ResourceRequestAdmissionReport,
    ResourceRuntimeSummary, ResourceRuntimeSummaryReadReport,
};

pub const REQUIRED_RESOURCE_CERTIFICATION_FAMILIES: [ResourceCertificationFamily; 5] = [
    ResourceCertificationFamily::AsyncResourceLifecycleParity,
    ResourceCertificationFamily::OutOfOrderCompletionSupersession,
    ResourceCertificationFamily::AsyncRollbackObservationEquivalence,
    ResourceCertificationFamily::AsyncBranchRestoreReplayEquivalence,
    ResourceCertificationFamily::AsyncInflightBoundedness,
];

pub const RESOURCE_CERTIFICATION_BUNDLE_SCHEMA_VERSION: &str =
    "forge-signal-resource-certification-bundle-v1";
pub const RESOURCE_CERTIFICATION_BUNDLE_PARITY_SCHEMA_VERSION: &str =
    "forge-signal-resource-certification-bundle-parity-v1";
pub const RESOURCE_MILESTONE_B_HOSTILE_SCENARIO_EVIDENCE_SCHEMA_VERSION: &str =
    "forge-signal-resource-milestone-b-hostile-scenario-evidence-v1";
pub const RESOURCE_MILESTONE_B_SCENARIO_MATRIX_SCHEMA_VERSION: &str =
    "forge-signal-resource-milestone-b-scenario-matrix-v1";
pub const RESOURCE_MILESTONE_B_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION: &str =
    "forge-signal-resource-milestone-b-performance-closeout-v1";
pub const RESOURCE_MILESTONE_B_CERTIFICATION_RUN_SCHEMA_VERSION: &str =
    "forge-signal-resource-milestone-b-certification-run-v1";

pub const REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS: [ResourceMilestoneBScenarioId; 9] = [
    ResourceMilestoneBScenarioId::LifecycleReplayParity,
    ResourceMilestoneBScenarioId::OutOfOrderSupersession,
    ResourceMilestoneBScenarioId::RollbackObservationEquivalence,
    ResourceMilestoneBScenarioId::BranchRestoreReplayEquivalence,
    ResourceMilestoneBScenarioId::InflightBoundedness,
    ResourceMilestoneBScenarioId::LateCompletionAfterSupersessionRejected,
    ResourceMilestoneBScenarioId::LateCompletionAfterCancellationRejected,
    ResourceMilestoneBScenarioId::LateCompletionAfterTimeoutRejected,
    ResourceMilestoneBScenarioId::MalformedCompletionRejected,
];

pub const REQUIRED_RESOURCE_MILESTONE_B_HOSTILE_SCENARIOS: [ResourceMilestoneBScenarioId; 4] = [
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
            | Self::MalformedCompletionRejected => None,
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
        replay: &ResourceReplayReconstructionReport,
    ) -> Result<Self, SignalError> {
        self.async_resource_lifecycle_parity = Some(Self::record(
            self.async_resource_lifecycle_parity.take(),
            ResourceCertificationFamily::AsyncResourceLifecycleParity,
            ResourceCertificationEvidence::lifecycle_parity(replay),
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
    ) -> Result<Self, SignalError> {
        self.async_rollback_observation_equivalence = Some(Self::record(
            self.async_rollback_observation_equivalence.take(),
            ResourceCertificationFamily::AsyncRollbackObservationEquivalence,
            ResourceCertificationEvidence::rollback_observation(rollback),
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
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Result<Self, SignalError> {
        self.async_inflight_boundedness = Some(Self::record(
            self.async_inflight_boundedness.take(),
            ResourceCertificationFamily::AsyncInflightBoundedness,
            ResourceCertificationEvidence::inflight_boundedness(summary, performance)?,
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

#[derive(Debug)]
struct ResourceCertificationEvidence {
    digest: String,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceCertificationEvidence {
    fn lifecycle_parity(replay: &ResourceReplayReconstructionReport) -> Self {
        Self {
            digest: resource_canonical_digest(&ResourceLifecycleParityEvidenceBasis {
                descriptor_digest: replay.descriptor_digest(),
                lifecycle_digest: replay.lifecycle_digest(),
                replay_digest: replay.replay_digest(),
                retained_history_unavailable_count: replay.retained_history_unavailable_count(),
                performance: replay.performance(),
            }),
            performance: replay.performance(),
        }
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

    fn rollback_observation(rollback: ResourceCompletionRollbackReport) -> Self {
        let performance = rollback.performance();
        let rolled_back = rollback.rolled_back_completion();
        Self {
            digest: resource_canonical_digest(&ResourceRollbackObservationEvidenceBasis {
                subject: rolled_back.subject(),
                performance,
            }),
            performance,
        }
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
        Ok(Self {
            digest: resource_canonical_digest(&ResourceInflightBoundednessEvidenceBasis {
                summary,
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
            Vec::with_capacity(REQUIRED_RESOURCE_MILESTONE_B_HOSTILE_SCENARIOS.len());
        let mut total_denied = 0_u32;
        for scenario in REQUIRED_RESOURCE_MILESTONE_B_HOSTILE_SCENARIOS {
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
                ResourceBoundaryKind::RequestAdmission,
                ResourceCostPosture::Verified,
            )?;
            if performance.input_width() != 1
                || performance.admitted_count() != 1
                || performance.denied_count() != 0
                || performance.lifecycle_transition_count() != 1
                || performance.operational_allocation_count() != 1
                || performance.retained_history_allocation_count() != 1
                || performance.diagnostics_allocation_count() != 0
                || performance.facade_report_allocation_count() != 1
                || performance.density_strategy() != ResourceDensityStrategy::SparseIndexedLookup
            {
                return Err(performance_claim_error(
                    id,
                    "inflight boundedness must stay a sparse one-request admission boundary",
                ));
            }
        }
        ResourceMilestoneBScenarioId::LateCompletionAfterSupersessionRejected
        | ResourceMilestoneBScenarioId::LateCompletionAfterCancellationRejected
        | ResourceMilestoneBScenarioId::LateCompletionAfterTimeoutRejected
        | ResourceMilestoneBScenarioId::MalformedCompletionRejected => {
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
    replay_digest: &'a str,
    retained_history_unavailable_count: u32,
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
struct ResourceRollbackObservationEvidenceBasis {
    subject: super::ResourceCompletionRollbackSubject,
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
