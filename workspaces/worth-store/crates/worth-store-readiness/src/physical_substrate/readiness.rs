use super::PhysicalSubstrateReadinessFacts;
use worth_store_contracts::{PhysicalSubstrateReadinessSnapshot, RoadmapScope};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalSubstrateReadiness {
    scope: RoadmapScope,
    facts: PhysicalSubstrateReadinessFacts,
    sealed: bool,
}

impl PhysicalSubstrateReadiness {
    pub const fn scope(&self) -> RoadmapScope {
        self.scope
    }

    pub const fn facts(&self) -> PhysicalSubstrateReadinessFacts {
        self.facts
    }

    pub const fn is_sealed(&self) -> bool {
        self.sealed
    }

    pub const fn physical_substrate_snapshot(&self) -> PhysicalSubstrateReadinessSnapshot {
        PhysicalSubstrateReadinessSnapshot::from_exact_counts(
            self.sealed,
            self.facts.physical_reference_count(),
            self.facts.header_decode_witness_count(),
            self.facts.payload_admission_witness_count(),
            self.facts.manifest_layout_evidence_count(),
            self.facts.no_materialization_witness_count(),
            self.facts.counter_evidence_count(),
        )
    }
}
