use super::*;

#[test]
fn repinned_legal_head_is_removed_before_forced_eviction() {
    let identity = store(33);
    let pool = PhysicalResidencyPool::open(identity, limits(64, 3, 1, 64, 3)).unwrap();
    let read = allocation(&pool, READ_SCOPE);
    let first_key = PhysicalFrameKey::new(identity, coordinate(1, 32));
    let second_key = PhysicalFrameKey::new(identity, coordinate(2, 32));

    drop(
        expect_fault(&pool, &read, first_key)
            .load(|bytes| fill(bytes, 1))
            .unwrap(),
    );
    drop(
        expect_fault(&pool, &read, second_key)
            .load(|bytes| fill(bytes, 2))
            .unwrap(),
    );

    let first = expect_hit(&pool, &read, first_key);
    let second = expect_hit(&pool, &read, second_key);
    let before = pool.counters();
    let incoming_key = PhysicalFrameKey::new(identity, coordinate(3, 32));

    let denial = match pool.access_frame(&read, incoming_key) {
        Err(denial) => denial,
        Ok(_) => panic!(
            "C5_PREDICATE:pinned-eviction: forced access succeeded while every resident frame was pinned"
        ),
    };
    assert_eq!(
        denial,
        PhysicalResidencyDenial::Pressure(PhysicalResidencyPressureDenial::new(
            identity,
            pool.incarnation(),
            PhysicalResidencyPressureDemand {
                dimension: PhysicalResidencyDimension::ResidentBytes,
                scope: READ_SCOPE,
                requested: 32,
                current: 64,
                limit: 64,
            },
        ))
    );
    assert_eq!(&*first, &[1; 32]);
    assert_eq!(&*second, &[2; 32]);
    let denied = pool.counters();
    assert_eq!(denied.evictions(), before.evictions());
    assert_eq!(
        denied.eviction_candidate_inspections(),
        before.eviction_candidate_inspections() + 1
    );
    assert_eq!(denied.pinned_frames(), 2);
    assert_eq!(denied.pin_leases(), 2);

    drop(first);
    let incoming = expect_fault(&pool, &read, incoming_key)
        .load(|bytes| fill(bytes, 3))
        .unwrap();
    assert_eq!(&*incoming, &[3; 32]);
    assert_eq!(pool.counters().evictions(), before.evictions() + 1);
    assert_eq!(&*second, &[2; 32]);
}
