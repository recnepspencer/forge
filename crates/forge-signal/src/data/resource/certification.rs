use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::data::error::SignalError;

use super::summary::{
    ResourceBoundaryKind, ResourceBoundaryPerformanceEnvelope, ResourceBranchRestoreReport,
    ResourceCompletionRollbackReport, ResourceReplayReconstructionReport,
    ResourceRequestAdmissionReport, ResourceRuntimeSummary,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResourceCertificationFamily {
    AsyncResourceLifecycleParity,
    OutOfOrderCompletionSupersession,
    AsyncRollbackObservationEquivalence,
    AsyncBranchRestoreReplayEquivalence,
    AsyncInflightBoundedness,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Serialize)]
struct ResourceCertificationBundleDigestBasis<'a> {
    schema_version: &'static str,
    records: &'a [ResourceCertificationRecord],
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
