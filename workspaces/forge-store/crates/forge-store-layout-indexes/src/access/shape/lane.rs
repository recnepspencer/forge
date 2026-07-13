use crate::catalog::ArtifactFamilyAccessLane;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessLaneClassification {
    Foreground,
    Maintenance,
    Verifier,
    Terminal,
}

impl AccessLaneClassification {
    pub const fn admitted_lane(self) -> ArtifactFamilyAccessLane {
        match self {
            Self::Foreground => ArtifactFamilyAccessLane::HotPath,
            Self::Maintenance => ArtifactFamilyAccessLane::MaintenancePath,
            Self::Verifier => ArtifactFamilyAccessLane::VerifierPath,
            Self::Terminal => ArtifactFamilyAccessLane::TerminalPath,
        }
    }
}
