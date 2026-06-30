use worth_spatial::facade::evidence_lookup_diagnostics::{
    derive_evidence_lookup_diagnostics, EvidenceLookupDiagnosticAdvisoryReason,
    EvidenceLookupDiagnosticCloseout, EvidenceLookupDiagnosticCounters,
    EvidenceLookupDiagnosticDenialReason, EvidenceLookupDiagnosticQuerySurfaceProvenance,
    EvidenceLookupDiagnosticRow, EvidenceLookupDiagnosticWitness, EvidenceLookupDiagnosticsError,
    EvidenceLookupDiagnosticsErrorKind,
};
use worth_spatial::facade::evidence_lookup_execution::EvidenceLookupExecutionReceipt;
use worth_spatial::facade::evidence_lookup_inventory::EvidenceLookupQuerySurface;
use worth_spatial::facade::evidence_lookup_plan_selection::EvidenceLookupSelectedPlan;
use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceStage;

#[test]
fn spatial_public_api_exports_lookup_diagnostics_boundary() {
    let _: fn(
        &EvidenceLookupSelectedPlan,
        &EvidenceLookupExecutionReceipt,
    ) -> Result<EvidenceLookupDiagnosticCloseout, EvidenceLookupDiagnosticsError> =
        derive_evidence_lookup_diagnostics;
}

#[test]
fn spatial_public_api_exposes_lookup_diagnostics_read_contract() {
    let _: fn(&EvidenceLookupDiagnosticCloseout) -> &[EvidenceLookupDiagnosticRow] =
        EvidenceLookupDiagnosticCloseout::rows;
    let _: fn(&EvidenceLookupDiagnosticCloseout) -> &EvidenceLookupDiagnosticCounters =
        EvidenceLookupDiagnosticCloseout::counters;
    let _: fn(&EvidenceLookupDiagnosticCloseout) -> &str =
        EvidenceLookupDiagnosticCloseout::diagnostic_digest;
    let _ = require_family_stage_witness;

    let _: fn(&EvidenceLookupDiagnosticRow) -> &str = EvidenceLookupDiagnosticRow::family_identity;
    let _: fn(&EvidenceLookupDiagnosticRow) -> &str =
        EvidenceLookupDiagnosticRow::family_declaration_digest;
    let _: fn(&EvidenceLookupDiagnosticRow) -> &str =
        EvidenceLookupDiagnosticRow::selected_plan_digest;
    let _: fn(&EvidenceLookupDiagnosticRow) -> &str =
        EvidenceLookupDiagnosticRow::execution_receipt_digest;
    let _: fn(&EvidenceLookupDiagnosticRow) -> EvidenceLookupDiagnosticWitness =
        EvidenceLookupDiagnosticRow::witness;
    let _: fn(&EvidenceLookupDiagnosticRow) -> EvidenceLookupQuerySurface =
        EvidenceLookupDiagnosticRow::query_surface;
    let _: fn(&EvidenceLookupDiagnosticRow) -> Option<&'static str> =
        EvidenceLookupDiagnosticRow::query_surface_type_name;
    let _: fn(
        &EvidenceLookupDiagnosticRow,
    ) -> Option<EvidenceLookupDiagnosticQuerySurfaceProvenance> =
        EvidenceLookupDiagnosticRow::query_surface_provenance;
    let _: fn(&EvidenceLookupDiagnosticRow) -> Option<&str> =
        EvidenceLookupDiagnosticRow::query_proof_digest;
    let _: fn(&EvidenceLookupDiagnosticRow) -> bool =
        EvidenceLookupDiagnosticRow::claims_lookup_execution_authority;
    let _: fn(&EvidenceLookupDiagnosticRow) -> bool =
        EvidenceLookupDiagnosticRow::claims_query_descriptor_authority;
}

fn require_family_stage_witness<'a>(
    closeout: &'a EvidenceLookupDiagnosticCloseout,
    family_identity: &str,
    stage: WorkloadEvidenceStage,
) -> Result<&'a EvidenceLookupDiagnosticRow, EvidenceLookupDiagnosticsError> {
    closeout.require_family_stage_witness(family_identity, stage)
}

#[test]
fn spatial_public_api_exposes_lookup_diagnostics_support_types() {
    let _: fn(&EvidenceLookupDiagnosticCounters) -> usize =
        EvidenceLookupDiagnosticCounters::row_count;
    let _: fn(&EvidenceLookupDiagnosticCounters) -> usize =
        EvidenceLookupDiagnosticCounters::success_row_count;
    let _: fn(&EvidenceLookupDiagnosticCounters) -> usize =
        EvidenceLookupDiagnosticCounters::advisory_row_count;
    let _: fn(&EvidenceLookupDiagnosticCounters) -> usize =
        EvidenceLookupDiagnosticCounters::denial_row_count;
    let _: fn(&EvidenceLookupDiagnosticCounters) -> usize =
        EvidenceLookupDiagnosticCounters::hidden_lookup_scan_count;
    let _: fn(&EvidenceLookupDiagnosticCounters) -> usize =
        EvidenceLookupDiagnosticCounters::hidden_broad_receipt_scan_count;

    let _: fn(&EvidenceLookupDiagnosticsError) -> EvidenceLookupDiagnosticsErrorKind =
        EvidenceLookupDiagnosticsError::kind;
    let _: fn(&EvidenceLookupDiagnosticsError) -> &str = EvidenceLookupDiagnosticsError::detail;

    let _ = (
        EvidenceLookupDiagnosticWitness::Success,
        EvidenceLookupDiagnosticWitness::Advisory(
            EvidenceLookupDiagnosticAdvisoryReason::UnaffectedFamily,
        ),
        EvidenceLookupDiagnosticWitness::Denied(
            EvidenceLookupDiagnosticDenialReason::RequiredQuerySupport,
        ),
        EvidenceLookupDiagnosticWitness::Denied(
            EvidenceLookupDiagnosticDenialReason::MissingProjectionConsumptionFact,
        ),
    );
}
