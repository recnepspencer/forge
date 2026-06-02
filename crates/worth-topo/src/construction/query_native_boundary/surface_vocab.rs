#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyConstructionQueryMutationSurface {
    ComposeGraph,
}

impl TopologyConstructionQueryMutationSurface {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ComposeGraph => "compose_graph",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyConstructionQueryReadSurface {
    ProjectionConsumptionFromInspectionReceipt,
}

impl TopologyConstructionQueryReadSurface {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ProjectionConsumptionFromInspectionReceipt => {
                "projection consumption from inspection receipt"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyConstructionQueryInspectionSurface {
    InspectReceipt,
}

impl TopologyConstructionQueryInspectionSurface {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InspectReceipt => "workspace.inspect(&receipt)",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyConstructionQueryFactProvenance {
    InspectionBackedProjectionConsumption,
}

impl TopologyConstructionQueryFactProvenance {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InspectionBackedProjectionConsumption => {
                "equivalent typed facts from inspection-backed projection consumption"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyConstructionQueryFactKind {
    VertexBirth,
    EdgeBirth,
    LoopMembership,
    WireMembership,
    FaceMembership,
    ShellMembership,
    BodyMembership,
}

impl TopologyConstructionQueryFactKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::VertexBirth => "vertex-birth",
            Self::EdgeBirth => "edge-birth",
            Self::LoopMembership => "loop-membership",
            Self::WireMembership => "wire-membership",
            Self::FaceMembership => "face-membership",
            Self::ShellMembership => "shell-membership",
            Self::BodyMembership => "body-membership",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyConstructionQueryFactRow {
    kind: TopologyConstructionQueryFactKind,
    fact_count: usize,
    row_digest: String,
}

impl TopologyConstructionQueryFactRow {
    pub(crate) fn new(kind: TopologyConstructionQueryFactKind, fact_count: usize) -> Self {
        let row_digest = super::digest_parts(&[kind.as_str().to_string(), fact_count.to_string()]);
        Self {
            kind,
            fact_count,
            row_digest,
        }
    }

    pub fn kind(&self) -> TopologyConstructionQueryFactKind {
        self.kind
    }

    pub fn fact_count(&self) -> usize {
        self.fact_count
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
