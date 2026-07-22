use super::RecordArtifactFile;

/// The exact durable byte range whose contents form one physical frame.
///
/// Store identity is deliberately not part of this value. A coordinate names
/// bytes within one store; the residency owner binds it to a stable store
/// identity before it can become a cache key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RecordFrameCoordinate {
    artifact: RecordArtifactFile,
    offset: u64,
    length: u32,
}

impl RecordFrameCoordinate {
    pub const fn new(artifact: RecordArtifactFile, offset: u64, length: u32) -> Option<Self> {
        if length == 0 || offset.checked_add(length as u64).is_none() {
            return None;
        }
        Some(Self {
            artifact,
            offset,
            length,
        })
    }

    pub const fn artifact(self) -> RecordArtifactFile {
        self.artifact
    }

    pub const fn offset(self) -> u64 {
        self.offset
    }

    pub const fn length(self) -> u32 {
        self.length
    }
}
