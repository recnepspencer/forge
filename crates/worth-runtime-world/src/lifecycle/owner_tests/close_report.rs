//! Proofs for the terminal close report.
//!
//! Close settles what it can, exposes every retained owner obligation it
//! cannot, and denies only a critical section that is still in flight.

use crate::lifecycle::{RuntimeWorldCloseDenial, RuntimeWorldOwnerLifecycleObservation};
use crate::publication::CompositeLateCancellationPosture;
use crate::recovery::{ProductUnpublishedCause, ProductUnpublishedNextAction};

use super::product_cas_loss::resolve_one_race;

/// SPEC-P4-008. A live `ProductUnpublishedOwnerEffects` record does not refuse
/// close; it becomes a report row naming its identity, cause, live obligation
/// counts, and derived next actions, and it survives the close that named it.
#[test]
fn close_exposes_every_retained_record_in_its_terminal_report() {
    let race = resolve_one_race(CompositeLateCancellationPosture::NotRequested);
    let owner = race.owner;
    let handle = race.retained.recovery_handle();
    let expected_actions = race.retained.next_actions().to_vec();
    let expected_identity = race.retained.identity().clone();
    let expected_obligations = race.retained.live_obligation_count();
    assert_eq!(owner.recovery_record_count(), 1);

    let pins_before = owner.state.retention.unique_pin_count();
    let report = owner
        .close()
        .expect("a retained record is exposed by close, never refused");
    let pins_after = owner.state.retention.unique_pin_count();

    assert_eq!(
        owner.lifecycle_observation(),
        RuntimeWorldOwnerLifecycleObservation::Closed
    );
    assert_eq!(
        report.retained_records().len(),
        1,
        "close names every retained record exactly once"
    );
    let row = &report.retained_records()[0];
    assert_eq!(row.identity(), &expected_identity);
    assert_eq!(row.cause(), ProductUnpublishedCause::ProductPublicationLost);
    assert_eq!(
        row.live_component_obligations(),
        2,
        "the retained record still holds its exact relational and signal pins"
    );
    assert_eq!(
        row.live_component_obligations() + row.live_composite_obligations(),
        expected_obligations,
        "the report's split must sum to the record's own live obligation count"
    );
    assert_eq!(row.next_actions(), expected_actions.as_slice());
    assert!(!expected_actions.contains(&ProductUnpublishedNextAction::SettleOwnerEffects));
    assert_eq!(
        report.settled_records(),
        1,
        "a record needing no further owner settlement is reported as settled"
    );
    assert_eq!(
        report.released_unique_component_pins(),
        pins_before - pins_after,
        "the report counts the pins close actually released"
    );

    assert_eq!(
        owner.recovery_record_count(),
        1,
        "exposure is never a discarded owner obligation"
    );
    assert!(
        owner.inspect_recovery(&handle).is_some(),
        "the named record is still inspectable after the close that named it"
    );
}

/// An operation parked inside its critical section is the only thing left that
/// close refuses. The retained record it leaves behind is enumerated instead.
#[test]
fn close_denies_only_an_undrainable_critical_section() {
    let race = resolve_one_race(CompositeLateCancellationPosture::NotRequested);
    let owner = race.owner;
    let retained = race.retained;

    let parked = owner
        .reserve_recovery_operation_if_open_and_bootstrapped()
        .expect("an open bootstrapped owner admits a recovery critical section");
    assert_eq!(
        owner
            .close()
            .expect_err("a live critical section cannot be drained"),
        RuntimeWorldCloseDenial::InFlightCriticalSection
    );
    assert_eq!(
        owner.lifecycle_observation(),
        RuntimeWorldOwnerLifecycleObservation::Open,
        "a denied close must not enter Closing"
    );

    drop(parked);
    let report = owner
        .close()
        .expect("close succeeds once the critical section releases");
    assert_eq!(
        owner.lifecycle_observation(),
        RuntimeWorldOwnerLifecycleObservation::Closed
    );
    assert_eq!(
        report.retained_records().len(),
        1,
        "the record that was never a denial is still a report row"
    );
    assert_eq!(report.retained_records()[0].identity(), retained.identity());
}
