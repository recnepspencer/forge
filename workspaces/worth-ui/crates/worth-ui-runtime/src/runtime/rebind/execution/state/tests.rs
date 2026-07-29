use super::*;

#[test]
fn configured_registry_bounds_and_releases_pending_plan_authority() {
    let state = UiRebindRuntimeState::new(super::super::super::UiRebindProfile::platform_pulse());
    let first = state.reserve_plan().expect("first plan fits");
    let second = state.reserve_plan().expect("second plan fits");
    assert_eq!(first.identity(), 1);
    assert_eq!(second.identity(), 2);
    assert_eq!(state.pending_plan_count(), 2);
    assert!(matches!(
        state.reserve_plan(),
        Err(UiRebindReservationDenial::PendingPlanCapacityExceeded { configured: 2 })
    ));
    drop(first);
    assert_eq!(state.pending_plan_count(), 1);
    drop(second);
    assert!(state.shutdown().is_empty());
    assert!(matches!(
        state.reserve_plan(),
        Err(UiRebindReservationDenial::AdmissionClosed)
    ));
}

#[test]
fn active_session_shutdown_reports_required_empty_rebind_state() {
    let session = crate::runtime::tests::active_application_session_test_support::
        source_backed_component_session();
    let shutdown = session.shutdown();
    assert!(shutdown.rebind().is_empty());
}

#[test]
fn every_managed_lane_is_bounded_enumerable_and_released_exactly_once() {
    assert_lane_lifecycle(
        |reservation| reservation.begin_effecting(),
        |report| assert_eq!(report.effecting_rebinds(), 1),
    );
    assert_lane_lifecycle(
        |reservation| {
            reservation.begin_effecting()?;
            reservation.retain_completion()
        },
        |report| assert_eq!(report.completion_handles(), 1),
    );
    assert_lane_lifecycle(
        |reservation| {
            reservation.begin_effecting()?;
            reservation.retain_recovery()
        },
        |report| assert_eq!(report.recovery_handles(), 1),
    );
    assert_lane_lifecycle(
        |reservation| {
            reservation.begin_effecting()?;
            reservation.retain_receipt()
        },
        |report| assert_eq!(report.retained_rebind_receipts(), 1),
    );
}

#[test]
fn effecting_admission_reserves_every_possible_post_effect_terminal_lane() {
    let state = UiRebindRuntimeState::new(super::super::super::UiRebindProfile::platform_pulse());
    let mut completion = state.reserve_plan().unwrap();
    completion.begin_effecting().unwrap();
    completion.retain_completion().unwrap();
    let mut candidate = state.reserve_plan().unwrap();
    assert!(matches!(
        candidate.begin_effecting(),
        Err(UiRebindReservationDenial::CompletionHandleCapacityExceeded { configured: 1 })
    ));
    drop((candidate, completion));
    assert!(state.shutdown().is_empty());
}

#[test]
fn comparison_snapshot_lane_saturates_at_two_and_releases_on_drop() {
    let state = UiRebindRuntimeState::new(super::super::super::UiRebindProfile::platform_pulse());
    let reservation = state
        .reserve_comparison_snapshots(2)
        .expect("the Pulse profile retains one predecessor/successor pair");
    assert!(matches!(
        state.reserve_comparison_snapshots(1),
        Err(
            UiRebindReservationDenial::RetainedComparisonSnapshotCapacityExceeded {
                configured: 2,
                required: 1,
            }
        )
    ));
    assert_eq!(state.shutdown().retained_comparison_snapshots(), 2);
    drop(reservation);
    assert!(state.shutdown().is_empty());
}

fn assert_lane_lifecycle(
    transition: impl FnOnce(&mut UiRebindReservation) -> Result<(), UiRebindReservationDenial>,
    inspect: impl FnOnce(UiRebindShutdownReport),
) {
    let state = UiRebindRuntimeState::new(super::super::super::UiRebindProfile::platform_pulse());
    let mut reservation = state.reserve_plan().unwrap();
    transition(&mut reservation).unwrap();
    inspect(state.shutdown());
    drop(reservation);
    assert!(state.shutdown().is_empty());
}
