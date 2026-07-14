use forge_store_buffer_pool::BufferPoolQueueExecutionDeclaration;
use forge_store_contracts::QueueProducerResourceShape;

use super::super::test_support::{
    backend_for, grouping_for, point_read_budget, policy_receipt, secure_io_for_work,
    GroupingTestMutation,
};
use crate::foreground_reservation::admitted_point_read_reservation_for_certification_test;
use crate::{
    admit_queue_execution_plan, lower_buffer_pool_queue_declaration, QueueDurabilityClass,
    QueueExecutionAdmissionDenial, QueueExecutionAdmissionRequest, QueueExecutionProgression,
    QueueGroupingDenial, QueueWorkDeclaration,
};

#[test]
fn admitted_queue_work_lowers_preserving_policy_and_grouping_basis() {
    let reservation = admitted_point_read_reservation_for_certification_test();
    let budget = point_read_budget();
    let work = QueueWorkDeclaration::foreground(
        reservation.execution_ready(),
        QueueDurabilityClass::ReadOnly,
        budget,
    )
    .with_grouping_basis(grouping_for(reservation.security_scope_identity()));
    let backend = backend_for(work);
    let work = work.with_secure_io_scope(secure_io_for_work(work, &backend));
    let plan = admit_queue_execution_plan(QueueExecutionAdmissionRequest::new(
        work,
        &backend,
        policy_receipt(budget),
    ))
    .expect("matching queue work should lower to an admitted execution plan");

    assert_eq!(plan.work(), work);
    assert_eq!(
        plan.progression(),
        QueueExecutionProgression::ExecutionReady
    );
    assert_eq!(
        plan.grouping_basis(),
        grouping_for(work.security_scope_identity())
    );
    assert_eq!(plan.replay_identity().work_class(), work.class());
    assert_eq!(
        plan.replay_identity().durability_class(),
        work.durability_class()
    );
    assert_eq!(plan.replay_identity().requested_budget(), budget);
    assert_eq!(
        plan.backend_completion_binding().replay_identity(),
        plan.replay_identity()
    );
}
#[test]
fn producer_declaration_lowers_through_scheduler_admission() {
    let reservation = admitted_point_read_reservation_for_certification_test();
    let resource_shape = QueueProducerResourceShape::new()
        .with_queue_slots(1)
        .with_bandwidth_tokens(4096)
        .with_read_ahead_windows(1)
        .with_worker_permits(1)
        .with_cache_residency_hints(1);
    let producer = BufferPoolQueueExecutionDeclaration::read_ahead(7, resource_shape);
    let work = lower_buffer_pool_queue_declaration(producer, reservation)
        .expect("producer shape should lower to queue work");
    let backend = backend_for(work);
    let work = work.with_secure_io_scope(secure_io_for_work(work, &backend));
    let plan = admit_queue_execution_plan(QueueExecutionAdmissionRequest::new(
        work,
        &backend,
        policy_receipt(work.requested_budget()),
    ))
    .expect("lowered producer work should admit through scheduler");

    assert_eq!(plan.work().class(), work.class());
    assert_eq!(plan.grouping_basis().flush_epoch(), 7);
    assert_eq!(
        plan.replay_identity().requested_budget(),
        work.requested_budget()
    );
}

#[test]
fn grouping_mismatch_is_a_typed_admission_denial() {
    let reservation = admitted_point_read_reservation_for_certification_test();
    let budget = point_read_budget();
    let work = QueueWorkDeclaration::foreground(
        reservation.execution_ready(),
        QueueDurabilityClass::ReadOnly,
        budget,
    )
    .with_grouping_basis(
        grouping_for(reservation.security_scope_identity()).with_different_durability_for_test(),
    );
    let backend = backend_for(work);

    let denial = admit_queue_execution_plan(QueueExecutionAdmissionRequest::new(
        work,
        &backend,
        policy_receipt(budget),
    ))
    .expect_err("durability mismatch must not silently batch");

    assert_eq!(
        denial,
        QueueExecutionAdmissionDenial::GroupingDenied(QueueGroupingDenial::DurabilityClassMismatch)
    );
}
