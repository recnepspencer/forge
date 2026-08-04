use super::super::super::digest::resource_canonical_digest;
use super::super::catalog::{
    ResourceMilestoneBPerformanceClaimId, ResourceMilestoneBScenarioId,
    REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS,
};
use super::super::digest_basis::ResourceMilestoneBPerformanceCloseoutDigestBasis;
use super::super::scenario_matrix::ResourceMilestoneBScenarioMatrix;
use super::contract::{
    ResourceMilestoneBPerformanceCloseout, ResourceMilestoneBPerformanceCloseoutRow,
    ResourceMilestoneBPerformanceCloseoutSummary,
    RESOURCE_MILESTONE_B_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION,
};
use crate::data::error::SignalError;
use crate::data::resource::ResourceDiagnosticsExpansionDenial;
use crate::data::resource::ResourceDiagnosticsSummary;
use crate::data::resource::ResourceRuntimeSummaryReadReport;

pub fn resource_milestone_b_performance_closeout(
    scenario_matrix: &ResourceMilestoneBScenarioMatrix,
    summary_read: ResourceRuntimeSummaryReadReport,
    diagnostics_summary: ResourceDiagnosticsSummary,
    diagnostics_denial: ResourceDiagnosticsExpansionDenial,
) -> Result<ResourceMilestoneBPerformanceCloseout, SignalError> {
    if !scenario_matrix.passed() {
        return Err(SignalError::invalid_input(
            "resource milestone B performance closeout requires a passing scenario matrix",
        ));
    }
    let rows = vec![
        ResourceMilestoneBPerformanceCloseoutRow::scenario_family(
            ResourceMilestoneBPerformanceClaimId::LifecycleReplayParityDebtBounded,
            ResourceMilestoneBScenarioId::LifecycleReplayParity,
            scenario_matrix,
        )?,
        ResourceMilestoneBPerformanceCloseoutRow::scenario_family(
            ResourceMilestoneBPerformanceClaimId::OutOfOrderSupersessionAdmissionBounded,
            ResourceMilestoneBScenarioId::OutOfOrderSupersession,
            scenario_matrix,
        )?,
        ResourceMilestoneBPerformanceCloseoutRow::scenario_family(
            ResourceMilestoneBPerformanceClaimId::RollbackObservationRollbackBounded,
            ResourceMilestoneBScenarioId::RollbackObservationEquivalence,
            scenario_matrix,
        )?,
        ResourceMilestoneBPerformanceCloseoutRow::scenario_family(
            ResourceMilestoneBPerformanceClaimId::BranchRestoreReplayRestoreBounded,
            ResourceMilestoneBScenarioId::BranchRestoreReplayEquivalence,
            scenario_matrix,
        )?,
        ResourceMilestoneBPerformanceCloseoutRow::scenario_family(
            ResourceMilestoneBPerformanceClaimId::InflightBoundednessAdmissionBounded,
            ResourceMilestoneBScenarioId::InflightBoundedness,
            scenario_matrix,
        )?,
        ResourceMilestoneBPerformanceCloseoutRow::summary_read(summary_read)?,
        ResourceMilestoneBPerformanceCloseoutRow::diagnostics_summary(&diagnostics_summary)?,
        ResourceMilestoneBPerformanceCloseoutRow::diagnostics_denial(diagnostics_denial)?,
        ResourceMilestoneBPerformanceCloseoutRow::hostile_completion_denials(scenario_matrix)?,
    ];
    let row_ids = rows.iter().map(|row| row.id()).collect::<Vec<_>>();
    if row_ids != REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS {
        return Err(SignalError::invalid_input(
            "resource milestone B performance closeout rows do not match required claims",
        ));
    }
    let summary = ResourceMilestoneBPerformanceCloseoutSummary {
        required_claim_count: REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS.len() as u32,
        certified_claim_count: rows.len() as u32,
        failed_claim_count: rows.iter().filter(|row| !row.passed()).count() as u32,
        scenario_matrix_digest: scenario_matrix.matrix_digest().to_owned(),
    };
    if summary.certified_claim_count != summary.required_claim_count
        || summary.failed_claim_count != 0
    {
        return Err(SignalError::invalid_input(
            "resource milestone B performance closeout did not cover every required claim",
        ));
    }
    let closeout_digest =
        resource_canonical_digest(&ResourceMilestoneBPerformanceCloseoutDigestBasis {
            schema_version: RESOURCE_MILESTONE_B_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION,
            required_claims: &REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS,
            scenario_matrix_digest: scenario_matrix.matrix_digest(),
            summary: &summary,
            rows: &rows,
        });
    Ok(ResourceMilestoneBPerformanceCloseout {
        schema_version: RESOURCE_MILESTONE_B_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION.to_owned(),
        scenario_matrix_digest: scenario_matrix.matrix_digest().to_owned(),
        rows,
        summary,
        closeout_digest,
        passed: true,
    })
}
