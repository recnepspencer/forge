use std::num::{NonZeroU32, NonZeroU64};

use worth_store_buffer_pool::{
    PhysicalFrameAccess, PhysicalFrameKey, PhysicalOperationAllocationScope,
    PhysicalResidencyLimits, PhysicalResidencyPool, PhysicalSpeculativeWorkKind,
};
use worth_store_physical_format::store_namespace::{
    ProposedStoreIdentity, StableStoreIdentity, StoreNamespaceIdentityRecord, StoreNamespaceVersion,
};
use worth_store_physical_format::{
    DurablePhysicalRootManifest, FreeSpaceBlockReference, FreeSpaceKey, PhysicalPageSizeClass,
    PhysicalRecordFormatDeclaration, RecordAllocationClass, RecordArtifactFile,
    RecordFrameCoordinate,
};

use super::root_manifest::admit_loaded_root_manifest;
use crate::physical_runtime::{
    LifecycleGeneration, ResidentAdmissionCounterCells, RootProtocolAdmissionDenial,
};

#[test]
fn exact_same_generation_hit_reuses_record_without_fresh_validation() {
    let store = store(71);
    let format = format();
    let bytes = manifest_bytes(7, format);
    let (pool, allocation, lease) = loaded_manifest(store, 7, &bytes);
    let lifecycle = generation(11);
    let counters = ResidentAdmissionCounterCells::default();

    let first = admit_loaded_root_manifest(&lease, lifecycle, store, format, 7, &counters).unwrap();
    assert_eq!(first.project(lifecycle, &counters).unwrap().generation(), 7);
    drop(lease);
    let hit = match pool
        .access_frame(
            &allocation,
            PhysicalFrameKey::new(store, manifest_coordinate(7, bytes.len())),
        )
        .unwrap()
    {
        PhysicalFrameAccess::Hit(lease) => lease,
        _ => panic!("the unchanged frame must remain a resident hit"),
    };
    let second = admit_loaded_root_manifest(&hit, lifecycle, store, format, 7, &counters).unwrap();
    let view = second.enter_owner_decoder(lifecycle, &counters).unwrap();
    assert_eq!(view.bytes(), bytes.as_slice());
    assert_eq!(view.scope().root_generation(), Some(7));

    let observed = counters.snapshot();
    assert_eq!(observed.fresh_validations(), 1);
    assert_eq!(observed.exact_record_reuses(), 1);
    assert_eq!(observed.owner_decoder_entries(), 2);
    assert_eq!(observed.rejections_before_decoder(), 0);
}

#[test]
fn invalidation_forces_rehash_and_stale_admission_cannot_enter_decoder() {
    let store = store(72);
    let format = format();
    let bytes = manifest_bytes(8, format);
    let (pool, _allocation, lease) = loaded_manifest(store, 8, &bytes);
    let lifecycle = generation(12);
    let counters = ResidentAdmissionCounterCells::default();

    let first = admit_loaded_root_manifest(&lease, lifecycle, store, format, 8, &counters).unwrap();
    first.project(lifecycle, &counters).unwrap();
    let stale = admit_loaded_root_manifest(&lease, lifecycle, store, format, 8, &counters).unwrap();
    pool.invalidate_integrity_validation_for_runtime_transition();
    let stale_denial = match stale.enter_owner_decoder(lifecycle, &counters) {
        Ok(_) => panic!("an invalidated retained record must not open the owner decoder"),
        Err(denial) => denial,
    };
    assert_eq!(stale_denial, RootProtocolAdmissionDenial::ResidentFrame);

    let readmitted =
        admit_loaded_root_manifest(&lease, lifecycle, store, format, 8, &counters).unwrap();
    readmitted.project(lifecycle, &counters).unwrap();
    let observed = counters.snapshot();
    assert_eq!(observed.fresh_validations(), 2);
    assert_eq!(observed.exact_record_reuses(), 1);
    assert_eq!(observed.rejections_before_decoder(), 1);
    assert_eq!(observed.owner_decoder_entries(), 2);
}

#[test]
fn artifact_and_lifecycle_substitution_reject_before_decoder_entry() {
    let store = store(73);
    let format = format();
    let bytes = manifest_bytes(9, format);
    let (_pool, _allocation, lease) = loaded_manifest(store, 9, &bytes);
    let lifecycle = generation(13);
    let counters = ResidentAdmissionCounterCells::default();

    let denial = match admit_loaded_root_manifest(&lease, lifecycle, store, format, 10, &counters) {
        Ok(_) => panic!("an artifact-generation substitution must be rejected"),
        Err(denial) => denial,
    };
    assert_eq!(denial, RootProtocolAdmissionDenial::SourceArtifactMismatch);
    let admitted =
        admit_loaded_root_manifest(&lease, lifecycle, store, format, 9, &counters).unwrap();
    let lifecycle_denial = match admitted.enter_owner_decoder(generation(14), &counters) {
        Ok(_) => panic!("a lifecycle substitution must not open the owner decoder"),
        Err(denial) => denial,
    };
    assert_eq!(lifecycle_denial, RootProtocolAdmissionDenial::ResidentFrame);

    let observed = counters.snapshot();
    assert_eq!(observed.fresh_validations(), 1);
    assert_eq!(observed.rejections_before_decoder(), 2);
    assert_eq!(observed.owner_decoder_entries(), 0);
}

#[test]
fn corrupt_fresh_bytes_reject_before_any_owner_decoder_entry() {
    let store = store(74);
    let format = format();
    let mut bytes = manifest_bytes(10, format);
    let last = bytes.len() - 1;
    bytes[last] ^= 0x80;
    let (_pool, _allocation, lease) = loaded_manifest(store, 10, &bytes);
    let counters = ResidentAdmissionCounterCells::default();

    assert!(
        admit_loaded_root_manifest(&lease, generation(15), store, format, 10, &counters,).is_err()
    );

    let observed = counters.snapshot();
    assert_eq!(observed.fresh_validations(), 1);
    assert_eq!(observed.exact_record_reuses(), 0);
    assert_eq!(observed.rejections_before_decoder(), 1);
    assert_eq!(observed.owner_decoder_entries(), 0);
}

fn loaded_manifest(
    store: StableStoreIdentity,
    generation: u64,
    bytes: &[u8],
) -> (
    PhysicalResidencyPool,
    worth_store_buffer_pool::OperationAllocationGrant,
    worth_store_buffer_pool::PhysicalFrameLease,
) {
    let pool = PhysicalResidencyPool::open(store, residency_limits(bytes.len() as u32)).unwrap();
    let allocation = pool
        .begin_operation(
            PhysicalOperationAllocationScope::ForegroundRead,
            NonZeroU64::new(bytes.len() as u64).unwrap(),
        )
        .unwrap();
    let key = PhysicalFrameKey::new(store, manifest_coordinate(generation, bytes.len()));
    let PhysicalFrameAccess::Fault(fault) = pool.access_frame(&allocation, key).unwrap() else {
        panic!("fresh pool must yield a frame fault");
    };
    let lease = fault
        .load(|target| {
            target.copy_from_slice(bytes);
            Ok::<(), ()>(())
        })
        .unwrap();
    (pool, allocation, lease)
}

fn manifest_coordinate(generation: u64, length: usize) -> RecordFrameCoordinate {
    RecordFrameCoordinate::new(
        RecordArtifactFile::RootManifest { generation },
        0,
        length as u32,
    )
    .unwrap()
}

fn residency_limits(frame_bytes: u32) -> PhysicalResidencyLimits {
    let bytes = NonZeroU64::new(u64::from(frame_bytes)).unwrap();
    let count = NonZeroU32::new(2).unwrap();
    let mut builder = PhysicalResidencyLimits::builder()
        .total_bytes(NonZeroU64::new(u64::from(frame_bytes) * 3 + 4096).unwrap())
        .resident_bytes(bytes)
        .metadata_bytes(NonZeroU64::new(4096).unwrap())
        .frame_entries(count)
        .pinned_frames(count)
        .pin_leases(count)
        .dirty_frames(count)
        .dirty_replacement_bytes(bytes)
        .operation_bytes(bytes);
    for scope in [
        PhysicalOperationAllocationScope::ForegroundRead,
        PhysicalOperationAllocationScope::ForegroundWrite,
        PhysicalOperationAllocationScope::Recovery,
        PhysicalOperationAllocationScope::Scrub,
        PhysicalOperationAllocationScope::Maintenance,
        PhysicalOperationAllocationScope::Verification,
        PhysicalOperationAllocationScope::Blob,
    ] {
        builder = builder.scope_bytes(scope, bytes);
    }
    for kind in [
        PhysicalSpeculativeWorkKind::Prefetch,
        PhysicalSpeculativeWorkKind::ReadAhead,
        PhysicalSpeculativeWorkKind::WriteBehind,
    ] {
        builder = builder.speculative_frames(kind, count);
    }
    builder.admit(NonZeroU64::MIN).unwrap()
}

fn store(byte: u8) -> StableStoreIdentity {
    StoreNamespaceIdentityRecord::new(
        StoreNamespaceVersion::CURRENT,
        ProposedStoreIdentity::from_nonzero_bytes([byte; 16]).unwrap(),
    )
    .published_identity()
}

fn format() -> PhysicalRecordFormatDeclaration {
    PhysicalRecordFormatDeclaration::builder()
        .page_size(PhysicalPageSizeClass::KiB16)
        .admit()
        .unwrap()
}

fn manifest_bytes(generation: u64, format: PhysicalRecordFormatDeclaration) -> Vec<u8> {
    let key = FreeSpaceKey::new(RecordAllocationClass::InlinePage, 1).unwrap();
    let free_space_root = FreeSpaceBlockReference::new(generation, 1, 0, 41, key, key).unwrap();
    DurablePhysicalRootManifest::builder(generation, 71, 2, 43)
        .free_space_root(Some(free_space_root))
        .admit()
        .unwrap()
        .encode(format)
}

fn generation(value: u64) -> LifecycleGeneration {
    LifecycleGeneration::from_reopened(NonZeroU64::new(value).unwrap())
}
