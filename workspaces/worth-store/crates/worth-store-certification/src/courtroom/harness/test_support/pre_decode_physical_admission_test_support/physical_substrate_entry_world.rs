use std::num::NonZeroU64;

use super::{
    checksum_fixture::checksum_declaration,
    physical_substrate_witness_world::{current_frame_bytes, current_validation, frame_witness},
};
use worth_store_physical_format::{
    PhysicalHeaderDecodeWitness, PhysicalReferenceValidationWitness,
};
use worth_store_physical_integrity::{
    IntegrityEntryAdmission, IntegrityEntryRequest, PhysicalIntegrityAdmission,
    PhysicalIntegrityAdmissionSeed, ProtectedPhysicalByteView,
};
use worth_store_test_support::harness::physical_residency::{
    PhysicalResidencyStoreWorld, SUCCESSOR_SCOPE_ALLOCATION_BYTES,
};

pub(crate) fn with_pre_decode_admission(
    payload: &[u8],
    run: impl FnOnce(
        PhysicalIntegrityAdmission<'_, '_>,
        PhysicalReferenceValidationWitness,
        PhysicalHeaderDecodeWitness,
    ),
) {
    let protected_bytes = current_frame_bytes(payload);
    with_entry_seed(&protected_bytes, |seed| {
        let admission = pre_decode_admission_from_seed(seed);
        run(admission, current_validation(), frame_witness(payload));
    });
}

pub(crate) fn with_entry_seed(
    protected_bytes: &[u8],
    run: impl FnOnce(PhysicalIntegrityAdmissionSeed<'_, '_>),
) {
    let world = PhysicalResidencyStoreWorld::initialize("physical-integrity-store-entry").unwrap();
    world
        .with_record_chunk(protected_bytes, |serving, chunk| {
            let verification = serving
                .physical_allocations()
                .admit_verification(
                    NonZeroU64::new(SUCCESSOR_SCOPE_ALLOCATION_BYTES)
                        .expect("fixture allocation is nonzero"),
                )
                .expect("real Store verification allocation admits");
            let protected = ProtectedPhysicalByteView::from_store_chunk(&chunk);
            let request = IntegrityEntryRequest::new(protected, verification);
            let lease = IntegrityEntryAdmission::admit(request)
                .expect("real Store view and verification authority admit");
            run(PhysicalIntegrityAdmission::from_entry(lease));
        })
        .expect("real Store record chunk world");
    assert!(!world.close().residency().requires_inspection());
}

fn pre_decode_admission_from_seed<'runtime, 'lease>(
    seed: PhysicalIntegrityAdmissionSeed<'runtime, 'lease>,
) -> PhysicalIntegrityAdmission<'runtime, 'lease> {
    let entry_witness = seed.entry_witness();
    seed.with_checksum_declaration(
        checksum_declaration().admit_for_physical_integrity_entry(entry_witness),
    )
    .unwrap()
}
