#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum State {
    BeforeFirstPresentation,
    Presented,
    SuccessorInFlight { predecessor: u64 },
    ProfileTransition { predecessor: u64 },
    Closing,
    Closed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Event {
    Pointer,
    Keyboard,
    Preedit,
    ImeCommit,
    ImeCancel,
    Scroll,
    Button,
    ButtonUnavailable,
    BeginSuccessor,
    BeginProfileTransition,
    BeginZeroSizedProfile,
    ExactCapacityText,
    OverCapacityText,
    ValidImeRange,
    UnprovableImeRange,
    TextWithoutRecipient,
    TextWithStaleRecipient,
    CompletePresentation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Effect {
    Retained,
    Ignored,
    Denied(Stop),
    PresentationCompleted,
    CloseDeferred,
    Closed,
    NoOp,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Stop {
    NoPresentationBasis,
    StalePresentationAffinity,
    PointerPositionUnavailable,
    OverCapacityText,
    ImeRangeNotScalarBoundary,
    MissingPendingPresentationContext,
    MissingInputRecipientAffinity,
    StaleInputRecipientAffinity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Action {
    CompletePresentation,
    EmitProfileEvidence,
    DrainRetained,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct Observation {
    pub(super) state: State,
    pub(super) effect: Effect,
    pub(super) retained_delta: u64,
    pub(super) predecessor: Option<u64>,
    pub(super) next_action: Option<Action>,
}

pub(super) const STATES: &[State] = &[
    State::BeforeFirstPresentation,
    State::Presented,
    State::SuccessorInFlight { predecessor: 7 },
    State::ProfileTransition { predecessor: 7 },
    State::Closing,
    State::Closed,
];

pub(super) const EVENTS: &[Event] = &[
    Event::Pointer,
    Event::Keyboard,
    Event::Preedit,
    Event::ImeCommit,
    Event::ImeCancel,
    Event::Scroll,
    Event::Button,
    Event::ButtonUnavailable,
    Event::BeginSuccessor,
    Event::BeginProfileTransition,
    Event::BeginZeroSizedProfile,
    Event::ExactCapacityText,
    Event::OverCapacityText,
    Event::ValidImeRange,
    Event::UnprovableImeRange,
    Event::TextWithoutRecipient,
    Event::TextWithStaleRecipient,
    Event::CompletePresentation,
];

pub(super) fn expected(state: State, event: Event) -> Observation {
    let (state, effect, retained_delta, next_action) = match (state, event) {
        (State::BeforeFirstPresentation, input) if is_input(input) => {
            (state, Effect::Denied(Stop::NoPresentationBasis), 0, None)
        }
        (State::Presented, Event::ButtonUnavailable)
        | (State::SuccessorInFlight { .. }, Event::ButtonUnavailable) => (
            state,
            Effect::Denied(Stop::PointerPositionUnavailable),
            0,
            None,
        ),
        (State::Presented, Event::OverCapacityText)
        | (State::SuccessorInFlight { .. }, Event::OverCapacityText) => {
            (state, Effect::Denied(Stop::OverCapacityText), 0, None)
        }
        (State::Presented, Event::UnprovableImeRange)
        | (State::SuccessorInFlight { .. }, Event::UnprovableImeRange) => (
            state,
            Effect::Denied(Stop::ImeRangeNotScalarBoundary),
            0,
            None,
        ),
        (State::Presented, Event::TextWithoutRecipient)
        | (State::SuccessorInFlight { .. }, Event::TextWithoutRecipient) => (
            state,
            Effect::Denied(Stop::MissingInputRecipientAffinity),
            0,
            None,
        ),
        (State::Presented, Event::TextWithStaleRecipient)
        | (State::SuccessorInFlight { .. }, Event::TextWithStaleRecipient) => (
            state,
            Effect::Denied(Stop::StaleInputRecipientAffinity),
            0,
            None,
        ),
        (State::ProfileTransition { .. }, input) if is_input(input) => (
            state,
            Effect::Denied(Stop::StalePresentationAffinity),
            0,
            None,
        ),
        (State::Presented, input) | (State::SuccessorInFlight { .. }, input) if is_input(input) => {
            (state, Effect::Retained, retained_delta(input), None)
        }
        (State::Presented, Event::BeginSuccessor) => (
            State::SuccessorInFlight { predecessor: 7 },
            Effect::NoOp,
            0,
            Some(Action::CompletePresentation),
        ),
        (State::BeforeFirstPresentation, Event::BeginSuccessor) => (
            State::BeforeFirstPresentation,
            Effect::NoOp,
            0,
            Some(Action::CompletePresentation),
        ),
        (State::SuccessorInFlight { predecessor }, Event::BeginSuccessor) => (
            State::SuccessorInFlight { predecessor },
            Effect::Denied(Stop::MissingPendingPresentationContext),
            0,
            None,
        ),
        (State::ProfileTransition { predecessor }, Event::BeginSuccessor) => (
            State::SuccessorInFlight { predecessor },
            Effect::NoOp,
            0,
            Some(Action::CompletePresentation),
        ),
        (State::Presented, Event::BeginProfileTransition) => (
            State::ProfileTransition { predecessor: 7 },
            Effect::NoOp,
            0,
            Some(Action::EmitProfileEvidence),
        ),
        (State::Presented, Event::BeginZeroSizedProfile) => (
            State::ProfileTransition { predecessor: 7 },
            Effect::NoOp,
            0,
            Some(Action::EmitProfileEvidence),
        ),
        (State::BeforeFirstPresentation, Event::BeginProfileTransition)
        | (State::BeforeFirstPresentation, Event::BeginZeroSizedProfile) => (
            State::BeforeFirstPresentation,
            Effect::NoOp,
            0,
            Some(Action::EmitProfileEvidence),
        ),
        (State::SuccessorInFlight { predecessor }, Event::BeginProfileTransition)
        | (State::SuccessorInFlight { predecessor }, Event::BeginZeroSizedProfile)
        | (State::ProfileTransition { predecessor }, Event::BeginProfileTransition)
        | (State::ProfileTransition { predecessor }, Event::BeginZeroSizedProfile) => (
            State::ProfileTransition { predecessor },
            Effect::NoOp,
            0,
            Some(Action::EmitProfileEvidence),
        ),
        (State::SuccessorInFlight { .. }, Event::CompletePresentation) => {
            (State::Presented, Effect::PresentationCompleted, 0, None)
        }
        (State::ProfileTransition { .. }, Event::CompletePresentation) => {
            (State::Presented, Effect::PresentationCompleted, 2, None)
        }
        (State::BeforeFirstPresentation, Event::CompletePresentation)
        | (State::Presented, Event::CompletePresentation) => {
            (State::Presented, Effect::PresentationCompleted, 0, None)
        }
        _ => (state, Effect::NoOp, 0, None),
    };
    Observation {
        state,
        effect,
        retained_delta,
        predecessor: predecessor(state),
        next_action,
    }
}

fn is_input(event: Event) -> bool {
    matches!(
        event,
        Event::Pointer
            | Event::Keyboard
            | Event::Preedit
            | Event::ImeCommit
            | Event::ImeCancel
            | Event::Scroll
            | Event::Button
            | Event::ButtonUnavailable
            | Event::ExactCapacityText
            | Event::OverCapacityText
            | Event::ValidImeRange
            | Event::UnprovableImeRange
            | Event::TextWithoutRecipient
            | Event::TextWithStaleRecipient
    )
}

fn retained_delta(event: Event) -> u64 {
    if matches!(
        event,
        Event::OverCapacityText
            | Event::UnprovableImeRange
            | Event::TextWithoutRecipient
            | Event::TextWithStaleRecipient
    ) {
        0
    } else {
        u64::from(matches!(event, Event::Keyboard)) + 1
    }
}

fn predecessor(state: State) -> Option<u64> {
    match state {
        State::SuccessorInFlight { predecessor } | State::ProfileTransition { predecessor } => {
            Some(predecessor)
        }
        _ => None,
    }
}
