use super::*;
use crate::runtime::allocation_frame_dispatch::UiAllocationFrameSourceAdmissionDenial;

#[test]
fn retry_retirement_is_scoped_to_source_identity_and_generation() {
    let initial = WorthUiRuntimeFrameEpoch::initial();
    let mut dispatcher = UiAllocationFrameDispatcher::launch_for_test(
        initial,
        NonZeroU16::new(4).expect("test capacity is non-zero"),
    );
    let source_one = source_lease(&mut dispatcher, 1, 1);
    assert!(submit(&mut dispatcher, ingress(&source_one, 10, 100)).is_queued());
    dispatched(dispatcher.dispatch_for_test());

    let source_two = source_lease(&mut dispatcher, 2, 1);
    let independent_source = submit(&mut dispatcher, ingress(&source_two, 20, 1));
    assert!(independent_source.is_queued());
    assert_eq!(independent_source.epoch(), initial.checked_next());
    assert!(submit(&mut dispatcher, ingress(&source_two, 21, 2)).is_queued());

    let stale_original_source = submit(&mut dispatcher, ingress(&source_one, 11, 99));
    assert_eq!(
        stale_original_source.denial(),
        Some(UiAllocationFrameSubmissionDenial::RetryWindowExpired)
    );
    assert!(dispatcher.retire_source(source_one).is_retired());
    let source_one_successor = source_lease(&mut dispatcher, 1, 2);
    assert!(submit(&mut dispatcher, ingress(&source_one_successor, 12, 1)).is_queued());
}

#[test]
fn ingress_identity_is_scoped_by_source_generation() {
    let mut left = UiAllocationFrameDispatcher::launch_for_test(
        WorthUiRuntimeFrameEpoch::initial(),
        NonZeroU16::new(4).expect("test capacity is non-zero"),
    );
    let old_lease = source_lease(&mut left, 1, 1);
    let old = ingress(&old_lease, 10, 1);
    let old_outcome = submit(&mut left, old.clone());
    assert!(old_outcome.is_queued());
    dispatched(left.dispatch_for_test());
    assert!(left.retire_source(old_lease).is_retired());
    let new_lease = source_lease(&mut left, 1, 2);
    let new = ingress(&new_lease, 10, 1);
    let new_outcome = submit(&mut left, new.clone());
    assert!(new_outcome.is_queued());
    assert_eq!(old_outcome.ingress_key(), old.key());
    assert_eq!(new_outcome.ingress_key(), new.key());
    assert_ne!(old_outcome.ingress_key(), new_outcome.ingress_key());
    assert_eq!(
        new_outcome.epoch(),
        WorthUiRuntimeFrameEpoch::initial().checked_next()
    );
}

#[test]
fn generation_rollover_reuses_one_stable_source_slot() {
    let mut dispatcher = UiAllocationFrameDispatcher::launch_for_test(
        WorthUiRuntimeFrameEpoch::initial(),
        NonZeroU16::new(2).expect("test capacity is non-zero"),
    );
    for generation in 1..=100 {
        let lease = source_lease(&mut dispatcher, 1, generation);
        let outcome = submit(&mut dispatcher, ingress(&lease, 10, generation));
        assert!(outcome.is_queued(), "generation {generation} denied");
        dispatched(dispatcher.dispatch_for_test());
        assert!(dispatcher.retire_source(lease).is_retired());
    }
}

#[test]
fn source_domain_retention_capacity_denies_without_unbounded_growth() {
    let initial = WorthUiRuntimeFrameEpoch::initial();
    let mut dispatcher = UiAllocationFrameDispatcher::launch_for_test(
        initial,
        NonZeroU16::new(64).expect("test capacity is non-zero"),
    );
    let mut leases = Vec::new();
    for source in 1..=64 {
        let lease = source_lease(&mut dispatcher, source, 1);
        assert!(submit(&mut dispatcher, ingress(&lease, source, 1)).is_queued());
        leases.push(lease);
    }
    dispatched(dispatcher.dispatch_for_test());

    let overflow_domain = dispatcher.admit_source_generation(
        UiAllocationFrameSourceLane::HostMeasurement,
        UiAllocationFrameSourceIdentity::for_test(65),
        UiAllocationFrameSourceGeneration::for_test(1),
    );
    assert_eq!(
        overflow_domain.unwrap_err(),
        UiAllocationFrameSourceAdmissionDenial::RegistryFull
    );
    assert!(dispatcher.retire_source(leases.remove(0)).is_retired());
    let replacement_lease = dispatcher
        .admit_source_generation(
            UiAllocationFrameSourceLane::HostMeasurement,
            UiAllocationFrameSourceIdentity::for_test(65),
            UiAllocationFrameSourceGeneration::for_test(1),
        )
        .expect("retirement must release one governed source slot");
    let replacement_ingress = UiAllocationFrameDispatcher::ingress_for_test(
        &replacement_lease,
        UiAllocationFrameIngressIdentity::for_test(65),
        UiAdmittedAllocationSourceOrder::for_test(1),
    );
    assert!(submit(&mut dispatcher, replacement_ingress).is_queued());
}

#[test]
fn active_source_registry_denies_a_sixty_fifth_live_source_before_submission() {
    let mut dispatcher = UiAllocationFrameDispatcher::launch_for_test(
        WorthUiRuntimeFrameEpoch::initial(),
        NonZeroU16::new(64).expect("test capacity is non-zero"),
    );
    let mut leases = Vec::new();
    for source in 1..=63 {
        let lease = source_lease(&mut dispatcher, source, 1);
        assert!(submit(&mut dispatcher, ingress(&lease, source, 1)).is_queued());
        leases.push(lease);
    }
    dispatched(dispatcher.dispatch_for_test());

    let source_64 = source_lease(&mut dispatcher, 64, 1);
    assert!(submit(&mut dispatcher, ingress(&source_64, 64, 1)).is_queued());
    leases.push(source_64);
    let no_source_slot = dispatcher.admit_source_generation(
        UiAllocationFrameSourceLane::HostMeasurement,
        UiAllocationFrameSourceIdentity::for_test(65),
        UiAllocationFrameSourceGeneration::for_test(1),
    );
    assert_eq!(
        no_source_slot.unwrap_err(),
        UiAllocationFrameSourceAdmissionDenial::RegistryFull
    );
    dispatched(dispatcher.dispatch_for_test());
}
