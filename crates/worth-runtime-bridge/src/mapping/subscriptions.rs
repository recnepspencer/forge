#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SubscriptionSliceKind {
    SignalAspect,
    SignalField,
    SignalLens,
    SignalRegion,
    SignalPartition,
    SignalFacet,
    SignalLifecycle,
    RegisteredCoarseWidening,
}
