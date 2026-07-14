use crate::{
    MinimalManifestVerifierReport, PhysicalReference, PhysicalStoreRuntimeCounterSnapshot,
};
use forge_store_contracts::RoadmapScope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalStoreRuntimeEvidence {
    scope: RoadmapScope,
    counters: PhysicalStoreRuntimeCounterSnapshot,
    verified_references: Vec<PhysicalReference>,
}

impl PhysicalStoreRuntimeEvidence {
    pub(crate) fn from_verifier_report(
        scope: RoadmapScope,
        counters: PhysicalStoreRuntimeCounterSnapshot,
        report: &MinimalManifestVerifierReport,
    ) -> Self {
        Self {
            scope,
            counters,
            verified_references: report.layout().discovered_references().to_vec(),
        }
    }

    pub const fn scope(&self) -> RoadmapScope {
        self.scope
    }

    pub const fn counters(&self) -> PhysicalStoreRuntimeCounterSnapshot {
        self.counters
    }

    pub fn verified_references(&self) -> &[PhysicalReference] {
        &self.verified_references
    }

    pub fn proves_platform_boundary(&self) -> bool {
        self.counters.opens() > 0
            && self.counters.appends() > 0
            && self.counters.root_publications() > 0
            && (self.counters.locates() > 0 || self.counters.reads() > 0)
            && self.counters.scans() > 0
            && !self.verified_references.is_empty()
    }
}
