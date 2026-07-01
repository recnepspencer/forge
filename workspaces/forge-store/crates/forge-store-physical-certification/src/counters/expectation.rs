#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CounterExpectationKind {
    Zero,
    Positive,
    Exact,
    Monotonic,
    Bounded,
    ProfileScoped,
}

pub type CounterExpectationStrength = CounterExpectationKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalCounterExpectation {
    kind: CounterExpectationKind,
    value: Option<u64>,
}

impl PhysicalCounterExpectation {
    pub const fn zero() -> Self {
        Self {
            kind: CounterExpectationKind::Zero,
            value: Some(0),
        }
    }

    pub const fn positive() -> Self {
        Self {
            kind: CounterExpectationKind::Positive,
            value: None,
        }
    }

    pub const fn exact(value: u64) -> Self {
        Self {
            kind: CounterExpectationKind::Exact,
            value: Some(value),
        }
    }

    pub const fn monotonic() -> Self {
        Self {
            kind: CounterExpectationKind::Monotonic,
            value: None,
        }
    }

    pub fn bounded(maximum: u64) -> Result<Self, CounterExpectationDenial> {
        if maximum == 0 {
            Err(CounterExpectationDenial::BoundedMaximumIsZero)
        } else {
            Ok(Self {
                kind: CounterExpectationKind::Bounded,
                value: Some(maximum),
            })
        }
    }

    pub const fn profile_scoped() -> Self {
        Self {
            kind: CounterExpectationKind::ProfileScoped,
            value: None,
        }
    }

    pub const fn kind(&self) -> CounterExpectationKind {
        self.kind
    }

    pub const fn value(&self) -> Option<u64> {
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CounterExpectationDenial {
    BoundedMaximumIsZero,
}

pub(crate) fn counter_expectation_kind_token(kind: CounterExpectationKind) -> &'static str {
    match kind {
        CounterExpectationKind::Zero => "zero",
        CounterExpectationKind::Positive => "positive",
        CounterExpectationKind::Exact => "exact",
        CounterExpectationKind::Monotonic => "monotonic",
        CounterExpectationKind::Bounded => "bounded",
        CounterExpectationKind::ProfileScoped => "profile-scoped",
    }
}
