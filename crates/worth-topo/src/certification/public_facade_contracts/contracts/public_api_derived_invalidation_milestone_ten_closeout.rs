use topology::derived_invalidation_deletion_closeout::DerivedInvalidationDeletionCloseout as MilestoneTenContractDeletionCloseout;
use topology::derived_invalidation_execution::DerivedInvalidationExecutionReceipt as MilestoneTenContractExecutionReceipt;
use topology::derived_invalidation_family_catalog::{
    DerivedInvalidationFamilyCatalogCloseout as MilestoneTenContractCatalogCloseout,
    DerivedTopologyProductFamilyIdentity as MilestoneTenContractProductFamilyIdentity,
};
use topology::derived_invalidation_migrated_products::{
    CoveredDerivedProductMigrationStatus as MilestoneTenContractMigrationStatus,
    CoveredDerivedProductMigrationSweepCloseout as MilestoneTenContractMigrationSweep,
};
use topology::derived_invalidation_milestone_ten_closeout::{
    close_derived_invalidation_milestone_ten, DerivedInvalidationMilestoneElevenLookupReadiness,
    DerivedInvalidationMilestoneElevenProductReceiptRef, DerivedInvalidationMilestoneElevenSeed,
    DerivedInvalidationMilestoneTenCloseout, DerivedInvalidationMilestoneTenCounters,
    DerivedInvalidationMilestoneTenError, DerivedInvalidationMilestoneTenErrorKind,
    DerivedInvalidationMilestoneTenPerformanceProof, DerivedInvalidationMilestoneTenPerformanceSlopeCase,
    DerivedInvalidationMilestoneTenProductSummaryReport,
    DerivedInvalidationMilestoneTenProductSummaryRow,
};
use topology::derived_invalidation_operator_cutover::DerivedInvalidationOperatorCutoverCloseout as MilestoneTenContractOperatorCutover;
use topology::derived_invalidation_selected_plan::DerivedInvalidationSelectedPlan as MilestoneTenContractSelectedPlan;

fn _derived_invalidation_milestone_ten_closeout_contract() {
    let _: fn(
        &MilestoneTenContractCatalogCloseout,
        &MilestoneTenContractSelectedPlan,
        &MilestoneTenContractExecutionReceipt,
        &MilestoneTenContractMigrationSweep,
        &MilestoneTenContractOperatorCutover,
        &MilestoneTenContractDeletionCloseout,
    ) -> Result<DerivedInvalidationMilestoneTenCloseout, DerivedInvalidationMilestoneTenError> =
        close_derived_invalidation_milestone_ten;

    let _: fn(&DerivedInvalidationMilestoneTenError) -> DerivedInvalidationMilestoneTenErrorKind =
        DerivedInvalidationMilestoneTenError::kind;
    let _: fn(&DerivedInvalidationMilestoneTenError) -> &str =
        DerivedInvalidationMilestoneTenError::reason;

    let _: fn(&DerivedInvalidationMilestoneTenCloseout) -> &str =
        DerivedInvalidationMilestoneTenCloseout::catalog_digest;
    let _: fn(&DerivedInvalidationMilestoneTenCloseout) -> &str =
        DerivedInvalidationMilestoneTenCloseout::selected_plan_digest;
    let _: fn(&DerivedInvalidationMilestoneTenCloseout) -> &str =
        DerivedInvalidationMilestoneTenCloseout::execution_receipt_digest;
    let _: fn(&DerivedInvalidationMilestoneTenCloseout) -> &str =
        DerivedInvalidationMilestoneTenCloseout::migration_sweep_digest;
    let _: fn(&DerivedInvalidationMilestoneTenCloseout) -> &str =
        DerivedInvalidationMilestoneTenCloseout::operator_cutover_digest;
    let _: fn(&DerivedInvalidationMilestoneTenCloseout) -> &str =
        DerivedInvalidationMilestoneTenCloseout::deletion_closeout_digest;
    let _: fn(
        &DerivedInvalidationMilestoneTenCloseout,
    ) -> &DerivedInvalidationMilestoneTenProductSummaryReport =
        DerivedInvalidationMilestoneTenCloseout::product_summary;
    let _: fn(
        &DerivedInvalidationMilestoneTenCloseout,
    ) -> &DerivedInvalidationMilestoneTenPerformanceProof =
        DerivedInvalidationMilestoneTenCloseout::performance_proof;
    let _: fn(&DerivedInvalidationMilestoneTenCloseout) -> &DerivedInvalidationMilestoneTenCounters =
        DerivedInvalidationMilestoneTenCloseout::counters;
    let _: fn(
        &DerivedInvalidationMilestoneTenCloseout,
    ) -> &DerivedInvalidationMilestoneElevenSeed =
        DerivedInvalidationMilestoneTenCloseout::milestone_eleven_seed;
    let _: fn(&DerivedInvalidationMilestoneTenCloseout) -> &str =
        DerivedInvalidationMilestoneTenCloseout::closeout_digest;

    let _: fn(&DerivedInvalidationMilestoneTenProductSummaryReport) -> &str =
        DerivedInvalidationMilestoneTenProductSummaryReport::selected_plan_digest;
    let _: fn(&DerivedInvalidationMilestoneTenProductSummaryReport) -> &str =
        DerivedInvalidationMilestoneTenProductSummaryReport::touched_closure_digest;
    let _: fn(&DerivedInvalidationMilestoneTenProductSummaryReport) -> &str =
        DerivedInvalidationMilestoneTenProductSummaryReport::execution_receipt_digest;
    let _: fn(&DerivedInvalidationMilestoneTenProductSummaryReport) -> &str =
        DerivedInvalidationMilestoneTenProductSummaryReport::migration_sweep_digest;
    let _: fn(
        &DerivedInvalidationMilestoneTenProductSummaryReport,
    ) -> &[DerivedInvalidationMilestoneTenProductSummaryRow] =
        DerivedInvalidationMilestoneTenProductSummaryReport::rows;
    let _: fn(&DerivedInvalidationMilestoneTenProductSummaryReport) -> &str =
        DerivedInvalidationMilestoneTenProductSummaryReport::report_digest;

    let _: fn(
        &DerivedInvalidationMilestoneTenProductSummaryRow,
    ) -> MilestoneTenContractProductFamilyIdentity =
        DerivedInvalidationMilestoneTenProductSummaryRow::family_identity;
    let _: fn(
        &DerivedInvalidationMilestoneTenProductSummaryRow,
    ) -> MilestoneTenContractMigrationStatus =
        DerivedInvalidationMilestoneTenProductSummaryRow::migration_status;
    let _: fn(&DerivedInvalidationMilestoneTenProductSummaryRow) -> bool =
        DerivedInvalidationMilestoneTenProductSummaryRow::ordinary_invalidation_consumable;
    let _: fn(&DerivedInvalidationMilestoneTenProductSummaryRow) -> &str =
        DerivedInvalidationMilestoneTenProductSummaryRow::migration_proof_digest;
    let _: fn(&DerivedInvalidationMilestoneTenProductSummaryRow) -> Option<&str> =
        DerivedInvalidationMilestoneTenProductSummaryRow::selected_row_digest;
    let _: fn(
        &DerivedInvalidationMilestoneTenProductSummaryRow,
    ) -> topology::derived_invalidation_family_catalog::DerivedTopologyInvalidationPredicate =
        DerivedInvalidationMilestoneTenProductSummaryRow::invalidation_predicate;
    let _: fn(&DerivedInvalidationMilestoneTenProductSummaryRow) -> &str =
        DerivedInvalidationMilestoneTenProductSummaryRow::consumed_graph_facts_digest;
    let _: fn(&DerivedInvalidationMilestoneTenProductSummaryRow) -> usize =
        DerivedInvalidationMilestoneTenProductSummaryRow::consumed_relation_kind_count;
    let _: fn(&DerivedInvalidationMilestoneTenProductSummaryRow) -> usize =
        DerivedInvalidationMilestoneTenProductSummaryRow::consumed_aspect_count;
    let _: fn(&DerivedInvalidationMilestoneTenProductSummaryRow) -> bool =
        DerivedInvalidationMilestoneTenProductSummaryRow::selected_by_touched_closure;
    let _: fn(&DerivedInvalidationMilestoneTenProductSummaryRow) -> usize =
        DerivedInvalidationMilestoneTenProductSummaryRow::executed_count;
    let _: fn(&DerivedInvalidationMilestoneTenProductSummaryRow) -> usize =
        DerivedInvalidationMilestoneTenProductSummaryRow::unaffected_count;
    let _: fn(&DerivedInvalidationMilestoneTenProductSummaryRow) -> usize =
        DerivedInvalidationMilestoneTenProductSummaryRow::denied_count;
    let _: fn(&DerivedInvalidationMilestoneTenProductSummaryRow) -> usize =
        DerivedInvalidationMilestoneTenProductSummaryRow::query_receipt_bound_count;
    let _: fn(&DerivedInvalidationMilestoneTenProductSummaryRow) -> usize =
        DerivedInvalidationMilestoneTenProductSummaryRow::legality_receipt_bound_count;
    let _: fn(&DerivedInvalidationMilestoneTenProductSummaryRow) -> usize =
        DerivedInvalidationMilestoneTenProductSummaryRow::product_output_bound_count;
    let _: fn(&DerivedInvalidationMilestoneTenProductSummaryRow) -> usize =
        DerivedInvalidationMilestoneTenProductSummaryRow::execution_work_count;
    let _: fn(&DerivedInvalidationMilestoneTenProductSummaryRow) -> usize =
        DerivedInvalidationMilestoneTenProductSummaryRow::whole_view_fallback_count;
    let _: fn(&DerivedInvalidationMilestoneTenProductSummaryRow) -> usize =
        DerivedInvalidationMilestoneTenProductSummaryRow::caller_owned_graph_work_count;
    let _: fn(&DerivedInvalidationMilestoneTenProductSummaryRow) -> &str =
        DerivedInvalidationMilestoneTenProductSummaryRow::row_digest;

    let _: fn(&DerivedInvalidationMilestoneTenPerformanceProof) -> &str =
        DerivedInvalidationMilestoneTenPerformanceProof::selected_plan_digest;
    let _: fn(&DerivedInvalidationMilestoneTenPerformanceProof) -> &str =
        DerivedInvalidationMilestoneTenPerformanceProof::execution_receipt_digest;
    let _: fn(&DerivedInvalidationMilestoneTenPerformanceProof) -> &str =
        DerivedInvalidationMilestoneTenPerformanceProof::deletion_closeout_digest;
    let _: fn(
        &DerivedInvalidationMilestoneTenPerformanceProof,
    ) -> &[DerivedInvalidationMilestoneTenPerformanceSlopeCase] =
        DerivedInvalidationMilestoneTenPerformanceProof::slope_cases;
    let _: fn(&DerivedInvalidationMilestoneTenPerformanceProof) -> &str =
        DerivedInvalidationMilestoneTenPerformanceProof::proof_digest;

    let _: fn(&DerivedInvalidationMilestoneTenPerformanceSlopeCase) -> &str =
        DerivedInvalidationMilestoneTenPerformanceSlopeCase::label;
    let _: fn(&DerivedInvalidationMilestoneTenPerformanceSlopeCase) -> usize =
        DerivedInvalidationMilestoneTenPerformanceSlopeCase::touched_or_declared_bound;
    let _: fn(&DerivedInvalidationMilestoneTenPerformanceSlopeCase) -> usize =
        DerivedInvalidationMilestoneTenPerformanceSlopeCase::observed_work_count;
    let _: fn(&DerivedInvalidationMilestoneTenPerformanceSlopeCase) -> usize =
        DerivedInvalidationMilestoneTenPerformanceSlopeCase::forbidden_global_work_count;
    let _: fn(&DerivedInvalidationMilestoneTenPerformanceSlopeCase) -> usize =
        DerivedInvalidationMilestoneTenPerformanceSlopeCase::allowed_global_work_count;
    let _: fn(&DerivedInvalidationMilestoneTenPerformanceSlopeCase) -> &str =
        DerivedInvalidationMilestoneTenPerformanceSlopeCase::row_digest;

    let _: fn(&DerivedInvalidationMilestoneTenCounters) -> usize =
        DerivedInvalidationMilestoneTenCounters::required_family_count;
    let _: fn(&DerivedInvalidationMilestoneTenCounters) -> usize =
        DerivedInvalidationMilestoneTenCounters::summary_row_count;
    let _: fn(&DerivedInvalidationMilestoneTenCounters) -> usize =
        DerivedInvalidationMilestoneTenCounters::executed_product_count;
    let _: fn(&DerivedInvalidationMilestoneTenCounters) -> usize =
        DerivedInvalidationMilestoneTenCounters::ordinary_consumable_family_count;
    let _: fn(&DerivedInvalidationMilestoneTenCounters) -> usize =
        DerivedInvalidationMilestoneTenCounters::source_firewall_violation_count;
    let _: fn(&DerivedInvalidationMilestoneTenCounters) -> usize =
        DerivedInvalidationMilestoneTenCounters::old_authority_residue_count;
    let _: fn(&DerivedInvalidationMilestoneTenCounters) -> usize =
        DerivedInvalidationMilestoneTenCounters::whole_view_fallback_count;
    let _: fn(&DerivedInvalidationMilestoneTenCounters) -> usize =
        DerivedInvalidationMilestoneTenCounters::caller_owned_graph_work_count;
    let _: fn(&DerivedInvalidationMilestoneTenCounters) -> usize =
        DerivedInvalidationMilestoneTenCounters::slope_case_count;
    let _: fn(&DerivedInvalidationMilestoneTenCounters) -> &str =
        DerivedInvalidationMilestoneTenCounters::counters_digest;

    let _: fn(&DerivedInvalidationMilestoneElevenSeed) -> &str =
        DerivedInvalidationMilestoneElevenSeed::milestone_ten_closeout_digest;
    let _: fn(&DerivedInvalidationMilestoneElevenSeed) -> &str =
        DerivedInvalidationMilestoneElevenSeed::selected_plan_digest;
    let _: fn(&DerivedInvalidationMilestoneElevenSeed) -> &str =
        DerivedInvalidationMilestoneElevenSeed::execution_receipt_digest;
    let _: fn(&DerivedInvalidationMilestoneElevenSeed) -> &str =
        DerivedInvalidationMilestoneElevenSeed::touched_closure_digest;
    let _: fn(&DerivedInvalidationMilestoneElevenSeed) -> &str =
        DerivedInvalidationMilestoneElevenSeed::query_support_digest;
    let _: fn(&DerivedInvalidationMilestoneElevenSeed) -> &str =
        DerivedInvalidationMilestoneElevenSeed::legality_support_digest;
    let _: fn(&DerivedInvalidationMilestoneElevenSeed) -> &str =
        DerivedInvalidationMilestoneElevenSeed::product_summary_digest;
    let _: fn(&DerivedInvalidationMilestoneElevenSeed) -> &str =
        DerivedInvalidationMilestoneElevenSeed::performance_proof_digest;
    let _: fn(&DerivedInvalidationMilestoneElevenSeed) -> &str =
        DerivedInvalidationMilestoneElevenSeed::deletion_audit_digest;
    let _: fn(&DerivedInvalidationMilestoneElevenSeed) -> &str =
        DerivedInvalidationMilestoneElevenSeed::counters_digest;
    let _: fn(
        &DerivedInvalidationMilestoneElevenSeed,
    ) -> DerivedInvalidationMilestoneElevenLookupReadiness =
        DerivedInvalidationMilestoneElevenSeed::lookup_readiness;
    let _: fn(
        &DerivedInvalidationMilestoneElevenSeed,
    ) -> &[DerivedInvalidationMilestoneElevenProductReceiptRef] =
        DerivedInvalidationMilestoneElevenSeed::topology_derived_product_receipts;
    let _: fn(&DerivedInvalidationMilestoneElevenSeed) -> &str =
        DerivedInvalidationMilestoneElevenSeed::seed_digest;
    let _: fn(DerivedInvalidationMilestoneElevenLookupReadiness) -> &'static str =
        DerivedInvalidationMilestoneElevenLookupReadiness::as_str;

    let _: fn(
        &DerivedInvalidationMilestoneElevenProductReceiptRef,
    ) -> MilestoneTenContractProductFamilyIdentity =
        DerivedInvalidationMilestoneElevenProductReceiptRef::family_identity;
    let _: fn(&DerivedInvalidationMilestoneElevenProductReceiptRef) -> &str =
        DerivedInvalidationMilestoneElevenProductReceiptRef::execution_row_digest;
    let _: fn(&DerivedInvalidationMilestoneElevenProductReceiptRef) -> Option<&str> =
        DerivedInvalidationMilestoneElevenProductReceiptRef::product_output_digest;
    let _: fn(&DerivedInvalidationMilestoneElevenProductReceiptRef) -> Option<&str> =
        DerivedInvalidationMilestoneElevenProductReceiptRef::query_receipt_digest;
    let _: fn(&DerivedInvalidationMilestoneElevenProductReceiptRef) -> Option<&str> =
        DerivedInvalidationMilestoneElevenProductReceiptRef::legality_receipt_digest;
    let _: fn(&DerivedInvalidationMilestoneElevenProductReceiptRef) -> &str =
        DerivedInvalidationMilestoneElevenProductReceiptRef::ref_digest;
}
