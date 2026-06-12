#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkloadBlockerSourceKind {
    DirtyTopology,
    OpenTopology,
}

impl WorkloadBlockerSourceKind {
    pub fn human_name(self) -> &'static str {
        match self {
            Self::DirtyTopology => "dirty topology",
            Self::OpenTopology => "open topology",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkloadBlockerBoundaryKind {
    CleanFailBoundary,
    UnsupportedSurface,
}

impl WorkloadBlockerBoundaryKind {
    pub fn human_name(self) -> &'static str {
        match self {
            Self::CleanFailBoundary => "clean-fail boundary",
            Self::UnsupportedSurface => "unsupported surface",
        }
    }
}
