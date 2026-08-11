use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::artifacts::InvariantCheck;
use super::failure::RegressionTarget;

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
