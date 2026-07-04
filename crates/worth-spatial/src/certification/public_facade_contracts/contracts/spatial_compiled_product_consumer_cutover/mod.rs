use worth_spatial::facade::evidence_lookup_execution::EvidenceLookupExecutionReceipt;
use worth_spatial::facade::evidence_lookup_index_product::{
    EvidenceLookupIndexProduct, EvidenceLookupIndexProductError, EvidenceLookupIndexReuseResolution,
};
use worth_spatial::facade::evidence_lookup_plan_selection::EvidenceLookupSelectedPlan;
use worth_spatial::facade::evidence_lookup_workload_cutover::EvidenceLookupConsumedWorkloadHandoff;
use worth_spatial::facade::spatial_compiled_product_consumer_cutover::{
    admit_lookup_execution_handoff_match, admit_lookup_product_handoff_match,
    lower_evidence_lookup_index_product, reuse_evidence_lookup_index_product,
    SpatialLookupConsumerRouteDenial, SpatialLookupConsumerRouteDenialKind,
};
use worth_spatial::facade::workload_vocabulary::SelectedLookupSliceLedger;

#[test]
fn spatial_public_api_exports_cutover_lookup_product_authority() {
    let _: fn(
        &EvidenceLookupSelectedPlan,
        &SelectedLookupSliceLedger,
    ) -> Result<EvidenceLookupIndexProduct, EvidenceLookupIndexProductError> =
        lower_evidence_lookup_index_product;
    let _: fn(
        &EvidenceLookupSelectedPlan,
        &SelectedLookupSliceLedger,
        &EvidenceLookupIndexProduct,
    ) -> Result<EvidenceLookupIndexReuseResolution, EvidenceLookupIndexProductError> =
        reuse_evidence_lookup_index_product;
}

#[test]
fn spatial_public_api_exports_cutover_lookup_route_contracts() {
    let _: fn(
        &EvidenceLookupConsumedWorkloadHandoff,
        &EvidenceLookupExecutionReceipt,
    ) -> Result<(), SpatialLookupConsumerRouteDenial> = admit_lookup_execution_handoff_match;
    let _: fn(
        &EvidenceLookupConsumedWorkloadHandoff,
        &EvidenceLookupIndexProduct,
    ) -> Result<(), SpatialLookupConsumerRouteDenial> = admit_lookup_product_handoff_match;
    let _: fn(&SpatialLookupConsumerRouteDenial) -> SpatialLookupConsumerRouteDenialKind =
        SpatialLookupConsumerRouteDenial::kind;
}
