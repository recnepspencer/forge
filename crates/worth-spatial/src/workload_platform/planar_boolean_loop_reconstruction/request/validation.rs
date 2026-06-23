use super::counters::PlanarBooleanLoopReconstructionRequestCounters;
use super::denial::{
    PlanarBooleanLoopReconstructionRequestDenial,
    PlanarBooleanLoopReconstructionRequestDenialKind as Kind,
};
use super::input::PlanarBooleanLoopReconstructionRequestInput;

pub(crate) fn validate_loop_reconstruction_request_input(
    input: &PlanarBooleanLoopReconstructionRequestInput<'_>,
    counters: &mut PlanarBooleanLoopReconstructionRequestCounters,
) -> Result<(), PlanarBooleanLoopReconstructionRequestDenial> {
    let split_consumption = input.split_consumption();
    reject_missing(
        split_consumption.consumption_identity(),
        Kind::MissingLoopSplitConsumption,
        "loop split consumption",
        counters,
        "loop reconstruction request requires the admitted loop split-consumption product",
    )?;
    reject_missing(
        split_consumption.split_ledger_receipt_identity(),
        Kind::MissingSplitLedgerReceipt,
        "split ledger receipt",
        counters,
        "loop reconstruction request requires a real split-ledger receipt lineage",
    )?;
    reject_missing(
        split_consumption.split_request_identity(),
        Kind::MissingSplitRequest,
        "split request",
        counters,
        "loop reconstruction request requires split request lineage",
    )?;
    reject_missing(
        split_consumption.workload_stage_index_identity(),
        Kind::MissingWorkloadStageIndex,
        "workload stage index",
        counters,
        "loop reconstruction request must preserve workload stage-index authority",
    )
}

fn reject_missing(
    observed: &str,
    kind: Kind,
    rejected_identity: &'static str,
    counters: &mut PlanarBooleanLoopReconstructionRequestCounters,
    human_reason: &'static str,
) -> Result<(), PlanarBooleanLoopReconstructionRequestDenial> {
    if observed.is_empty() {
        counters.rejected_missing_authority();
        return Err(PlanarBooleanLoopReconstructionRequestDenial::new(
            kind,
            rejected_identity,
            *counters,
            human_reason,
        ));
    }
    Ok(())
}
