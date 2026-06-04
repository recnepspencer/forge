#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SubscriptionSliceKind {
    SignalField,
    SignalLens,
    SignalRegion,
    SignalPartition,
    SignalFacet,
    RegisteredCoarseWidening,
}
