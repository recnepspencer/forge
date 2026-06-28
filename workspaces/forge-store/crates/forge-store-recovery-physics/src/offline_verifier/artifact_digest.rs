use super::PersistedRecoveryArtifacts;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistedRecoveryArtifactDigest {
    value: String,
    format_version: String,
    backend_profile: String,
    recovery_profile: String,
    record_count: usize,
    byte_count: usize,
}

impl PersistedRecoveryArtifactDigest {
    pub fn from_artifacts(artifacts: &PersistedRecoveryArtifacts) -> Self {
        let mut state = DeterministicDigestState::new();
        state.feed_text(artifacts.format_version());
        state.feed_text(artifacts.backend_profile());
        state.feed_text(artifacts.recovery_profile().as_str());
        for record in artifacts.records() {
            state.feed_text(record.record_id());
            state.feed_bytes(record.bytes());
        }
        Self {
            value: format!("{:016x}", state.finish()),
            format_version: artifacts.format_version().to_string(),
            backend_profile: artifacts.backend_profile().to_string(),
            recovery_profile: artifacts.recovery_profile().as_str().to_string(),
            record_count: artifacts.records().len(),
            byte_count: artifacts.total_bytes(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn format_version(&self) -> &str {
        &self.format_version
    }

    pub fn backend_profile(&self) -> &str {
        &self.backend_profile
    }

    pub fn recovery_profile(&self) -> &str {
        &self.recovery_profile
    }

    pub const fn record_count(&self) -> usize {
        self.record_count
    }

    pub const fn byte_count(&self) -> usize {
        self.byte_count
    }
}

struct DeterministicDigestState {
    value: u64,
}

impl DeterministicDigestState {
    const fn new() -> Self {
        Self {
            value: 0xcbf2_9ce4_8422_2325,
        }
    }

    fn feed_text(&mut self, text: &str) {
        self.feed_usize(text.len());
        self.feed_bytes(text.as_bytes());
    }

    fn feed_bytes(&mut self, bytes: &[u8]) {
        self.feed_usize(bytes.len());
        self.feed_raw_bytes(bytes);
    }

    fn feed_usize(&mut self, value: usize) {
        self.feed_raw_bytes(&value.to_le_bytes());
    }

    fn feed_raw_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.value ^= u64::from(*byte);
            self.value = self.value.wrapping_mul(0x100_0000_01b3);
        }
    }

    const fn finish(self) -> u64 {
        self.value
    }
}
