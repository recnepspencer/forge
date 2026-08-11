use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::artifacts::{ArtifactBundle, ArtifactSurface, InvariantReport};
use super::failure::FailureBundle;
use super::planning::{CheckpointSemantics, FailureInjectionPoint, WorkflowState};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowCaptureRequest {
    pub step_index: Option<usize>,
    pub boundary: WorkflowState,
    pub requested_surfaces: BTreeSet<ArtifactSurface>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowStepOutcome {
    pub detail: Option<String>,
    pub request_checkpoint: bool,
}

impl WorkflowStepOutcome {
    pub fn applied() -> Self {
        Self {
            detail: None,
            request_checkpoint: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowFailureContext {
    pub step_index: Option<usize>,
    pub state: WorkflowState,
    pub failure_injection: Option<FailureInjectionPoint>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowStepTraceEntry {
    pub step_index: usize,
    pub step_name: String,
    pub state: WorkflowState,
    pub detail: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowCheckpointTraceEntry {
    pub step_index: usize,
    pub checkpoint_name: String,
    pub semantics: CheckpointSemantics,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowSession<SessionData> {
    pub adapter_name: String,
    pub workflow_name: String,
    pub scenario_name: String,
    pub state: WorkflowState,
    pub next_step_index: usize,
    pub step_trace: Vec<WorkflowStepTraceEntry>,
    pub checkpoint_trace: Vec<WorkflowCheckpointTraceEntry>,
    pub artifacts: Vec<ArtifactBundle>,
    pub invariant_reports: Vec<InvariantReport>,
    pub session_data: SessionData,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowCertificationReport<SessionData> {
    pub session: WorkflowSession<SessionData>,
    pub failure_bundle: Option<FailureBundle>,
}
