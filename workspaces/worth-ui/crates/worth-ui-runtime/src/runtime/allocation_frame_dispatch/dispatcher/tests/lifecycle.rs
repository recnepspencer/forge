use super::*;

#[test]
fn replacement_successor_is_fully_prepared_without_mutating_the_live_dispatcher() {
    let initial = WorthUiRuntimeFrameEpoch::initial();
    let authority = UiAllocationFrameDispatcher::launch_for_test(
        initial,
        NonZeroU16::new(2).expect("test capacity is non-zero"),
    );

    let (assignment, transition, successor) = authority
        .prepare_replacement_successor()
        .expect("empty open dispatcher admits prepared replacement");

    assert_eq!(
        authority.state(),
        UiAllocationFrameDispatcherState::Open(initial)
    );
    assert_eq!(assignment.epoch(), initial.checked_next().unwrap());
    assert_eq!(
        transition.queue_disposition().reason(),
        UiAllocationFramePauseReason::Replacement
    );
    assert!(transition.queue_disposition().ingress().is_empty());
    assert!(transition
        .queue_disposition()
        .successor_ingress()
        .is_empty());
    assert_eq!(
        successor.state(),
        UiAllocationFrameDispatcherState::Open(assignment.epoch())
    );
}

#[test]
fn queued_predecessor_work_denies_replacement_preparation_without_consuming_it() {
    let mut authority = UiAllocationFrameDispatcher::launch_for_test(
        WorthUiRuntimeFrameEpoch::initial(),
        NonZeroU16::new(2).expect("test capacity is non-zero"),
    );
    let lease = source_lease(&mut authority, 1, 1);
    let admitted = ingress(&lease, 1, 1);
    assert!(submit(&mut authority, admitted.clone()).is_queued());

    let denial = authority
        .prepare_replacement_successor()
        .expect_err("queued work cannot cross application replacement");

    assert_eq!(
        denial,
        UiAllocationFrameDispatchDenial::ReplacementNotQuiescent
    );
    let frame = dispatched(authority.dispatch_for_test());
    assert_eq!(frame.ingress(), &[admitted]);
}

#[test]
fn closing_frame_assigns_late_ingress_to_the_reserved_successor() {
    let initial = WorthUiRuntimeFrameEpoch::initial();
    let mut authority = UiAllocationFrameDispatcher::launch_for_test(
        initial,
        NonZeroU16::new(1).expect("test capacity is non-zero"),
    );
    let lease = source_lease(&mut authority, 1, 1);
    submit(&mut authority, ingress(&lease, 1, 1));
    assert_eq!(
        authority.begin_close_for_runtime_pump(UiAllocationFrameCloseTrigger::runtime_pump_turn()),
        Ok(())
    );
    let successor = initial.checked_next().expect("initial epoch increments");
    assert_eq!(
        authority.state(),
        UiAllocationFrameDispatcherState::Closing {
            epoch: initial,
            next_epoch: successor,
        }
    );
    let late_ingress = ingress(&lease, 2, 2);
    let late = submit(&mut authority, late_ingress.clone());
    assert!(late.is_queued());
    assert!(late.is_late_ingress());
    assert_eq!(late.ingress_identity(), late_ingress.identity());
    assert_eq!(late.epoch(), Some(successor));
    assert_eq!(late.counters().late_ingress_count(), 1);
    let overflow = submit(&mut authority, ingress(&lease, 3, 3));
    assert!(overflow.is_backpressured());
    assert_eq!(overflow.retry_epoch(), Some(successor));
    assert_eq!(overflow.backpressure_watermark(), Some(1));
    authority.finish_close_for_runtime_pump();
    let frame = dispatched(authority.dispatch_sealed_frame());
    assert_eq!(frame.epoch(), initial);
    assert_eq!(
        authority.state(),
        UiAllocationFrameDispatcherState::Dispatched(initial)
    );
    let successor_frame = dispatched(authority.dispatch_for_test());
    assert_eq!(successor_frame.epoch(), successor);
    assert_eq!(successor_frame.ingress(), &[late_ingress]);
    let assignment = successor_frame
        .submission_assignments()
        .next()
        .expect("late ingress receives a sealed successor assignment");
    assert_eq!(assignment.epoch(), successor);
    assert_eq!(assignment.sequence().canonical_ordinal(), 1);
}

#[test]
fn replacement_pause_preserves_both_sides_of_an_active_close() {
    let mut authority = UiAllocationFrameDispatcher::launch_for_test(
        WorthUiRuntimeFrameEpoch::initial(),
        NonZeroU16::new(1).expect("test capacity is non-zero"),
    );
    let lease = source_lease(&mut authority, 1, 1);
    let closing_ingress = ingress(&lease, 1, 1);
    let successor_ingress = ingress(&lease, 2, 2);
    submit(&mut authority, closing_ingress.clone());
    authority
        .begin_close_for_runtime_pump(UiAllocationFrameCloseTrigger::runtime_pump_turn())
        .expect("open frame begins closing");
    assert!(submit(&mut authority, successor_ingress.clone()).is_late_ingress());

    let transition = authority.pause_for_replacement();
    assert_eq!(transition.queue_disposition().ingress(), &[closing_ingress]);
    assert_eq!(
        transition.queue_disposition().successor_ingress(),
        &[successor_ingress]
    );
}

#[test]
fn replacement_shutdown_and_epoch_exhaustion_are_terminal_typed_outcomes() {
    let initial = WorthUiRuntimeFrameEpoch::initial();
    let mut replacement = UiAllocationFrameDispatcher::launch_for_test(
        initial,
        NonZeroU16::new(2).expect("test capacity is non-zero"),
    );
    let replacement_lease = source_lease(&mut replacement, 1, 1);
    let replacement_ingress = ingress(&replacement_lease, 1, 1);
    submit(&mut replacement, replacement_ingress.clone());
    let replacement_transition = replacement.pause_for_replacement();
    let replacement_disposition = replacement_transition.queue_disposition();
    assert_eq!(
        replacement_transition.successor_epoch(),
        WorthUiRuntimeFrameEpoch::initial().checked_next()
    );
    assert_eq!(
        replacement_disposition.reason(),
        UiAllocationFramePauseReason::Replacement
    );
    assert!(replacement_disposition
        .ingress()
        .iter()
        .eq(std::iter::once(&replacement_ingress)));
    assert_eq!(replacement_disposition.counters().ingress_count(), 1);
    assert_eq!(
        replacement_disposition.counters().canonical_drain_count(),
        1
    );
    let replacement_denied = submit(&mut replacement, replacement_ingress.clone());
    assert_eq!(
        replacement_denied.denial(),
        Some(UiAllocationFrameSubmissionDenial::ReplacementPaused)
    );
    assert_eq!(
        replacement_denied.ingress_identity(),
        replacement_ingress.identity()
    );
    assert_eq!(replacement_denied.counters().terminal_denial_count(), 1);

    let mut shutdown = UiAllocationFrameDispatcher::launch_for_test(
        initial,
        NonZeroU16::new(2).expect("test capacity is non-zero"),
    );
    let shutdown_lease = source_lease(&mut shutdown, 1, 1);
    let shutdown_ingress = ingress(&shutdown_lease, 1, 1);
    let shutdown_disposition = shutdown.shutdown();
    assert_eq!(
        shutdown_disposition.reason(),
        UiAllocationFramePauseReason::Shutdown
    );
    assert!(shutdown_disposition.ingress().is_empty());
    assert_eq!(shutdown_disposition.counters().canonical_drain_count(), 1);
    let shutdown_denied = submit(&mut shutdown, shutdown_ingress.clone());
    assert_eq!(
        shutdown_denied.denial(),
        Some(UiAllocationFrameSubmissionDenial::Shutdown)
    );
    assert_eq!(
        shutdown_denied.ingress_identity(),
        shutdown_ingress.identity()
    );
    assert_eq!(shutdown_denied.counters().terminal_denial_count(), 1);

    let mut exhausted = UiAllocationFrameDispatcher::launch_for_test(
        WorthUiRuntimeFrameEpoch::for_test(u64::MAX),
        NonZeroU16::new(1).expect("test capacity is non-zero"),
    );
    let exhausted_lease = source_lease(&mut exhausted, 1, 1);
    let exhausted_ingress = ingress(&exhausted_lease, 1, 1);
    let exhaustion_denied = submit(&mut exhausted, exhausted_ingress.clone());
    assert_eq!(
        exhaustion_denied.denial(),
        Some(UiAllocationFrameSubmissionDenial::EpochExhausted)
    );
    assert_eq!(
        exhaustion_denied.ingress_identity(),
        exhausted_ingress.identity()
    );
    assert_eq!(exhaustion_denied.counters().terminal_denial_count(), 1);
    assert_eq!(exhausted.counters().ingress_count(), 0);
    assert_eq!(exhausted.counters().terminal_denial_count(), 1);
    assert_eq!(
        exhausted.state(),
        UiAllocationFrameDispatcherState::Paused(UiAllocationFramePauseReason::EpochExhausted)
    );
    let exhausted_dispatch = exhausted.dispatch_for_test();
    assert_eq!(
        exhausted_dispatch.denial(),
        Some(UiAllocationFrameDispatchDenial::EpochExhausted)
    );
    assert_eq!(exhausted_dispatch.counters().terminal_denial_count(), 1);
}

#[test]
fn pause_preserves_a_sealed_frame_as_terminal_output() {
    let mut authority = UiAllocationFrameDispatcher::launch_for_test(
        WorthUiRuntimeFrameEpoch::initial(),
        NonZeroU16::new(1).expect("test capacity is non-zero"),
    );
    let lease = source_lease(&mut authority, 1, 1);
    let admitted = ingress(&lease, 1, 1);
    submit(&mut authority, admitted.clone());
    assert_eq!(
        authority.seal_for_runtime_pump(UiAllocationFrameCloseTrigger::runtime_pump_turn()),
        Ok(())
    );

    let transition = authority.pause_for_replacement();
    let disposition = transition.queue_disposition();
    assert_eq!(
        transition.successor_epoch(),
        WorthUiRuntimeFrameEpoch::initial().checked_next()
    );
    assert_eq!(
        disposition.reason(),
        UiAllocationFramePauseReason::Replacement
    );
    assert_eq!(disposition.ingress(), &[admitted]);
    assert_eq!(
        disposition
            .sealed_frame()
            .map(UiAdmittedAllocationStreamFrame::epoch),
        Some(WorthUiRuntimeFrameEpoch::initial())
    );
}

#[test]
fn sealed_epoch_dispatches_once_and_cannot_reopen() {
    let initial = WorthUiRuntimeFrameEpoch::initial();
    let mut authority = UiAllocationFrameDispatcher::launch_for_test(
        initial,
        NonZeroU16::new(1).expect("test capacity is non-zero"),
    );
    let lease = source_lease(&mut authority, 1, 1);
    submit(&mut authority, ingress(&lease, 1, 1));

    let first = authority.dispatch_for_test();
    assert_eq!(
        first
            .dispatched_frame()
            .map(UiAdmittedAllocationStreamFrame::epoch),
        Some(initial)
    );
    assert_eq!(
        authority.state(),
        UiAllocationFrameDispatcherState::Dispatched(initial)
    );

    let duplicate = authority.dispatch_for_test();
    assert_eq!(
        duplicate.denial(),
        Some(UiAllocationFrameDispatchDenial::EmptyFrame)
    );
    assert_eq!(duplicate.counters().frame_count(), 1);
    assert_eq!(
        authority.state(),
        UiAllocationFrameDispatcherState::Dispatched(initial)
    );
}
