#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkloadStageRequirement {
    Topology,
    GeometryBinding,
    SurfaceSupport,
    Projection,
    Transform,
    RetainedReplay,
    BatchAdmissionExecution,
    Diagnostics,
    Response,
    BooleanDeclarationEntry,
    BooleanRoutePlan,
    BooleanOperandPairConstruction,
    BooleanBlockerProvenance,
    BooleanPrecisionAgreement,
    BooleanSharedPlaneIdentity,
    BooleanLocalFrameSelection,
    BooleanOperandAProjectionConsumption,
    BooleanOperandBProjectionConsumption,
    BooleanReducedOperandPair,
    BooleanEventExtractionRequest,
    BooleanSegmentPairEnumeration,
    BooleanEventLedger,
    BooleanSplit,
    BooleanLoopReconstruction,
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
            Self::BatchAdmissionExecution => "batch-admission execution workload receipt",
            Self::Diagnostics => "diagnostic workload receipt",
            Self::Response => "response workload receipt",
            Self::BooleanDeclarationEntry => "boolean declaration entry receipt",
            Self::BooleanRoutePlan => "boolean route-plan receipt",
            Self::BooleanOperandPairConstruction => "boolean operand-pair construction receipt",
            Self::BooleanBlockerProvenance => "boolean blocker provenance receipt",
            Self::BooleanPrecisionAgreement => "boolean precision-agreement receipt",
            Self::BooleanSharedPlaneIdentity => "boolean shared-plane identity receipt",
            Self::BooleanLocalFrameSelection => "boolean local-frame selection receipt",
            Self::BooleanOperandAProjectionConsumption => {
                "boolean operand-A projection consumption receipt"
            }
            Self::BooleanOperandBProjectionConsumption => {
                "boolean operand-B projection consumption receipt"
            }
            Self::BooleanReducedOperandPair => "boolean reduced operand-pair receipt",
            Self::BooleanEventExtractionRequest => "boolean event extraction request receipt",
            Self::BooleanSegmentPairEnumeration => "boolean segment-pair enumeration receipt",
            Self::BooleanEventLedger => "boolean event ledger receipt",
            Self::BooleanSplit => "boolean split receipt",
            Self::BooleanLoopReconstruction => "boolean loop reconstruction receipt",
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
            Self::BatchAdmissionExecution => "batch_admission_execution",
            Self::Diagnostics => "diagnostics",
            Self::Response => "response",
            Self::BooleanDeclarationEntry => "boolean_declaration_entry",
            Self::BooleanRoutePlan => "boolean_route_plan",
            Self::BooleanOperandPairConstruction => "boolean_operand_pair_construction",
            Self::BooleanBlockerProvenance => "boolean_blocker_provenance",
            Self::BooleanPrecisionAgreement => "boolean_precision_agreement",
            Self::BooleanSharedPlaneIdentity => "boolean_shared_plane_identity",
            Self::BooleanLocalFrameSelection => "boolean_local_frame_selection",
            Self::BooleanOperandAProjectionConsumption => {
                "boolean_operand_a_projection_consumption"
            }
            Self::BooleanOperandBProjectionConsumption => {
                "boolean_operand_b_projection_consumption"
            }
            Self::BooleanReducedOperandPair => "boolean_reduced_operand_pair",
            Self::BooleanEventExtractionRequest => "boolean_event_extraction_request",
            Self::BooleanSegmentPairEnumeration => "boolean_segment_pair_enumeration",
            Self::BooleanEventLedger => "boolean_event_ledger",
            Self::BooleanSplit => "boolean_split",
            Self::BooleanLoopReconstruction => "boolean_loop_reconstruction",
            Self::EvidenceLedger => "evidence_ledger",
        }
    }
}
