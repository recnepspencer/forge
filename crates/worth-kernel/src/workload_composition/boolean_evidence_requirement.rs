use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceLedgerError, WorkloadEvidenceStage,
};

use super::{WorkloadCompositionError, WorkloadStageRequirement};

pub(crate) fn map_boolean_ledger_error(
    error: WorkloadEvidenceLedgerError,
    requirement: WorkloadStageRequirement,
) -> WorkloadCompositionError {
    let Some(stage) = boolean_stage_for_requirement(requirement) else {
        return WorkloadCompositionError::UnsupportedStage(requirement);
    };
    match error {
        WorkloadEvidenceLedgerError::MissingBooleanStage(_) => {
            WorkloadCompositionError::MissingEvidenceStage(stage)
        }
        WorkloadEvidenceLedgerError::ManualBooleanStage(_) => {
            WorkloadCompositionError::ManualEvidenceStage(stage)
        }
        WorkloadEvidenceLedgerError::CounterlessBooleanStage(_) => {
            WorkloadCompositionError::CounterlessEvidenceStage(stage)
        }
        WorkloadEvidenceLedgerError::MismatchedBooleanStage(_) => {
            WorkloadCompositionError::MismatchedEvidenceStage(stage)
        }
        WorkloadEvidenceLedgerError::UnsupportedBooleanStage(_) => {
            WorkloadCompositionError::UnsupportedStage(requirement)
        }
        _ => WorkloadCompositionError::UnsupportedStage(requirement),
    }
}

fn boolean_stage_for_requirement(
    requirement: WorkloadStageRequirement,
) -> Option<WorkloadEvidenceStage> {
    match requirement {
        WorkloadStageRequirement::BooleanDeclarationEntry => {
            Some(WorkloadEvidenceStage::BooleanDeclarationEntry)
        }
        WorkloadStageRequirement::BooleanRoutePlan => Some(WorkloadEvidenceStage::BooleanRoutePlan),
        WorkloadStageRequirement::BooleanOperandPairConstruction => {
            Some(WorkloadEvidenceStage::BooleanOperandPairConstruction)
        }
        WorkloadStageRequirement::BooleanBlockerProvenance => {
            Some(WorkloadEvidenceStage::BooleanBlockerProvenance)
        }
        WorkloadStageRequirement::BooleanPrecisionAgreement => {
            Some(WorkloadEvidenceStage::BooleanPrecisionAgreement)
        }
        WorkloadStageRequirement::BooleanSharedPlaneIdentity => {
            Some(WorkloadEvidenceStage::BooleanSharedPlaneIdentity)
        }
        WorkloadStageRequirement::BooleanLocalFrameSelection => {
            Some(WorkloadEvidenceStage::BooleanLocalFrameSelection)
        }
        WorkloadStageRequirement::BooleanOperandAProjectionConsumption => {
            Some(WorkloadEvidenceStage::BooleanOperandAProjectionConsumption)
        }
        WorkloadStageRequirement::BooleanOperandBProjectionConsumption => {
            Some(WorkloadEvidenceStage::BooleanOperandBProjectionConsumption)
        }
        WorkloadStageRequirement::BooleanReducedOperandPair => {
            Some(WorkloadEvidenceStage::BooleanReducedOperandPair)
        }
        WorkloadStageRequirement::BooleanEventExtractionRequest => {
            Some(WorkloadEvidenceStage::BooleanEventExtractionRequest)
        }
        WorkloadStageRequirement::BooleanSegmentPairEnumeration => {
            Some(WorkloadEvidenceStage::BooleanSegmentPairEnumeration)
        }
        WorkloadStageRequirement::BooleanEventLedger => {
            Some(WorkloadEvidenceStage::BooleanEventLedger)
        }
        WorkloadStageRequirement::BooleanSplit => Some(WorkloadEvidenceStage::BooleanSplit),
        _ => None,
    }
}
