use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

use super::catalog::{CompatibilityAuthorityClassification, CompatibilityFamilyDeclaration};
use worth_store_contracts::ArtifactFamilyId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ArtifactFormatVersion(u32);

impl ArtifactFormatVersion {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ArtifactSemanticVersion(u32);

impl ArtifactSemanticVersion {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArtifactCompatibilityWindow {
    minimum_format: ArtifactFormatVersion,
    maximum_format: ArtifactFormatVersion,
    minimum_semantic: ArtifactSemanticVersion,
    maximum_semantic: ArtifactSemanticVersion,
}

impl ArtifactCompatibilityWindow {
    pub fn new(
        minimum_format: ArtifactFormatVersion,
        maximum_format: ArtifactFormatVersion,
        minimum_semantic: ArtifactSemanticVersion,
        maximum_semantic: ArtifactSemanticVersion,
    ) -> Self {
        Self {
            minimum_format,
            maximum_format,
            minimum_semantic,
            maximum_semantic,
        }
    }

    pub fn native(version: u32) -> Self {
        Self::new(
            ArtifactFormatVersion::new(version),
            ArtifactFormatVersion::new(version),
            ArtifactSemanticVersion::new(version),
            ArtifactSemanticVersion::new(version),
        )
    }

    pub fn minimum_format(&self) -> ArtifactFormatVersion {
        self.minimum_format
    }

    pub fn maximum_format(&self) -> ArtifactFormatVersion {
        self.maximum_format
    }

    pub fn minimum_semantic(&self) -> ArtifactSemanticVersion {
        self.minimum_semantic
    }

    pub fn maximum_semantic(&self) -> ArtifactSemanticVersion {
        self.maximum_semantic
    }
}

impl ArtifactCompatibilityWindow {
    pub fn contains_format(&self, version: ArtifactFormatVersion) -> bool {
        self.minimum_format <= version && version <= self.maximum_format
    }

    pub fn contains_semantic(&self, version: ArtifactSemanticVersion) -> bool {
        self.minimum_semantic <= version && version <= self.maximum_semantic
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CompatibilityManifestDigest(String);

impl CompatibilityManifestDigest {
    pub(crate) fn compute(
        family_id: &ArtifactFamilyId,
        window: &ArtifactCompatibilityWindow,
        authority_label: &str,
    ) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(family_id.as_str().as_bytes());
        hasher.update(authority_label.as_bytes());
        hasher.update(window.minimum_format().value().to_le_bytes());
        hasher.update(window.maximum_format().value().to_le_bytes());
        hasher.update(window.minimum_semantic().value().to_le_bytes());
        hasher.update(window.maximum_semantic().value().to_le_bytes());
        Self(format!("{:x}", hasher.finalize()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthoritativeCompatibilityManifest {
    family_id: ArtifactFamilyId,
    window: ArtifactCompatibilityWindow,
    digest: CompatibilityManifestDigest,
}

impl AuthoritativeCompatibilityManifest {
    pub(crate) fn new(family_id: ArtifactFamilyId, window: ArtifactCompatibilityWindow) -> Self {
        let digest = CompatibilityManifestDigest::compute(&family_id, &window, "authoritative");
        Self {
            family_id,
            window,
            digest,
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn window(&self) -> &ArtifactCompatibilityWindow {
        &self.window
    }

    pub fn digest(&self) -> &CompatibilityManifestDigest {
        &self.digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedCompatibilityManifest {
    family_id: ArtifactFamilyId,
    window: ArtifactCompatibilityWindow,
    digest: CompatibilityManifestDigest,
}

impl DerivedCompatibilityManifest {
    pub(crate) fn new(family_id: ArtifactFamilyId, window: ArtifactCompatibilityWindow) -> Self {
        let digest = CompatibilityManifestDigest::compute(&family_id, &window, "derived");
        Self {
            family_id,
            window,
            digest,
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn window(&self) -> &ArtifactCompatibilityWindow {
        &self.window
    }

    pub fn digest(&self) -> &CompatibilityManifestDigest {
        &self.digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityManifestPublicationUnit {
    family_id: ArtifactFamilyId,
    manifest_digest: CompatibilityManifestDigest,
}

impl CompatibilityManifestPublicationUnit {
    pub(crate) fn new(
        family_id: ArtifactFamilyId,
        manifest_digest: CompatibilityManifestDigest,
    ) -> Self {
        Self {
            family_id,
            manifest_digest,
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn manifest_digest(&self) -> &CompatibilityManifestDigest {
        &self.manifest_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityManifestPublicationRecord {
    family_id: ArtifactFamilyId,
    window: ArtifactCompatibilityWindow,
    authority_classification: CompatibilityAuthorityClassification,
    manifest_digest: CompatibilityManifestDigest,
    publication_sequence: u64,
}

impl CompatibilityManifestPublicationRecord {
    pub(crate) fn from_declaration(
        declaration: &CompatibilityFamilyDeclaration,
        publication_sequence: u64,
    ) -> Self {
        let manifest = declaration.manifest();
        Self {
            family_id: manifest.family_id().clone(),
            window: manifest.window().clone(),
            authority_classification: declaration.authority_classification(),
            manifest_digest: manifest.digest().clone(),
            publication_sequence,
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn window(&self) -> &ArtifactCompatibilityWindow {
        &self.window
    }

    pub fn authority_classification(&self) -> CompatibilityAuthorityClassification {
        self.authority_classification
    }

    pub fn manifest_digest(&self) -> &CompatibilityManifestDigest {
        &self.manifest_digest
    }

    pub fn publication_sequence(&self) -> u64 {
        self.publication_sequence
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityManifestFrontier {
    publication_count: u64,
    frontier_identity: String,
}

impl CompatibilityManifestFrontier {
    fn from_records(records: &[CompatibilityManifestPublicationRecord]) -> Self {
        let mut hasher = Sha256::new();
        for record in records {
            hasher.update(record.family_id().as_str().as_bytes());
            hasher.update(record.manifest_digest().as_str().as_bytes());
            hasher.update(record.publication_sequence().to_le_bytes());
        }
        Self {
            publication_count: records.len() as u64,
            frontier_identity: format!("{:x}", hasher.finalize()),
        }
    }

    pub fn publication_count(&self) -> u64 {
        self.publication_count
    }

    pub fn identity(&self) -> &str {
        &self.frontier_identity
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityManifestPublicationReceipt {
    record: CompatibilityManifestPublicationRecord,
    frontier: CompatibilityManifestFrontier,
}

impl CompatibilityManifestPublicationReceipt {
    pub(crate) fn new(
        record: CompatibilityManifestPublicationRecord,
        frontier: CompatibilityManifestFrontier,
    ) -> Self {
        Self { record, frontier }
    }

    pub fn record(&self) -> &CompatibilityManifestPublicationRecord {
        &self.record
    }

    pub fn frontier(&self) -> &CompatibilityManifestFrontier {
        &self.frontier
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityManifestPublicationLedger {
    records: Vec<CompatibilityManifestPublicationRecord>,
}

impl CompatibilityManifestPublicationLedger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn publish_declaration(
        &mut self,
        declaration: &CompatibilityFamilyDeclaration,
    ) -> CompatibilityManifestPublicationReceipt {
        let sequence = self.records.len() as u64 + 1;
        let record =
            CompatibilityManifestPublicationRecord::from_declaration(declaration, sequence);
        self.records.push(record.clone());
        CompatibilityManifestPublicationReceipt::new(record, self.frontier())
    }

    pub fn records(&self) -> &[CompatibilityManifestPublicationRecord] {
        &self.records
    }

    pub fn frontier(&self) -> CompatibilityManifestFrontier {
        CompatibilityManifestFrontier::from_records(&self.records)
    }

    pub fn recover(&self) -> CompatibilityRecoveredManifestIndex {
        CompatibilityRecoveredManifestIndex::new(self.records.clone(), self.frontier())
    }

    pub(crate) fn from_records(records: Vec<CompatibilityManifestPublicationRecord>) -> Self {
        Self { records }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_digest_identity_is_deterministic() {
        let family = ArtifactFamilyId::new("canonical_commit_envelope");
        let window = ArtifactCompatibilityWindow::native(1);
        let left = CompatibilityManifestDigest::compute(&family, &window, "authoritative");
        let right = CompatibilityManifestDigest::compute(&family, &window, "authoritative");
        assert_eq!(left, right);
    }
}
