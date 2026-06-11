#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkloadStageRequirement {
    Topology,
    GeometryBinding,
    SurfaceSupport,
    Projection,
    Transform,
    RetainedReplay,
    Diagnostics,
    Response,
    EvidenceLedger,
}

impl WorkloadStageRequirement {
    pub fn human_name(self) -> &'static str {
        match self {
            Self::Topology => "topology workload receipt",
            Self::GeometryBinding => "geometry binding workload receipt",
            Self::SurfaceSupport => "surface support workload receipt",
            Self::Projection => "projection workload receipt",
            Self::Transform => "transform workload receipt",
            Self::RetainedReplay => "retained replay workload receipt",
            Self::Diagnostics => "diagnostic workload receipt",
            Self::Response => "response workload receipt",
            Self::EvidenceLedger => "workload evidence ledger",
        }
    }

    pub fn query_key(self) -> &'static str {
        match self {
            Self::Topology => "topology",
            Self::GeometryBinding => "geometry_binding",
            Self::SurfaceSupport => "surface_support",
            Self::Projection => "projection",
            Self::Transform => "transform",
            Self::RetainedReplay => "retained_replay",
            Self::Diagnostics => "diagnostics",
            Self::Response => "response",
            Self::EvidenceLedger => "evidence_ledger",
        }
    }
}
