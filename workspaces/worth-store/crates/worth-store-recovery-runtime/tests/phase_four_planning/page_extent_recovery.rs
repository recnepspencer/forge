use super::*;

#[test]
fn extent_recovery_planning_admits_manifest_and_chunks_before_redo() {
    let root = prepare_extent_recovery_root("c9-page-extent-recovery");
    let planned = selected_ordinary_recovery(root.path()).plan().unwrap();
    assert!(planned.redo_plan().resolved_decisions().any(|decision| {
        matches!(
            decision.target().identity(),
            PhysicalRedoTargetIdentity::ExtentChunk { .. }
        )
    }));
    let counters = planned.planning_counters();
    assert_eq!(counters.page_extent_integrity_attempts(), 4);
    assert_eq!(counters.page_extent_integrity_admissions(), 4);
    assert_eq!(counters.page_extent_integrity_rejections(), 0);
    assert_eq!(counters.page_extent_owner_projections(), 4);
    assert_eq!(counters.page_extent_owner_decoders(), 3);
}

#[test]
fn corrupt_extent_recovery_frame_stops_before_owner_projection() {
    let root = prepare_extent_recovery_root("c9-corrupt-extent-recovery");
    let extent_directory = root.path().join("families/records/extents");
    for entry in std::fs::read_dir(extent_directory).unwrap() {
        let path = entry.unwrap().path();
        let mut bytes = std::fs::read(&path).unwrap();
        if bytes.len() > 120 {
            bytes[120] ^= 1;
            std::fs::write(path, bytes).unwrap();
        }
    }
    let blocked = match selected_ordinary_recovery(root.path()).plan() {
        Ok(_) => panic!("a corrupt clean extent frame cannot form a recovery plan"),
        Err(outcome) => expect_blocked(outcome),
    };
    let counters = blocked.evidence().planning_counters.unwrap();
    assert_eq!(counters.page_extent_integrity_attempts(), 2);
    assert_eq!(counters.page_extent_integrity_admissions(), 1);
    assert_eq!(counters.page_extent_integrity_rejections(), 1);
    assert_eq!(counters.page_extent_owner_projections(), 1);
    assert_eq!(counters.page_extent_owner_decoders(), 0);
    assert_eq!(blocked.recovery_effects(), 0);
}

fn prepare_extent_recovery_root(name: &str) -> worth_store_test_support::TemporaryDirectory {
    let world = PhysicalResidencyStoreWorld::initialize_for_recovery(name).unwrap();
    let retained_root = world.retained_root();
    let payload = vec![0x61; 40_000];
    canonical_physical_mutation_acknowledgment(&world, [0x51; 32], &payload);
    let request = PhysicalCheckpointRequest::fuzzy(
        PhysicalCheckpointIdempotencyKey::new([0x52; 32]),
        PhysicalCheckpointDeadline::after_milliseconds(5_000).unwrap(),
    );
    let TransitionOutcome::Success(handle) =
        world.serving().checkpoints().start(request).into_raw()
    else {
        panic!("extent checkpoint admission must succeed")
    };
    assert!(matches!(
        handle.wait(),
        PhysicalCheckpointOutcome::Completed(_)
    ));
    canonical_rooted_mutation_without_acknowledgment(&world, [0x53; 32], &payload);
    canonical_durable_wal_attempt_without_execution(&world, [0x54; 32], &payload);
    drop(world);
    retained_root
}
