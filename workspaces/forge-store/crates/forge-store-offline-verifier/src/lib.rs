#![forbid(unsafe_code)]

mod boundary;
mod custody_capsule_observation;
#[cfg(test)]
mod custody_capsule_observation_tests;
mod repair_blast_radius_observation;
#[cfg(test)]
mod repair_blast_radius_observation_tests;

use forge_store_physical_format::PhysicalReference;

pub use boundary::OfflineVerifierBoundarySeam;
pub use custody_capsule_observation::{
    OfflineCustodyCapsuleObservation, OfflineCustodyCapsuleObservationDenial,
};
pub use repair_blast_radius_observation::{
    OfflineRepairBlastRadiusObservation, OfflineRepairBlastRadiusObservationDenial,
    OfflineRepairEvidenceKind,
};

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
