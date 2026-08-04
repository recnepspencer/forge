use super::super::digest::resource_canonical_digest;
use super::super::family::{ResourceCertificationBundle, REQUIRED_RESOURCE_CERTIFICATION_FAMILIES};
use super::catalog::{
    REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS, REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS,
};
use super::digest_basis::ResourceMilestoneBCertificationRunDigestBasis;
use super::performance::ResourceMilestoneBPerformanceCloseout;
use super::scenario_matrix::ResourceMilestoneBScenarioMatrix;
use crate::data::error::SignalError;
use serde::Serialize;

pub const RESOURCE_MILESTONE_B_CERTIFICATION_RUN_SCHEMA_VERSION: &str =
    "worth-signal-resource-milestone-b-certification-run-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneBCertificationRunSummary {
    required_family_count: u32,
    certified_family_count: u32,
    failed_family_count: u32,
    bundle_digest: String,
    required_scenario_count: u32,
    certified_scenario_count: u32,
    scenario_matrix_digest: String,
    required_performance_claim_count: u32,
    certified_performance_claim_count: u32,
    performance_closeout_digest: String,
}

impl ResourceMilestoneBCertificationRunSummary {
    pub fn required_family_count(&self) -> u32 {
        self.required_family_count
    }

    pub fn certified_family_count(&self) -> u32 {
        self.certified_family_count
    }

    pub fn failed_family_count(&self) -> u32 {
        self.failed_family_count
    }

    pub fn bundle_digest(&self) -> &str {
        &self.bundle_digest
    }

    pub fn required_scenario_count(&self) -> u32 {
        self.required_scenario_count
    }

    pub fn certified_scenario_count(&self) -> u32 {
        self.certified_scenario_count
    }

    pub fn scenario_matrix_digest(&self) -> &str {
        &self.scenario_matrix_digest
    }

    pub fn required_performance_claim_count(&self) -> u32 {
        self.required_performance_claim_count
    }

    pub fn certified_performance_claim_count(&self) -> u32 {
        self.certified_performance_claim_count
    }

    pub fn performance_closeout_digest(&self) -> &str {
        &self.performance_closeout_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneBCertificationRun {
    schema_version: String,
    bundle: ResourceCertificationBundle,
    scenario_matrix: ResourceMilestoneBScenarioMatrix,
    performance_closeout: ResourceMilestoneBPerformanceCloseout,
    summary: ResourceMilestoneBCertificationRunSummary,
    run_digest: String,
    passed: bool,
}

impl ResourceMilestoneBCertificationRun {
    pub fn schema_version(&self) -> &str {
        &self.schema_version
    }

    pub fn bundle(&self) -> &ResourceCertificationBundle {
        &self.bundle
    }

    pub fn scenario_matrix(&self) -> &ResourceMilestoneBScenarioMatrix {
        &self.scenario_matrix
    }

    pub fn performance_closeout(&self) -> &ResourceMilestoneBPerformanceCloseout {
        &self.performance_closeout
    }

    pub fn summary(&self) -> &ResourceMilestoneBCertificationRunSummary {
        &self.summary
    }

    pub fn run_digest(&self) -> &str {
        &self.run_digest
    }

    pub fn passed(&self) -> bool {
        self.passed
    }
}

pub fn resource_milestone_b_certification_run(
    bundle: ResourceCertificationBundle,
    scenario_matrix: ResourceMilestoneBScenarioMatrix,
    performance_closeout: ResourceMilestoneBPerformanceCloseout,
) -> Result<ResourceMilestoneBCertificationRun, SignalError> {
    bundle.ensure_passed()?;
    if !scenario_matrix.passed() {
        return Err(SignalError::invalid_input(
            "resource milestone B certification run requires a passing scenario matrix",
        ));
    }
    if !performance_closeout.passed() {
        return Err(SignalError::invalid_input(
            "resource milestone B certification run requires a passing performance closeout",
        ));
    }
    if scenario_matrix.bundle_digest() != bundle.bundle_digest() {
        return Err(SignalError::invalid_input(
            "resource milestone B certification run requires scenario matrix evidence from the same bundle",
        ));
    }
    if performance_closeout.scenario_matrix_digest() != scenario_matrix.matrix_digest() {
        return Err(SignalError::invalid_input(
            "resource milestone B certification run requires performance closeout evidence from the same scenario matrix",
        ));
    }
    if scenario_matrix.rows().len() != REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS.len()
        || scenario_matrix.summary().required_scenario_count()
            != REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS.len() as u32
        || scenario_matrix.summary().certified_scenario_count()
            != REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS.len() as u32
        || scenario_matrix.summary().failed_scenario_count() != 0
    {
        return Err(SignalError::invalid_input(
            "resource milestone B certification run requires one passing row for every required scenario",
        ));
    }
    if performance_closeout.rows().len() != REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS.len()
        || performance_closeout.summary().required_claim_count()
            != REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS.len() as u32
        || performance_closeout.summary().certified_claim_count()
            != REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS.len() as u32
        || performance_closeout.summary().failed_claim_count() != 0
    {
        return Err(SignalError::invalid_input(
            "resource milestone B certification run requires one passing row for every required performance claim",
        ));
    }
    let bundle_summary = bundle.summary();
    let required_family_count = REQUIRED_RESOURCE_CERTIFICATION_FAMILIES.len() as u32;
    if bundle.records().len() != REQUIRED_RESOURCE_CERTIFICATION_FAMILIES.len()
        || bundle_summary.required_family_count() != required_family_count
        || bundle_summary.passed_family_count() != required_family_count
        || bundle_summary.failed_family_count() != 0
        || bundle_summary.missing_family_count() != 0
        || bundle_summary.duplicate_family_count() != 0
        || !bundle.failures().is_empty()
    {
        return Err(SignalError::invalid_input(
            "resource milestone B certification run requires one passing record for every required family",
        ));
    }

    let summary = ResourceMilestoneBCertificationRunSummary {
        required_family_count,
        certified_family_count: bundle_summary.passed_family_count(),
        failed_family_count: bundle_summary.failed_family_count(),
        bundle_digest: bundle.bundle_digest().to_owned(),
        required_scenario_count: scenario_matrix.summary().required_scenario_count(),
        certified_scenario_count: scenario_matrix.summary().certified_scenario_count(),
        scenario_matrix_digest: scenario_matrix.matrix_digest().to_owned(),
        required_performance_claim_count: performance_closeout.summary().required_claim_count(),
        certified_performance_claim_count: performance_closeout.summary().certified_claim_count(),
        performance_closeout_digest: performance_closeout.closeout_digest().to_owned(),
    };
    let run_digest = resource_canonical_digest(&ResourceMilestoneBCertificationRunDigestBasis {
        schema_version: RESOURCE_MILESTONE_B_CERTIFICATION_RUN_SCHEMA_VERSION,
        required_families: &REQUIRED_RESOURCE_CERTIFICATION_FAMILIES,
        required_scenarios: &REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS,
        required_performance_claims: &REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS,
        summary: &summary,
        bundle_digest: bundle.bundle_digest(),
        scenario_matrix_digest: scenario_matrix.matrix_digest(),
        performance_closeout_digest: performance_closeout.closeout_digest(),
        record_digests: bundle
            .records()
            .iter()
            .map(|record| (record.family(), record.evidence_digest()))
            .collect::<Vec<_>>(),
        scenario_digests: scenario_matrix
            .rows()
            .iter()
            .map(|row| (row.id(), row.evidence_digest()))
            .collect::<Vec<_>>(),
        performance_claim_digests: performance_closeout
            .rows()
            .iter()
            .map(|row| (row.id(), row.evidence_digest()))
            .collect::<Vec<_>>(),
    });

    Ok(ResourceMilestoneBCertificationRun {
        schema_version: RESOURCE_MILESTONE_B_CERTIFICATION_RUN_SCHEMA_VERSION.to_owned(),
        bundle,
        scenario_matrix,
        performance_closeout,
        summary,
        run_digest,
        passed: true,
    })
}
