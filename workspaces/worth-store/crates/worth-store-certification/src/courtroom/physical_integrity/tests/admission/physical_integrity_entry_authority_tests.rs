use std::num::NonZeroU64;

use worth_store::physical_runtime::PhysicalOperationAllocationScope;
use worth_store_physical_integrity::{
    IntegrityEntryAdmission, IntegrityEntryDenialKind, IntegrityEntryRequest,
    ProtectedPhysicalByteView,
};
use worth_store_test_support::harness::physical_residency::{
    PhysicalResidencyStoreWorld, SUCCESSOR_SCOPE_ALLOCATION_BYTES,
};

#[test]
fn entry_basis_is_derived_from_the_store_chunk_and_exact_verification_allocation() {
    let world = PhysicalResidencyStoreWorld::initialize("integrity-entry-authority").unwrap();
    world
        .with_record_chunk(b"store-bound-integrity-entry", |serving, chunk| {
            let chunk_basis = chunk.basis();
            let allocation = serving
                .physical_allocations()
                .admit_verification(allocation_bytes())
                .unwrap();
            let runtime = allocation.runtime_identity();
            let protected = ProtectedPhysicalByteView::from_store_chunk(&chunk);
            let lease = IntegrityEntryAdmission::admit(IntegrityEntryRequest::new(
                protected,
                allocation,
            ))
            .unwrap();
            let basis = lease.entry_witness().entry_basis();

            assert_eq!(basis.store_identity(), chunk_basis.store_identity());
            assert_eq!(basis.store_generation(), chunk_basis.store_generation());
            assert_eq!(basis.record(), chunk_basis.record());
            assert_eq!(basis.frame_coordinate(), chunk_basis.frame_coordinate());
            assert_eq!(basis.verification_runtime(), runtime);
            assert_eq!(
                basis.verification_bytes(),
                SUCCESSOR_SCOPE_ALLOCATION_BYTES
            );
            assert_eq!(lease.protected_bytes().as_bytes(), b"store-bound-integrity-entry");
            assert_eq!(
                serving
                    .residency_observation()
                    .counters()
                    .active_operation_bytes_for(PhysicalOperationAllocationScope::Verification),
                SUCCESSOR_SCOPE_ALLOCATION_BYTES,
            );

            drop(lease);
            assert_eq!(
                serving
                    .residency_observation()
                    .counters()
                    .active_operation_bytes_for(PhysicalOperationAllocationScope::Verification),
                0,
            );
        })
        .unwrap();
    assert!(!world.close().residency().requires_inspection());
}

#[test]
fn verification_allocation_from_another_store_is_denied_before_witness_minting() {
    let viewed = PhysicalResidencyStoreWorld::initialize("integrity-viewed-store").unwrap();
    let allocating =
        PhysicalResidencyStoreWorld::initialize("integrity-allocation-store").unwrap();

    viewed
        .with_record_chunk(b"viewed-store-bytes", |_viewed_runtime, chunk| {
            allocating
                .with_record_chunk(b"allocation-store-bytes", |allocating_runtime, _| {
                    let allocation = allocating_runtime
                        .physical_allocations()
                        .admit_verification(allocation_bytes())
                        .unwrap();
                    let denial = IntegrityEntryAdmission::admit(IntegrityEntryRequest::new(
                        ProtectedPhysicalByteView::from_store_chunk(&chunk),
                        allocation,
                    ))
                    .unwrap_err();

                    assert_eq!(
                        denial.kind(),
                        IntegrityEntryDenialKind::VerificationStoreMismatch
                    );
                    assert_eq!(
                        allocating_runtime
                            .residency_observation()
                            .counters()
                            .active_operation_bytes_for(
                                PhysicalOperationAllocationScope::Verification
                            ),
                        0,
                    );
                })
                .unwrap();
        })
        .unwrap();

    assert!(!viewed.close().residency().requires_inspection());
    assert!(!allocating.close().residency().requires_inspection());
}

fn allocation_bytes() -> NonZeroU64 {
    NonZeroU64::new(SUCCESSOR_SCOPE_ALLOCATION_BYTES).unwrap()
}
