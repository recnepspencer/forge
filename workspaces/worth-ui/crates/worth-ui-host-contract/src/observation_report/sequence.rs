#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiHostObservationSequence(u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostObservationSequenceRange {
    first: UiHostObservationSequence,
    last: UiHostObservationSequence,
}

impl UiHostObservationSequence {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

impl UiHostObservationSequenceRange {
    pub const fn new(first: UiHostObservationSequence, last: UiHostObservationSequence) -> Self {
        Self { first, last }
    }

    pub const fn first(self) -> UiHostObservationSequence {
        self.first
    }

    pub const fn last(self) -> UiHostObservationSequence {
        self.last
    }

    pub const fn is_ordered(self) -> bool {
        self.first.0 <= self.last.0
    }

    pub const fn contains(self, sequence: UiHostObservationSequence) -> bool {
        self.first.0 <= sequence.0 && sequence.0 <= self.last.0
    }
}
