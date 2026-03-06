//! Inter-subscriber data identifiers for feature-pipeline lifecycle runtime.

/// Typed data identifiers exchanged through `SubscriberContext`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KernelSubscriberDataId {
    DecisionDrain,
    Finalization,
    OperationEnvelope,
}
