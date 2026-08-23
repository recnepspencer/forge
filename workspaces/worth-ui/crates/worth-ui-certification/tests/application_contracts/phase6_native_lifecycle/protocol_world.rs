use super::oracle::{Action, Effect, Event, Observation, State, Stop, EVENTS, STATES};
pub(super) use super::production_world::{
    UiNativeLifecycleAction, UiNativeLifecycleEffect, UiNativeLifecycleEvent,
    UiNativeLifecycleObservation, UiNativeLifecycleState, UiNativeLifecycleWorld,
};
use super::schedule_inventory::{Schedule, SCHEDULES};
use super::schedule_requirements::REQUIRED_SCHEDULE_IDS;
use serde_json::json;
use std::collections::BTreeSet;
use worth_ui_host_native::UiNativeInputObservationStop;

#[test]
fn native_lifecycle_protocol_world_matches_independent_oracle_for_all_schedules() {
    let mut comparisons = 0;
    for state in STATES.iter().copied() {
        for event in EVENTS.iter().copied() {
            comparisons += run_schedule("single-state-event", state, std::slice::from_ref(&event));
        }
    }
    for schedule in SCHEDULES {
        comparisons += run_inventory_schedule(schedule);
    }
    assert_eq!(comparisons, super::schedule_inventory::EXPECTED_COMPARISONS);
    println!("WORTH_UI_LEDGER_COUNTERS={{\"P6-PROTOCOL-WORLD-01\":{comparisons}}}");
    let cases: Vec<_> = SCHEDULES.iter().map(|schedule| schedule.name).collect();
    println!(
        "WORTH_UI_LEDGER_CASES={}",
        json!({"P6-PROTOCOL-WORLD-01": cases})
    );
}

#[test]
fn native_lifecycle_protocol_world_preserves_retention_before_close() {
    let mut world = UiNativeLifecycleWorld::new(UiNativeLifecycleState::Presented);
    assert_eq!(
        world.apply(UiNativeLifecycleEvent::Pointer).effect,
        UiNativeLifecycleEffect::Retained
    );
    assert_eq!(
        world.apply(UiNativeLifecycleEvent::Close).effect,
        UiNativeLifecycleEffect::CloseDeferred
    );
    assert!(world.drain_retained() >= 1);
    assert_eq!(
        world.apply(UiNativeLifecycleEvent::Close).effect,
        UiNativeLifecycleEffect::Closed
    );
}

#[test]
fn text_capacity_boundary_denial_preserves_sequence_continuity() {
    let mut world = UiNativeLifecycleWorld::new(UiNativeLifecycleState::Presented);
    let before = world.report();
    assert_eq!(
        world
            .apply(UiNativeLifecycleEvent::ExactCapacityText)
            .effect,
        UiNativeLifecycleEffect::Retained
    );
    let exact = world.report();
    assert_eq!(
        exact.retained_event_count(),
        before.retained_event_count() + 1
    );
    assert_eq!(
        world.apply(UiNativeLifecycleEvent::OverCapacityText).effect,
        UiNativeLifecycleEffect::Denied(UiNativeInputObservationStop::OverCapacityText)
    );
    let denied = world.report();
    assert_eq!(denied.retained_event_count(), exact.retained_event_count());
    assert_eq!(
        denied.last_retained_sequence(),
        exact.last_retained_sequence()
    );
    assert_eq!(denied.terminal_stop(), None);
    assert_eq!(world.drain_retained(), 1);
    assert_eq!(
        world.apply(UiNativeLifecycleEvent::ImeCommit).effect,
        UiNativeLifecycleEffect::Retained
    );
    assert_eq!(
        world.report().last_retained_sequence(),
        exact.last_retained_sequence().map(|sequence| sequence + 1)
    );
}

#[test]
fn invalid_ime_range_denial_preserves_revision_and_sequence_continuity() {
    let mut world = UiNativeLifecycleWorld::new(UiNativeLifecycleState::Presented);
    assert_eq!(
        world.apply(UiNativeLifecycleEvent::ValidImeRange).effect,
        UiNativeLifecycleEffect::Retained
    );
    let valid = world.report();
    assert_eq!(
        world
            .apply(UiNativeLifecycleEvent::UnprovableImeRange)
            .effect,
        UiNativeLifecycleEffect::Denied(UiNativeInputObservationStop::ImeRangeNotScalarBoundary)
    );
    let denied = world.report();
    assert_eq!(denied.retained_event_count(), valid.retained_event_count());
    assert_eq!(
        denied.last_retained_sequence(),
        valid.last_retained_sequence()
    );
    assert_eq!(denied.terminal_stop(), None);
    assert_eq!(
        world.apply(UiNativeLifecycleEvent::ValidImeRange).effect,
        UiNativeLifecycleEffect::Retained
    );
    assert_eq!(
        world.report().last_retained_sequence(),
        valid.last_retained_sequence().map(|sequence| sequence + 1)
    );
}

#[test]
fn schedule_inventory_covers_the_independently_declared_phase_six_cases() {
    let actual = SCHEDULES
        .iter()
        .map(|schedule| schedule.name)
        .collect::<BTreeSet<_>>();
    let required = REQUIRED_SCHEDULE_IDS
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    assert_eq!(actual, required);
    assert_eq!(actual.len(), REQUIRED_SCHEDULE_IDS.len());
}

fn run_inventory_schedule(schedule: &Schedule) -> usize {
    run_schedule(schedule.name, schedule.initial, schedule.events)
}

fn run_schedule(_name: &str, initial: State, events: &[Event]) -> usize {
    let mut expected_state = initial;
    let mut generation = 7;
    let mut world = UiNativeLifecycleWorld::new(production_state(initial));
    for event in events {
        let expected = super::oracle::expected(expected_state, *event);
        let expected = Observation {
            predecessor: expected.predecessor.map(|_| generation),
            ..expected
        };
        let observed = convert_observation(world.apply(production_event(*event)));
        assert_eq!(
            observed, expected,
            "protocol mismatch for {initial:?} + {event:?}"
        );
        if matches!(
            (expected_state, *event),
            (
                State::SuccessorInFlight { .. } | State::ProfileTransition { .. },
                Event::CompletePresentation
            )
        ) {
            generation += 1;
        }
        expected_state = expected.state;
    }
    events.len()
}

fn production_state(state: State) -> UiNativeLifecycleState {
    match state {
        State::BeforeFirstPresentation => UiNativeLifecycleState::BeforeFirstPresentation,
        State::Presented => UiNativeLifecycleState::Presented,
        State::SuccessorInFlight { .. } => UiNativeLifecycleState::SuccessorInFlight,
        State::ProfileTransition { .. } => UiNativeLifecycleState::ProfileTransition,
        State::Closing => UiNativeLifecycleState::Closing,
        State::Closed => UiNativeLifecycleState::Closed,
    }
}

fn production_event(event: Event) -> UiNativeLifecycleEvent {
    match event {
        Event::Pointer => UiNativeLifecycleEvent::Pointer,
        Event::Keyboard => UiNativeLifecycleEvent::Keyboard,
        Event::Preedit => UiNativeLifecycleEvent::Preedit,
        Event::ImeCommit => UiNativeLifecycleEvent::ImeCommit,
        Event::ImeCancel => UiNativeLifecycleEvent::ImeCancel,
        Event::Scroll => UiNativeLifecycleEvent::Scroll,
        Event::Button => UiNativeLifecycleEvent::Button,
        Event::ButtonUnavailable => UiNativeLifecycleEvent::ButtonUnavailable,
        Event::BeginSuccessor => UiNativeLifecycleEvent::BeginSuccessor,
        Event::BeginProfileTransition => UiNativeLifecycleEvent::BeginProfileTransition,
        Event::BeginZeroSizedProfile => UiNativeLifecycleEvent::BeginZeroSizedProfile,
        Event::ExactCapacityText => UiNativeLifecycleEvent::ExactCapacityText,
        Event::OverCapacityText => UiNativeLifecycleEvent::OverCapacityText,
        Event::ValidImeRange => UiNativeLifecycleEvent::ValidImeRange,
        Event::UnprovableImeRange => UiNativeLifecycleEvent::UnprovableImeRange,
        Event::TextWithoutRecipient => UiNativeLifecycleEvent::TextWithoutRecipient,
        Event::TextWithStaleRecipient => UiNativeLifecycleEvent::TextWithStaleRecipient,
        Event::CompletePresentation => UiNativeLifecycleEvent::CompletePresentation,
        Event::Close => UiNativeLifecycleEvent::Close,
    }
}

fn convert_observation(observed: UiNativeLifecycleObservation) -> Observation {
    Observation {
        state: match observed.state {
            UiNativeLifecycleState::BeforeFirstPresentation => State::BeforeFirstPresentation,
            UiNativeLifecycleState::Presented => State::Presented,
            UiNativeLifecycleState::SuccessorInFlight => {
                State::SuccessorInFlight { predecessor: 7 }
            }
            UiNativeLifecycleState::ProfileTransition => {
                State::ProfileTransition { predecessor: 7 }
            }
            UiNativeLifecycleState::Closing => State::Closing,
            UiNativeLifecycleState::Closed => State::Closed,
        },
        effect: match observed.effect {
            UiNativeLifecycleEffect::Retained => Effect::Retained,
            UiNativeLifecycleEffect::Ignored => Effect::Ignored,
            UiNativeLifecycleEffect::Denied(stop) => Effect::Denied(convert_stop(stop)),
            UiNativeLifecycleEffect::PresentationCompleted => Effect::PresentationCompleted,
            UiNativeLifecycleEffect::CloseDeferred => Effect::CloseDeferred,
            UiNativeLifecycleEffect::Closed => Effect::Closed,
            UiNativeLifecycleEffect::NoOp => Effect::NoOp,
        },
        retained_delta: observed.retained_delta,
        predecessor: observed.predecessor,
        next_action: observed.next_action.map(convert_action),
    }
}

fn convert_action(action: UiNativeLifecycleAction) -> Action {
    match action {
        UiNativeLifecycleAction::CompletePresentation => Action::CompletePresentation,
        UiNativeLifecycleAction::EmitProfileEvidence => Action::EmitProfileEvidence,
        UiNativeLifecycleAction::DrainRetained => Action::DrainRetained,
    }
}

fn convert_stop(stop: UiNativeInputObservationStop) -> Stop {
    match stop {
        UiNativeInputObservationStop::NoPresentationBasis => Stop::NoPresentationBasis,
        UiNativeInputObservationStop::StalePresentationAffinity => Stop::StalePresentationAffinity,
        UiNativeInputObservationStop::PointerPositionUnavailable => {
            Stop::PointerPositionUnavailable
        }
        UiNativeInputObservationStop::OverCapacityText => Stop::OverCapacityText,
        UiNativeInputObservationStop::ImeRangeNotScalarBoundary => Stop::ImeRangeNotScalarBoundary,
        UiNativeInputObservationStop::MissingPendingPresentationContext => {
            Stop::MissingPendingPresentationContext
        }
        UiNativeInputObservationStop::MissingInputRecipientAffinity => {
            Stop::MissingInputRecipientAffinity
        }
        UiNativeInputObservationStop::StaleInputRecipientAffinity => {
            Stop::StaleInputRecipientAffinity
        }
        other => panic!("unexpected stop in the qualified production schedule: {other:?}"),
    }
}
