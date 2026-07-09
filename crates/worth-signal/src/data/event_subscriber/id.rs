/// Stable deterministic identity for one subscriber.
///
/// Lower values run earlier when DAG depth is equal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubscriberId(u32);

impl SubscriberId {
    /// Create a new deterministic subscriber ID.
    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }

    /// Raw numeric value for diagnostics.
    pub const fn get(self) -> u32 {
        self.0
    }
}
