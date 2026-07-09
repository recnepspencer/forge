use worth_store_buffer_pool::BufferPoolQueueExecutionDeclaration;
use worth_store_contracts::S6QueueProducerResourceShape;
use worth_store_physical_backend::{
    BackendQueueExecutionBackpressure, BackendQueueExecutionPlanBinding,
    BackendQueueSpeculativeScope, BackendTargetProfile,
};

use super::execute_admitted_queue_plan;
use super::test_support::{
    admitted_plan, admitted_plan_for_backend_profile, backend_for, completion_for_binding,
    completion_for_group, completion_for_plan, grouping_for, point_read_budget, policy_receipt,
    secure_io_for_work, speculative_scope, GroupingTestMutation,
};
use super::QueueExecutionObservation;
use crate::foreground_reservation::admitted_point_read_reservation_for_certification_test;
use crate::{
    admit_queue_execution_plan, execute_grouped_ready_queue_plans, execute_ready_queue_plan,
    group_ready_queue_pair, lower_buffer_pool_queue_declaration, QueueBackpressureCause,
    QueueExecutionAdmissionDenial, QueueExecutionAdmissionRequest, QueueExecutionOutcome,
    QueueExecutionProgression, QueueGroupingDenial, QueueGroupingOutcome, QueueWorkDeclaration,
    S6QueueDurabilityClass,
};

#[test]
fn admitted_queue_work_lowers_preserving_policy_and_grouping_basis() {
    let reservation = admitted_point_read_reservation_for_certification_test();
    let budget = point_read_budget();
    let work = QueueWorkDeclaration::foreground(
        reservation.execution_ready(),
        S6QueueDurabilityClass::ReadOnly,
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
    let resource_shape = S6QueueProducerResourceShape::new()
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

fn admitted_write_back_plan() -> crate::QueueExecutionReadyPlan {
    let reservation = admitted_point_read_reservation_for_certification_test();
    let resource_shape = S6QueueProducerResourceShape::new()
        .with_queue_slots(1)
        .with_bandwidth_tokens(4096)
        .with_write_back_windows(1)
        .with_worker_permits(1);
    let producer = BufferPoolQueueExecutionDeclaration::write_back(7, resource_shape);
    let work = lower_buffer_pool_queue_declaration(producer, reservation)
        .expect("write-back producer should lower to queue work");
    let backend = backend_for(work);
    let work = work.with_secure_io_scope(secure_io_for_work(work, &backend));
    admit_queue_execution_plan(QueueExecutionAdmissionRequest::new(
        work,
        &backend,
        policy_receipt(work.requested_budget()),
    ))
    .expect("write-back queue work should admit")
}

#[test]
fn grouping_mismatch_is_a_typed_admission_denial() {
    let reservation = admitted_point_read_reservation_for_certification_test();
    let budget = point_read_budget();
    let work = QueueWorkDeclaration::foreground(
        reservation.execution_ready(),
        S6QueueDurabilityClass::ReadOnly,
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

#[test]
fn backpressure_is_typed_and_counter_visible_after_admission() {
    let plan = admitted_plan();
    let completion_builder = completion_for_plan(&plan, 0, None, 0, None)
        .observe_queue_depth(4)
        .observe_foreground_wait_events(1)
        .observe_backpressure(BackendQueueExecutionBackpressure::QueueDepthSaturated);
    let outcome = execute_ready_queue_plan(plan, completion_builder.complete());

    let QueueExecutionOutcome::Backpressured(backpressured) = outcome else {
        panic!("expected typed backpressure outcome");
    };
    assert_eq!(
        backpressured.cause(),
        QueueBackpressureCause::QueueDepthSaturated
    );
    assert_eq!(backpressured.counters().backpressure_events(), 1);
    assert_eq!(backpressured.counters().foreground_wait_events(), 1);
    assert_eq!(
        backpressured.counters().backpressure_cause(),
        Some(QueueBackpressureCause::QueueDepthSaturated)
    );
}

#[test]
fn over_budget_read_ahead_is_a_typed_denial() {
    let plan = admitted_plan();
    let scope = speculative_scope(&plan);
    let completion_builder = completion_for_plan(&plan, 2, Some(scope), 0, None);
    let outcome = execute_ready_queue_plan(plan, completion_builder.complete());

    let QueueExecutionOutcome::Denied(denied) = outcome else {
        panic!("over-budget read-ahead must be denied");
    };
    assert_eq!(denied.cause(), QueueBackpressureCause::ReadAheadDenied);
    assert_eq!(denied.counters().denied_units(), 1);
    assert_eq!(denied.counters().read_ahead_units(), 2);
}

#[test]
fn queue_depth_above_admitted_slots_is_backpressure() {
    let plan = admitted_plan();
    let completion_builder = completion_for_plan(&plan, 0, None, 0, None).observe_queue_depth(2);
    let outcome = execute_ready_queue_plan(plan, completion_builder.complete());

    let QueueExecutionOutcome::Backpressured(backpressured) = outcome else {
        panic!("queue-depth saturation must be backpressure");
    };
    assert_eq!(
        backpressured.cause(),
        QueueBackpressureCause::QueueDepthSaturated
    );
    assert_eq!(backpressured.counters().backpressure_events(), 1);
}

#[test]
fn mechanical_execution_cannot_reclassify_or_change_durability() {
    let plan = admitted_plan();
    let outcome = execute_admitted_queue_plan(
        plan,
        QueueExecutionObservation::empty()
            .with_attempted_durability_class(S6QueueDurabilityClass::PlatformDurable),
    );

    let QueueExecutionOutcome::Violation(violation) = outcome else {
        panic!("durability strengthening must be a violation");
    };
    assert_eq!(violation.counters().violation_events(), 1);
    assert_eq!(
        violation.plan().progression(),
        QueueExecutionProgression::Executed
    );
}

#[test]
fn certification_completion_executes_inside_admitted_envelope() {
    let plan = admitted_plan();
    let scope = speculative_scope(&plan);
    let completion_builder =
        completion_for_plan(&plan, 1, Some(scope), 0, None).observe_mechanical_adaptation(1, 1, 1);
    let outcome = execute_ready_queue_plan(plan, completion_builder.complete());

    let QueueExecutionOutcome::Executed(executed) = outcome else {
        panic!("backend completion should execute inside admitted envelope");
    };
    assert_eq!(executed.counters().grouped_writes(), 0);
    assert_eq!(executed.counters().read_ahead_units(), 1);
    assert_eq!(executed.counters().mechanical_retries(), 1);
    assert_eq!(executed.counters().partial_read_events(), 1);
    assert_eq!(executed.counters().short_write_events(), 1);
}

#[test]
fn write_back_with_cross_key_scope_is_backpressured_and_counter_visible() {
    let plan = admitted_write_back_plan();
    let wrong_scope = BackendQueueSpeculativeScope::admitted(
        plan.grouping_basis().security_scope_identity(),
        plan.grouping_basis().tenant_scope(),
        worth_store_security::StoreKeyScope::BackupExportEnvelope,
    );
    let completion_builder = completion_for_plan(&plan, 0, None, 1, Some(wrong_scope));
    let outcome = execute_ready_queue_plan(plan, completion_builder.complete());

    let QueueExecutionOutcome::Backpressured(backpressured) = outcome else {
        panic!("cross-key write-back must be typed backpressure");
    };
    assert_eq!(
        backpressured.cause(),
        QueueBackpressureCause::WriteBackWindowSaturated
    );
    assert_eq!(backpressured.counters().write_back_units(), 1);
}

#[test]
fn grouping_denies_backend_capability_mismatch() {
    let outcome = group_ready_queue_pair(
        admitted_plan(),
        admitted_plan_for_backend_profile(BackendTargetProfile::SimulatedStrictDurable),
    );

    let QueueGroupingOutcome::Denied(denied) = outcome else {
        panic!("mixed backend profile plans must not group");
    };
    assert_eq!(
        denied.denial(),
        QueueGroupingDenial::BackendCapabilityMismatch
    );
}

#[test]
fn backend_completion_binding_rejects_different_ready_plan() {
    let plan = admitted_plan();
    let completion_builder = completion_for_binding(
        BackendQueueExecutionPlanBinding::from_store_replay_binding(
            plan.backend_completion_binding()
                .backend_execution_binding()
                .primary(),
            None,
            BackendTargetProfile::SimulatedStrictDurable,
            plan.backend_evidence_class(),
            0,
        ),
        1,
        Some(speculative_scope(&plan)),
        0,
        None,
    )
    .observe_mechanical_adaptation(2, 1, 1);
    let outcome = execute_ready_queue_plan(plan, completion_builder.complete());

    let QueueExecutionOutcome::Violation(violation) = outcome else {
        panic!("wrong backend completion binding must violate");
    };
    assert_eq!(violation.counters().violation_events(), 1);
    assert_eq!(violation.counters().read_ahead_units(), 1);
    assert_eq!(violation.counters().mechanical_retries(), 2);
}

#[test]
fn grouped_ready_execution_proves_and_counts_both_plans() {
    let outcome = group_ready_queue_pair(admitted_plan(), admitted_plan());
    let QueueGroupingOutcome::Grouped(grouped) = outcome else {
        panic!("equivalent ready plans should group");
    };
    let expected_units = grouped
        .first()
        .admitted_budget()
        .queue_slots()
        .saturating_add(grouped.first().admitted_budget().bandwidth_tokens())
        .saturating_add(grouped.first().admitted_budget().read_ahead_window())
        .saturating_add(grouped.first().admitted_budget().worker_permits())
        .saturating_add(grouped.first().admitted_budget().cache_residency_hints())
        * 2;
    let scope = speculative_scope(grouped.first());
    let completion_builder = completion_for_group(&grouped, 1, Some(scope), 0, None)
        .observe_mechanical_adaptation(2, 1, 1);
    let execution = execute_grouped_ready_queue_plans(grouped, completion_builder.complete());

    let QueueExecutionOutcome::Executed(executed) = execution else {
        panic!("grouped ready plans should execute with scheduler grouping evidence");
    };
    assert_eq!(executed.counters().grouped_writes(), 2);
    assert_eq!(executed.counters().submitted_units(), expected_units);
    assert_eq!(executed.counters().admitted_units(), expected_units);
    assert!(executed.secondary_plan().is_some());
    assert_eq!(executed.counters().read_ahead_units(), 1);
    assert_eq!(executed.counters().mechanical_retries(), 2);
    assert_eq!(executed.counters().partial_read_events(), 1);
    assert_eq!(executed.counters().short_write_events(), 1);
}

#[test]
fn grouped_backpressure_preserves_grouping_speculation_and_adaptation_counters() {
    let outcome = group_ready_queue_pair(admitted_plan(), admitted_plan());
    let QueueGroupingOutcome::Grouped(grouped) = outcome else {
        panic!("equivalent ready plans should group");
    };
    let scope = speculative_scope(grouped.first());
    let completion_builder = completion_for_group(&grouped, 1, Some(scope), 0, None)
        .observe_mechanical_adaptation(2, 1, 1)
        .observe_backpressure(BackendQueueExecutionBackpressure::QueueDepthSaturated);
    let execution = execute_grouped_ready_queue_plans(grouped, completion_builder.complete());

    let QueueExecutionOutcome::Backpressured(backpressured) = execution else {
        panic!("backend saturation should be typed backpressure");
    };
    assert_eq!(backpressured.counters().grouped_writes(), 2);
    assert_eq!(backpressured.counters().read_ahead_units(), 1);
    assert_eq!(backpressured.counters().mechanical_retries(), 2);
    assert_eq!(backpressured.counters().partial_read_events(), 1);
    assert_eq!(backpressured.counters().short_write_events(), 1);
}

#[test]
fn ready_queue_grouping_is_typed_and_replay_visible() {
    let outcome = group_ready_queue_pair(admitted_plan(), admitted_plan());

    let QueueGroupingOutcome::Grouped(grouped) = outcome else {
        panic!("equivalent ready plans should group");
    };
    assert_eq!(grouped.grouped_writes(), 2);
    assert_eq!(
        grouped.replay_identities()[0],
        grouped.first().replay_identity()
    );
    assert_eq!(
        grouped.replay_identities()[1],
        grouped.second().replay_identity()
    );
}

#[test]
fn grouped_ready_execution_rejects_single_plan_backend_completion() {
    let outcome = group_ready_queue_pair(admitted_plan(), admitted_plan());
    let QueueGroupingOutcome::Grouped(grouped) = outcome else {
        panic!("equivalent ready plans should group");
    };
    let scope = speculative_scope(grouped.first());
    let completion_builder = completion_for_plan(grouped.first(), 1, Some(scope), 0, None);
    let execution = execute_grouped_ready_queue_plans(grouped, completion_builder.complete());

    let QueueExecutionOutcome::Violation(violation) = execution else {
        panic!("single-plan backend completion must not claim grouped execution");
    };
    assert_eq!(violation.counters().violation_events(), 1);
}

#[test]
fn backend_completion_with_cross_scope_read_ahead_is_denied() {
    let plan = admitted_plan();
    let wrong_scope = BackendQueueSpeculativeScope::admitted(
        plan.grouping_basis().security_scope_identity(),
        plan.grouping_basis().tenant_scope(),
        worth_store_security::StoreKeyScope::BackupExportEnvelope,
    );
    let completion_builder = completion_for_plan(&plan, 1, Some(wrong_scope), 0, None);
    let outcome = execute_ready_queue_plan(plan, completion_builder.complete());

    let QueueExecutionOutcome::Denied(denied) = outcome else {
        panic!("cross-scope speculative read-ahead must deny");
    };
    assert_eq!(denied.cause(), QueueBackpressureCause::ReadAheadDenied);
}
