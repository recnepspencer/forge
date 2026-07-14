#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WalFrameDurablePublicationScope {
    segment_id: u64,
    generation: u64,
    lsn_start: u64,
    lsn_end: u64,
    frame_digest: String,
    expected_bytes: u64,
}

impl WalFrameDurablePublicationScope {
    pub fn new(
        segment_id: u64,
        generation: u64,
        lsn_start: u64,
        lsn_end: u64,
        frame_digest: impl Into<String>,
        expected_bytes: u64,
    ) -> Option<Self> {
        if lsn_start >= lsn_end || expected_bytes == 0 {
            return None;
        }
        let frame_digest = frame_digest.into();
        if frame_digest.is_empty() {
            return None;
        }
        Some(Self {
            segment_id,
            generation,
            lsn_start,
            lsn_end,
            frame_digest,
            expected_bytes,
        })
    }

    pub const fn segment_id(&self) -> u64 {
        self.segment_id
    }

    pub const fn generation(&self) -> u64 {
        self.generation
    }

    pub const fn lsn_start(&self) -> u64 {
        self.lsn_start
    }

    pub const fn lsn_end(&self) -> u64 {
        self.lsn_end
    }

    pub fn frame_digest(&self) -> &str {
        &self.frame_digest
    }

    pub const fn expected_bytes(&self) -> u64 {
        self.expected_bytes
    }
}
