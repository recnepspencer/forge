use forge_query::facade::ProjectionConsumptionReceipt;
use worth_spatial::facade::evidence_lookup_execution::{
    execute_evidence_lookup, EvidenceLookupExecutionCounters, EvidenceLookupExecutionError,
    EvidenceLookupExecutionErrorKind, EvidenceLookupExecutionOutcome,
    EvidenceLookupExecutionReceipt, EvidenceLookupExecutionRequest,
    EvidenceLookupExecutionTopologySupportState, EvidenceLookupProductOutput,
};
use worth_spatial::facade::evidence_lookup_family_catalog::EvidenceLookupProjectionFactFamily;
use worth_spatial::facade::evidence_lookup_index_product::{
    EvidenceLookupIndexDisposalPosture, EvidenceLookupIndexLifecyclePosture,
    EvidenceLookupIndexProduct,
};
use worth_spatial::facade::evidence_lookup_plan_selection::EvidenceLookupSelectedPlan;

#[test]
fn spatial_public_api_exports_lookup_execution_boundary() {
    let _: fn(
        &EvidenceLookupExecutionRequest<'_>,
    ) -> Result<EvidenceLookupExecutionReceipt, EvidenceLookupExecutionError> =
        execute_evidence_lookup;
    let _ = request_from_plan_and_index;
}

#[test]
fn spatial_public_api_exposes_lookup_execution_request_and_receipt_contract() {
    let _ = request_with_projection_receipt;
    let _ = EvidenceLookupExecutionOutcome::MissingProjectionConsumptionFact;

    let _: fn(&EvidenceLookupExecutionReceipt) -> &str =
        EvidenceLookupExecutionReceipt::execution_receipt_digest;
    let _: fn(&EvidenceLookupExecutionReceipt) -> &str =
        EvidenceLookupExecutionReceipt::selected_plan_digest;
    let _: fn(&EvidenceLookupExecutionReceipt) -> &str =
        EvidenceLookupExecutionReceipt::index_product_digest;
    let _: fn(&EvidenceLookupExecutionReceipt) -> &str =
        EvidenceLookupExecutionReceipt::spatial_touch_digest;
    let _: fn(&EvidenceLookupExecutionReceipt) -> &str =
        EvidenceLookupExecutionReceipt::stage_receipt_digest;
    let _: fn(&EvidenceLookupExecutionReceipt) -> &str =
        EvidenceLookupExecutionReceipt::evidence_ledger_basis_digest;
    let _: fn(&EvidenceLookupExecutionReceipt) -> &str =
        EvidenceLookupExecutionReceipt::topology_support_digest;
    let _: fn(&EvidenceLookupExecutionReceipt) -> EvidenceLookupExecutionTopologySupportState =
        EvidenceLookupExecutionReceipt::topology_support_state;
    let _: fn(&EvidenceLookupExecutionReceipt) -> &str =
        EvidenceLookupExecutionReceipt::query_support_digest;
    let _: fn(&EvidenceLookupExecutionReceipt) -> EvidenceLookupIndexLifecyclePosture =
        EvidenceLookupExecutionReceipt::index_lifecycle_posture;
    let _: fn(&EvidenceLookupExecutionReceipt) -> EvidenceLookupIndexDisposalPosture =
        EvidenceLookupExecutionReceipt::index_disposal_posture;
    let _: fn(&EvidenceLookupExecutionReceipt) -> EvidenceLookupExecutionOutcome =
        EvidenceLookupExecutionReceipt::outcome;
    let _: fn(&EvidenceLookupExecutionReceipt) -> &EvidenceLookupExecutionCounters =
        EvidenceLookupExecutionReceipt::counters;
    let _: fn(&EvidenceLookupExecutionReceipt) -> &str =
        EvidenceLookupExecutionReceipt::counter_digest;
    let _: fn(&EvidenceLookupExecutionReceipt) -> &str =
        EvidenceLookupExecutionReceipt::lookup_product_output_digest;
    let _: fn(&EvidenceLookupExecutionReceipt) -> &EvidenceLookupProductOutput =
        EvidenceLookupExecutionReceipt::lookup_product_output;
    let _: fn(&EvidenceLookupExecutionReceipt) -> bool =
        EvidenceLookupExecutionReceipt::claims_query_descriptor_authority;
}

#[test]
fn spatial_public_api_exposes_lookup_execution_support_types() {
    let _: fn(&EvidenceLookupExecutionCounters) -> usize =
        EvidenceLookupExecutionCounters::selected_family_count;
    let _: fn(&EvidenceLookupExecutionCounters) -> usize =
        EvidenceLookupExecutionCounters::selected_region_count;
    let _: fn(&EvidenceLookupExecutionCounters) -> usize =
        EvidenceLookupExecutionCounters::evidence_candidate_count;
    let _: fn(&EvidenceLookupExecutionCounters) -> usize =
        EvidenceLookupExecutionCounters::ledger_rows_touched_count;
    let _: fn(&EvidenceLookupExecutionCounters) -> usize =
        EvidenceLookupExecutionCounters::index_rows_consumed_count;
    let _: fn(&EvidenceLookupExecutionCounters) -> usize =
        EvidenceLookupExecutionCounters::resident_byte_count;
    let _: fn(&EvidenceLookupExecutionCounters) -> usize =
        EvidenceLookupExecutionCounters::indexed_hit_count;
    let _: fn(&EvidenceLookupExecutionCounters) -> usize =
        EvidenceLookupExecutionCounters::indexed_miss_count;
    let _: fn(&EvidenceLookupExecutionCounters) -> usize =
        EvidenceLookupExecutionCounters::caller_owned_scan_count;
    let _: fn(&EvidenceLookupExecutionCounters) -> usize =
        EvidenceLookupExecutionCounters::query_artifact_count;

    let _: fn(&EvidenceLookupProductOutput) -> &str = EvidenceLookupProductOutput::output_digest;
    let _: fn(&EvidenceLookupProductOutput) -> &str =
        EvidenceLookupProductOutput::execution_receipt_digest;
    let _: fn(&EvidenceLookupProductOutput) -> &[String] =
        EvidenceLookupProductOutput::evidence_receipt_digests;

    let _: fn(&EvidenceLookupExecutionError) -> EvidenceLookupExecutionErrorKind =
        EvidenceLookupExecutionError::kind;
    let _: fn(&EvidenceLookupExecutionError) -> &str = EvidenceLookupExecutionError::detail;
}

fn request_from_plan_and_index<'a>(
    selected_plan: &'a EvidenceLookupSelectedPlan,
    index_product: &'a EvidenceLookupIndexProduct,
) -> EvidenceLookupExecutionRequest<'a> {
    EvidenceLookupExecutionRequest::new(selected_plan, index_product)
}

fn request_with_projection_receipt<'a>(
    request: EvidenceLookupExecutionRequest<'a>,
    projection_receipt: &'a ProjectionConsumptionReceipt,
) -> EvidenceLookupExecutionRequest<'a> {
    request.with_projection_consumption_receipt(
        String::new(),
        EvidenceLookupProjectionFactFamily::SpatialTouchOperandProjection,
        projection_receipt,
    )
}
