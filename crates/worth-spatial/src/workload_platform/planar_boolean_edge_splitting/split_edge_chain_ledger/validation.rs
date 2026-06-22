use super::counters::PlanarBooleanSplitEdgeChainLedgerCounters;
use super::denial::{
    PlanarBooleanSplitEdgeChainLedgerDenial, PlanarBooleanSplitEdgeChainLedgerDenialKind,
};
use super::input::PlanarBooleanSplitEdgeChainLedgerInput;

pub(crate) fn validate_product_lineage(
    input: &PlanarBooleanSplitEdgeChainLedgerInput<'_>,
    counters: &mut PlanarBooleanSplitEdgeChainLedgerCounters,
) -> Result<(), PlanarBooleanSplitEdgeChainLedgerDenial> {
    let declaration = input.declaration();
    if declaration.split_request_identity() != input.split_request().split_request_identity() {
        counters.rejected_foreign_product();
        return Err(foreign(
            PlanarBooleanSplitEdgeChainLedgerDenialKind::ForeignValidationReceipt,
            declaration.split_request_identity(),
            *counters,
            "split ledger declaration must bind the provided split request",
        ));
    }
    if declaration.split_chain_validation_receipt_identity()
        != input.split_chain_validation().receipt_identity()
    {
        counters.rejected_foreign_product();
        return Err(foreign(
            PlanarBooleanSplitEdgeChainLedgerDenialKind::ForeignValidationReceipt,
            declaration.split_chain_validation_receipt_identity(),
            *counters,
            "split ledger declaration must bind the provided chain-validation receipt",
        ));
    }
    if declaration.split_persistent_naming_receipt_identity()
        != input.split_persistent_names().receipt_identity()
    {
        counters.rejected_foreign_product();
        return Err(foreign(
            PlanarBooleanSplitEdgeChainLedgerDenialKind::ForeignPersistentNamingReceipt,
            declaration.split_persistent_naming_receipt_identity(),
            *counters,
            "split ledger declaration must bind the provided persistent-naming receipt",
        ));
    }
    if declaration.split_decision_log_receipt_identity()
        != input.split_decision_log().receipt().receipt_identity()
    {
        counters.rejected_foreign_product();
        return Err(foreign(
            PlanarBooleanSplitEdgeChainLedgerDenialKind::ForeignDecisionLogReceipt,
            declaration.split_decision_log_receipt_identity(),
            *counters,
            "split ledger declaration must bind the provided decision-log receipt",
        ));
    }
    validate_schedule_and_artifact_lineage(input, counters)?;
    if !input
        .split_chain_validation()
        .certifies_split_chain_integrity()
    {
        counters.rejected_missing_validation();
        return Err(foreign(
            PlanarBooleanSplitEdgeChainLedgerDenialKind::MissingFragmentValidationCoverage,
            input.split_chain_validation().receipt_identity(),
            *counters,
            "split ledger requires a successful split-chain validation receipt",
        ));
    }
    if !input
        .split_persistent_names()
        .certifies_query_native_split_persistent_naming()
    {
        counters.rejected_missing_persistent_name();
        return Err(foreign(
            PlanarBooleanSplitEdgeChainLedgerDenialKind::MissingPersistentNameBinding,
            input.split_persistent_names().receipt_identity(),
            *counters,
            "split ledger requires Query-native persistent naming",
        ));
    }
    if !input
        .split_decision_log()
        .certifies_query_owned_decision_log()
    {
        counters.rejected_missing_decision_log();
        return Err(foreign(
            PlanarBooleanSplitEdgeChainLedgerDenialKind::MissingDecisionLogReceipt,
            input.split_decision_log().receipt().receipt_identity(),
            *counters,
            "split ledger requires a Query-owned decision log with coverage proof",
        ));
    }
    counters.consumed_validation_receipt();
    Ok(())
}

fn validate_schedule_and_artifact_lineage(
    input: &PlanarBooleanSplitEdgeChainLedgerInput<'_>,
    counters: &mut PlanarBooleanSplitEdgeChainLedgerCounters,
) -> Result<(), PlanarBooleanSplitEdgeChainLedgerDenial> {
    if input.endpoint_boundary_schedules().schedule_set_identity()
        != input
            .interval_subdivision_schedules()
            .endpoint_boundary_schedule_set_identity()
    {
        counters.rejected_foreign_product();
        return Err(foreign(
            PlanarBooleanSplitEdgeChainLedgerDenialKind::ForeignScheduleProduct,
            input
                .interval_subdivision_schedules()
                .schedule_set_identity(),
            *counters,
            "split ledger interval schedules must consume the endpoint-boundary product lineage",
        ));
    }
    if input
        .interval_subdivision_schedules()
        .schedule_set_identity()
        != input
            .split_vertices()
            .interval_subdivision_schedule_set_identity()
    {
        counters.rejected_foreign_product();
        return Err(foreign(
            PlanarBooleanSplitEdgeChainLedgerDenialKind::ForeignScheduleProduct,
            input.split_vertices().split_vertex_identity_set_identity(),
            *counters,
            "split ledger vertices must consume the interval-subdivision product lineage",
        ));
    }
    if input
        .interval_subdivision_schedules()
        .schedule_set_identity()
        != input
            .split_fragments()
            .interval_subdivision_schedule_set_identity()
    {
        counters.rejected_foreign_product();
        return Err(foreign(
            PlanarBooleanSplitEdgeChainLedgerDenialKind::ForeignSplitArtifactProduct,
            input.split_fragments().fragment_set_identity(),
            *counters,
            "split ledger fragments must consume the interval-subdivision product lineage",
        ));
    }
    if input.split_vertices().split_vertex_identity_set_identity()
        != input.split_fragments().split_vertex_identity_set_identity()
    {
        counters.rejected_foreign_product();
        return Err(foreign(
            PlanarBooleanSplitEdgeChainLedgerDenialKind::ForeignSplitArtifactProduct,
            input.split_fragments().fragment_set_identity(),
            *counters,
            "split ledger fragments must consume the split-vertex product lineage",
        ));
    }
    if input
        .interval_subdivision_schedules()
        .schedule_set_identity()
        != input
            .overlap_chains()
            .interval_subdivision_schedule_set_identity()
    {
        counters.rejected_foreign_product();
        return Err(foreign(
            PlanarBooleanSplitEdgeChainLedgerDenialKind::ForeignSplitArtifactProduct,
            input.overlap_chains().chain_set_identity(),
            *counters,
            "split ledger overlap chains must consume the interval-subdivision product lineage",
        ));
    }
    if input.split_fragments().fragment_set_identity()
        != input.overlap_chains().split_edge_fragment_set_identity()
    {
        counters.rejected_foreign_product();
        return Err(foreign(
            PlanarBooleanSplitEdgeChainLedgerDenialKind::ForeignSplitArtifactProduct,
            input.overlap_chains().chain_set_identity(),
            *counters,
            "split ledger overlap chains must consume the split-fragment product lineage",
        ));
    }
    if input.split_fragments().fragment_set_identity()
        != input
            .split_chain_validation()
            .split_edge_fragment_set_identity()
    {
        counters.rejected_foreign_product();
        return Err(foreign(
            PlanarBooleanSplitEdgeChainLedgerDenialKind::ForeignValidationReceipt,
            input.split_chain_validation().receipt_identity(),
            *counters,
            "split ledger validation must consume the split-fragment product lineage",
        ));
    }
    if input.overlap_chains().chain_set_identity()
        != input
            .split_chain_validation()
            .overlap_edge_chain_set_identity()
    {
        counters.rejected_foreign_product();
        return Err(foreign(
            PlanarBooleanSplitEdgeChainLedgerDenialKind::ForeignValidationReceipt,
            input.split_chain_validation().receipt_identity(),
            *counters,
            "split ledger validation must consume the overlap-chain product lineage",
        ));
    }
    if input
        .interval_subdivision_schedules()
        .schedule_set_identity()
        != input
            .split_chain_validation()
            .interval_subdivision_schedule_set_identity()
    {
        counters.rejected_foreign_product();
        return Err(foreign(
            PlanarBooleanSplitEdgeChainLedgerDenialKind::ForeignValidationReceipt,
            input.split_chain_validation().receipt_identity(),
            *counters,
            "split ledger validation must consume the interval-subdivision product lineage",
        ));
    }
    if input.split_chain_validation().receipt_identity()
        != input
            .split_persistent_names()
            .split_chain_validation_receipt_identity()
    {
        counters.rejected_foreign_product();
        return Err(foreign(
            PlanarBooleanSplitEdgeChainLedgerDenialKind::ForeignPersistentNamingReceipt,
            input.split_persistent_names().receipt_identity(),
            *counters,
            "split ledger persistent names must consume the validation receipt lineage",
        ));
    }
    if input.split_fragments().fragment_set_identity()
        != input
            .split_persistent_names()
            .split_edge_fragment_set_identity()
    {
        counters.rejected_foreign_product();
        return Err(foreign(
            PlanarBooleanSplitEdgeChainLedgerDenialKind::ForeignPersistentNamingReceipt,
            input.split_persistent_names().receipt_identity(),
            *counters,
            "split ledger persistent names must consume the split-fragment product lineage",
        ));
    }
    if input.split_vertices().split_vertex_identity_set_identity()
        != input
            .split_persistent_names()
            .split_vertex_identity_set_identity()
    {
        counters.rejected_foreign_product();
        return Err(foreign(
            PlanarBooleanSplitEdgeChainLedgerDenialKind::ForeignPersistentNamingReceipt,
            input.split_persistent_names().receipt_identity(),
            *counters,
            "split ledger persistent names must consume the split-vertex product lineage",
        ));
    }
    if input.overlap_chains().chain_set_identity()
        != input
            .split_persistent_names()
            .overlap_edge_chain_set_identity()
    {
        counters.rejected_foreign_product();
        return Err(foreign(
            PlanarBooleanSplitEdgeChainLedgerDenialKind::ForeignPersistentNamingReceipt,
            input.split_persistent_names().receipt_identity(),
            *counters,
            "split ledger persistent names must consume the overlap-chain product lineage",
        ));
    }
    Ok(())
}

fn foreign(
    kind: PlanarBooleanSplitEdgeChainLedgerDenialKind,
    artifact_identity: &str,
    counters: PlanarBooleanSplitEdgeChainLedgerCounters,
    message: &str,
) -> PlanarBooleanSplitEdgeChainLedgerDenial {
    PlanarBooleanSplitEdgeChainLedgerDenial::new(kind, artifact_identity, counters, message)
}
