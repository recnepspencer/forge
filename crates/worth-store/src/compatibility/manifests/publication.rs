use serde::{Deserialize, Serialize};

use sha2::{Digest, Sha256};

use super::super::catalog::CompatibilityFamilyDeclaration;
use worth_store_contracts::CompatibilityAuthorityClassification;

use super::identity::{ArtifactFamilyId, CompatibilityManifestDigest};
use super::recovery::CompatibilityRecoveredManifestIndex;
use super::versions::ArtifactCompatibilityWindow;

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
