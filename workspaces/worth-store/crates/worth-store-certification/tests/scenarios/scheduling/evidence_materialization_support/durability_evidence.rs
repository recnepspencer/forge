use worth_store_certification::S6FlushDurabilityEvidenceRow;
use worth_store_test_support::harness::physical_residency::{
    canonical_physical_mutation_acknowledgment, PhysicalResidencyStoreWorld,
};

pub(super) fn flush_row() -> S6FlushDurabilityEvidenceRow {
    let world = PhysicalResidencyStoreWorld::initialize("s6-materialized-flush-evidence").unwrap();
    let acknowledgment = canonical_physical_mutation_acknowledgment(
        &world,
        [64; 32],
        b"materialized-flush-evidence",
    );
    let row = S6FlushDurabilityEvidenceRow::from_physical_acknowledgment(&acknowledgment);
    world.close();
    row
}
