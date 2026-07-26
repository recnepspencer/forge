use super::*;
use crate::facade::{
    BridgeAsyncRequestTruthViewBasis, BridgeExecutionBasisDenialKind,
    BridgeExecutionBasisFinalizationFailureKind, BridgeExecutionBasisSignalTerminal,
    BridgeExecutionBasisTerminalDisposition, BridgeExecutionQueuePressureState,
    BridgeExecutionSafePointSignalState, BridgeManagedExecutionIntent,
    BridgeManagedExecutionPartialEffectPosture, BridgeManagedExecutionStepContract,
    BridgeManagedExecutionStepLimits, BridgeManagedQueueFailureKind, PlannedTruthViewPacket,
};
use crate::source::with_async_request_signal_runtime;
use crate::truth_identity_fixtures::{truth_branch_fixture, truth_snapshot_fixture};
use worth_signal::facade::{ResourceCancellationReason, ResourceInFlightStatus};

#[test]
fn bridge_mints_and_fulfills_fresh_signal_attempt_for_exact_managed_intent() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let basis = runtime
        .admit_managed_execution_basis(
            managed_intent("attempt-a"),
            step_contract(),
            truth_basis("snapshot-a"),
            planned_truth_view(&runtime),
        )
        .expect("matching managed intent and truth should admit");
    let handle = basis.request().request_handle();

    assert_eq!(basis.bridge_runtime_key(), runtime.signal_runtime_key);
    assert_eq!(
        basis.managed_intent().resource_attempt_identity(),
        "attempt-a"
    );
    assert_eq!(basis.counters().managed_intent_check_count(), 1);
    assert_eq!(basis.counters().truth_basis_check_count(), 1);
    assert_eq!(basis.counters().reservation_check_count(), 1);
    assert_eq!(basis.counters().truth_materialization_count(), 1);
    assert_eq!(basis.counters().signal_attempt_admission_count(), 1);
    assert_eq!(basis.counters().signal_attempt_check_count(), 1);

    let finalized = basis
        .finalize(BridgeExecutionBasisTerminalDisposition::Completed)
        .expect("managed execution completion should terminalize Signal");
    assert_eq!(
        finalized.signal_terminal(),
        BridgeExecutionBasisSignalTerminal::Fulfilled
    );
    assert!(finalized.reservation_released());
    assert_eq!(
        signal_status(&runtime, handle),
        ResourceInFlightStatus::Fulfilled
    );
}

#[test]
fn one_phase_five_attempt_cannot_own_two_live_bridge_attempts() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let intent = managed_intent("attempt-a");
    let first = runtime
        .admit_managed_execution_basis(
            intent.clone(),
            step_contract(),
            truth_basis("snapshot-a"),
            planned_truth_view(&runtime),
        )
        .expect("first managed execution basis should reserve the intent");
    let first_handle = first.request().request_handle();

    let denial = runtime
        .admit_managed_execution_basis(
            intent.clone(),
            step_contract(),
            truth_basis("snapshot-a"),
            planned_truth_view(&runtime),
        )
        .expect_err("second Signal attempt shared one Phase 5 intent");
    assert_eq!(
        denial.kind(),
        BridgeExecutionBasisDenialKind::ManagedExecutionIntentAlreadyReserved
    );
    assert_eq!(denial.counters().signal_attempt_admission_count(), 0);
    assert_eq!(denial.counters().truth_materialization_count(), 0);

    drop(first);
    assert_eq!(
        signal_status(&runtime, first_handle),
        ResourceInFlightStatus::Cancelled
    );
    let replacement = runtime
        .admit_managed_execution_basis(
            intent,
            step_contract(),
            truth_basis("snapshot-a"),
            planned_truth_view(&runtime),
        )
        .expect("dropping the first authority should cancel Signal and release intent");
    replacement
        .finalize(BridgeExecutionBasisTerminalDisposition::Cancelled)
        .expect("replacement should cancel cleanly");
}

#[test]
fn independently_valid_phase_five_intents_receive_distinct_signal_attempts() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let first = runtime
        .admit_managed_execution_basis(
            managed_intent("attempt-a"),
            step_contract(),
            truth_basis("snapshot-a"),
            planned_truth_view(&runtime),
        )
        .expect("first intent should admit");
    let second = runtime
        .admit_managed_execution_basis(
            managed_intent("attempt-b"),
            step_contract(),
            truth_basis("snapshot-a"),
            planned_truth_view(&runtime),
        )
        .expect("different intent should admit independently");

    assert_ne!(first.identity(), second.identity());
    assert_ne!(
        first.request().request_handle(),
        second.request().request_handle()
    );
    assert_ne!(
        first.request().request_identity(),
        second.request().request_identity()
    );
    assert_eq!(first.request().attempt().get(), 0);
    assert_eq!(second.request().attempt().get(), 0);
    assert_eq!(
        signal_status(&runtime, first.request().request_handle()),
        ResourceInFlightStatus::Active
    );
    assert_eq!(
        signal_status(&runtime, second.request().request_handle()),
        ResourceInFlightStatus::Active
    );
    first
        .finalize(BridgeExecutionBasisTerminalDisposition::Completed)
        .expect("first attempt should complete");
    second
        .finalize(BridgeExecutionBasisTerminalDisposition::Completed)
        .expect("second attempt should complete");
}

#[test]
fn mismatched_truth_denies_before_materialization_or_signal_admission() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let denial = runtime
        .admit_managed_execution_basis(
            managed_intent("attempt-a"),
            step_contract(),
            truth_basis("snapshot-b"),
            planned_truth_view(&runtime),
        )
        .expect_err("mismatched truth basis admitted");

    assert_eq!(
        denial.kind(),
        BridgeExecutionBasisDenialKind::TruthBasisMismatch
    );
    assert_eq!(denial.counters().managed_intent_check_count(), 1);
    assert_eq!(denial.counters().truth_basis_check_count(), 1);
    assert_eq!(denial.counters().reservation_check_count(), 0);
    assert_eq!(denial.counters().truth_materialization_count(), 0);
    assert_eq!(denial.counters().signal_attempt_admission_count(), 0);
}

#[test]
fn explicit_cancellation_terminalizes_signal_and_releases_intent() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let intent = managed_intent("attempt-a");
    let basis = runtime
        .admit_managed_execution_basis(
            intent.clone(),
            step_contract(),
            truth_basis("snapshot-a"),
            planned_truth_view(&runtime),
        )
        .expect("managed execution should admit");
    let handle = basis.request().request_handle();

    let receipt = basis
        .finalize(BridgeExecutionBasisTerminalDisposition::Cancelled)
        .expect("managed cancellation should terminalize Signal");
    assert_eq!(
        receipt.signal_terminal(),
        BridgeExecutionBasisSignalTerminal::Cancelled
    );
    assert_eq!(
        signal_status(&runtime, handle),
        ResourceInFlightStatus::Cancelled
    );

    let replacement = runtime
        .admit_managed_execution_basis(
            intent,
            step_contract(),
            truth_basis("snapshot-a"),
            planned_truth_view(&runtime),
        )
        .expect("finalization should release the exact intent reservation");
    drop(replacement);
}

#[test]
fn thread_affinity_failure_returns_the_basis_for_recovery() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let basis = runtime
        .admit_managed_execution_basis(
            managed_intent("attempt-a"),
            step_contract(),
            truth_basis("snapshot-a"),
            planned_truth_view(&runtime),
        )
        .expect("managed execution should admit on the owner thread");

    let failure = std::thread::spawn(move || {
        basis
            .finalize(BridgeExecutionBasisTerminalDisposition::Completed)
            .expect_err("foreign thread must not terminalize the Signal attempt")
    })
    .join()
    .expect("foreign-thread finalization probe should return");
    assert_eq!(
        failure.kind(),
        BridgeExecutionBasisFinalizationFailureKind::SignalRuntimeThreadAffinityViolation
    );

    failure
        .into_basis()
        .finalize(BridgeExecutionBasisTerminalDisposition::Completed)
        .expect("returned basis should remain recoverable on its Signal owner thread");
}

#[test]
fn safe_points_project_exact_signal_lifecycle_and_queue_pressure() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let mut basis = runtime
        .admit_managed_execution_basis(
            managed_intent("safe-point-attempt"),
            step_contract(),
            truth_basis("snapshot-a"),
            planned_truth_view(&runtime),
        )
        .expect("managed execution should admit");
    let available = basis
        .observe_safe_point()
        .expect("active request should produce safe-point evidence");

    assert_eq!(
        available.signal_state(),
        BridgeExecutionSafePointSignalState::Active
    );
    assert_eq!(available.queue_depth(), 0);
    assert_eq!(
        available.pressure_state(),
        BridgeExecutionQueuePressureState::Available
    );
    assert_eq!(available.observation_ordinal(), 0);
    assert_eq!(available.counters().exact_signal_request_lookup_count(), 1);
    assert_eq!(available.counters().pressure_classification_count(), 1);

    let admission = basis
        .enqueue_managed_queue(4)
        .expect("exact bridge basis should fill its Signal-owned queue");
    let saturation = admission.mutation();
    assert_eq!(
        saturation.pressure_state(),
        BridgeExecutionQueuePressureState::Saturated
    );
    assert_eq!(saturation.queue_depth(), 4);
    with_async_request_signal_runtime(runtime.signal_runtime_key, |signal_runtime| {
        signal_runtime.cancel_resource_request(
            basis.request().request_handle(),
            ResourceCancellationReason::HostRequested,
        )
    })
    .expect("test runtime should stay on one thread")
    .expect("exact request cancellation should succeed");
    let saturated = basis
        .observe_safe_point()
        .expect("cancelled request should retain observable lifecycle evidence");
    assert_eq!(
        saturated.signal_state(),
        BridgeExecutionSafePointSignalState::Cancelled
    );
    assert_eq!(
        saturated.pressure_state(),
        BridgeExecutionQueuePressureState::Saturated
    );
    assert_eq!(saturated.observation_ordinal(), 1);
    let (_, occupancy) = admission.into_parts();
    let release = basis
        .release_managed_queue_occupancy(occupancy)
        .expect("terminal Signal requests must still release exact queue occupancy");
    assert_eq!(release.queue_depth(), 0);
}

#[test]
fn queue_overflow_denies_without_mutating_signal_pressure() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let mut basis = runtime
        .admit_managed_execution_basis(
            managed_intent("queue-contract-attempt"),
            step_contract(),
            truth_basis("snapshot-a"),
            planned_truth_view(&runtime),
        )
        .expect("managed execution should admit");

    let denial = basis
        .enqueue_managed_queue(5)
        .expect_err("the bound queue capacity must reject overflow");
    assert_eq!(
        denial.kind(),
        BridgeManagedQueueFailureKind::SignalQueueMutationDenied
    );
    let first_valid = basis
        .observe_safe_point()
        .expect("overflow denial should preserve the bound queue");
    assert_eq!(first_valid.observation_ordinal(), 0);
    assert_eq!(first_valid.queue_depth(), 0);
    assert_eq!(first_valid.queue_capacity(), 4);

    basis
        .finalize(BridgeExecutionBasisTerminalDisposition::Cancelled)
        .expect("test basis should clean up");
}

#[test]
fn execution_basis_cannot_finalize_while_queue_occupancy_is_outstanding() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let mut basis = runtime
        .admit_managed_execution_basis(
            managed_intent("queue-finalization-attempt"),
            step_contract(),
            truth_basis("snapshot-a"),
            planned_truth_view(&runtime),
        )
        .expect("managed execution should admit");
    let admission = basis
        .enqueue_managed_queue(1)
        .expect("one queue unit should admit");
    let (_, occupancy) = admission.into_parts();

    let failure = basis
        .finalize(BridgeExecutionBasisTerminalDisposition::Abandoned)
        .expect_err("outstanding queue occupancy must retain the execution basis");
    assert_eq!(
        failure.kind(),
        BridgeExecutionBasisFinalizationFailureKind::ManagedQueueOccupied
    );
    let mut basis = failure.into_basis();
    basis
        .release_managed_queue_occupancy(occupancy)
        .expect("exact occupancy should remain releasable");
    basis
        .finalize(BridgeExecutionBasisTerminalDisposition::Abandoned)
        .expect("released queue occupancy should permit finalization");
}

fn managed_intent(attempt: &str) -> BridgeManagedExecutionIntent {
    BridgeManagedExecutionIntent::new("query-operation-binding", attempt)
}

fn step_contract() -> BridgeManagedExecutionStepContract {
    BridgeManagedExecutionStepContract::new(
        "chunk-boundary",
        BridgeManagedExecutionStepLimits::new(8, 4, 2).with_memory_ceilings(64, 32),
        BridgeManagedExecutionPartialEffectPosture::None,
    )
    .expect("test step contract should be bounded")
}

fn truth_basis(snapshot: &str) -> BridgeAsyncRequestTruthViewBasis {
    BridgeAsyncRequestTruthViewBasis::branch_head(
        truth_branch_fixture("analysis"),
        truth_snapshot_fixture(snapshot),
    )
}

fn signal_status(
    runtime: &RuntimeBridge,
    handle: worth_signal::facade::ResourceRequestHandle,
) -> ResourceInFlightStatus {
    with_async_request_signal_runtime(runtime.signal_runtime_key, |signal_runtime| {
        signal_runtime
            .in_flight_resource_request(handle)
            .expect("managed Signal request should retain terminal lifecycle")
            .status()
    })
    .expect("test runtime should stay on one thread")
}

fn planned_truth_view(runtime: &RuntimeBridge) -> PlannedTruthViewPacket {
    let declaration = HistoricalEvaluationDeclaration::new(
        BridgeTruthViewSelector::branch_snapshot(
            truth_branch_fixture("analysis"),
            truth_snapshot_fixture("snapshot-a"),
        ),
        BridgeReplayMode::Enabled,
        BridgeDiagnosticsTier::Standard,
        BridgeDeliveryIntent::PrepareSignalEvaluation,
    );
    runtime
        .plan_truth_view_packet(declaration, SnapshotReadPacket::new(vec![]))
        .expect("registered truth view should plan")
}

#[path = "execution_basis/readmission.rs"]
mod readmission;
