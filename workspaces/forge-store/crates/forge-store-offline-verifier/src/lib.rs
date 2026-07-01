#![forbid(unsafe_code)]

mod boundary;

use forge_store_physical_format::PhysicalReference;

pub use boundary::OfflineVerifierBoundarySeam;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflineLayoutReport {
    discovered_records: Vec<PhysicalReference>,
}

impl OfflineLayoutReport {
    pub fn new(discovered_records: Vec<PhysicalReference>) -> Self {
        Self { discovered_records }
    }

    pub fn discovered_records(&self) -> &[PhysicalReference] {
        &self.discovered_records
    }
}
