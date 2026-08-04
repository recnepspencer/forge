use super::super::catalog::ResourceMilestoneCPolicyCertificationFamily;
use crate::data::error::SignalError;
use crate::data::resource::ResourceBoundaryPerformanceEnvelope;
use serde::Deserialize;
use serde::Serialize;

pub const RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_BUNDLE_SCHEMA_VERSION: &str =
    "worth-signal-resource-milestone-c-policy-certification-bundle-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneCPolicyCertificationRecord {
    pub(super) family: ResourceMilestoneCPolicyCertificationFamily,
    pub(super) evidence_digest: String,
    pub(super) performance: ResourceBoundaryPerformanceEnvelope,
    pub(super) passed: bool,
}

impl ResourceMilestoneCPolicyCertificationRecord {
    pub(super) fn passing(
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
    pub(super) required_family_count: u32,
    pub(super) provided_record_count: u32,
    pub(super) certified_family_count: u32,
    pub(super) failed_family_count: u32,
    pub(super) missing_family_count: u32,
    pub(super) duplicate_family_count: u32,
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
    pub(super) schema_version: String,
    pub(super) records: Vec<ResourceMilestoneCPolicyCertificationRecord>,
    pub(super) summary: ResourceMilestoneCPolicyCertificationSummary,
    pub(super) bundle_digest: String,
    pub(super) passed: bool,
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
