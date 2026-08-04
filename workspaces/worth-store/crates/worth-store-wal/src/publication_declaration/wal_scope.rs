use crate::{WalLsnRange, WalSegmentGeneration, WalSegmentId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WalFramePublicationScope {
    segment_id: WalSegmentId,
    generation: WalSegmentGeneration,
    lsn_range: WalLsnRange,
    frame_digest: String,
    expected_bytes: u64,
}

impl WalFramePublicationScope {
    pub fn new(
        segment_id: WalSegmentId,
        generation: WalSegmentGeneration,
        lsn_range: WalLsnRange,
        frame_digest: impl Into<String>,
        expected_bytes: u64,
    ) -> Option<Self> {
        if expected_bytes == 0 {
            return None;
        }
        let frame_digest = frame_digest.into();
        if frame_digest.is_empty() {
            return None;
        }
        Some(Self {
            segment_id,
            generation,
            lsn_range,
            frame_digest,
            expected_bytes,
        })
    }

    pub const fn segment_id(&self) -> u64 {
        self.segment_id.get()
    }

    pub const fn generation(&self) -> u64 {
        self.generation.get()
    }

    pub const fn lsn_start(&self) -> u64 {
        self.lsn_range.start().get()
    }

    pub const fn lsn_end(&self) -> u64 {
        self.lsn_range.end_exclusive().get()
    }

    pub fn frame_digest(&self) -> &str {
        &self.frame_digest
    }

    pub const fn expected_bytes(&self) -> u64 {
        self.expected_bytes
    }
}
