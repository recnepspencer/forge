use crate::WalArtifactStoreDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WalActiveTailInspectionFailure {
    denial: WalActiveTailInspectionDenial,
    frames_scanned: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalActiveTailInspectionDenial {
    Artifact(WalArtifactStoreDenial),
    FrameLimitExceeded { observed: u64, admitted: u64 },
}

impl WalActiveTailInspectionFailure {
    pub(super) const fn artifact(denial: WalArtifactStoreDenial, frames_scanned: u64) -> Self {
        Self {
            denial: WalActiveTailInspectionDenial::Artifact(denial),
            frames_scanned,
        }
    }

    pub(super) const fn frame_limit(observed: u64, admitted: u64) -> Self {
        Self {
            denial: WalActiveTailInspectionDenial::FrameLimitExceeded { observed, admitted },
            frames_scanned: observed,
        }
    }

    pub const fn denial(self) -> WalActiveTailInspectionDenial {
        self.denial
    }

    pub const fn frames_scanned(self) -> u64 {
        self.frames_scanned
    }
}
