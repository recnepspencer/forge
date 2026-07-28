use std::num::NonZeroU64;

use worth_store::physical_runtime::PhysicalResidencyObservation;

use super::PhysicalResidencyStoreWorld;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalResidencyFixtureWorkload {
    Blob,
    Maintenance,
    Recovery,
    Verification,
}

pub fn observed_store_residency(
    label: &str,
    workload: PhysicalResidencyFixtureWorkload,
    allocation_bytes: u64,
) -> PhysicalResidencyObservation {
    let bytes = NonZeroU64::new(allocation_bytes).expect("fixture allocation is non-zero");
    let world = PhysicalResidencyStoreWorld::initialize(label).unwrap();
    let serving = world.serving();
    let allocations = serving.physical_allocations();
    let observation = match workload {
        PhysicalResidencyFixtureWorkload::Blob => {
            let _allocation = allocations.admit_blob(bytes).unwrap();
            serving.residency_observation()
        }
        PhysicalResidencyFixtureWorkload::Maintenance => {
            let _allocation = allocations.admit_maintenance(bytes).unwrap();
            serving.residency_observation()
        }
        PhysicalResidencyFixtureWorkload::Recovery => {
            let _allocation = allocations.admit_recovery(bytes).unwrap();
            serving.residency_observation()
        }
        PhysicalResidencyFixtureWorkload::Verification => {
            let _allocation = allocations.admit_verification(bytes).unwrap();
            serving.residency_observation()
        }
    };
    assert!(!world.close().residency().requires_inspection());
    observation
}
