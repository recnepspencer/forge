#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologySeedKind {
    Cube,
    Tetrahedron,
    SingleFaceLoop,
    MultiFaceShell,
    OpenSheet,
    OpenWire,
    HighValenceVertex,
    SelfIntersectingLoop,
    NonManifoldWire,
    ThinWallLocalBasis,
    OrientationInconsistency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologySeedTopologyPosture {
    ClosedValid,
    OpenValid,
    Dirty,
}

impl TopologySeedKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cube => "cube",
            Self::Tetrahedron => "tetrahedron",
            Self::SingleFaceLoop => "single-face-loop",
            Self::MultiFaceShell => "multi-face-shell",
            Self::OpenSheet => "open-sheet",
            Self::OpenWire => "open-wire",
            Self::HighValenceVertex => "high-valence-vertex",
            Self::SelfIntersectingLoop => "self-intersecting-loop",
            Self::NonManifoldWire => "non-manifold-wire",
            Self::ThinWallLocalBasis => "thin-wall-local-basis",
            Self::OrientationInconsistency => "orientation-inconsistency",
        }
    }

    pub(crate) fn default_declaration(self) -> String {
        format!("topology seed {}", self.as_str())
    }

    pub fn topology_posture(self) -> TopologySeedTopologyPosture {
        match self {
            Self::OpenSheet | Self::OpenWire | Self::HighValenceVertex => {
                TopologySeedTopologyPosture::OpenValid
            }
            Self::SelfIntersectingLoop
            | Self::NonManifoldWire
            | Self::ThinWallLocalBasis
            | Self::OrientationInconsistency => TopologySeedTopologyPosture::Dirty,
            Self::Cube | Self::Tetrahedron | Self::SingleFaceLoop | Self::MultiFaceShell => {
                TopologySeedTopologyPosture::ClosedValid
            }
        }
    }
}
