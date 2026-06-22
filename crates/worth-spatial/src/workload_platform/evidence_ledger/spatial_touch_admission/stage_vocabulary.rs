use crate::workload_platform::evidence_ledger::{BooleanEvidenceStageKind, WorkloadEvidenceStage};

pub const SPATIAL_TOUCH_BOOLEAN_EVIDENCE_STAGE_KINDS: [BooleanEvidenceStageKind; 18] = [
    BooleanEvidenceStageKind::DeclarationEntry,
    BooleanEvidenceStageKind::RoutePlan,
    BooleanEvidenceStageKind::OperandPairConstruction,
    BooleanEvidenceStageKind::BlockerProvenance,
    BooleanEvidenceStageKind::PrecisionAgreement,
    BooleanEvidenceStageKind::SharedPlaneIdentity,
    BooleanEvidenceStageKind::LocalFrameSelection,
    BooleanEvidenceStageKind::OperandAProjectionConsumption,
    BooleanEvidenceStageKind::OperandBProjectionConsumption,
    BooleanEvidenceStageKind::ReducedOperandPair,
    BooleanEvidenceStageKind::EventExtractionRequest,
    BooleanEvidenceStageKind::SegmentPairEnumeration,
    BooleanEvidenceStageKind::EventLedger,
    BooleanEvidenceStageKind::Split,
    BooleanEvidenceStageKind::LoopReconstruction,
    BooleanEvidenceStageKind::Classify,
    BooleanEvidenceStageKind::Assemble,
    BooleanEvidenceStageKind::Cleanup,
];

pub fn spatial_touch_workload_evidence_stage(
    kind: BooleanEvidenceStageKind,
) -> WorkloadEvidenceStage {
    kind.evidence_stage()
}
