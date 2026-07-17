#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ClockOrderingAssumption {
    NoClockDependency,
    MonotonicProcessClock,
    PersistedLeaseEpochOrdering,
}
