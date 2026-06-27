use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceReceipt, WorkloadEvidenceLedgerError, WorkloadEvidenceStage,
    WorkloadEvidenceStageCounters, WorkloadEvidenceSupport,
};

use super::product::WorkloadEvidenceStageIndexProduct;
use super::receipt_lookup::WorkloadEvidenceBooleanReceiptLookupProduct;

pub(crate) fn match_boolean_receipt_lookup<T: BooleanEvidenceReceipt + 'static>(
    product: &WorkloadEvidenceStageIndexProduct,
    receipt: &T,
) -> Result<WorkloadEvidenceBooleanReceiptLookupProduct, WorkloadEvidenceLedgerError> {
    let stage = receipt.boolean_stage().evidence_stage();
    let lookup = match_boolean_row_lookup(
        product,
        stage,
        receipt.evidence_identity(),
        receipt.evidence_support(),
        receipt.evidence_counters(),
    )?;
    let row = product
        .row_for_stage(stage)
        .ok_or(WorkloadEvidenceLedgerError::MissingBooleanStage(stage))?;
    if !row.matches_receipt_type::<T>() {
        return Err(WorkloadEvidenceLedgerError::MismatchedBooleanStage(stage));
    }
    Ok(WorkloadEvidenceBooleanReceiptLookupProduct::new(
        receipt.boolean_stage(),
        lookup.evidence_stage(),
        lookup.evidence_identity(),
        lookup.support(),
        lookup.counters(),
        lookup.stage_index_identity(),
    ))
}

pub(crate) fn match_boolean_row_lookup(
    product: &WorkloadEvidenceStageIndexProduct,
    stage: WorkloadEvidenceStage,
    evidence_identity: &str,
    support: WorkloadEvidenceSupport,
    counters: WorkloadEvidenceStageCounters,
) -> Result<WorkloadEvidenceBooleanReceiptLookupProduct, WorkloadEvidenceLedgerError> {
    let row = product
        .row_for_stage(stage)
        .ok_or(WorkloadEvidenceLedgerError::MissingBooleanStage(stage))?;
    if !row.is_receipt_backed() {
        return Err(WorkloadEvidenceLedgerError::ManualBooleanStage(stage));
    }
    if !row.counters().has_receipt_backed_counter_for_stage(stage) {
        return Err(WorkloadEvidenceLedgerError::CounterlessBooleanStage(stage));
    }
    if row.counters() != counters {
        return Err(WorkloadEvidenceLedgerError::MismatchedBooleanStageCounters(
            stage,
        ));
    }
    if row.evidence_identity() != evidence_identity {
        return Err(WorkloadEvidenceLedgerError::MismatchedBooleanStage(stage));
    }
    if row.support() != support {
        return Err(match support {
            WorkloadEvidenceSupport::Manual => {
                WorkloadEvidenceLedgerError::ManualBooleanStage(stage)
            }
            WorkloadEvidenceSupport::Admitted
            | WorkloadEvidenceSupport::Unsupported
            | WorkloadEvidenceSupport::Blocked => {
                WorkloadEvidenceLedgerError::UnsupportedBooleanStage(stage)
            }
        });
    }
    Ok(WorkloadEvidenceBooleanReceiptLookupProduct::new(
        boolean_stage_kind(stage),
        stage,
        row.evidence_identity(),
        row.support(),
        row.counters(),
        product.index_identity(),
    ))
}

fn boolean_stage_kind(
    stage: WorkloadEvidenceStage,
) -> crate::workload_platform::evidence_ledger::BooleanEvidenceStageKind {
    match stage {
        WorkloadEvidenceStage::BooleanDeclarationEntry => {
            crate::workload_platform::evidence_ledger::BooleanEvidenceStageKind::DeclarationEntry
        }
        WorkloadEvidenceStage::BooleanRoutePlan => {
            crate::workload_platform::evidence_ledger::BooleanEvidenceStageKind::RoutePlan
        }
        WorkloadEvidenceStage::BooleanOperandPairConstruction => crate::workload_platform::evidence_ledger::BooleanEvidenceStageKind::OperandPairConstruction,
        WorkloadEvidenceStage::BooleanBlockerProvenance => {
            crate::workload_platform::evidence_ledger::BooleanEvidenceStageKind::BlockerProvenance
        }
        WorkloadEvidenceStage::BooleanPrecisionAgreement => {
            crate::workload_platform::evidence_ledger::BooleanEvidenceStageKind::PrecisionAgreement
        }
        WorkloadEvidenceStage::BooleanSharedPlaneIdentity => {
            crate::workload_platform::evidence_ledger::BooleanEvidenceStageKind::SharedPlaneIdentity
        }
        WorkloadEvidenceStage::BooleanLocalFrameSelection => {
            crate::workload_platform::evidence_ledger::BooleanEvidenceStageKind::LocalFrameSelection
        }
        WorkloadEvidenceStage::BooleanOperandAProjectionConsumption => crate::workload_platform::evidence_ledger::BooleanEvidenceStageKind::OperandAProjectionConsumption,
        WorkloadEvidenceStage::BooleanOperandBProjectionConsumption => crate::workload_platform::evidence_ledger::BooleanEvidenceStageKind::OperandBProjectionConsumption,
        WorkloadEvidenceStage::BooleanReducedOperandPair => {
            crate::workload_platform::evidence_ledger::BooleanEvidenceStageKind::ReducedOperandPair
        }
        WorkloadEvidenceStage::BooleanEventExtractionRequest => crate::workload_platform::evidence_ledger::BooleanEvidenceStageKind::EventExtractionRequest,
        WorkloadEvidenceStage::BooleanSegmentPairEnumeration => crate::workload_platform::evidence_ledger::BooleanEvidenceStageKind::SegmentPairEnumeration,
        WorkloadEvidenceStage::BooleanEventLedger => {
            crate::workload_platform::evidence_ledger::BooleanEvidenceStageKind::EventLedger
        }
        WorkloadEvidenceStage::BooleanSplit => {
            crate::workload_platform::evidence_ledger::BooleanEvidenceStageKind::Split
        }
        WorkloadEvidenceStage::BooleanLoopReconstruction => crate::workload_platform::evidence_ledger::BooleanEvidenceStageKind::LoopReconstruction,
        WorkloadEvidenceStage::BooleanClassify => {
            crate::workload_platform::evidence_ledger::BooleanEvidenceStageKind::Classify
        }
        WorkloadEvidenceStage::BooleanAssemble => {
            crate::workload_platform::evidence_ledger::BooleanEvidenceStageKind::Assemble
        }
        WorkloadEvidenceStage::BooleanCleanup => {
            crate::workload_platform::evidence_ledger::BooleanEvidenceStageKind::Cleanup
        }
        other => panic!("boolean row lookup requires a boolean stage, got {other:?}"),
    }
}
