use super::oracle::{Event, State};

pub(super) struct Schedule {
    pub(super) name: &'static str,
    pub(super) initial: State,
    pub(super) events: &'static [Event],
}

pub(super) const EXPECTED_COMPARISONS: usize = 177;

const BEFORE_FIRST_INPUT: &[Event] = &[
    Event::Pointer,
    Event::Keyboard,
    Event::Preedit,
    Event::ImeCommit,
    Event::ImeCancel,
    Event::Scroll,
    Event::Button,
    Event::ButtonUnavailable,
];
const SUCCESSOR: &[Event] = &[
    Event::BeginSuccessor,
    Event::Pointer,
    Event::CompletePresentation,
    Event::Pointer,
    Event::Close,
];
const PROFILE: &[Event] = &[
    Event::BeginProfileTransition,
    Event::Pointer,
    Event::CompletePresentation,
    Event::Pointer,
    Event::Close,
];
const IME: &[Event] = &[Event::Preedit, Event::ImeCommit, Event::ImeCancel];
const POINTER_WITNESSES: &[Event] = &[Event::Button, Event::ButtonUnavailable];
const READINESS_AND_CLOSE: &[Event] = &[
    Event::BeginSuccessor,
    Event::CompletePresentation,
    Event::Close,
    Event::Close,
];
const MIXED_ORDERING: &[Event] = &[
    Event::Pointer,
    Event::Scroll,
    Event::BeginSuccessor,
    Event::Keyboard,
    Event::CompletePresentation,
    Event::BeginProfileTransition,
    Event::Preedit,
    Event::CompletePresentation,
    Event::Close,
];
const RETENTION_ORDERING: &[Event] = &[
    Event::Pointer,
    Event::Keyboard,
    Event::Scroll,
    Event::Pointer,
    Event::Preedit,
    Event::ImeCommit,
    Event::ImeCancel,
    Event::ButtonUnavailable,
];
const POST_CLOSE: &[Event] = &[Event::Pointer, Event::Keyboard, Event::Close];
const CLOSING_PENDING: &[Event] = &[Event::Pointer, Event::Close];
const STALE_PROFILE_AFTER_COMPLETION: &[Event] =
    &[Event::Pointer, Event::CompletePresentation, Event::Pointer];
const EMPTY_CLOSE: &[Event] = &[Event::Close];
const RESIZE_DPI_ZERO_SIZED: &[Event] = &[
    Event::BeginZeroSizedProfile,
    Event::Pointer,
    Event::CompletePresentation,
];
const OVER_CAPACITY_TEXT: &[Event] = &[Event::ExactCapacityText, Event::OverCapacityText];
const UNPROVABLE_IME_RANGE: &[Event] = &[
    Event::ValidImeRange,
    Event::UnprovableImeRange,
    Event::ValidImeRange,
];
const NO_RECIPIENT: &[Event] = &[Event::TextWithoutRecipient];
const STALE_RECIPIENT: &[Event] = &[Event::TextWithStaleRecipient];

pub(super) const SCHEDULES: &[Schedule] = &[
    Schedule {
        name: "every-input-family-before-first-presentation",
        initial: State::BeforeFirstPresentation,
        events: BEFORE_FIRST_INPUT,
    },
    Schedule {
        name: "successor-in-flight-affinity",
        initial: State::Presented,
        events: SUCCESSOR,
    },
    Schedule {
        name: "profile-transition-stale-input",
        initial: State::Presented,
        events: PROFILE,
    },
    Schedule {
        name: "ime-preedit-commit-cancel",
        initial: State::Presented,
        events: IME,
    },
    Schedule {
        name: "pointer-event-time-or-unavailable",
        initial: State::Presented,
        events: POINTER_WITNESSES,
    },
    Schedule {
        name: "readiness-and-close-drain",
        initial: State::Presented,
        events: READINESS_AND_CLOSE,
    },
    Schedule {
        name: "mixed-ordering-capacity-and-wake",
        initial: State::Presented,
        events: MIXED_ORDERING,
    },
    Schedule {
        name: "retention-ordering-and-bounded-capacity",
        initial: State::Presented,
        events: RETENTION_ORDERING,
    },
    Schedule {
        name: "post-close-input-is-noop",
        initial: State::Closed,
        events: POST_CLOSE,
    },
    Schedule {
        name: "closing-pending-and-drain-request",
        initial: State::Closing,
        events: CLOSING_PENDING,
    },
    Schedule {
        name: "stale-profile-after-completion",
        initial: State::ProfileTransition { predecessor: 7 },
        events: STALE_PROFILE_AFTER_COMPLETION,
    },
    Schedule {
        name: "empty-close-before-first-presentation",
        initial: State::BeforeFirstPresentation,
        events: EMPTY_CLOSE,
    },
    Schedule {
        name: "resize-dpi-zero-sized-profile-around-input",
        initial: State::Presented,
        events: RESIZE_DPI_ZERO_SIZED,
    },
    Schedule {
        name: "over-capacity-text-stops-before-retention",
        initial: State::Presented,
        events: OVER_CAPACITY_TEXT,
    },
    Schedule {
        name: "unprovable-ime-range-stops-before-retention",
        initial: State::Presented,
        events: UNPROVABLE_IME_RANGE,
    },
    Schedule {
        name: "missing-recipient-stops-before-text-retention",
        initial: State::Presented,
        events: NO_RECIPIENT,
    },
    Schedule {
        name: "stale-recipient-stops-before-text-retention",
        initial: State::Presented,
        events: STALE_RECIPIENT,
    },
];
