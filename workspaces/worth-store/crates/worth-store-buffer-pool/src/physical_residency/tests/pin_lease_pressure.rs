use super::*;

#[test]
fn pin_leases_deny_one_past_live_posture() {
    let identity = store(15);
    let pool = PhysicalResidencyPool::open(identity, limits(256, 2, 1, 64, 4)).unwrap();
    let allocation = allocation(&pool, READ_SCOPE);
    let first = expect_fault(
        &pool,
        &allocation,
        PhysicalFrameKey::new(identity, coordinate(1, 32)),
    )
    .load(|bytes| fill(bytes, 1))
    .unwrap();
    let second = expect_fault(
        &pool,
        &allocation,
        PhysicalFrameKey::new(identity, coordinate(2, 32)),
    )
    .load(|bytes| fill(bytes, 2))
    .unwrap();

    assert_eq!(
        pool.access_frame(
            &allocation,
            PhysicalFrameKey::new(identity, coordinate(3, 32)),
        )
        .unwrap_err(),
        PhysicalResidencyDenial::Pressure(PhysicalResidencyPressureDenial::new(
            identity,
            pool.incarnation(),
            PhysicalResidencyPressureDemand {
                dimension: PhysicalResidencyDimension::PinLeases,
                scope: READ_SCOPE,
                requested: 1,
                current: 2,
                limit: 2,
            },
        ),)
    );

    drop((first, second, allocation));
    assert!(!pool.close().requires_inspection());
}
