use super::BridgeSubscriptionCounters;

mod checkpoint;
mod continuation;
mod declaration;
mod delivery;
mod diagnostics;
mod fanout;
mod historical;
mod lifecycle;
mod mixed_cause;
mod preview;
mod resume;
mod shared_delivery;
mod temporal;

impl BridgeSubscriptionCounters {
    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
