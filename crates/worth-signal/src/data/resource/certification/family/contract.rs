use super::catalog::ResourceCertificationFamily;
use super::evidence::invalid_resource_certification_evidence;
use crate::data::error::SignalError;
use crate::data::resource::ResourceBoundaryPerformanceEnvelope;
use serde::Deserialize;
use serde::Serialize;

pub const RESOURCE_CERTIFICATION_BUNDLE_SCHEMA_VERSION: &str =
    "worth-signal-resource-certification-bundle-v1";

pub const RESOURCE_CERTIFICATION_BUNDLE_PARITY_SCHEMA_VERSION: &str =
    "worth-signal-resource-certification-bundle-parity-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceCertificationRecord {
    pub(super) family: ResourceCertificationFamily,
    pub(super) evidence_digest: String,
    pub(super) performance: ResourceBoundaryPerformanceEnvelope,
    pub(super) passed: bool,
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
    pub(super) required_family_count: u32,
    pub(super) provided_record_count: u32,
    pub(super) passed_family_count: u32,
    pub(super) failed_family_count: u32,
    pub(super) missing_family_count: u32,
    pub(super) duplicate_family_count: u32,
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
    pub(super) schema_version: String,
    pub(super) records: Vec<ResourceCertificationRecord>,
    pub(super) summary: ResourceCertificationSummary,
    pub(super) bundle_digest: String,
    pub(super) passed: bool,
    pub(super) failures: Vec<ResourceCertificationFailure>,
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
    pub(super) proof_schema_version: String,
    pub(super) expected: ResourceCertificationBundle,
    pub(super) replayed: ResourceCertificationBundle,
    pub(super) parity: bool,
    pub(super) mismatch_classes: Vec<ResourceCertificationBundleMismatchClass>,
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
