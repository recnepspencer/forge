use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::identity::{ArtifactFamilyId, CompatibilityManifestDigest};

use super::publication::{
    CompatibilityManifestFrontier, CompatibilityManifestPublicationRecord,
    CompatibilityManifestPublicationUnit,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityManifestRecoveryPlan {
    expected_family_count: u64,
    recovered_record_count: u64,
    frontier: CompatibilityManifestFrontier,
}

impl CompatibilityManifestRecoveryPlan {
    pub(crate) fn new(
        expected_family_count: u64,
        recovered_record_count: u64,
        frontier: CompatibilityManifestFrontier,
    ) -> Self {
        Self {
            expected_family_count,
            recovered_record_count,
            frontier,
        }
    }

    pub fn expected_family_count(&self) -> u64 {
        self.expected_family_count
    }

    pub fn recovered_record_count(&self) -> u64 {
        self.recovered_record_count
    }

    pub fn frontier(&self) -> &CompatibilityManifestFrontier {
        &self.frontier
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityRecoveredManifestIndex {
    records_by_family: BTreeMap<ArtifactFamilyId, CompatibilityManifestPublicationRecord>,
    recovery_plan: CompatibilityManifestRecoveryPlan,
}

impl CompatibilityRecoveredManifestIndex {
    pub(crate) fn new(
        records: Vec<CompatibilityManifestPublicationRecord>,
        frontier: CompatibilityManifestFrontier,
    ) -> Self {
        let mut records_by_family = BTreeMap::new();
        for record in records {
            records_by_family.insert(record.family_id().clone(), record);
        }
        let recovered_record_count = records_by_family.len() as u64;
        Self {
            records_by_family,
            recovery_plan: CompatibilityManifestRecoveryPlan::new(
                recovered_record_count,
                recovered_record_count,
                frontier,
            ),
        }
    }

    pub fn get(
        &self,
        family_id: &ArtifactFamilyId,
    ) -> Option<&CompatibilityManifestPublicationRecord> {
        self.records_by_family.get(family_id)
    }

    pub fn records(&self) -> impl Iterator<Item = &CompatibilityManifestPublicationRecord> {
        self.records_by_family.values()
    }

    pub fn recovery_plan(&self) -> &CompatibilityManifestRecoveryPlan {
        &self.recovery_plan
    }

    pub fn frontier(&self) -> &CompatibilityManifestFrontier {
        self.recovery_plan.frontier()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestPublicationWitness {
    publication_unit: CompatibilityManifestPublicationUnit,
}

impl ManifestPublicationWitness {
    pub(crate) fn new(publication_unit: CompatibilityManifestPublicationUnit) -> Self {
        Self { publication_unit }
    }

    pub fn publication_unit(&self) -> &CompatibilityManifestPublicationUnit {
        &self.publication_unit
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestRecoverySummary {
    manifest_count: u64,
    recovered_summary_count: u64,
    publication_gap_count: u64,
}

impl ManifestRecoverySummary {
    pub fn new(
        manifest_count: u64,
        recovered_summary_count: u64,
        publication_gap_count: u64,
    ) -> Self {
        Self {
            manifest_count,
            recovered_summary_count,
            publication_gap_count,
        }
    }

    pub fn manifest_count(&self) -> u64 {
        self.manifest_count
    }

    pub fn recovered_summary_count(&self) -> u64 {
        self.recovered_summary_count
    }

    pub fn publication_gap_count(&self) -> u64 {
        self.publication_gap_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestPublicationGap {
    family_id: ArtifactFamilyId,
}

impl ManifestPublicationGap {
    pub fn new(family_id: ArtifactFamilyId) -> Self {
        Self { family_id }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestDigestMismatch {
    family_id: ArtifactFamilyId,
    expected: CompatibilityManifestDigest,
    observed: CompatibilityManifestDigest,
}

impl ManifestDigestMismatch {
    pub fn new(
        family_id: ArtifactFamilyId,
        expected: CompatibilityManifestDigest,
        observed: CompatibilityManifestDigest,
    ) -> Self {
        Self {
            family_id,
            expected,
            observed,
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn expected(&self) -> &CompatibilityManifestDigest {
        &self.expected
    }

    pub fn observed(&self) -> &CompatibilityManifestDigest {
        &self.observed
    }
}
