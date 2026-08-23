#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiNativeClientObservationIngressObservation {
    applied_batches: u64,
    duplicate_batches: u64,
    quarantined_batches: u64,
    denied_batches: u64,
    drain_denied: u64,
}

impl UiNativeClientObservationIngressObservation {
    pub const fn reported(counts: [u64; 5]) -> Self {
        Self {
            applied_batches: counts[0],
            duplicate_batches: counts[1],
            quarantined_batches: counts[2],
            denied_batches: counts[3],
            drain_denied: counts[4],
        }
    }

    pub const fn counts(self) -> [u64; 5] {
        [
            self.applied_batches,
            self.duplicate_batches,
            self.quarantined_batches,
            self.denied_batches,
            self.drain_denied,
        ]
    }
}
