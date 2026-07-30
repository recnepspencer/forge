use worth_store_contracts::QueueProducerResourceShape;

use crate::foreground_reservation::admitted_point_read_reservation_for_certification_test;
use crate::{
    lower_physical_foreground_work, PhysicalForegroundWorkDeclaration, QueueDurabilityClass,
    QueueLocalityIdentity, QueueRecoveryOrdering, QueueWritebackPolicy,
};

#[test]
fn physical_foreground_declarations_derive_security_and_fixed_posture() {
    let locality = QueueLocalityIdentity::from_digest([71; 32]);
    let resources = QueueProducerResourceShape::new()
        .with_queue_slots(1)
        .with_bandwidth_tokens(4096)
        .with_read_ahead_windows(1)
        .with_worker_permits(1);
    let cases = [
        (
            PhysicalForegroundWorkDeclaration::read(
                admitted_point_read_reservation_for_certification_test(),
                locality.clone(),
                resources,
                17,
            ),
            QueueDurabilityClass::ReadOnly,
            QueueWritebackPolicy::None,
            locality.clone(),
        ),
        (
            PhysicalForegroundWorkDeclaration::buffered_write(
                admitted_point_read_reservation_for_certification_test(),
                locality.clone(),
                resources,
                17,
            ),
            QueueDurabilityClass::BufferedWrite,
            QueueWritebackPolicy::DeferredWithinFlushEpoch,
            locality.clone(),
        ),
        (
            PhysicalForegroundWorkDeclaration::durable_write(
                admitted_point_read_reservation_for_certification_test(),
                locality.clone(),
                resources,
                17,
            ),
            QueueDurabilityClass::PlatformDurable,
            QueueWritebackPolicy::Immediate,
            locality,
        ),
    ];

    for (declaration, durability, writeback, expected_locality) in cases {
        let work = lower_physical_foreground_work(declaration).expect("physical work should lower");
        let grouping = work
            .grouping_basis()
            .expect("physical work carries exact grouping");

        assert_eq!(
            work.security_scope_identity(),
            grouping.security_scope_identity()
        );
        assert_eq!(work.durability_class(), durability);
        assert_eq!(grouping.durability_class(), durability);
        assert_eq!(grouping.flush_epoch(), 17);
        assert_eq!(
            grouping.recovery_ordering(),
            QueueRecoveryOrdering::NotRecoveryCritical
        );
        assert_eq!(grouping.writeback_policy(), writeback);
        assert_eq!(grouping.locality(), Some(&expected_locality));
    }
}
