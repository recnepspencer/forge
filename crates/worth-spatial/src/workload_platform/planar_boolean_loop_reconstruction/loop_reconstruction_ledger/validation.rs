use super::counters::PlanarBooleanLoopReconstructionLedgerCounters;
use super::denial::{
    PlanarBooleanLoopReconstructionLedgerDenial,
    PlanarBooleanLoopReconstructionLedgerDenialKind as Kind,
};
use super::input::PlanarBooleanLoopReconstructionLedgerInput;

pub(crate) fn validate_input(
    input: PlanarBooleanLoopReconstructionLedgerInput<'_>,
    counters: &mut PlanarBooleanLoopReconstructionLedgerCounters,
) -> Result<(), PlanarBooleanLoopReconstructionLedgerDenial> {
    let request_identity = input.request().request_identity();
    if input.decision_log().split_ledger_receipt_identity()
        != input.request().split_ledger_receipt_identity()
    {
        counters.denied_split_ledger_lineage_mismatch();
        return Err(PlanarBooleanLoopReconstructionLedgerDenial::new(
            Kind::SplitLedgerLineageMismatch,
            input.decision_log().split_ledger_receipt_identity(),
            *counters,
            "loop reconstruction ledger requires the decision log to preserve the same split-ledger receipt lineage as the loop reconstruction request",
        ));
    }
    for observed in [
        input.decision_log().request_identity(),
        input.loop_identity_map().request_identity(),
        input.persistent_name_map().request_identity(),
        input.subshape_signature_map().request_identity(),
        input.reconstructed_loops().request_identity(),
        input.born_loops().request_identity(),
        input.island_partition().request_identity(),
        input.split_attribution().request_identity(),
        input.role_outcomes().request_identity(),
        input.degenerate_outcomes().request_identity(),
    ] {
        if observed != request_identity {
            counters.denied_request_identity_mismatch();
            return Err(PlanarBooleanLoopReconstructionLedgerDenial::new(
                Kind::RequestIdentityMismatch,
                observed,
                *counters,
                "loop reconstruction ledger only admits products from one request boundary",
            ));
        }
    }
    Ok(())
}
