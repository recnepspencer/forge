use worth_spatial::facade::evidence_lookup_family_catalog::current_evidence_lookup_family_catalog;
use worth_spatial::facade::evidence_lookup_input_admission::{
    admit_evidence_lookup_input, EvidenceLookupAdmittedInput,
};
use worth_spatial::facade::evidence_lookup_plan_selection::{
    select_evidence_lookup_plan, EvidenceLookupPlanQueryPosture,
    EvidenceLookupPlanQueryPostureState, EvidenceLookupPlanQuerySurface,
    EvidenceLookupPlanRowOutcome, EvidenceLookupPlanSelectionCounters,
    EvidenceLookupPlanSelectionError, EvidenceLookupPlanSelectionErrorKind,
    EvidenceLookupPlanTopologyPosture, EvidenceLookupPlanTopologyPostureState,
    EvidenceLookupSelectedPlan, EvidenceLookupSelectedPlanRow, EvidenceLookupSelectedStrategy,
    EvidenceLookupSelectedStrategyKind,
};

#[test]
fn spatial_public_api_exports_selected_lookup_plan_boundary() {
    let _: fn(
        &worth_spatial::facade::evidence_lookup_family_catalog::EvidenceLookupFamilyCatalogCloseout,
        &EvidenceLookupAdmittedInput,
    ) -> Result<EvidenceLookupSelectedPlan, EvidenceLookupPlanSelectionError> =
        select_evidence_lookup_plan;

    let catalog = current_evidence_lookup_family_catalog().expect("catalog closes");
    assert_eq!(catalog.counters().family_count(), 3);
    let _: fn(
        &worth_spatial::facade::evidence_lookup_family_catalog::EvidenceLookupFamilyCatalogCloseout,
        worth_spatial::facade::evidence_lookup_input_admission::EvidenceLookupInputAdmissionRequest<
            '_,
        >,
    ) -> Result<
        EvidenceLookupAdmittedInput,
        worth_spatial::facade::evidence_lookup_input_admission::EvidenceLookupInputAdmissionError,
    > = admit_evidence_lookup_input;
}

#[test]
fn spatial_public_api_exposes_selected_plan_read_contract() {
    let _: fn(&EvidenceLookupSelectedPlan) -> &str =
        EvidenceLookupSelectedPlan::selected_plan_digest;
    let _: fn(&EvidenceLookupSelectedPlan) -> &str = EvidenceLookupSelectedPlan::catalog_digest;
    let _: fn(&EvidenceLookupSelectedPlan) -> &str =
        EvidenceLookupSelectedPlan::admitted_input_digest;
    let _: fn(&EvidenceLookupSelectedPlan) -> &str =
        EvidenceLookupSelectedPlan::spatial_touch_digest;
    let _: fn(&EvidenceLookupSelectedPlan) -> &str =
        EvidenceLookupSelectedPlan::stage_receipt_digest;
    let _: fn(&EvidenceLookupSelectedPlan) -> &[EvidenceLookupSelectedPlanRow] =
        EvidenceLookupSelectedPlan::rows;
    let _: fn(&EvidenceLookupSelectedPlan) -> &EvidenceLookupPlanSelectionCounters =
        EvidenceLookupSelectedPlan::counters;
    let _: fn(&EvidenceLookupSelectedPlan) -> bool =
        EvidenceLookupSelectedPlan::claims_lookup_execution;
    let _: fn(&EvidenceLookupSelectedPlan) -> bool =
        EvidenceLookupSelectedPlan::claims_index_construction;
    let _: fn(&EvidenceLookupSelectedPlan) -> bool =
        EvidenceLookupSelectedPlan::claims_query_descriptor_authority;
}

#[test]
fn spatial_public_api_exposes_selected_row_and_query_posture_contract() {
    let _: fn(&EvidenceLookupSelectedPlanRow) -> &str =
        EvidenceLookupSelectedPlanRow::family_identity;
    let _: fn(&EvidenceLookupSelectedPlanRow) -> &str =
        EvidenceLookupSelectedPlanRow::family_declaration_digest;
    let _: fn(&EvidenceLookupSelectedPlanRow) -> EvidenceLookupPlanRowOutcome =
        EvidenceLookupSelectedPlanRow::outcome;
    let _: fn(&EvidenceLookupSelectedPlanRow) -> Option<&EvidenceLookupSelectedStrategy> =
        EvidenceLookupSelectedPlanRow::strategy;
    let _: fn(&EvidenceLookupSelectedPlanRow) -> &EvidenceLookupPlanQueryPosture =
        EvidenceLookupSelectedPlanRow::query_posture;
    let _: fn(&EvidenceLookupSelectedPlanRow) -> &EvidenceLookupPlanTopologyPosture =
        EvidenceLookupSelectedPlanRow::topology_posture;

    let _: fn(&EvidenceLookupPlanQueryPosture) -> &EvidenceLookupPlanQueryPostureState =
        EvidenceLookupPlanQueryPosture::state;
    let _: fn(&EvidenceLookupPlanQueryPosture) -> EvidenceLookupPlanQuerySurface =
        EvidenceLookupPlanQueryPosture::surface;
    let _: fn(&EvidenceLookupPlanTopologyPosture) -> &EvidenceLookupPlanTopologyPostureState =
        EvidenceLookupPlanTopologyPosture::state;
    let _: fn(&EvidenceLookupSelectedStrategy) -> EvidenceLookupSelectedStrategyKind =
        EvidenceLookupSelectedStrategy::kind;
}

#[test]
fn spatial_public_api_exposes_selection_counters_and_errors() {
    let _: fn(&EvidenceLookupPlanSelectionCounters) -> usize =
        EvidenceLookupPlanSelectionCounters::candidate_family_count;
    let _: fn(&EvidenceLookupPlanSelectionCounters) -> usize =
        EvidenceLookupPlanSelectionCounters::selected_family_count;
    let _: fn(&EvidenceLookupPlanSelectionCounters) -> usize =
        EvidenceLookupPlanSelectionCounters::unaffected_family_count;
    let _: fn(&EvidenceLookupPlanSelectionCounters) -> usize =
        EvidenceLookupPlanSelectionCounters::required_query_posture_row_count;
    let _: fn(&EvidenceLookupPlanSelectionCounters) -> usize =
        EvidenceLookupPlanSelectionCounters::caller_owned_evidence_work_count;
    let _: fn(&EvidenceLookupPlanSelectionCounters) -> usize =
        EvidenceLookupPlanSelectionCounters::topology_support_rows_consumed_count;
    let _: fn(&EvidenceLookupPlanSelectionCounters) -> usize =
        EvidenceLookupPlanSelectionCounters::query_support_rows_consumed_count;
    let _: fn(&EvidenceLookupPlanSelectionCounters) -> usize =
        EvidenceLookupPlanSelectionCounters::selected_family_membership_probe_count;
    let _: fn(&EvidenceLookupPlanSelectionCounters) -> usize =
        EvidenceLookupPlanSelectionCounters::raw_evidence_row_scan_count;
    let _: fn(&EvidenceLookupPlanSelectionCounters) -> usize =
        EvidenceLookupPlanSelectionCounters::broad_receipt_scan_count;
    let _: fn(&EvidenceLookupPlanSelectionError) -> EvidenceLookupPlanSelectionErrorKind =
        EvidenceLookupPlanSelectionError::kind;
    let _: fn(&EvidenceLookupPlanSelectionError) -> &str = EvidenceLookupPlanSelectionError::detail;
    let _: fn(&EvidenceLookupPlanSelectionError) -> &EvidenceLookupPlanSelectionCounters =
        EvidenceLookupPlanSelectionError::counters;
}
