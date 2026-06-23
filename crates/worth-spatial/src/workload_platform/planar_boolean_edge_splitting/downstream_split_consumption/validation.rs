use crate::workload_platform::evidence_ledger::{
    WorkloadEvidenceLedgerError, WorkloadEvidenceStage,
};

use super::counters::PlanarBooleanDownstreamSplitConsumptionCounters;
use super::denial::{
    PlanarBooleanDownstreamSplitConsumptionDenial,
    PlanarBooleanDownstreamSplitConsumptionDenialKind as Kind,
};
use super::input::PlanarBooleanDownstreamSplitConsumptionInput;

pub(crate) fn validate_downstream_split_consumption_input(
    input: &PlanarBooleanDownstreamSplitConsumptionInput<'_>,
    counters: &mut PlanarBooleanDownstreamSplitConsumptionCounters,
) -> Result<(), PlanarBooleanDownstreamSplitConsumptionDenial> {
    reject_missing(
        input.split_ledger_receipt().receipt_identity(),
        Kind::MissingSplitLedgerReceipt,
        "split ledger receipt",
        *counters,
        "downstream split consumption requires a Query-owned split ledger receipt",
    )?;
    reject_missing(
        input.decision_log_receipt().receipt_identity(),
        Kind::MissingDecisionLogReceipt,
        "decision log receipt",
        *counters,
        "downstream split consumption requires the Query-owned decision-log receipt",
    )?;
    reject_missing(
        input.validation_receipt().receipt_identity(),
        Kind::MissingValidationReceipt,
        "split-chain validation receipt",
        *counters,
        "downstream split consumption requires split-chain validation authority",
    )?;
    reject_missing(
        input.persistent_naming_receipt().receipt_identity(),
        Kind::MissingPersistentNamingReceipt,
        "persistent naming receipt",
        *counters,
        "downstream split consumption requires persistent naming authority",
    )?;
    reject_missing(
        input.replay_parity_receipt().receipt_identity(),
        Kind::MissingReplayParityReceipt,
        "replay parity receipt",
        *counters,
        "downstream split consumption requires retained replay parity authority",
    )?;
    reject_missing(
        input.stage_index().index_identity(),
        Kind::MissingWorkloadStageIndex,
        "workload stage index",
        *counters,
        "downstream split consumption requires Query-owned workload evidence stage index",
    )?;

    reject_receipt_mismatch(
        input
            .split_ledger_receipt()
            .split_decision_log_receipt_identity(),
        input.decision_log_receipt().receipt_identity(),
        Kind::ForeignDecisionLogReceipt,
        "decision-log-receipt",
        counters,
        "split ledger and decision log receipt must describe the same split decisions",
    )?;
    reject_receipt_mismatch(
        input
            .split_ledger_receipt()
            .split_chain_validation_receipt_identity(),
        input.validation_receipt().receipt_identity(),
        Kind::ForeignValidationReceipt,
        "split-chain-validation-receipt",
        counters,
        "split ledger and validation receipt must describe the same split chain proof",
    )?;
    reject_receipt_mismatch(
        input
            .split_ledger_receipt()
            .split_persistent_naming_receipt_identity(),
        input.persistent_naming_receipt().receipt_identity(),
        Kind::ForeignPersistentNamingReceipt,
        "persistent-naming-receipt",
        counters,
        "split ledger and persistent naming receipt must describe the same named split artifacts",
    )?;
    reject_receipt_mismatch(
        input.split_ledger_receipt().receipt_identity(),
        input
            .replay_parity_receipt()
            .original_split_ledger_receipt_identity(),
        Kind::ForeignReplayParityReceipt,
        "replay-parity-ledger-receipt",
        counters,
        "replay parity receipt must certify the split ledger being consumed downstream",
    )?;
    reject_receipt_mismatch(
        input
            .split_ledger_receipt()
            .downstream_consumption_identity(),
        input
            .replay_parity_receipt()
            .original_downstream_consumption_identity(),
        Kind::ForeignReplayParityReceipt,
        "replay-parity-downstream-identity",
        counters,
        "replay parity receipt must certify the downstream identity being consumed",
    )?;

    input
        .stage_index()
        .require_boolean_receipt(input.split_ledger_receipt())
        .map_err(|error| stage_index_denial(error, input, counters))?;
    Ok(())
}

fn reject_missing(
    observed: &str,
    kind: Kind,
    rejected_identity: &'static str,
    mut counters: PlanarBooleanDownstreamSplitConsumptionCounters,
    human_reason: &'static str,
) -> Result<(), PlanarBooleanDownstreamSplitConsumptionDenial> {
    if observed.is_empty() {
        counters.rejected_missing_receipt();
        return Err(PlanarBooleanDownstreamSplitConsumptionDenial::new(
            kind,
            rejected_identity,
            "non-empty receipt identity",
            observed,
            counters,
            human_reason,
        ));
    }
    Ok(())
}

fn reject_receipt_mismatch(
    expected: &str,
    observed: &str,
    kind: Kind,
    rejected_identity: &'static str,
    counters: &mut PlanarBooleanDownstreamSplitConsumptionCounters,
    human_reason: &'static str,
) -> Result<(), PlanarBooleanDownstreamSplitConsumptionDenial> {
    if expected != observed {
        counters.rejected_foreign_receipt();
        return Err(PlanarBooleanDownstreamSplitConsumptionDenial::new(
            kind,
            rejected_identity,
            expected,
            observed,
            *counters,
            human_reason,
        ));
    }
    Ok(())
}

fn stage_index_denial(
    error: WorkloadEvidenceLedgerError,
    input: &PlanarBooleanDownstreamSplitConsumptionInput<'_>,
    counters: &mut PlanarBooleanDownstreamSplitConsumptionCounters,
) -> PlanarBooleanDownstreamSplitConsumptionDenial {
    match error {
        WorkloadEvidenceLedgerError::MissingBooleanStage(WorkloadEvidenceStage::BooleanSplit)
        | WorkloadEvidenceLedgerError::ManualBooleanStage(WorkloadEvidenceStage::BooleanSplit)
        | WorkloadEvidenceLedgerError::CounterlessBooleanStage(
            WorkloadEvidenceStage::BooleanSplit,
        ) => {
            counters.rejected_non_receipt_evidence();
            PlanarBooleanDownstreamSplitConsumptionDenial::new(
                Kind::NonReceiptBackedBooleanSplitEvidence,
                "boolean-split-stage",
                input.split_ledger_receipt().receipt_identity(),
                "missing or counterless BooleanSplit evidence",
                *counters,
                "downstream split consumption requires receipt-backed BooleanSplit evidence",
            )
        }
        _ => {
            counters.rejected_foreign_receipt();
            PlanarBooleanDownstreamSplitConsumptionDenial::new(
                Kind::ForeignWorkloadStageIndex,
                "workload-stage-index",
                input.split_ledger_receipt().receipt_identity(),
                format!("{error:?}"),
                *counters,
                "workload stage index must contain the exact split ledger receipt",
            )
        }
    }
}
