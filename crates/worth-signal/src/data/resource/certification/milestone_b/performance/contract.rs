use super::super::catalog::ResourceMilestoneBPerformanceClaimId;
use crate::data::resource::ResourceBoundaryPerformanceEnvelope;
use serde::Serialize;

pub const RESOURCE_MILESTONE_B_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION: &str =
    "worth-signal-resource-milestone-b-performance-closeout-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneBPerformanceCloseoutRow {
    pub(super) id: ResourceMilestoneBPerformanceClaimId,
    pub(super) evidence_digest: String,
    pub(super) performance: ResourceBoundaryPerformanceEnvelope,
    pub(super) passed: bool,
}

impl ResourceMilestoneBPerformanceCloseoutRow {
    pub fn id(&self) -> ResourceMilestoneBPerformanceClaimId {
        self.id
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
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
pub struct ResourceMilestoneBPerformanceCloseoutSummary {
    pub(super) required_claim_count: u32,
    pub(super) certified_claim_count: u32,
    pub(super) failed_claim_count: u32,
    pub(super) scenario_matrix_digest: String,
}

impl ResourceMilestoneBPerformanceCloseoutSummary {
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
pub struct ResourceMilestoneBPerformanceCloseout {
    pub(super) schema_version: String,
    pub(super) scenario_matrix_digest: String,
    pub(super) rows: Vec<ResourceMilestoneBPerformanceCloseoutRow>,
    pub(super) summary: ResourceMilestoneBPerformanceCloseoutSummary,
    pub(super) closeout_digest: String,
    pub(super) passed: bool,
}

impl ResourceMilestoneBPerformanceCloseout {
    pub fn schema_version(&self) -> &str {
        &self.schema_version
    }

    pub fn scenario_matrix_digest(&self) -> &str {
        &self.scenario_matrix_digest
    }

    pub fn rows(&self) -> &[ResourceMilestoneBPerformanceCloseoutRow] {
        &self.rows
    }

    pub fn summary(&self) -> &ResourceMilestoneBPerformanceCloseoutSummary {
        &self.summary
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }

    pub fn passed(&self) -> bool {
        self.passed
    }
}
