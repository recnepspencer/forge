#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkloadBlockerSourceKind {
    DirtyTopology,
    OpenTopology,
    PlanarBooleanEntryBasis,
    PlanarBooleanDeclaration,
}

impl WorkloadBlockerSourceKind {
    pub fn human_name(self) -> &'static str {
        match self {
            Self::DirtyTopology => "dirty topology",
            Self::OpenTopology => "open topology",
            Self::PlanarBooleanEntryBasis => "planar boolean entry basis",
            Self::PlanarBooleanDeclaration => "planar boolean declaration",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkloadBlockerBoundaryKind {
    CleanFailBoundary,
    UnsupportedSurface,
    BooleanLanePolicy,
    BooleanSupportMatrix,
    BooleanEvidenceBoundary,
    BooleanExecutionBoundary,
}

impl WorkloadBlockerBoundaryKind {
    pub fn human_name(self) -> &'static str {
        match self {
            Self::CleanFailBoundary => "clean-fail boundary",
            Self::UnsupportedSurface => "unsupported surface",
            Self::BooleanLanePolicy => "boolean lane policy",
            Self::BooleanSupportMatrix => "boolean support matrix",
            Self::BooleanEvidenceBoundary => "boolean evidence boundary",
            Self::BooleanExecutionBoundary => "boolean execution boundary",
        }
    }
}
