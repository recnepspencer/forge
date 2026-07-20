#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryTemporalCondition {
    AfterNanoseconds(u64),
    AtOrAfterUnixNanoseconds(u64),
    DebounceNanoseconds(u64),
    ThrottleNanoseconds(u64),
    StaleAfterNanoseconds(u64),
    IntervalNanoseconds(u64),
    SnapshotAdvance,
}

impl WorthQueryTemporalCondition {
    pub(crate) fn duration_is_valid(self) -> bool {
        match self {
            Self::AfterNanoseconds(value)
            | Self::DebounceNanoseconds(value)
            | Self::ThrottleNanoseconds(value)
            | Self::StaleAfterNanoseconds(value)
            | Self::IntervalNanoseconds(value) => value > 0,
            Self::AtOrAfterUnixNanoseconds(_) | Self::SnapshotAdvance => true,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryTemporalWake {
    MonotonicClock,
    WallClock,
    OnSnapshotAdvance,
}

pub(crate) fn temporal_condition_token(condition: WorthQueryTemporalCondition) -> String {
    match condition {
        WorthQueryTemporalCondition::AfterNanoseconds(value) => format!("after-ns:{value}"),
        WorthQueryTemporalCondition::AtOrAfterUnixNanoseconds(value) => {
            format!("at-or-after-unix-ns:{value}")
        }
        WorthQueryTemporalCondition::DebounceNanoseconds(value) => format!("debounce-ns:{value}"),
        WorthQueryTemporalCondition::ThrottleNanoseconds(value) => format!("throttle-ns:{value}"),
        WorthQueryTemporalCondition::StaleAfterNanoseconds(value) => {
            format!("stale-after-ns:{value}")
        }
        WorthQueryTemporalCondition::IntervalNanoseconds(value) => format!("interval-ns:{value}"),
        WorthQueryTemporalCondition::SnapshotAdvance => "snapshot-advance".to_string(),
    }
}

pub(crate) fn temporal_wake_token(wake: WorthQueryTemporalWake) -> &'static str {
    match wake {
        WorthQueryTemporalWake::MonotonicClock => "monotonic-clock",
        WorthQueryTemporalWake::WallClock => "wall-clock",
        WorthQueryTemporalWake::OnSnapshotAdvance => "snapshot-advance",
    }
}
