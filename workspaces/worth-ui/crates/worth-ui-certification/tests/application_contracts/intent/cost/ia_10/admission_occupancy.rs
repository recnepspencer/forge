use worth_ui::facade::intent::{
    UiAdmittedIntent, UiIntent, UiIntentAdmissionCost, UiIntentAdmissionDecision,
    UiIntentAdmissionMetrics, UiIntentAdmissionStopReason, UiIntentExecutionAdvanceOutcome,
    UiIntentExecutionDispatchOutcome, UiIntentExecutionReservationDenial,
    UiIntentExecutionTransitionPosture, UiIntentProviderVersion,
};

use crate::intent::admission::phase3::world::AdmissionWorld;
use crate::intent::execution::{execution_deadline, execution_reading};
use crate::intent::operability::PrimaryIntent;

#[test]
fn occupancy_and_activation_bursts_follow_exact_attempt_and_execution_slopes() {
    let (mut world, provider) = AdmissionWorld::launch_with_provider_observation(17);
    let admitted = admit_capacity_burst(&mut world);
    assert_seventeenth_stops(&mut world);
    assert_eq!(provider.begin_calls(), 0);
    dispatch_burst(&mut world, admitted);
    assert_execution_slope(&mut world, &provider);
    let _ = world.session.shutdown();
}

fn admit_capacity_burst(world: &mut AdmissionWorld) -> Vec<UiAdmittedIntent<PrimaryIntent>> {
    assert_active(world.session.intent_admission_metrics(), 0);
    let mut admitted = Vec::new();
    for target in 0..16 {
        let handle = world.admit_exact(target);
        assert_admission_cost(handle.cost(), target + 1, 16);
        admitted.push(handle);
        if target == 14 {
            assert_active(world.session.intent_admission_metrics(), 15);
        }
    }
    assert_active(world.session.intent_admission_metrics(), 16);
    assert_distinct_slots(&admitted);
    admitted
}

fn assert_seventeenth_stops(world: &mut AdmissionWorld) {
    let UiIntentAdmissionDecision::Stopped(full) = world.admit(16) else {
        panic!("the seventeenth activation must stop at provider capacity")
    };
    assert_eq!(
        full.reason(),
        &UiIntentAdmissionStopReason::ExecutionReservation(
            UiIntentExecutionReservationDenial::ProviderCapacityExceeded {
                intent: PrimaryIntent::ID,
                provider_version: UiIntentProviderVersion::stable(1),
                maximum: 16,
            }
        )
    );
    assert_admission_cost(full.cost(), 16, 0);
}

fn dispatch_burst(world: &mut AdmissionWorld, admitted: Vec<UiAdmittedIntent<PrimaryIntent>>) {
    for handle in admitted {
        let UiIntentExecutionDispatchOutcome::AttemptPrepared(_) = world
            .session
            .dispatch_admitted_intent(handle, execution_deadline(20))
        else {
            panic!("each independently admitted target must dispatch")
        };
    }
}

fn assert_execution_slope(
    world: &mut AdmissionWorld,
    provider: &worth_ui_certification::WorthUiCertificationProviderObservation,
) {
    let report = match world
        .session
        .advance_intent_executions(execution_reading(1))
    {
        UiIntentExecutionAdvanceOutcome::Advanced(report) => report,
        UiIntentExecutionAdvanceOutcome::Stopped(stop) => {
            panic!("activation burst advance stopped: {stop:?}")
        }
    };
    assert_eq!(report.active_slots_visited(), 16);
    assert_eq!(report.provider_calls(), 16);
    assert_eq!(report.provider_polls(), 0);
    assert_eq!(report.cancellation_calls(), 0);
    assert_eq!(report.settlements(), 16);
    assert_eq!(report.transitions().len(), 16);
    assert!(report.transitions().iter().all(|transition| matches!(
        transition.posture(),
        UiIntentExecutionTransitionPosture::RejectedBeforeEffect { .. }
    )));
    assert_eq!(provider.begin_calls(), 16);
}

fn assert_distinct_slots(admitted: &[UiAdmittedIntent<PrimaryIntent>]) {
    let mut slots = admitted
        .iter()
        .map(|handle| handle.slot_identity())
        .collect::<Vec<_>>();
    slots.sort_by_key(|slot| (slot.slot(), slot.generation()));
    slots.dedup();
    assert_eq!(slots.len(), admitted.len());
}

fn assert_active(metrics: UiIntentAdmissionMetrics, expected: usize) {
    assert_eq!(metrics.active_attempts(), expected);
    assert_eq!(metrics.active_occupancy(), expected);
    assert_eq!(metrics.retained_candidates(), expected);
    assert_eq!(metrics.retained_payloads(), expected);
    assert_eq!(metrics.retained_owner_references(), expected * 5);
}

fn assert_admission_cost(
    cost: UiIntentAdmissionCost,
    expected_attempt_slots: usize,
    expected_occupancy_slots: usize,
) {
    assert_eq!(cost.route_resolution().route_rows_resolved(), 1);
    assert_eq!(cost.payload_projection().declared_fields(), 0);
    assert_eq!(cost.operability_dependencies_visited(), 7);
    assert_eq!(cost.currentness_checks(), 9);
    assert_eq!(cost.occupancy_slots_inspected(), expected_occupancy_slots);
    assert_eq!(cost.slots_inspected(), expected_attempt_slots);
}
