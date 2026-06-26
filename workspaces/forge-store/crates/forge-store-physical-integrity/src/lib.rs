#![forbid(unsafe_code)]

mod scrub_planning_memory_envelope;

use forge_store_physical_format::PhysicalReference;
pub use scrub_planning_memory_envelope::{
    ScrubPlanningMemoryEnvelope, ScrubPlanningMemoryEnvelopeDenial,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalQuarantine {
    damaged_reference: PhysicalReference,
}

impl PhysicalQuarantine {
    pub const fn new(damaged_reference: PhysicalReference) -> Self {
        Self { damaged_reference }
    }

    pub const fn damaged_reference(&self) -> PhysicalReference {
        self.damaged_reference
    }
}
