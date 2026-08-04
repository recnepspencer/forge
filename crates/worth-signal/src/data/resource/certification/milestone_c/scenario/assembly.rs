use super::super::super::digest::resource_canonical_digest;
use super::super::catalog::{
    ResourceMilestoneCPolicyScenarioId, REQUIRED_RESOURCE_MILESTONE_C_POLICY_SCENARIOS,
};
use super::super::digest_basis::ResourceMilestoneCPolicyScenarioMatrixDigestBasis;
use super::super::family::ResourceMilestoneCPolicyCertificationBundle;
use super::contract::{
    ResourceMilestoneCPolicyScenarioMatrix, ResourceMilestoneCPolicyScenarioMatrixSummary,
    ResourceMilestoneCPolicyScenarioRow,
    RESOURCE_MILESTONE_C_POLICY_SCENARIO_MATRIX_SCHEMA_VERSION,
};
use crate::data::error::SignalError;
use crate::data::resource::DeniedResourcePolicyRestoreCompatibility;
use crate::data::resource::ResourceDiagnosticsExpansionDenial;
use crate::data::resource::ResourceLifecycleRetentionCompactionReport;
use crate::data::resource::ResourcePolicyRegistryFreezeReport;
use crate::data::resource::ResourcePolicyRestoreCompatibilityProof;
use crate::data::resource::ResourceRetryScheduleReport;
use crate::data::resource::ResourceTimeoutHeartbeatExtensionReport;
use std::collections::BTreeMap;

pub fn resource_milestone_c_policy_scenario_matrix(
    bundle: &ResourceMilestoneCPolicyCertificationBundle,
    freeze_report: &ResourcePolicyRegistryFreezeReport,
    retry_schedule_report: &ResourceRetryScheduleReport,
    timeout_heartbeat_report: &ResourceTimeoutHeartbeatExtensionReport,
    retention_compaction_report: &ResourceLifecycleRetentionCompactionReport,
    diagnostics_denial: &ResourceDiagnosticsExpansionDenial,
    compatible_restore: &ResourcePolicyRestoreCompatibilityProof,
    incompatible_restore: &DeniedResourcePolicyRestoreCompatibility,
    missing_restore: &DeniedResourcePolicyRestoreCompatibility,
) -> Result<ResourceMilestoneCPolicyScenarioMatrix, SignalError> {
    bundle.ensure_passed()?;

    let mut rows = vec![
        ResourceMilestoneCPolicyScenarioRow::from_registry_freeze(freeze_report)?,
        ResourceMilestoneCPolicyScenarioRow::from_retry_denial(retry_schedule_report)?,
        ResourceMilestoneCPolicyScenarioRow::from_timeout_heartbeat_denial(
            timeout_heartbeat_report,
        )?,
        ResourceMilestoneCPolicyScenarioRow::from_retention_compaction(
            retention_compaction_report,
        )?,
        ResourceMilestoneCPolicyScenarioRow::from_diagnostics_denial(diagnostics_denial)?,
        ResourceMilestoneCPolicyScenarioRow::from_restore_proof(compatible_restore)?,
        ResourceMilestoneCPolicyScenarioRow::from_restore_denial(
            ResourceMilestoneCPolicyScenarioId::IncompatibleDescriptorRestoreDenied,
            incompatible_restore,
        )?,
        ResourceMilestoneCPolicyScenarioRow::from_restore_denial(
            ResourceMilestoneCPolicyScenarioId::MissingDescriptorRestoreDenied,
            missing_restore,
        )?,
    ];
    rows.sort_by_key(|row| row.id);

    let mut row_counts: BTreeMap<ResourceMilestoneCPolicyScenarioId, u32> = BTreeMap::new();
    for row in &rows {
        *row_counts.entry(row.id()).or_default() += 1;
    }
    for scenario in REQUIRED_RESOURCE_MILESTONE_C_POLICY_SCENARIOS {
        let count = row_counts.get(&scenario).copied().unwrap_or(0);
        if count != 1 {
            return Err(SignalError::invalid_input(format!(
                "resource milestone C policy scenario matrix requires exactly one row for {}",
                scenario.label()
            )));
        }
    }

    let summary = ResourceMilestoneCPolicyScenarioMatrixSummary {
        required_scenario_count: REQUIRED_RESOURCE_MILESTONE_C_POLICY_SCENARIOS.len() as u32,
        certified_scenario_count: rows.iter().filter(|row| row.passed()).count() as u32,
        failed_scenario_count: rows.iter().filter(|row| !row.passed()).count() as u32,
        bundle_digest: bundle.bundle_digest().to_owned(),
    };
    let matrix_digest =
        resource_canonical_digest(&ResourceMilestoneCPolicyScenarioMatrixDigestBasis {
            schema_version: RESOURCE_MILESTONE_C_POLICY_SCENARIO_MATRIX_SCHEMA_VERSION,
            required_scenarios: &REQUIRED_RESOURCE_MILESTONE_C_POLICY_SCENARIOS,
            bundle_digest: bundle.bundle_digest(),
            summary: &summary,
            rows: &rows,
        });
    Ok(ResourceMilestoneCPolicyScenarioMatrix {
        schema_version: RESOURCE_MILESTONE_C_POLICY_SCENARIO_MATRIX_SCHEMA_VERSION.to_owned(),
        rows,
        summary,
        matrix_digest,
        passed: true,
    })
}
