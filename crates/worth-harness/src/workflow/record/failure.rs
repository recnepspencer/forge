use serde::{Deserialize, Serialize};

use super::artifacts::InvariantReport;
use super::planning::FailureInjectionPoint;
use super::session::{WorkflowCheckpointTraceEntry, WorkflowStepTraceEntry};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FailureBundleVersion {
    V1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReproductionMetadata {
    pub format: String,
    pub payload: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FailureBundle {
    pub version: FailureBundleVersion,
    pub crate_name: String,
    pub domain_name: String,
    pub workflow_name: String,
    pub scenario_name: String,
    pub seed: Option<u64>,
    pub runtime_profile: String,
    pub policy_name: Option<String>,
    pub executor_name: Option<String>,
    pub step_trace: Vec<WorkflowStepTraceEntry>,
    pub checkpoint_trace: Vec<WorkflowCheckpointTraceEntry>,
    pub failure_injection_point: Option<FailureInjectionPoint>,
    pub invariant_failures: Vec<InvariantReport>,
    pub artifact_diffs: Vec<String>,
    pub reproduction: ReproductionMetadata,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegressionTargetKind {
    KnownFailing,
    ExpectedFailure,
    Quarantined,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegressionTarget {
    pub kind: RegressionTargetKind,
    pub issue_key: String,
    pub summary: String,
    pub reproduction_hint: Option<String>,
}
