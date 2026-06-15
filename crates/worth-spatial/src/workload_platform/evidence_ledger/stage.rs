#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkloadEvidenceStage {
    Topology,
    GeometryBinding,
    SurfaceSupport,
    Projection,
    Transform,
    RetainedReplay,
    Diagnostics,
    Response,
    Operator,
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
    BooleanSplit,
    BooleanClassify,
    BooleanAssemble,
    BooleanCleanup,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BooleanEvidenceStageKind {
    DeclarationEntry,
    RoutePlan,
    OperandPairConstruction,
    BlockerProvenance,
    PrecisionAgreement,
    SharedPlaneIdentity,
    LocalFrameSelection,
    OperandAProjectionConsumption,
    OperandBProjectionConsumption,
    ReducedOperandPair,
    Split,
    Classify,
    Assemble,
    Cleanup,
}

impl WorkloadEvidenceStage {
    pub const AUTHORITY_STAGES: [Self; 8] = [
        Self::Topology,
        Self::GeometryBinding,
        Self::SurfaceSupport,
        Self::Projection,
        Self::Transform,
        Self::RetainedReplay,
        Self::Diagnostics,
        Self::Response,
    ];

    pub const BOOLEAN_STAGES: [Self; 14] = [
        Self::BooleanDeclarationEntry,
        Self::BooleanRoutePlan,
        Self::BooleanOperandPairConstruction,
        Self::BooleanBlockerProvenance,
        Self::BooleanPrecisionAgreement,
        Self::BooleanSharedPlaneIdentity,
        Self::BooleanLocalFrameSelection,
        Self::BooleanOperandAProjectionConsumption,
        Self::BooleanOperandBProjectionConsumption,
        Self::BooleanReducedOperandPair,
        Self::BooleanSplit,
        Self::BooleanClassify,
        Self::BooleanAssemble,
        Self::BooleanCleanup,
    ];

    pub fn human_name(self) -> &'static str {
        match self {
            Self::Topology => "topology evidence",
            Self::GeometryBinding => "geometry binding evidence",
            Self::SurfaceSupport => "surface support evidence",
            Self::Projection => "projection evidence",
            Self::Transform => "transform evidence",
            Self::RetainedReplay => "retained replay evidence",
            Self::Diagnostics => "diagnostic evidence",
            Self::Response => "response evidence",
            Self::Operator => "operator evidence",
            Self::BooleanDeclarationEntry => "boolean declaration entry evidence",
            Self::BooleanRoutePlan => "boolean route plan evidence",
            Self::BooleanOperandPairConstruction => "boolean operand pair construction evidence",
            Self::BooleanBlockerProvenance => "boolean blocker provenance evidence",
            Self::BooleanPrecisionAgreement => "boolean precision agreement evidence",
            Self::BooleanSharedPlaneIdentity => "boolean shared plane identity evidence",
            Self::BooleanLocalFrameSelection => "boolean local-frame selection evidence",
            Self::BooleanOperandAProjectionConsumption => {
                "boolean operand-A projection consumption evidence"
            }
            Self::BooleanOperandBProjectionConsumption => {
                "boolean operand-B projection consumption evidence"
            }
            Self::BooleanReducedOperandPair => "boolean reduced operand-pair evidence",
            Self::BooleanSplit => "boolean split evidence",
            Self::BooleanClassify => "boolean classify evidence",
            Self::BooleanAssemble => "boolean assemble evidence",
            Self::BooleanCleanup => "boolean cleanup evidence",
        }
    }

    pub fn is_boolean_stage(self) -> bool {
        Self::BOOLEAN_STAGES.contains(&self)
    }
}

impl BooleanEvidenceStageKind {
    pub fn evidence_stage(self) -> WorkloadEvidenceStage {
        match self {
            Self::DeclarationEntry => WorkloadEvidenceStage::BooleanDeclarationEntry,
            Self::RoutePlan => WorkloadEvidenceStage::BooleanRoutePlan,
            Self::OperandPairConstruction => WorkloadEvidenceStage::BooleanOperandPairConstruction,
            Self::BlockerProvenance => WorkloadEvidenceStage::BooleanBlockerProvenance,
            Self::PrecisionAgreement => WorkloadEvidenceStage::BooleanPrecisionAgreement,
            Self::SharedPlaneIdentity => WorkloadEvidenceStage::BooleanSharedPlaneIdentity,
            Self::LocalFrameSelection => WorkloadEvidenceStage::BooleanLocalFrameSelection,
            Self::OperandAProjectionConsumption => {
                WorkloadEvidenceStage::BooleanOperandAProjectionConsumption
            }
            Self::OperandBProjectionConsumption => {
                WorkloadEvidenceStage::BooleanOperandBProjectionConsumption
            }
            Self::ReducedOperandPair => WorkloadEvidenceStage::BooleanReducedOperandPair,
            Self::Split => WorkloadEvidenceStage::BooleanSplit,
            Self::Classify => WorkloadEvidenceStage::BooleanClassify,
            Self::Assemble => WorkloadEvidenceStage::BooleanAssemble,
            Self::Cleanup => WorkloadEvidenceStage::BooleanCleanup,
        }
    }
}
