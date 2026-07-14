use crate::{
    ManifestTraversalReport, PhysicalReference, PhysicalStoreRuntimeCounterSnapshot,
    PlatformPhysicalScanReport,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeLayoutObservationSource {
    PlatformFacadeScan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeLayoutObservation {
    source: RuntimeLayoutObservationSource,
    discovered_references: Vec<PhysicalReference>,
    traversal: ManifestTraversalReport,
    semantic_decode_attempts: u32,
    counters: PhysicalStoreRuntimeCounterSnapshot,
}

impl RuntimeLayoutObservation {
    pub fn from_facade_scan(report: &PlatformPhysicalScanReport) -> Self {
        let runtime_report = report.runtime_report();
        Self {
            source: RuntimeLayoutObservationSource::PlatformFacadeScan,
            discovered_references: runtime_report.discovered_references().to_vec(),
            traversal: runtime_report.traversal().clone(),
            semantic_decode_attempts: runtime_report.semantic_decode_attempts(),
            counters: report.counters(),
        }
    }

    pub const fn source(&self) -> RuntimeLayoutObservationSource {
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

    pub const fn counters(&self) -> PhysicalStoreRuntimeCounterSnapshot {
        self.counters
    }
}
