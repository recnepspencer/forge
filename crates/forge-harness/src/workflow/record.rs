use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::artifact::AttachmentRecord;

use super::capability::UnsupportedWorkflowComparison;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WorkflowState {
    Initialized,
    StepApplied,
    Checkpointed,
    Inspected,
    Failed,
    Completed,
}

impl WorkflowState {
    pub fn can_transition_to(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Initialized, Self::StepApplied)
                | (Self::Initialized, Self::Failed)
                | (Self::StepApplied, Self::Checkpointed)
                | (Self::StepApplied, Self::Inspected)
                | (Self::StepApplied, Self::Failed)
                | (Self::Checkpointed, Self::Inspected)
                | (Self::Checkpointed, Self::Failed)
                | (Self::Inspected, Self::StepApplied)
                | (Self::Inspected, Self::Completed)
                | (Self::Inspected, Self::Failed)
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowPlan<StepData> {
    pub workflow_name: String,
    pub scenario_name: String,
    pub crate_name: String,
    pub domain_name: String,
    pub seed: Option<u64>,
    pub steps: Vec<WorkflowStep<StepData>>,
    pub invariants: Vec<InvariantCheck>,
    pub regression_target: Option<RegressionTarget>,
    pub metadata: BTreeMap<String, String>,
}

impl<StepData> WorkflowPlan<StepData> {
    pub fn new(
        workflow_name: impl Into<String>,
        scenario_name: impl Into<String>,
        crate_name: impl Into<String>,
        domain_name: impl Into<String>,
    ) -> Self {
        Self {
            workflow_name: workflow_name.into(),
            scenario_name: scenario_name.into(),
            crate_name: crate_name.into(),
            domain_name: domain_name.into(),
            seed: None,
            steps: Vec::new(),
            invariants: Vec::new(),
            regression_target: None,
            metadata: BTreeMap::new(),
        }
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn step(mut self, step: WorkflowStep<StepData>) -> Self {
        self.steps.push(step);
        self
    }

    pub fn invariant(mut self, check: InvariantCheck) -> Self {
        self.invariants.push(check);
        self
    }

    pub fn with_regression_target(mut self, target: RegressionTarget) -> Self {
        self.regression_target = Some(target);
        self
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowStep<StepData> {
    pub name: String,
    pub operation: StepData,
    pub checkpoint_after: bool,
    pub capture_boundaries: BTreeSet<WorkflowState>,
    pub invariant_boundaries: BTreeSet<WorkflowState>,
    pub failure_injection: Option<FailureInjectionPoint>,
    pub metadata: BTreeMap<String, String>,
}

impl<StepData> WorkflowStep<StepData> {
    pub fn new(name: impl Into<String>, operation: StepData) -> Self {
        Self {
            name: name.into(),
            operation,
            checkpoint_after: false,
            capture_boundaries: BTreeSet::new(),
            invariant_boundaries: BTreeSet::new(),
            failure_injection: None,
            metadata: BTreeMap::new(),
        }
    }

    pub fn checkpoint_after(mut self) -> Self {
        self.checkpoint_after = true;
        self
    }

    pub fn capture_at(mut self, state: WorkflowState) -> Self {
        self.capture_boundaries.insert(state);
        self
    }

    pub fn inspect_at(mut self, state: WorkflowState) -> Self {
        self.invariant_boundaries.insert(state);
        self
    }

    pub fn with_failure_injection(mut self, injection: FailureInjectionPoint) -> Self {
        self.failure_injection = Some(injection);
        self
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowCheckpoint {
    pub checkpoint_name: String,
    pub semantics: CheckpointSemantics,
    pub step_index: usize,
    pub metadata: BTreeMap<String, String>,
}

impl WorkflowCheckpoint {
    pub fn new(
        checkpoint_name: impl Into<String>,
        semantics: CheckpointSemantics,
        step_index: usize,
    ) -> Self {
        Self {
            checkpoint_name: checkpoint_name.into(),
            semantics,
            step_index,
            metadata: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CheckpointSemantics {
    SnapshotRestore,
    DurableRecovery,
    BranchHeadSnapshot,
    ReplayAnchor,
    AdapterDefined,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowRuntimeProfile {
    pub runtime_profile: String,
    pub policy_name: Option<String>,
    pub executor_name: Option<String>,
    pub capability_profile: Option<String>,
}

impl WorkflowRuntimeProfile {
    pub fn new(runtime_profile: impl Into<String>) -> Self {
        Self {
            runtime_profile: runtime_profile.into(),
            policy_name: None,
            executor_name: None,
            capability_profile: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FailureInjectionPoint {
    pub boundary: WorkflowState,
    pub location: String,
    pub detail: Option<String>,
}

impl FailureInjectionPoint {
    pub fn new(boundary: WorkflowState, location: impl Into<String>) -> Self {
        Self {
            boundary,
            location: location.into(),
            detail: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ArtifactClass {
    Truth,
    Observability,
    Forensic,
    Performance,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ArtifactSurface {
    SnapshotVisibleTruth,
    BranchHeadState,
    ReplayRecoveryTruthState,
    Diagnostics,
    PatchChangeSurface,
    StepTrace,
    CheckpointTrace,
    FailureInjectionTrace,
    ReproductionMetadata,
    ComplexityCounters,
    BudgetOutcome,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArtifactBundle {
    pub artifact_class: ArtifactClass,
    pub surface: ArtifactSurface,
    pub name: String,
    pub boundary: WorkflowState,
    pub payload: Value,
    pub attachments: Vec<AttachmentRecord>,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvariantCheck {
    pub check_id: String,
    pub description: String,
    pub boundary: WorkflowState,
    pub required: bool,
}

impl InvariantCheck {
    pub fn new(
        check_id: impl Into<String>,
        description: impl Into<String>,
        boundary: WorkflowState,
    ) -> Self {
        Self {
            check_id: check_id.into(),
            description: description.into(),
            boundary,
            required: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvariantReport {
    pub check_id: String,
    pub boundary: WorkflowState,
    pub passed: bool,
    pub detail: String,
    pub fields: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DifferentialComparison {
    pub comparison_name: String,
    pub left_label: String,
    pub right_label: String,
    pub active_profile: WorkflowRuntimeProfile,
    pub guaranteed_overlap: BTreeSet<ArtifactSurface>,
    pub skipped_surfaces: Vec<UnsupportedWorkflowComparison>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DifferentialOutcome {
    pub matched: bool,
    pub compared_surfaces: BTreeSet<ArtifactSurface>,
    pub mismatches: Vec<String>,
    pub skipped_surfaces: Vec<UnsupportedWorkflowComparison>,
}

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
