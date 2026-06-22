#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubscriptionSupportTrustClass {
    Exact,
    Degraded,
    Rebuildable,
    NonResumable,
}
