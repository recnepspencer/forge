use super::super::catalog::{
    ResourceMilestoneCPolicyCertificationFamily, ResourceMilestoneCPolicyScenarioEvidenceKind,
    ResourceMilestoneCPolicyScenarioId,
};
use crate::data::resource::ResourceBoundaryPerformanceEnvelope;
use crate::data::resource::ResourcePolicyRestoreCompatibilityDenialClass;
use crate::data::resource::ResourceRetryDenialClass;
use crate::data::resource::ResourceTimeoutHeartbeatExtensionDenialClass;
use serde::Deserialize;
use serde::Serialize;

pub const RESOURCE_MILESTONE_C_POLICY_SCENARIO_MATRIX_SCHEMA_VERSION: &str =
    "worth-signal-resource-milestone-c-policy-scenario-matrix-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneCPolicyScenarioRow {
    pub(super) id: ResourceMilestoneCPolicyScenarioId,
    pub(super) evidence_kind: ResourceMilestoneCPolicyScenarioEvidenceKind,
    pub(super) certification_family: Option<ResourceMilestoneCPolicyCertificationFamily>,
    pub(super) policy_provenance_digest: Option<String>,
    pub(super) retry_denial_class: Option<ResourceRetryDenialClass>,
    pub(super) timeout_heartbeat_denial_class: Option<ResourceTimeoutHeartbeatExtensionDenialClass>,
    pub(super) replay_restore_denial_class: Option<ResourcePolicyRestoreCompatibilityDenialClass>,
    pub(super) evidence_digest: String,
    pub(super) performance: ResourceBoundaryPerformanceEnvelope,
    pub(super) passed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneCPolicyScenarioMatrixSummary {
    pub(super) required_scenario_count: u32,
    pub(super) certified_scenario_count: u32,
    pub(super) failed_scenario_count: u32,
    pub(super) bundle_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMilestoneCPolicyScenarioMatrix {
    pub(super) schema_version: String,
    pub(super) rows: Vec<ResourceMilestoneCPolicyScenarioRow>,
    pub(super) summary: ResourceMilestoneCPolicyScenarioMatrixSummary,
    pub(super) matrix_digest: String,
    pub(super) passed: bool,
}

impl ResourceMilestoneCPolicyScenarioMatrix {
    pub fn schema_version(&self) -> &str {
        &self.schema_version
    }

    pub fn rows(&self) -> &[ResourceMilestoneCPolicyScenarioRow] {
        &self.rows
    }

    pub fn summary(&self) -> &ResourceMilestoneCPolicyScenarioMatrixSummary {
        &self.summary
    }

    pub fn matrix_digest(&self) -> &str {
        &self.matrix_digest
    }

    pub fn passed(&self) -> bool {
        self.passed
    }
}

impl ResourceMilestoneCPolicyScenarioMatrixSummary {
    pub fn required_scenario_count(&self) -> u32 {
        self.required_scenario_count
    }

    pub fn certified_scenario_count(&self) -> u32 {
        self.certified_scenario_count
    }

    pub fn failed_scenario_count(&self) -> u32 {
        self.failed_scenario_count
    }

    pub fn bundle_digest(&self) -> &str {
        &self.bundle_digest
    }
}

impl ResourceMilestoneCPolicyScenarioRow {
    pub fn id(&self) -> ResourceMilestoneCPolicyScenarioId {
        self.id
    }

    pub fn evidence_kind(&self) -> ResourceMilestoneCPolicyScenarioEvidenceKind {
        self.evidence_kind
    }

    pub fn certification_family(&self) -> Option<ResourceMilestoneCPolicyCertificationFamily> {
        self.certification_family
    }

    pub fn policy_provenance_digest(&self) -> Option<&str> {
        self.policy_provenance_digest.as_deref()
    }

    pub fn retry_denial_class(&self) -> Option<ResourceRetryDenialClass> {
        self.retry_denial_class
    }

    pub fn timeout_heartbeat_denial_class(
        &self,
    ) -> Option<ResourceTimeoutHeartbeatExtensionDenialClass> {
        self.timeout_heartbeat_denial_class
    }

    pub fn replay_restore_denial_class(
        &self,
    ) -> Option<ResourcePolicyRestoreCompatibilityDenialClass> {
        self.replay_restore_denial_class
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
