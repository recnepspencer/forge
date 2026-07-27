use std::num::NonZeroU64;

use worth_store::physical_runtime::{
    BlobPhysicalAllocation, MaintenancePhysicalAllocation, PhysicalScopedAllocationAdmission,
    RecoveryPhysicalAllocation, ScrubPhysicalAllocation, VerificationPhysicalAllocation,
};

fn admit_exact_successor_scopes(
    allocations: &PhysicalScopedAllocationAdmission<'_>,
    bytes: NonZeroU64,
) {
    let _: RecoveryPhysicalAllocation = allocations.admit_recovery(bytes).unwrap();
    let _: ScrubPhysicalAllocation = allocations.admit_scrub(bytes).unwrap();
    let _: MaintenancePhysicalAllocation = allocations.admit_maintenance(bytes).unwrap();
    let _: VerificationPhysicalAllocation = allocations.admit_verification(bytes).unwrap();
    let _: BlobPhysicalAllocation = allocations.admit_blob(bytes).unwrap();
}

fn main() {}
