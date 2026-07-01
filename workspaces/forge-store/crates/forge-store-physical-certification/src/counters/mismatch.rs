use crate::PhysicalSimulationProfile;

use super::{CounterContractKind, CounterExpectationKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CounterMismatchEvidence {
    MissingCounterSpec {
        kind: CounterContractKind,
    },
    DuplicateCounterRow {
        kind: CounterContractKind,
    },
    UnexpectedCounterRow {
        kind: CounterContractKind,
    },
    NonZeroForbiddenCounter {
        kind: CounterContractKind,
        actual: u64,
    },
    CounterValueMismatch {
        kind: CounterContractKind,
        expected: u64,
        actual: u64,
    },
    UnderStrengthEvidence {
        kind: CounterContractKind,
        required: CounterExpectationKind,
        actual: CounterExpectationKind,
    },
    PositiveCounterNotPositive {
        kind: CounterContractKind,
        actual: u64,
    },
    BoundedCounterExceeded {
        kind: CounterContractKind,
        maximum: u64,
        actual: u64,
    },
    MonotonicCounterRegressed {
        kind: CounterContractKind,
        previous: u64,
        actual: u64,
    },
    ProfileMismatch {
        expected: PhysicalSimulationProfile,
        actual: PhysicalSimulationProfile,
    },
    ResourceEnvelopeExceeded {
        kind: CounterContractKind,
        maximum: u64,
        actual: u64,
    },
    ExecutedEvidencePlanMismatch,
    OverExactCounter {
        kind: CounterContractKind,
    },
    FoundationalReceiptDenied,
}
