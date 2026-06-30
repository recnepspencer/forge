use super::counters::PlanarBooleanLoopReconstructionSplitConsumptionCounters;
use super::denial::{
    PlanarBooleanLoopReconstructionSplitConsumptionDenial,
    PlanarBooleanLoopReconstructionSplitConsumptionDenialKind as Kind,
};
use super::input::PlanarBooleanLoopReconstructionSplitConsumptionInput;

pub(crate) fn validate_loop_reconstruction_split_consumption_input(
    input: &PlanarBooleanLoopReconstructionSplitConsumptionInput<'_>,
    counters: &mut PlanarBooleanLoopReconstructionSplitConsumptionCounters,
) -> Result<(), PlanarBooleanLoopReconstructionSplitConsumptionDenial> {
    let downstream = input.downstream_consumption();
    reject_missing(
        downstream.consumption_identity(),
        Kind::MissingDownstreamSplitConsumption,
        "downstream split consumption",
        counters,
        "loop reconstruction must consume the downstream split-consumption product",
    )?;
    reject_missing(
        downstream.split_ledger_receipt_identity(),
        Kind::MissingSplitLedgerReceipt,
        "split ledger receipt",
        counters,
        "loop reconstruction requires the split ledger receipt already admitted downstream",
    )?;
    reject_missing(
        downstream.split_ledger_downstream_identity(),
        Kind::MissingSplitLedgerDownstreamIdentity,
        "split ledger downstream identity",
        counters,
        "loop reconstruction requires the split ledger downstream identity",
    )?;
    reject_missing(
        downstream.split_request_identity(),
        Kind::MissingSplitRequest,
        "split request",
        counters,
        "loop reconstruction requires split request lineage",
    )?;
    reject_missing(
        downstream.lookup_execution_receipt_digest(),
        Kind::MissingSpatialLookupProduct,
        "lookup execution receipt",
        counters,
        "loop reconstruction must preserve downstream receipt-backed lookup execution proof",
    )
}

fn reject_missing(
    observed: &str,
    kind: Kind,
    rejected_identity: &'static str,
    counters: &mut PlanarBooleanLoopReconstructionSplitConsumptionCounters,
    human_reason: &'static str,
) -> Result<(), PlanarBooleanLoopReconstructionSplitConsumptionDenial> {
    if observed.is_empty() {
        counters.rejected_missing_authority();
        return Err(PlanarBooleanLoopReconstructionSplitConsumptionDenial::new(
            kind,
            rejected_identity,
            *counters,
            human_reason,
        ));
    }
    Ok(())
}
