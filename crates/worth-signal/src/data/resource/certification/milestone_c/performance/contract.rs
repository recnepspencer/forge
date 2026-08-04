use super::super::catalog::ResourceMilestoneCPolicyPerformanceClaimId;
use crate::data::resource::ResourceBoundaryPerformanceEnvelope;
use serde::Serialize;

pub const RESOURCE_MILESTONE_C_POLICY_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION: &str =
    "worth-signal-resource-milestone-c-policy-performance-closeout-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneCPolicyPerformanceCloseoutRow {
    pub(super) id: ResourceMilestoneCPolicyPerformanceClaimId,
    pub(super) evidence_digest: String,
    pub(super) policy_provenance_digest: String,
    pub(super) performance: ResourceBoundaryPerformanceEnvelope,
    pub(super) passed: bool,
}

impl ResourceMilestoneCPolicyPerformanceCloseoutRow {
    pub fn id(&self) -> ResourceMilestoneCPolicyPerformanceClaimId {
        self.id
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    pub fn policy_provenance_digest(&self) -> &str {
        &self.policy_provenance_digest
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }

    pub fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneCPolicyPerformanceCloseoutSummary {
    pub(super) required_claim_count: u32,
    pub(super) certified_claim_count: u32,
    pub(super) failed_claim_count: u32,
    pub(super) scenario_matrix_digest: String,
}

impl ResourceMilestoneCPolicyPerformanceCloseoutSummary {
    pub fn required_claim_count(&self) -> u32 {
        self.required_claim_count
    }

    pub fn certified_claim_count(&self) -> u32 {
        self.certified_claim_count
    }

    pub fn failed_claim_count(&self) -> u32 {
        self.failed_claim_count
    }

    pub fn scenario_matrix_digest(&self) -> &str {
        &self.scenario_matrix_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneCPolicyPerformanceCloseout {
    pub(super) schema_version: String,
    pub(super) scenario_matrix_digest: String,
    pub(super) rows: Vec<ResourceMilestoneCPolicyPerformanceCloseoutRow>,
    pub(super) summary: ResourceMilestoneCPolicyPerformanceCloseoutSummary,
    pub(super) closeout_digest: String,
    pub(super) passed: bool,
}

impl ResourceMilestoneCPolicyPerformanceCloseout {
    pub fn schema_version(&self) -> &str {
        &self.schema_version
    }

    pub fn scenario_matrix_digest(&self) -> &str {
        &self.scenario_matrix_digest
    }

    pub fn rows(&self) -> &[ResourceMilestoneCPolicyPerformanceCloseoutRow] {
        &self.rows
    }

    pub fn summary(&self) -> &ResourceMilestoneCPolicyPerformanceCloseoutSummary {
        &self.summary
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }

    pub fn passed(&self) -> bool {
        self.passed
    }
}
