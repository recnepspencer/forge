use super::counters::PlanarBooleanLoopSourceProvenanceCounters;
use super::denial::{
    PlanarBooleanLoopSourceProvenanceDenial, PlanarBooleanLoopSourceProvenanceDenialKind as Kind,
};
use super::input::PlanarBooleanLoopSourceProvenanceRecoveryInput;

pub(crate) fn validate_loop_source_provenance_input(
    input: &PlanarBooleanLoopSourceProvenanceRecoveryInput<'_>,
    counters: &mut PlanarBooleanLoopSourceProvenanceCounters,
) -> Result<(), PlanarBooleanLoopSourceProvenanceDenial> {
    if input.request().split_ledger_receipt_identity()
        != input.split_ledger_receipt().receipt_identity()
    {
        counters.rejected_foreign_lineage();
        return Err(PlanarBooleanLoopSourceProvenanceDenial::new(
            Kind::ForeignSplitLedgerReceipt,
            input.split_ledger_receipt().receipt_identity(),
            *counters,
            "loop source provenance requires the exact split-ledger receipt bound by the loop request",
        ));
    }
    if input.request().split_request_identity() != input.split_ledger().split_request_identity()
        || input.request().split_request_identity()
            != input.split_ledger_receipt().split_request_identity()
    {
        counters.rejected_foreign_lineage();
        return Err(PlanarBooleanLoopSourceProvenanceDenial::new(
            Kind::ForeignSplitRequestLineage,
            input.request().split_request_identity(),
            *counters,
            "loop source provenance requires split-ledger request lineage to match the loop request",
        ));
    }
    if input.recovered_source_carriers().split_request_identity()
        != input.request().split_request_identity()
    {
        counters.rejected_foreign_lineage();
        return Err(PlanarBooleanLoopSourceProvenanceDenial::new(
            Kind::ForeignSourceCarrierLineage,
            input.recovered_source_carriers().carrier_set_identity(),
            *counters,
            "loop source provenance requires recovered source carriers from the same split request lineage",
        ));
    }
    if input.split_fragments().certifies_domain_coverage().not() {
        counters.rejected_foreign_lineage();
        return Err(PlanarBooleanLoopSourceProvenanceDenial::new(
            Kind::ForeignFragmentLineage,
            input.split_fragments().fragment_set_identity(),
            *counters,
            "loop source provenance requires certified split-fragment domain coverage",
        ));
    }
    if input.overlap_chains().split_edge_fragment_set_identity()
        != input.split_fragments().fragment_set_identity()
    {
        counters.rejected_foreign_lineage();
        return Err(PlanarBooleanLoopSourceProvenanceDenial::new(
            Kind::ForeignOverlapChainLineage,
            input.overlap_chains().chain_set_identity(),
            *counters,
            "loop source provenance requires overlap-chain lineage derived from the exact split-fragment set being consumed",
        ));
    }
    if input.overlap_chains().emits_topology_truth() {
        counters.rejected_foreign_lineage();
        return Err(PlanarBooleanLoopSourceProvenanceDenial::new(
            Kind::ForeignOverlapChainLineage,
            input.overlap_chains().chain_set_identity(),
            *counters,
            "loop source provenance may consume only prepared overlap-chain lineage, not topology truth products",
        ));
    }
    Ok(())
}

trait BoolExt {
    fn not(self) -> bool;
}

impl BoolExt for bool {
    fn not(self) -> bool {
        !self
    }
}
