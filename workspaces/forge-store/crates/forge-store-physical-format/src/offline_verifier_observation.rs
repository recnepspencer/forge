use crate::{ManifestTraversalReport, MinimalManifestVerifierReport, PhysicalReference};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineVerifierObservationSource {
    PersistedPhysicalBytes,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflineVerifierLayoutObservation {
    source: OfflineVerifierObservationSource,
    discovered_references: Vec<PhysicalReference>,
    traversal: ManifestTraversalReport,
    semantic_decode_attempts: u32,
}

impl OfflineVerifierLayoutObservation {
    pub fn from_verifier_report(report: &MinimalManifestVerifierReport) -> Self {
        Self {
            source: OfflineVerifierObservationSource::PersistedPhysicalBytes,
            discovered_references: report.layout().discovered_references().to_vec(),
            traversal: report.traversal().clone(),
            semantic_decode_attempts: report.semantic_decode_attempts(),
        }
    }

    pub const fn source(&self) -> OfflineVerifierObservationSource {
        self.source
    }

    pub fn discovered_references(&self) -> &[PhysicalReference] {
        &self.discovered_references
    }

    pub const fn traversal(&self) -> &ManifestTraversalReport {
        &self.traversal
    }

    pub const fn semantic_decode_attempts(&self) -> u32 {
        self.semantic_decode_attempts
    }
}
