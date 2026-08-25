use super::oracle::{Action, Effect, Event, Observation, State, Stop, EVENTS, STATES};
pub(super) use super::production_world::{
    UiNativeLifecycleAction, UiNativeLifecycleEffect, UiNativeLifecycleEvent,
    UiNativeLifecycleObservation, UiNativeLifecycleState, UiNativeLifecycleWorld,
};
use super::schedule_inventory::{Schedule, SCHEDULES};
use worth_ui_host_native::UiNativeInputObservationStop;

#[test]
fn native_lifecycle_protocol_world_matches_independent_oracle_for_all_schedules() {
    for state in STATES.iter().copied() {
        for event in EVENTS.iter().copied() {
            run_schedule("single-state-event", state, std::slice::from_ref(&event));
        }
    }
    for schedule in SCHEDULES {
        run_inventory_schedule(schedule);
    }
}

#[test]
fn native_lifecycle_protocol_world_preserves_retention_before_close() {
    let mut world = UiNativeLifecycleWorld::new(UiNativeLifecycleState::Presented);
    assert_eq!(
        world.apply(UiNativeLifecycleEvent::Pointer).effect,
        UiNativeLifecycleEffect::Retained
    );
    assert_eq!(
        world.request_close(),
        UiNativeLifecycleObservation {
            state: UiNativeLifecycleState::Closing,
            effect: UiNativeLifecycleEffect::CloseDeferred,
            retained_delta: 0,
            predecessor: None,
            next_action: Some(UiNativeLifecycleAction::DrainRetained),
        }
    );
    assert!(world.drain_retained() >= 1);
    assert_eq!(
        world.request_close(),
        UiNativeLifecycleObservation {
            state: UiNativeLifecycleState::Closed,
            effect: UiNativeLifecycleEffect::Closed,
            retained_delta: 0,
            predecessor: None,
            next_action: None,
        }
    );
}

#[test]
fn native_lifecycle_protocol_world_closes_an_empty_presented_session() {
    let mut world = UiNativeLifecycleWorld::new(UiNativeLifecycleState::Presented);
    assert_eq!(world.drain_retained(), 0);
    assert_eq!(
        world.request_close(),
        UiNativeLifecycleObservation {
            state: UiNativeLifecycleState::Closed,
            effect: UiNativeLifecycleEffect::Closed,
            retained_delta: 0,
            predecessor: None,
            next_action: None,
        }
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

fn run_inventory_schedule(schedule: &Schedule) {
    run_schedule(schedule.name, schedule.initial, schedule.events)
}

fn run_schedule(name: &str, initial: State, events: &[Event]) {
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
            "protocol mismatch in {name} for {initial:?} + {event:?}"
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
