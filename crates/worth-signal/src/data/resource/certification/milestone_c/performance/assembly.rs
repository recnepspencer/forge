use super::super::super::digest::resource_canonical_digest;
use super::super::catalog::{
    ResourceMilestoneCPolicyPerformanceClaimId, ResourceMilestoneCPolicyScenarioId,
    REQUIRED_RESOURCE_MILESTONE_C_POLICY_PERFORMANCE_CLAIMS,
};
use super::super::digest_basis::ResourceMilestoneCPolicyPerformanceCloseoutDigestBasis;
use super::super::scenario::ResourceMilestoneCPolicyScenarioMatrix;
use super::contract::{
    ResourceMilestoneCPolicyPerformanceCloseout, ResourceMilestoneCPolicyPerformanceCloseoutRow,
    ResourceMilestoneCPolicyPerformanceCloseoutSummary,
    RESOURCE_MILESTONE_C_POLICY_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION,
};
use crate::data::error::SignalError;

pub fn resource_milestone_c_policy_performance_closeout(
    scenario_matrix: &ResourceMilestoneCPolicyScenarioMatrix,
) -> Result<ResourceMilestoneCPolicyPerformanceCloseout, SignalError> {
    if !scenario_matrix.passed() {
        return Err(SignalError::invalid_input(
            "resource milestone C policy performance closeout requires a passing scenario matrix",
        ));
    }
    let rows = vec![
        ResourceMilestoneCPolicyPerformanceCloseoutRow::scenario_row(
            ResourceMilestoneCPolicyPerformanceClaimId::RegistryFreezeOrderBounded,
            ResourceMilestoneCPolicyScenarioId::RegistryOrderCanonicalization,
            scenario_matrix,
        )?,
        ResourceMilestoneCPolicyPerformanceCloseoutRow::scenario_row(
            ResourceMilestoneCPolicyPerformanceClaimId::RetryBudgetDenialZeroWake,
            ResourceMilestoneCPolicyScenarioId::RetryBudgetExhaustionRejected,
            scenario_matrix,
        )?,
        ResourceMilestoneCPolicyPerformanceCloseoutRow::scenario_row(
            ResourceMilestoneCPolicyPerformanceClaimId::RetentionCompactionAvailabilityBounded,
            ResourceMilestoneCPolicyScenarioId::RetentionCompactionReportsUnavailableHistory,
            scenario_matrix,
        )?,
        ResourceMilestoneCPolicyPerformanceCloseoutRow::scenario_row(
            ResourceMilestoneCPolicyPerformanceClaimId::DiagnosticsBudgetDenialZeroCold,
            ResourceMilestoneCPolicyScenarioId::DiagnosticsExpansionBudgetDeniedZeroCold,
            scenario_matrix,
        )?,
        ResourceMilestoneCPolicyPerformanceCloseoutRow::replay_descriptor_bound(scenario_matrix)?,
    ];
    let row_ids = rows.iter().map(|row| row.id()).collect::<Vec<_>>();
    if row_ids != REQUIRED_RESOURCE_MILESTONE_C_POLICY_PERFORMANCE_CLAIMS {
        return Err(SignalError::invalid_input(
            "resource milestone C policy performance closeout rows do not match required claims",
        ));
    }
    let summary = ResourceMilestoneCPolicyPerformanceCloseoutSummary {
        required_claim_count: REQUIRED_RESOURCE_MILESTONE_C_POLICY_PERFORMANCE_CLAIMS.len() as u32,
        certified_claim_count: rows.len() as u32,
        failed_claim_count: rows.iter().filter(|row| !row.passed()).count() as u32,
        scenario_matrix_digest: scenario_matrix.matrix_digest().to_owned(),
    };
    if summary.certified_claim_count != summary.required_claim_count
        || summary.failed_claim_count != 0
    {
        return Err(SignalError::invalid_input(
            "resource milestone C policy performance closeout did not cover every required claim",
        ));
    }
    let closeout_digest =
        resource_canonical_digest(&ResourceMilestoneCPolicyPerformanceCloseoutDigestBasis {
            schema_version: RESOURCE_MILESTONE_C_POLICY_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION,
            required_claims: &REQUIRED_RESOURCE_MILESTONE_C_POLICY_PERFORMANCE_CLAIMS,
            scenario_matrix_digest: scenario_matrix.matrix_digest(),
            summary: &summary,
            rows: &rows,
        });
    Ok(ResourceMilestoneCPolicyPerformanceCloseout {
        schema_version: RESOURCE_MILESTONE_C_POLICY_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION.to_owned(),
        scenario_matrix_digest: scenario_matrix.matrix_digest().to_owned(),
        rows,
        summary,
        closeout_digest,
        passed: true,
    })
}
