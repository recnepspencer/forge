use worth_spatial::facade::evidence_lookup_index_product::{
    require_persistent_evidence_lookup_index_product, EvidenceLookupIndexDisposalPosture,
    EvidenceLookupIndexDisposalPostureKind, EvidenceLookupIndexLifecyclePosture,
    EvidenceLookupIndexLifecyclePostureKind, EvidenceLookupIndexProduct,
    EvidenceLookupIndexProductCounters, EvidenceLookupIndexProductError,
    EvidenceLookupIndexProductErrorKind,
};
use worth_spatial::facade::evidence_lookup_plan_selection::EvidenceLookupSelectedPlan;
use worth_spatial::facade::workload_vocabulary::SelectedLookupSliceLedger;

#[test]
fn spatial_public_api_keeps_lookup_index_product_boundary_read_only() {
    let _: fn(
        &EvidenceLookupSelectedPlan,
        &SelectedLookupSliceLedger,
    ) -> Result<EvidenceLookupIndexProduct, EvidenceLookupIndexProductError> =
        require_persistent_evidence_lookup_index_product;
}

#[test]
fn spatial_public_api_exposes_lookup_index_product_read_contract() {
    let _: fn(&EvidenceLookupIndexProduct) -> &str =
        EvidenceLookupIndexProduct::index_product_digest;
    let _: fn(&EvidenceLookupIndexProduct) -> &str =
        EvidenceLookupIndexProduct::selected_plan_digest;
    let _: fn(&EvidenceLookupIndexProduct) -> &str =
        EvidenceLookupIndexProduct::spatial_touch_digest;
    let _: fn(&EvidenceLookupIndexProduct) -> &str =
        EvidenceLookupIndexProduct::stage_receipt_digest;
    let _: fn(&EvidenceLookupIndexProduct) -> &str =
        EvidenceLookupIndexProduct::evidence_ledger_basis_digest;
    let _: fn(&EvidenceLookupIndexProduct) -> &str =
        EvidenceLookupIndexProduct::topology_support_digest;
    let _: fn(&EvidenceLookupIndexProduct) -> &str =
        EvidenceLookupIndexProduct::query_support_digest;
    let _: fn(&EvidenceLookupIndexProduct) -> EvidenceLookupIndexLifecyclePosture =
        EvidenceLookupIndexProduct::lifecycle_posture;
    let _: fn(&EvidenceLookupIndexProduct) -> EvidenceLookupIndexDisposalPosture =
        EvidenceLookupIndexProduct::disposal_posture;
    let _: fn(&EvidenceLookupIndexProduct) -> &EvidenceLookupIndexProductCounters =
        EvidenceLookupIndexProduct::counters;
    let _: fn(&EvidenceLookupIndexProduct) -> bool =
        EvidenceLookupIndexProduct::claims_lookup_execution;
    let _: fn(&EvidenceLookupIndexProduct) -> bool =
        EvidenceLookupIndexProduct::claims_persistent_capability;
    let _: fn(&EvidenceLookupIndexProduct) -> bool =
        EvidenceLookupIndexProduct::claims_query_descriptor_authority;
}

#[test]
fn spatial_public_api_exposes_lifecycle_counters_and_errors() {
    let _: fn(&EvidenceLookupIndexDisposalPosture) -> EvidenceLookupIndexDisposalPostureKind =
        EvidenceLookupIndexDisposalPosture::kind;
    let _: fn(&EvidenceLookupIndexLifecyclePosture) -> EvidenceLookupIndexLifecyclePostureKind =
        EvidenceLookupIndexLifecyclePosture::kind;
    let _: fn(&EvidenceLookupIndexLifecyclePosture) -> bool =
        EvidenceLookupIndexLifecyclePosture::claims_persistent_capability;
    let _: fn(&EvidenceLookupIndexProductCounters) -> usize =
        EvidenceLookupIndexProductCounters::selected_basis_row_count;
    let _: fn(&EvidenceLookupIndexProductCounters) -> usize =
        EvidenceLookupIndexProductCounters::total_ledger_row_count;
    let _: fn(&EvidenceLookupIndexProductCounters) -> usize =
        EvidenceLookupIndexProductCounters::indexed_family_count;
    let _: fn(&EvidenceLookupIndexProductCounters) -> usize =
        EvidenceLookupIndexProductCounters::resident_byte_count;
    let _: fn(&EvidenceLookupIndexProductCounters) -> usize =
        EvidenceLookupIndexProductCounters::reused_index_count;
    let _: fn(&EvidenceLookupIndexProductError) -> EvidenceLookupIndexProductErrorKind =
        EvidenceLookupIndexProductError::kind;
    let _: fn(&EvidenceLookupIndexProductError) -> &str = EvidenceLookupIndexProductError::detail;
    let _: fn(&EvidenceLookupIndexProductError) -> &EvidenceLookupIndexProductCounters =
        EvidenceLookupIndexProductError::counters;
    let _: fn(&EvidenceLookupIndexProductError) -> Option<EvidenceLookupIndexLifecyclePosture> =
        EvidenceLookupIndexProductError::required_lifecycle_posture;
}
