use forge_query::facade::runtime::{ForgeQueryReadReceipt, ForgeQueryWriteReceipt};
use forge_query::facade::ProjectionConsumptionReceipt;
use topology::derived_invalidation_selected_plan::{
    DerivedInvalidationDenialKind, DerivedInvalidationDenialRow,
    DerivedInvalidationDensityPolicy, DerivedInvalidationExecutionAdmission,
    DerivedInvalidationLegalitySupportEvidence, DerivedInvalidationPhaseFourSeed,
    DerivedInvalidationPlannedDisposition, DerivedInvalidationQuerySupportEvidence,
    DerivedInvalidationResidueRow, DerivedInvalidationSelectedPlan,
    DerivedInvalidationSelectedRow, DerivedInvalidationSelectionCounters,
    DerivedInvalidationTouchedClosure, DerivedInvalidationUnaffectedRow,
};
use topology::facade::{
    WorthTopologySelectedLegalityObligationPlan, WorthTopologySelectedValidatorEnforcementReceipt,
};

fn _derived_invalidation_selected_plan_contract() {
    let query_support = DerivedInvalidationQuerySupportEvidence::missing();
    let legality_support = DerivedInvalidationLegalitySupportEvidence::missing();
    assert!(query_support.support_digest().contains("derived-invalidation"));
    assert!(legality_support.support_digest().contains("derived-invalidation"));

    let _: fn(&ProjectionConsumptionReceipt) -> DerivedInvalidationQuerySupportEvidence =
        DerivedInvalidationQuerySupportEvidence::from_projection_consumption_receipt;
    let _: fn(&ForgeQueryReadReceipt) -> DerivedInvalidationQuerySupportEvidence =
        DerivedInvalidationQuerySupportEvidence::from_native_read_receipt;
    let _: fn(&ForgeQueryWriteReceipt) -> DerivedInvalidationQuerySupportEvidence =
        DerivedInvalidationQuerySupportEvidence::from_native_write_receipt;
    let _: fn(
        Option<&ProjectionConsumptionReceipt>,
        Option<&ForgeQueryReadReceipt>,
        Option<&ForgeQueryWriteReceipt>,
    ) -> DerivedInvalidationQuerySupportEvidence =
        DerivedInvalidationQuerySupportEvidence::from_query_receipts;
    let _: fn(&WorthTopologySelectedLegalityObligationPlan) -> DerivedInvalidationLegalitySupportEvidence =
        DerivedInvalidationLegalitySupportEvidence::from_selected_legality_plan;
    let _: fn(&WorthTopologySelectedValidatorEnforcementReceipt) -> DerivedInvalidationLegalitySupportEvidence =
        DerivedInvalidationLegalitySupportEvidence::from_selected_validator_receipt;

    let _: fn(&DerivedInvalidationSelectedPlan) -> &str =
        DerivedInvalidationSelectedPlan::selected_plan_digest;
    let _: fn(&DerivedInvalidationSelectedPlan) -> &[DerivedInvalidationSelectedRow] =
        DerivedInvalidationSelectedPlan::selected_rows;
    let _: fn(&DerivedInvalidationSelectedPlan) -> &[DerivedInvalidationUnaffectedRow] =
        DerivedInvalidationSelectedPlan::unaffected_rows;
    let _: fn(&DerivedInvalidationSelectedPlan) -> &[DerivedInvalidationDenialRow] =
        DerivedInvalidationSelectedPlan::denied_rows;
    let _: fn(&DerivedInvalidationSelectedPlan) -> &[DerivedInvalidationResidueRow] =
        DerivedInvalidationSelectedPlan::residue_rows;
    let _: fn(&DerivedInvalidationSelectedPlan) -> &DerivedInvalidationSelectionCounters =
        DerivedInvalidationSelectedPlan::counters;
    let _: fn(&DerivedInvalidationSelectedPlan) -> DerivedInvalidationExecutionAdmission =
        DerivedInvalidationSelectedPlan::execution_admission;
    let _: fn(&DerivedInvalidationSelectedPlan) -> &DerivedInvalidationPhaseFourSeed =
        DerivedInvalidationSelectedPlan::phase_four_seed;
    let _: fn(&DerivedInvalidationTouchedClosure) -> &str =
        DerivedInvalidationTouchedClosure::closure_digest;
    let _: fn(&DerivedInvalidationSelectionCounters) -> usize =
        DerivedInvalidationSelectionCounters::touched_entity_count;
    let _: fn(&DerivedInvalidationSelectionCounters) -> usize =
        DerivedInvalidationSelectionCounters::touched_relation_count;
    let _: fn(&DerivedInvalidationSelectionCounters) -> usize =
        DerivedInvalidationSelectionCounters::touched_relation_kind_count;
    let _: fn(&DerivedInvalidationSelectionCounters) -> usize =
        DerivedInvalidationSelectionCounters::touched_aspect_count;
    let _: fn(&DerivedInvalidationSelectionCounters) -> usize =
        DerivedInvalidationSelectionCounters::touched_scope_count;
    let _: DerivedInvalidationDensityPolicy = DerivedInvalidationDensityPolicy::Sparse;
    let _: DerivedInvalidationPlannedDisposition =
        DerivedInvalidationPlannedDisposition::IncrementalUpdate;
    let _: DerivedInvalidationDenialKind = DerivedInvalidationDenialKind::MissingQuerySupport;
}
