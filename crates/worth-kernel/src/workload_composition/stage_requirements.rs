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
    BooleanDeclarationEntry,
    BooleanRoutePlan,
    BooleanOperandPairConstruction,
    BooleanBlockerProvenance,
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
            Self::BooleanDeclarationEntry => "boolean declaration entry receipt",
            Self::BooleanRoutePlan => "boolean route-plan receipt",
            Self::BooleanOperandPairConstruction => "boolean operand-pair construction receipt",
            Self::BooleanBlockerProvenance => "boolean blocker provenance receipt",
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
            Self::BooleanDeclarationEntry => "boolean_declaration_entry",
            Self::BooleanRoutePlan => "boolean_route_plan",
            Self::BooleanOperandPairConstruction => "boolean_operand_pair_construction",
            Self::BooleanBlockerProvenance => "boolean_blocker_provenance",
            Self::EvidenceLedger => "evidence_ledger",
        }
    }
}
