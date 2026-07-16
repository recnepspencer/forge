use super::{ProtocolCheckBounds, ProtocolCheckStatistics, ProtocolCounterexample};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtocolCheckVerdict {
    CheckedWithinBounds {
        bounds: ProtocolCheckBounds,
        statistics: ProtocolCheckStatistics,
    },
    DeadlockFound {
        counterexample: ProtocolCounterexample,
        statistics: ProtocolCheckStatistics,
    },
    BoundExhausted {
        bounds: ProtocolCheckBounds,
        statistics: ProtocolCheckStatistics,
    },
    CounterexampleFound {
        counterexample: ProtocolCounterexample,
        statistics: ProtocolCheckStatistics,
    },
    UnsupportedBackendAssumptions,
}
