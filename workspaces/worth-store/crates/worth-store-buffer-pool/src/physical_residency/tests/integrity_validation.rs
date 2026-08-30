use super::*;
use worth_store_physical_integrity::{
    PhysicalArtifactScope, PhysicalByteRange, PhysicalIntegrityValidationDigest,
    PhysicalIntegrityValidationMechanism, PhysicalIntegrityValidationRecord,
};

fn validation(
    identity: worth_store_physical_format::store_namespace::StableStoreIdentity,
    length: u64,
    seed: u32,
) -> PhysicalIntegrityValidationRecord {
    PhysicalIntegrityValidationRecord::for_test(
        PhysicalArtifactScope::current_root_selector(
            identity,
            PhysicalByteRange::new(0, length).unwrap(),
        ),
        PhysicalIntegrityValidationDigest::crc32c(seed),
        PhysicalIntegrityValidationDigest::crc32c(seed.wrapping_add(1)),
        PhysicalIntegrityValidationMechanism::Crc32cV1,
    )
}

#[test]
fn resident_hits_retain_one_byte_image_generation() {
    let identity = store(201);
    let pool = PhysicalResidencyPool::open(identity, limits(32, 2, 1, 32, 2)).unwrap();
    let read = allocation(&pool, READ_SCOPE);
    let key = PhysicalFrameKey::new(identity, coordinate(1, 8));

    let loaded = expect_fault(&pool, &read, key)
        .load(|bytes| fill(bytes, 7))
        .unwrap();
    let generation = loaded.resident_generation();
    let validation = validation(identity, 8, 10);
    loaded.commit_integrity_validation(validation).unwrap();
    let hit = expect_hit(&pool, &read, key);

    assert_eq!(hit.resident_generation(), generation);
    assert_eq!(hit.integrity_validation(), Some(validation));
}

#[test]
fn dirty_replacement_installs_a_new_byte_image_generation() {
    let identity = store(202);
    let pool = PhysicalResidencyPool::open(identity, limits(32, 2, 1, 32, 2)).unwrap();
    let write = pool
        .begin_foreground_write_operation(nonzero_bytes(8))
        .unwrap();
    let key = PhysicalFrameKey::new(identity, coordinate(1, 8));
    let clean = expect_fault(&pool, &write, key)
        .load(|bytes| fill(bytes, 3))
        .unwrap();
    let clean_generation = clean.resident_generation();
    let validation = validation(identity, 8, 20);
    clean.commit_integrity_validation(validation).unwrap();
    assert_eq!(clean.integrity_validation(), Some(validation));

    let dirty = clean
        .begin_dirty_replacement(&write)
        .unwrap()
        .replace(|_, bytes| fill(bytes, 4))
        .unwrap();

    assert!(dirty.resident_generation().get() > clean_generation.get());
    assert_eq!(
        dirty
            .lease
            .as_ref()
            .expect("dirty frame retains its exact lease")
            .integrity_validation(),
        None
    );
}

#[test]
fn eviction_and_reload_cannot_reuse_a_byte_image_generation() {
    let identity = store(203);
    let pool = PhysicalResidencyPool::open(identity, limits(8, 1, 1, 24, 1)).unwrap();
    let read = allocation(&pool, READ_SCOPE);
    let first_key = PhysicalFrameKey::new(identity, coordinate(1, 8));
    let second_key = PhysicalFrameKey::new(identity, coordinate(2, 8));

    let first = expect_fault(&pool, &read, first_key)
        .load(|bytes| fill(bytes, 1))
        .unwrap();
    let first_generation = first.resident_generation();
    let validation = validation(identity, 8, 30);
    first.commit_integrity_validation(validation).unwrap();
    assert_eq!(first.integrity_validation(), Some(validation));
    drop(first);
    let second = expect_fault(&pool, &read, second_key)
        .load(|bytes| fill(bytes, 2))
        .unwrap();
    assert_eq!(second.integrity_validation(), None);
    drop(second);
    let reloaded = expect_fault(&pool, &read, first_key)
        .load(|bytes| fill(bytes, 1))
        .unwrap();

    assert!(reloaded.resident_generation().get() > first_generation.get());
    assert_eq!(reloaded.integrity_validation(), None);
}

#[test]
fn runtime_invalidation_preserves_the_live_byte_image_generation() {
    let identity = store(204);
    let pool = PhysicalResidencyPool::open(identity, limits(16, 1, 1, 16, 1)).unwrap();
    let read = allocation(&pool, READ_SCOPE);
    let key = PhysicalFrameKey::new(identity, coordinate(1, 8));
    let loaded = expect_fault(&pool, &read, key)
        .load(|bytes| fill(bytes, 9))
        .unwrap();
    let generation = loaded.resident_generation();
    let validation = validation(identity, 8, 0x89ab_cdef);
    loaded.commit_integrity_validation(validation).unwrap();
    assert_eq!(loaded.integrity_validation(), Some(validation));

    pool.invalidate_integrity_validation_for_runtime_transition();

    assert_eq!(loaded.resident_generation(), generation);
    assert_eq!(loaded.integrity_validation(), None);
}
