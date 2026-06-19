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
    BooleanEventExtractionRequest,
    BooleanSegmentPairEnumeration,
    BooleanEventLedger,
    BooleanSplit,
    BooleanLoopReconstruction,
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
    EventExtractionRequest,
    SegmentPairEnumeration,
    EventLedger,
    Split,
    LoopReconstruction,
    Classify,
    Assemble,
    Cleanup,
}

impl WorkloadEvidenceStage {
    pub(crate) const STAGE_COUNT: usize = 27;

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

    pub const BOOLEAN_STAGES: [Self; 18] = [
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
        Self::BooleanEventExtractionRequest,
        Self::BooleanSegmentPairEnumeration,
        Self::BooleanEventLedger,
        Self::BooleanSplit,
        Self::BooleanLoopReconstruction,
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
            Self::BooleanEventExtractionRequest => "boolean event extraction request evidence",
            Self::BooleanSegmentPairEnumeration => "boolean segment-pair enumeration evidence",
            Self::BooleanEventLedger => "boolean event ledger evidence",
            Self::BooleanSplit => "boolean split evidence",
            Self::BooleanLoopReconstruction => "boolean loop reconstruction evidence",
            Self::BooleanClassify => "boolean classify evidence",
            Self::BooleanAssemble => "boolean assemble evidence",
            Self::BooleanCleanup => "boolean cleanup evidence",
        }
    }

    pub fn is_boolean_stage(self) -> bool {
        Self::BOOLEAN_STAGES.contains(&self)
    }

    pub(crate) fn index_slot(self) -> usize {
        match self {
            Self::Topology => 0,
            Self::GeometryBinding => 1,
            Self::SurfaceSupport => 2,
            Self::Projection => 3,
            Self::Transform => 4,
            Self::RetainedReplay => 5,
            Self::Diagnostics => 6,
            Self::Response => 7,
            Self::Operator => 8,
            Self::BooleanDeclarationEntry => 9,
            Self::BooleanRoutePlan => 10,
            Self::BooleanOperandPairConstruction => 11,
            Self::BooleanBlockerProvenance => 12,
            Self::BooleanPrecisionAgreement => 13,
            Self::BooleanSharedPlaneIdentity => 14,
            Self::BooleanLocalFrameSelection => 15,
            Self::BooleanOperandAProjectionConsumption => 16,
            Self::BooleanOperandBProjectionConsumption => 17,
            Self::BooleanReducedOperandPair => 18,
            Self::BooleanEventExtractionRequest => 19,
            Self::BooleanSegmentPairEnumeration => 20,
            Self::BooleanEventLedger => 21,
            Self::BooleanSplit => 22,
            Self::BooleanLoopReconstruction => 23,
            Self::BooleanClassify => 24,
            Self::BooleanAssemble => 25,
            Self::BooleanCleanup => 26,
        }
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
            Self::EventExtractionRequest => WorkloadEvidenceStage::BooleanEventExtractionRequest,
            Self::SegmentPairEnumeration => WorkloadEvidenceStage::BooleanSegmentPairEnumeration,
            Self::EventLedger => WorkloadEvidenceStage::BooleanEventLedger,
            Self::Split => WorkloadEvidenceStage::BooleanSplit,
            Self::LoopReconstruction => WorkloadEvidenceStage::BooleanLoopReconstruction,
            Self::Classify => WorkloadEvidenceStage::BooleanClassify,
            Self::Assemble => WorkloadEvidenceStage::BooleanAssemble,
            Self::Cleanup => WorkloadEvidenceStage::BooleanCleanup,
        }
    }
}
