//! Private owner proof for allocation-frame linearization.

use super::*;
use crate::runtime::allocation_frame_dispatch::UiAllocationFrameSourceLease;
use crate::runtime::WorthUiRuntimeFrameEpoch;
use crate::runtime::{
    UiAdmittedAllocationSourceOrder, UiAllocationFrameIngressIdentity,
    UiAllocationFrameIngressSequence, UiAllocationFrameSourceGeneration,
    UiAllocationFrameSourceIdentity, UiAllocationFrameSourceLane,
};
use std::num::NonZeroU16;

mod lifecycle;
mod retry_domain;

fn source_lease(
    dispatcher: &mut UiAllocationFrameDispatcher,
    source: u64,
    generation: u64,
) -> UiAllocationFrameSourceLease {
    source_lease_for(
        dispatcher,
        UiAllocationFrameSourceLane::HostMeasurement,
        source,
        generation,
    )
}

fn source_lease_for(
    dispatcher: &mut UiAllocationFrameDispatcher,
    lane: UiAllocationFrameSourceLane,
    source: u64,
    generation: u64,
) -> UiAllocationFrameSourceLease {
    dispatcher
        .admit_source_generation(
            lane,
            UiAllocationFrameSourceIdentity::for_test(source),
            UiAllocationFrameSourceGeneration::for_test(generation),
        )
        .expect("test support authority admits the source generation")
}

fn replay_cross_source_order(
    order: [usize; 3],
) -> (
    UiAdmittedAllocationStreamFrame,
    Vec<(
        u64,
        UiAllocationFrameEpoch,
        UiAllocationFrameIngressSequence,
    )>,
) {
    let mut dispatcher = UiAllocationFrameDispatcher::launch_for_test(
        WorthUiRuntimeFrameEpoch::initial(),
        NonZeroU16::new(3).expect("test capacity is non-zero"),
    );
    let leases = [
        source_lease_for(
            &mut dispatcher,
            UiAllocationFrameSourceLane::HostMeasurement,
            7,
            1,
        ),
        source_lease_for(
            &mut dispatcher,
            UiAllocationFrameSourceLane::QueryProjection,
            5,
            2,
        ),
        source_lease_for(
            &mut dispatcher,
            UiAllocationFrameSourceLane::Interaction,
            9,
            1,
        ),
    ];
    let admitted = [
        ingress(&leases[0], 30, 3),
        ingress(&leases[1], 10, 1),
        ingress(&leases[2], 20, 2),
    ];
    for index in order {
        assert!(submit(&mut dispatcher, admitted[index].clone()).is_queued());
    }
    let frame = dispatched(dispatcher.dispatch_for_test());
    let assignments = assignment_projection(&frame);
    (frame, assignments)
}

fn ingress(
    lease: &UiAllocationFrameSourceLease,
    identity: u64,
    sequence: u64,
) -> UiAdmittedAllocationStreamIngress {
    UiAllocationFrameDispatcher::ingress_for_test(
        lease,
        UiAllocationFrameIngressIdentity::for_test(identity),
        UiAdmittedAllocationSourceOrder::for_test(sequence),
    )
}

fn dispatched(outcome: UiAllocationFrameTransitionOutcome) -> UiAdmittedAllocationStreamFrame {
    outcome
        .into_dispatched_frame()
        .unwrap_or_else(|denial| panic!("unexpected denial: {denial:?}"))
}

fn submit(
    authority: &mut UiAllocationFrameDispatcher,
    ingress: UiAdmittedAllocationStreamIngress,
) -> UiAllocationFrameSubmissionOutcome {
    authority.submit(ingress).outcome()
}

fn assignment_projection(
    frame: &UiAdmittedAllocationStreamFrame,
) -> Vec<(
    u64,
    UiAllocationFrameEpoch,
    UiAllocationFrameIngressSequence,
)> {
    let mut assignments = frame
        .submission_assignments()
        .map(|assignment| {
            (
                assignment.ingress_key().ingress_identity().as_u64(),
                assignment.epoch(),
                assignment.sequence(),
            )
        })
        .collect::<Vec<_>>();
    assignments.sort_unstable();
    assignments
}

#[test]
fn canonical_frame_replays_independent_of_arrival_order() {
    let mut left = UiAllocationFrameDispatcher::launch_for_test(
        WorthUiRuntimeFrameEpoch::initial(),
        NonZeroU16::new(4).expect("test capacity is non-zero"),
    );
    let mut right = UiAllocationFrameDispatcher::launch_for_test(
        WorthUiRuntimeFrameEpoch::initial(),
        NonZeroU16::new(4).expect("test capacity is non-zero"),
    );
    let left_lease = source_lease(&mut left, 1, 1);
    let right_lease = source_lease(&mut right, 1, 1);
    let left_submissions = [
        submit(&mut left, ingress(&left_lease, 20, 2)),
        submit(&mut left, ingress(&left_lease, 10, 1)),
    ];
    let right_submissions = [
        submit(&mut right, ingress(&right_lease, 10, 1)),
        submit(&mut right, ingress(&right_lease, 20, 2)),
    ];

    assert!(left_submissions
        .iter()
        .all(UiAllocationFrameSubmissionOutcome::is_queued));
    assert!(right_submissions
        .iter()
        .all(UiAllocationFrameSubmissionOutcome::is_queued));
    let left = dispatched(left.dispatch_for_test());
    let right = dispatched(right.dispatch_for_test());
    assert_eq!(assignment_projection(&left), assignment_projection(&right));

    assert_eq!(left, right);
    assert_eq!(left.epoch(), WorthUiRuntimeFrameEpoch::initial());
    assert_eq!(
        left.ingress()
            .iter()
            .map(|entry| entry.source_order().as_u64())
            .collect::<Vec<_>>(),
        vec![1, 2]
    );
    assert_eq!(left.counters().identity_lookup_count(), 1);
    assert_eq!(left.counters().sequence_lookup_count(), 1);
    assert_eq!(left.counters().mailbox_order_comparison_count(), 4);
    assert_eq!(left.counters().mailbox_canonical_write_count(), 2);
    assert_eq!(left.counters().retry_ledger_comparison_count(), 6);
    assert_eq!(left.counters().retry_ledger_write_count(), 6);
    assert_eq!(left.counters().canonical_drain_count(), 1);
    assert_eq!(left.counters().mailbox_capacity(), 4);
    assert_eq!(left.counters().mailbox_high_watermark(), 2);
    assert_eq!(
        left.counters()
            .mailbox_storage_posture()
            .inline_slot_count(),
        64
    );
}

#[test]
fn every_cross_source_arrival_permutation_replays_identically() {
    let permutations = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];
    let (expected_frame, expected_assignments) = replay_cross_source_order(permutations[0]);

    for permutation in permutations.into_iter().skip(1) {
        let (frame, assignments) = replay_cross_source_order(permutation);
        assert_eq!(frame, expected_frame);
        assert_eq!(assignments, expected_assignments);
        assert_eq!(frame.counters(), expected_frame.counters());
    }

    assert_eq!(expected_frame.counters().ingress_count(), 3);
    assert_eq!(expected_frame.counters().frame_count(), 1);
    assert_eq!(expected_frame.counters().mailbox_high_watermark(), 3);
    assert_eq!(expected_frame.counters().identity_lookup_count(), 3);
    assert_eq!(expected_frame.counters().sequence_lookup_count(), 3);
    assert_eq!(
        expected_frame.counters().mailbox_order_comparison_count(),
        9
    );
    assert_eq!(expected_frame.counters().mailbox_canonical_write_count(), 3);
    assert_eq!(
        expected_frame.counters().retry_ledger_comparison_count(),
        19
    );
    assert_eq!(expected_frame.counters().retry_ledger_write_count(), 9);
}

#[test]
fn fixed_mailbox_reuses_its_bounded_storage_across_epochs() {
    let mut dispatcher = UiAllocationFrameDispatcher::launch_for_test(
        WorthUiRuntimeFrameEpoch::initial(),
        NonZeroU16::new(2).expect("test capacity is non-zero"),
    );
    let lease = source_lease(&mut dispatcher, 1, 1);
    let empty = dispatcher.dispatch_for_test();
    assert_eq!(
        empty.denial(),
        Some(UiAllocationFrameDispatchDenial::EmptyFrame)
    );
    assert_eq!(empty.counters().canonical_drain_count(), 0);
    assert_eq!(
        dispatcher.state(),
        UiAllocationFrameDispatcherState::Open(WorthUiRuntimeFrameEpoch::initial())
    );
    submit(&mut dispatcher, ingress(&lease, 20, 2));
    submit(&mut dispatcher, ingress(&lease, 10, 1));
    let first = dispatched(dispatcher.dispatch_for_test());
    assert_eq!(
        first.ingress(),
        &[ingress(&lease, 10, 1), ingress(&lease, 20, 2)]
    );

    submit(&mut dispatcher, ingress(&lease, 30, 3));
    let second = dispatched(dispatcher.dispatch_for_test());
    assert_eq!(second.ingress(), &[ingress(&lease, 30, 3)]);
    assert_eq!(second.counters().frame_count(), 2);
    assert_eq!(second.counters().canonical_drain_count(), 2);
    assert_eq!(second.counters().mailbox_capacity(), 2);
    assert_eq!(second.counters().mailbox_high_watermark(), 2);
    assert_eq!(
        second
            .counters()
            .mailbox_storage_posture()
            .admitted_capacity(),
        2
    );
}

#[test]
fn retry_and_evidence_metadata_are_bounded_and_losslessly_cloneable() {
    fn assert_clone<T: Clone>() {}

    assert_clone::<super::super::UiAllocationFrameIngressDescriptor>();
    assert_clone::<super::super::UiAllocationFrameRetryState>();
    assert!(std::mem::size_of::<super::super::UiAllocationFrameSourceIdentity>() <= 24);
    assert!(std::mem::size_of::<super::super::UiAllocationFrameIngressDescriptor>() <= 96);
}

#[test]
fn sealed_retry_survives_but_disposed_pending_ingress_requeues_after_replacement() {
    let initial = WorthUiRuntimeFrameEpoch::initial();
    let mut dispatcher = UiAllocationFrameDispatcher::launch_for_test(
        initial,
        NonZeroU16::new(2).expect("test capacity is non-zero"),
    );
    let lease = source_lease(&mut dispatcher, 1, 1);
    let accepted = submit(&mut dispatcher, ingress(&lease, 10, 1));
    assert!(accepted.is_queued());
    let frame = dispatched(dispatcher.dispatch_for_test());
    let accepted_sequence = frame
        .submission_assignments()
        .next()
        .expect("sealed frame contains accepted assignment")
        .sequence();

    let retry = submit(&mut dispatcher, ingress(&lease, 10, 1));
    assert!(retry.is_duplicate());
    assert_eq!(retry.epoch(), Some(initial));
    assert_eq!(retry.sequence(), Some(accepted_sequence));
    assert_eq!(retry.counters().ingress_count(), 1);

    let next = submit(&mut dispatcher, ingress(&lease, 20, 2));
    assert!(next.is_queued());
    assert_eq!(next.epoch(), initial.checked_next());
    let expired = submit(&mut dispatcher, ingress(&lease, 10, 1));
    assert_eq!(
        expired.denial(),
        Some(UiAllocationFrameSubmissionDenial::RetryWindowExpired)
    );
    assert_eq!(expired.counters().ingress_count(), 2);

    let transition = dispatcher.pause_for_replacement();
    let successor_epoch = transition.successor_epoch().expect("successor epoch");
    let mut successor = UiAllocationFrameDispatcher::launch_with_runtime_state(
        successor_epoch,
        transition.retry_state(),
    );
    let successor_retry = submit(&mut successor, ingress(&lease, 20, 2));
    assert!(successor_retry.is_queued());
    assert_eq!(successor_retry.epoch(), Some(successor_epoch));
    assert!(successor_retry.sequence().is_none());
    assert_eq!(successor_retry.counters().ingress_count(), 1);
}

#[test]
fn retry_conflicts_and_backpressure_are_typed_without_extra_admission() {
    let mut authority = UiAllocationFrameDispatcher::launch_for_test(
        WorthUiRuntimeFrameEpoch::initial(),
        NonZeroU16::new(1).expect("test capacity is non-zero"),
    );
    let lease = source_lease(&mut authority, 1, 1);
    let accepted_ingress = ingress(&lease, 10, 1);
    let accepted = submit(&mut authority, accepted_ingress.clone());
    assert!(accepted.is_queued());
    assert_eq!(accepted.ingress_identity(), accepted_ingress.identity());
    assert_eq!(accepted.counters().ingress_count(), 1);
    let duplicate = submit(&mut authority, accepted_ingress.clone());
    assert!(duplicate.is_duplicate());
    assert_eq!(duplicate.ingress_identity(), accepted_ingress.identity());
    assert_eq!(duplicate.counters().duplicate_count(), 1);
    assert_eq!(
        submit(&mut authority, ingress(&lease, 10, 2)).denial(),
        Some(UiAllocationFrameSubmissionDenial::ConflictingIdentity)
    );
    assert_eq!(
        submit(&mut authority, ingress(&lease, 20, 1)).denial(),
        Some(UiAllocationFrameSubmissionDenial::ConflictingSourceOrder)
    );
    let backpressured_ingress = ingress(&lease, 20, 2);
    let backpressured = submit(&mut authority, backpressured_ingress.clone());
    assert!(backpressured.is_backpressured());
    assert_eq!(
        backpressured.ingress_identity(),
        backpressured_ingress.identity()
    );
    assert_eq!(backpressured.backpressure_watermark(), Some(1));
    assert_eq!(backpressured.counters().backpressure_denial_count(), 1);
    assert_eq!(authority.counters().ingress_count(), 1);
    assert_eq!(authority.counters().duplicate_count(), 1);
    assert_eq!(authority.counters().backpressure_denial_count(), 1);
}
