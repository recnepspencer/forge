use crate::{WalSegmentGeneration, WalSegmentId};

/// Canonical identity encoded by one Store-owned WAL segment artifact name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WalSegmentArtifactIdentity {
    segment: WalSegmentId,
    generation: WalSegmentGeneration,
}

impl WalSegmentArtifactIdentity {
    pub const fn new(segment: WalSegmentId, generation: WalSegmentGeneration) -> Self {
        Self {
            segment,
            generation,
        }
    }

    pub fn parse(file_name: &str) -> Option<Self> {
        let body = file_name.strip_prefix("segment-")?.strip_suffix(".wal")?;
        let (segment, generation) = body.split_once("-generation-")?;
        let identity = Self::new(
            WalSegmentId::new(segment.parse().ok()?).ok()?,
            WalSegmentGeneration::new(generation.parse().ok()?).ok()?,
        );
        (identity.file_name() == file_name).then_some(identity)
    }

    pub fn file_name(self) -> String {
        format!(
            "segment-{}-generation-{}.wal",
            self.segment.get(),
            self.generation.get()
        )
    }

    pub const fn segment(self) -> WalSegmentId {
        self.segment
    }

    pub const fn generation(self) -> WalSegmentGeneration {
        self.generation
    }
}
