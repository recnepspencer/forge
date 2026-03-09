use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::capture::{DiagnosticsLevel, ExecutionMode};
use crate::runtime::{CaptureDepth, DeterminismMode};
use crate::timeline::{ClockDomain, FeedBatch, TimeMarker};
use crate::workload::{WorkBudget, WorkloadProfile};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenarioPlan<FixtureData> {
    pub name: String,
    pub fixture: FixtureData,
    pub declared_inputs: Vec<String>,
    pub declared_observations: Vec<String>,
    pub metadata: BTreeMap<String, String>,
}

impl<FixtureData> ScenarioPlan<FixtureData> {
    pub fn new(name: impl Into<String>, fixture: FixtureData) -> Self {
        Self {
            name: name.into(),
            fixture,
            declared_inputs: Vec::new(),
            declared_observations: Vec::new(),
            metadata: BTreeMap::new(),
        }
    }

    pub fn declare_input(mut self, input: impl Into<String>) -> Self {
        self.declared_inputs.push(input.into());
        self
    }

    pub fn input(self, input: impl Into<String>) -> Self {
        self.declare_input(input)
    }

    pub fn declare_observation(mut self, observation: impl Into<String>) -> Self {
        self.declared_observations.push(observation.into());
        self
    }

    pub fn observe(self, observation: impl Into<String>) -> Self {
        self.declare_observation(observation)
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    pub fn meta(self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.with_metadata(key, value)
    }

    pub fn compile(self) -> ScenarioFixture<FixtureData> {
        ScenarioFixture {
            name: self.name,
            fixture: self.fixture,
            declared_inputs: self.declared_inputs,
            declared_observations: self.declared_observations,
            metadata: self.metadata,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenarioFixture<FixtureData> {
    pub name: String,
    pub fixture: FixtureData,
    pub declared_inputs: Vec<String>,
    pub declared_observations: Vec<String>,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationBatch<MutationData> {
    pub name: String,
    pub operations: Vec<MutationData>,
    pub metadata: BTreeMap<String, String>,
}

impl<MutationData> MutationBatch<MutationData> {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            operations: Vec::new(),
            metadata: BTreeMap::new(),
        }
    }

    pub fn push(mut self, operation: MutationData) -> Self {
        self.operations.push(operation);
        self
    }

    pub fn operation(self, operation: MutationData) -> Self {
        self.push(operation)
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    pub fn meta(self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.with_metadata(key, value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionRequest<TargetId = String> {
    pub name: String,
    pub targets: Vec<TargetId>,
    pub capture_pre_snapshot: bool,
    pub capture_post_snapshot: bool,
    pub feed_batch: Option<FeedBatch>,
}

impl<TargetId> ExecutionRequest<TargetId> {
    pub fn new(name: impl Into<String>, targets: Vec<TargetId>) -> Self {
        Self {
            name: name.into(),
            targets,
            capture_pre_snapshot: true,
            capture_post_snapshot: true,
            feed_batch: None,
        }
    }

    pub fn target(name: impl Into<String>, target: TargetId) -> Self {
        Self::new(name, vec![target])
    }

    pub fn named(name: impl Into<String>, targets: Vec<TargetId>) -> Self {
        Self::new(name, targets)
    }

    pub fn without_pre_snapshot(mut self) -> Self {
        self.capture_pre_snapshot = false;
        self
    }

    pub fn without_post_snapshot(mut self) -> Self {
        self.capture_post_snapshot = false;
        self
    }

    pub fn without_snapshots(mut self) -> Self {
        self.capture_pre_snapshot = false;
        self.capture_post_snapshot = false;
        self
    }

    pub fn with_snapshots(mut self) -> Self {
        self.capture_pre_snapshot = true;
        self.capture_post_snapshot = true;
        self
    }

    pub fn with_feed_batch(mut self, feed_batch: FeedBatch) -> Self {
        self.feed_batch = Some(feed_batch);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionProfile {
    pub name: String,
    pub execution_mode: ExecutionMode,
    pub diagnostics_level: DiagnosticsLevel,
    pub capture_depth: CaptureDepth,
    pub determinism_mode: DeterminismMode,
    pub clock_domain: ClockDomain,
    pub time_marker: Option<TimeMarker>,
    pub workload: Option<WorkloadProfile>,
    pub work_budget: Option<WorkBudget>,
    pub metadata: BTreeMap<String, String>,
}

impl ExecutionProfile {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            execution_mode: ExecutionMode::RuntimeDefault,
            diagnostics_level: DiagnosticsLevel::Operational,
            capture_depth: CaptureDepth::Standard,
            determinism_mode: DeterminismMode::Strict,
            clock_domain: ClockDomain::Logical,
            time_marker: None,
            workload: None,
            work_budget: None,
            metadata: BTreeMap::new(),
        }
    }

    pub fn serial(name: impl Into<String>) -> Self {
        Self::new(name).with_execution_mode(ExecutionMode::Serial)
    }

    pub fn operational(name: impl Into<String>) -> Self {
        Self::serial(name).with_diagnostics_level(DiagnosticsLevel::Operational)
    }

    pub fn development(name: impl Into<String>) -> Self {
        Self::serial(name).with_diagnostics_level(DiagnosticsLevel::Development)
    }

    pub fn forensic(name: impl Into<String>) -> Self {
        Self::serial(name).with_diagnostics_level(DiagnosticsLevel::Forensic)
    }

    pub fn staged_parallel(name: impl Into<String>) -> Self {
        Self::new(name).with_execution_mode(ExecutionMode::StagedParallel)
    }

    pub fn replay(name: impl Into<String>) -> Self {
        Self::serial(name)
            .with_clock_domain(ClockDomain::Replay)
            .with_diagnostics_level(DiagnosticsLevel::Forensic)
    }

    pub fn frame_budget(name: impl Into<String>, frame_budget_micros: u64) -> Self {
        Self::serial(name).with_work_budget(WorkBudget {
            max_operations: None,
            max_duration_millis: None,
            frame_budget_micros: Some(frame_budget_micros),
        })
    }

    pub fn with_execution_mode(mut self, execution_mode: ExecutionMode) -> Self {
        self.execution_mode = execution_mode;
        self
    }

    pub fn with_diagnostics_level(mut self, diagnostics_level: DiagnosticsLevel) -> Self {
        self.diagnostics_level = diagnostics_level;
        self
    }

    pub fn with_capture_depth(mut self, capture_depth: CaptureDepth) -> Self {
        self.capture_depth = capture_depth;
        self
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    pub fn with_clock_domain(mut self, clock_domain: ClockDomain) -> Self {
        self.clock_domain = clock_domain;
        self
    }

    pub fn with_time_marker(mut self, time_marker: TimeMarker) -> Self {
        self.time_marker = Some(time_marker);
        self
    }

    pub fn with_workload(mut self, workload: WorkloadProfile) -> Self {
        self.workload = Some(workload);
        self
    }

    pub fn with_work_budget(mut self, work_budget: WorkBudget) -> Self {
        self.work_budget = Some(work_budget);
        self
    }
}
