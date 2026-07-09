#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeAsyncForwardCausalityClass {
    RetryAfterTimeout,
    RetryAfterCancellation,
    RevalidationAfterTruthBasisDrift,
    RevalidationAfterPreviewBasisDrift,
    RevalidationAfterSubscriptionInstanceDrift,
}
