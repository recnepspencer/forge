use crate::artifact_family::ArtifactFamilyAccessLane;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8AccessLaneClassification {
    Foreground,
    Maintenance,
    Verifier,
    Terminal,
}

impl S8AccessLaneClassification {
    pub const fn admitted_lane(self) -> ArtifactFamilyAccessLane {
        match self {
            Self::Foreground => ArtifactFamilyAccessLane::HotPath,
            Self::Maintenance => ArtifactFamilyAccessLane::MaintenancePath,
            Self::Verifier => ArtifactFamilyAccessLane::VerifierPath,
            Self::Terminal => ArtifactFamilyAccessLane::TerminalPath,
        }
    }
}
