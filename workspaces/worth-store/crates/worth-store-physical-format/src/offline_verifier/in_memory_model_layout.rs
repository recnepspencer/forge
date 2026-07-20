use crate::{
    InMemoryPhysicalFormatModelCounterSnapshot, ManifestTraversalReport, PhysicalReference,
    PlatformPhysicalScanReport,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InMemoryModelLayoutObservationSource {
    InMemoryModelScan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InMemoryModelLayoutObservation {
    source: InMemoryModelLayoutObservationSource,
    discovered_references: Vec<PhysicalReference>,
    traversal: ManifestTraversalReport,
    semantic_decode_attempts: u32,
    counters: InMemoryPhysicalFormatModelCounterSnapshot,
}

impl InMemoryModelLayoutObservation {
    pub fn from_model_scan(report: &PlatformPhysicalScanReport) -> Self {
        let model_report = report.model_report();
        Self {
            source: InMemoryModelLayoutObservationSource::InMemoryModelScan,
            discovered_references: model_report.discovered_references().to_vec(),
            traversal: model_report.traversal().clone(),
            semantic_decode_attempts: model_report.semantic_decode_attempts(),
            counters: report.counters(),
        }
    }

    pub const fn source(&self) -> InMemoryModelLayoutObservationSource {
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

    pub const fn counters(&self) -> InMemoryPhysicalFormatModelCounterSnapshot {
        self.counters
    }
}
