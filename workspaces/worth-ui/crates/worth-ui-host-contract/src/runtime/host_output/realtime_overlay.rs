#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiRealtimeHostOutput {
    overlay_row_count: u16,
    policy_digest: u64,
}

impl WorthUiRealtimeHostOutput {
    pub fn new(overlay_row_count: u16, policy_digest: u64) -> Self {
        Self {
            overlay_row_count,
            policy_digest,
        }
    }

    pub fn overlay_row_count(self) -> u16 {
        self.overlay_row_count
    }

    pub fn policy_digest(self) -> u64 {
        self.policy_digest
    }

    pub fn meaning_digest(self) -> u64 {
        u64::from(self.overlay_row_count) ^ self.policy_digest.rotate_left(29)
    }
}
