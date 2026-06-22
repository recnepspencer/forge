#![forbid(unsafe_code)]

use forge_store_physical_format::PhysicalReference;

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
