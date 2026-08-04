use super::super::family::ResourceCertificationFamily;
use super::catalog::{ResourceMilestoneBPerformanceClaimId, ResourceMilestoneBScenarioId};
use super::hostile_evidence::ResourceMilestoneBHostileScenarioEvidenceRow;
use super::performance::{
    ResourceMilestoneBPerformanceCloseoutRow, ResourceMilestoneBPerformanceCloseoutSummary,
};
use super::run::ResourceMilestoneBCertificationRunSummary;
use super::scenario_matrix::{
    ResourceMilestoneBScenarioMatrixSummary, ResourceMilestoneBScenarioRow,
};
use crate::data::resource::CompletionDenialClass;
use crate::data::resource::DeniedResourceCompletion;
use crate::data::resource::ResourceBoundaryPerformanceEnvelope;
use crate::data::resource::ResourceDiagnosticsExpansionBudget;
use crate::data::resource::ResourceDiagnosticsExpansionDenialClass;
use crate::data::resource::ResourceRuntimeSummary;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(super) struct ResourceMilestoneBScenarioMatrixDigestBasis<'a> {
    pub(super) schema_version: &'static str,
    pub(super) required_scenarios: &'a [ResourceMilestoneBScenarioId],
    pub(super) bundle_digest: &'a str,
    pub(super) summary: &'a ResourceMilestoneBScenarioMatrixSummary,
    pub(super) rows: &'a [ResourceMilestoneBScenarioRow],
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceMilestoneBHostileScenarioEvidenceDigestBasis<'a> {
    pub(super) schema_version: &'static str,
    pub(super) required_scenarios: &'a [ResourceMilestoneBScenarioId],
    pub(super) rows: &'a [ResourceMilestoneBHostileScenarioEvidenceRow],
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceMilestoneBHostileScenarioEvidenceRowDigestBasis {
    pub(super) id: ResourceMilestoneBScenarioId,
    pub(super) expected_denial_class: CompletionDenialClass,
    pub(super) denied_completion: DeniedResourceCompletion,
    pub(super) performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceMilestoneBPerformanceCloseoutDigestBasis<'a> {
    pub(super) schema_version: &'static str,
    pub(super) required_claims: &'a [ResourceMilestoneBPerformanceClaimId],
    pub(super) scenario_matrix_digest: &'a str,
    pub(super) summary: &'a ResourceMilestoneBPerformanceCloseoutSummary,
    pub(super) rows: &'a [ResourceMilestoneBPerformanceCloseoutRow],
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceMilestoneBPerformanceScenarioEvidenceBasis<'a> {
    pub(super) claim: ResourceMilestoneBPerformanceClaimId,
    pub(super) scenario: ResourceMilestoneBScenarioId,
    pub(super) scenario_evidence_digest: &'a str,
    pub(super) performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceMilestoneBPerformanceSummaryReadEvidenceBasis {
    pub(super) summary: ResourceRuntimeSummary,
    pub(super) performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceMilestoneBPerformanceDiagnosticsDenialBasis {
    pub(super) class: ResourceDiagnosticsExpansionDenialClass,
    pub(super) budget: ResourceDiagnosticsExpansionBudget,
    pub(super) replay_reconstruction_width: u32,
    pub(super) performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceMilestoneBPerformanceHostileDenialBasis<'a> {
    pub(super) scenario_matrix_digest: &'a str,
    pub(super) hostile_digests: &'a [(ResourceMilestoneBScenarioId, String)],
    pub(super) performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceMilestoneBCertificationRunDigestBasis<'a> {
    pub(super) schema_version: &'static str,
    pub(super) required_families: &'a [ResourceCertificationFamily],
    pub(super) required_scenarios: &'a [ResourceMilestoneBScenarioId],
    pub(super) required_performance_claims: &'a [ResourceMilestoneBPerformanceClaimId],
    pub(super) summary: &'a ResourceMilestoneBCertificationRunSummary,
    pub(super) bundle_digest: &'a str,
    pub(super) scenario_matrix_digest: &'a str,
    pub(super) performance_closeout_digest: &'a str,
    pub(super) record_digests: Vec<(ResourceCertificationFamily, &'a str)>,
    pub(super) scenario_digests: Vec<(ResourceMilestoneBScenarioId, &'a str)>,
    pub(super) performance_claim_digests: Vec<(ResourceMilestoneBPerformanceClaimId, &'a str)>,
}
