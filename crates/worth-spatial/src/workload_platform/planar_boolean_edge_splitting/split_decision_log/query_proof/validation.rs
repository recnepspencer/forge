use super::counters::PlanarBooleanSplitDecisionLogCounters;
use super::denial::{PlanarBooleanSplitDecisionLogDenial, PlanarBooleanSplitDecisionLogDenialKind};
use super::input::PlanarBooleanSplitDecisionLogInput;

pub(super) fn validate_product_lineage(
    input: &PlanarBooleanSplitDecisionLogInput<'_>,
    counters: &mut PlanarBooleanSplitDecisionLogCounters,
) -> Result<(), PlanarBooleanSplitDecisionLogDenial> {
    if let Some(split_request) = input.split_request() {
        if input.declaration().split_request_identity() != split_request.split_request_identity() {
            counters.rejected_foreign_product();
            return Err(denial(
                PlanarBooleanSplitDecisionLogDenialKind::ForeignSplitRequestProduct,
                split_request.split_request_identity(),
                *counters,
                "split decision log Query declaration must bind the consumed split request",
            ));
        }
    }
    if input
        .declaration()
        .split_chain_validation_receipt_identity()
        != input.split_chain_validation().receipt_identity()
    {
        counters.rejected_foreign_product();
        return Err(denial(
            PlanarBooleanSplitDecisionLogDenialKind::ForeignChainValidationProduct,
            input.split_chain_validation().receipt_identity(),
            *counters,
            "split decision log Query declaration must bind the consumed chain-validation receipt",
        ));
    }
    if input
        .declaration()
        .split_persistent_naming_receipt_identity()
        != input.split_persistent_names().receipt_identity()
    {
        counters.rejected_foreign_product();
        return Err(denial(
            PlanarBooleanSplitDecisionLogDenialKind::ForeignPersistentNamingProduct,
            input.split_persistent_names().receipt_identity(),
            *counters,
            "split decision log Query declaration must bind the consumed persistent-naming receipt",
        ));
    }
    if input.endpoint_boundary_schedules().schedule_set_identity()
        != input
            .interval_subdivision_schedules()
            .endpoint_boundary_schedule_set_identity()
    {
        counters.rejected_foreign_product();
        return Err(denial(
            PlanarBooleanSplitDecisionLogDenialKind::ForeignIntervalSubdivisionProduct,
            input
                .interval_subdivision_schedules()
                .schedule_set_identity(),
            *counters,
            "interval subdivision decisions must consume the endpoint-boundary product lineage",
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
        return Err(denial(
            PlanarBooleanSplitDecisionLogDenialKind::ForeignSplitVertexProduct,
            input.split_vertices().split_vertex_identity_set_identity(),
            *counters,
            "split vertex decisions must consume the interval-subdivision product lineage",
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
        return Err(denial(
            PlanarBooleanSplitDecisionLogDenialKind::ForeignSplitFragmentProduct,
            input.split_fragments().fragment_set_identity(),
            *counters,
            "split fragment decisions must consume the interval-subdivision product lineage",
        ));
    }
    if input.split_vertices().split_vertex_identity_set_identity()
        != input.split_fragments().split_vertex_identity_set_identity()
    {
        counters.rejected_foreign_product();
        return Err(denial(
            PlanarBooleanSplitDecisionLogDenialKind::ForeignSplitFragmentProduct,
            input.split_fragments().fragment_set_identity(),
            *counters,
            "split fragment decisions must consume the split-vertex product lineage",
        ));
    }
    if input.split_fragments().fragment_set_identity()
        != input
            .split_chain_validation()
            .split_edge_fragment_set_identity()
    {
        counters.rejected_foreign_product();
        return Err(denial(
            PlanarBooleanSplitDecisionLogDenialKind::ForeignChainValidationProduct,
            input.split_chain_validation().receipt_identity(),
            *counters,
            "chain-validation decisions must consume the split-fragment product lineage",
        ));
    }
    if input.split_chain_validation().receipt_identity()
        != input
            .split_persistent_names()
            .split_chain_validation_receipt_identity()
    {
        counters.rejected_foreign_product();
        return Err(denial(
            PlanarBooleanSplitDecisionLogDenialKind::ForeignPersistentNamingProduct,
            input.split_persistent_names().receipt_identity(),
            *counters,
            "persistent-name decisions must consume the split-chain validation lineage",
        ));
    }
    Ok(())
}

fn denial(
    kind: PlanarBooleanSplitDecisionLogDenialKind,
    evidence_identity: impl Into<String>,
    counters: PlanarBooleanSplitDecisionLogCounters,
    human_reason: impl Into<String>,
) -> PlanarBooleanSplitDecisionLogDenial {
    PlanarBooleanSplitDecisionLogDenial::new(kind, evidence_identity, counters, human_reason)
}
