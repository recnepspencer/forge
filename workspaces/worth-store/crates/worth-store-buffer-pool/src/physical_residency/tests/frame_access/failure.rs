use super::*;

#[test]
fn source_failure_has_one_terminal_for_owner_and_waiter_then_refaults() {
    let identity = store(8);
    let pool = PhysicalResidencyPool::open(identity, limits(1024, 2, 1, 64, 4)).unwrap();
    let allocation = allocation(&pool, READ_SCOPE);
    let key = PhysicalFrameKey::new(identity, coordinate(1, 32));
    let owner = expect_fault(&pool, &allocation, key);
    let loading = owner.loading_identity();
    let waiter = match pool.access_frame(&allocation, key).unwrap() {
        PhysicalFrameAccess::Coalesced(waiter) => waiter,
        _ => panic!("the overlapping access must attach to the existing fault"),
    };

    let owner_terminal = match owner.load(|_| Err("hostile source failure")) {
        Err(PhysicalFrameFaultError::Source { terminal, failure }) => {
            assert_eq!(failure, "hostile source failure");
            terminal
        }
        other => panic!("unexpected owner result: {other:?}"),
    };
    assert_eq!(
        pool.access_frame(&allocation, key).unwrap_err(),
        PhysicalResidencyDenial::FrameLoadTerminated(owner_terminal)
    );
    let waiter_terminal = waiter.wait().unwrap_err();
    assert_eq!(owner_terminal, waiter_terminal);
    assert_eq!(owner_terminal.identity(), loading);
    assert_eq!(
        owner_terminal.kind(),
        PhysicalFrameLoadTerminalKind::SourceExecutionFailed
    );
    assert_eq!(pool.counters().source_loads(), 1);
    assert_eq!(pool.counters().resident_bytes(), 0);
    assert_eq!(pool.counters().pin_leases(), 0);

    let recovered = expect_fault(&pool, &allocation, key)
        .load(|target| {
            target.fill(3);
            Ok::<_, ()>(())
        })
        .unwrap();
    assert_eq!(&*recovered, &[3; 32]);
    assert_eq!(pool.counters().source_loads(), 2);
}

#[test]
fn pre_source_rejection_is_shared_and_cannot_loop_as_another_fault() {
    let identity = store(13);
    let pool = PhysicalResidencyPool::open(identity, limits(1024, 2, 1, 64, 4)).unwrap();
    let allocation = allocation(&pool, READ_SCOPE);
    let key = PhysicalFrameKey::new(identity, coordinate(1, 32));
    let owner = expect_fault(&pool, &allocation, key);
    let waiter = match pool.access_frame(&allocation, key).unwrap() {
        PhysicalFrameAccess::Coalesced(waiter) => waiter,
        _ => panic!("the overlapping access must attach to the existing fault"),
    };

    let terminal = owner.reject_before_source();
    assert_eq!(
        terminal.kind(),
        PhysicalFrameLoadTerminalKind::SourcePreparationFailed
    );
    assert_eq!(pool.counters().source_loads(), 0);
    assert_eq!(
        pool.access_frame(&allocation, key).unwrap_err(),
        PhysicalResidencyDenial::FrameLoadTerminated(terminal)
    );
    assert_eq!(waiter.wait().unwrap_err(), terminal);
    assert_eq!(pool.counters().source_loads(), 0);

    drop(expect_fault(&pool, &allocation, key));
    assert_eq!(pool.counters().active_loading_frames(), 0);
}

#[test]
fn abandoned_fault_owner_terminates_waiter_and_releases_all_reservations() {
    let identity = store(9);
    let pool = PhysicalResidencyPool::open(identity, limits(1024, 2, 1, 64, 4)).unwrap();
    let allocation = allocation(&pool, READ_SCOPE);
    let key = PhysicalFrameKey::new(identity, coordinate(1, 32));
    let owner = expect_fault(&pool, &allocation, key);
    let loading = owner.loading_identity();
    let waiter = match pool.access_frame(&allocation, key).unwrap() {
        PhysicalFrameAccess::Coalesced(waiter) => waiter,
        _ => panic!("the overlapping access must attach to the existing fault"),
    };

    drop(owner);
    let terminal = waiter.wait().unwrap_err();
    assert_eq!(terminal.identity(), loading);
    assert_eq!(
        terminal.kind(),
        PhysicalFrameLoadTerminalKind::FaultOwnerAbandoned
    );
    let counters = pool.counters();
    assert_eq!(counters.resident_bytes(), 0);
    assert_eq!(counters.pinned_frames(), 0);
    assert_eq!(counters.pin_leases(), 0);
    assert_eq!(counters.active_loading_frames(), 0);
    drop(expect_fault(&pool, &allocation, key));
}

#[test]
fn dropping_waiter_releases_only_its_reserved_pin() {
    let identity = store(10);
    let pool = PhysicalResidencyPool::open(identity, limits(1024, 2, 1, 64, 4)).unwrap();
    let allocation = allocation(&pool, READ_SCOPE);
    let key = PhysicalFrameKey::new(identity, coordinate(1, 32));
    let owner = expect_fault(&pool, &allocation, key);
    let waiter = match pool.access_frame(&allocation, key).unwrap() {
        PhysicalFrameAccess::Coalesced(waiter) => waiter,
        _ => panic!("the overlapping access must attach to the existing fault"),
    };
    assert_eq!(pool.counters().pin_leases(), 2);
    drop(waiter);
    assert_eq!(pool.counters().pin_leases(), 1);
    drop(owner);
    assert_eq!(pool.counters().pin_leases(), 0);
    assert_eq!(pool.counters().active_loading_frames(), 0);
}
