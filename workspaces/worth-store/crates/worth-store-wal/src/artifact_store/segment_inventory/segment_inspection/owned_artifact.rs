use super::{VerifiedWalFrame, VerifiedWalFramePayload, VerifiedWalSegment, WalSegmentInspection};

/// Owned proof that one complete WAL artifact and every retained frame came
/// from the same digest-verified bounded inspection.
///
/// This is the only WAL value accepted by post-publication cleanup admission.
/// A copyable [`WalSegmentInspection`] describes an artifact, but cannot prove
/// that caller-supplied frame payloads came from those exact artifact bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedWalArtifact {
    inspection: WalSegmentInspection,
    frames: Box<[VerifiedWalFrame]>,
}

impl VerifiedWalSegment<'_> {
    pub fn to_owned_artifact(&self) -> VerifiedWalArtifact {
        VerifiedWalArtifact {
            inspection: self.inspection,
            frames: self
                .frames
                .iter()
                .copied()
                .map(VerifiedWalFramePayload::to_owned_verified)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }
}

impl VerifiedWalArtifact {
    pub const fn inspection(&self) -> WalSegmentInspection {
        self.inspection
    }

    pub fn frames(&self) -> &[VerifiedWalFrame] {
        &self.frames
    }
}
