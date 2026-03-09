use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::capture::{
    DiagnosticsLevel, DiagnosticsRecord, EventRecord, EventStreamRecord, ExecutionMode,
    ExplanationRecord, ProvenanceRecord, RunRecord, ScenarioRecord, SnapshotRecord,
};
use crate::comparison::{ComparisonMode, ComparisonProfile, ComparisonRecord};
use crate::replay::{ReplayRecord, ReplayRequest};
use crate::scenario::{ExecutionProfile, ExecutionRequest, MutationBatch, ScenarioFixture};
use crate::timeline::{ClockDomain, ExecutionPhase};

use super::adapter::{
    DiagnosticsHarnessAdapter, EventHarnessAdapter, EventStreamHarnessAdapter,
    ExplanationHarnessAdapter, HarnessAdapter, PerformanceHarnessAdapter, ProvenanceHarnessAdapter,
    ReplayHarnessAdapter,
};
use super::capability::{AdapterSupport, CaptureDepth};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HarnessCoreBundle<TargetId = String> {
    pub scenario: ScenarioRecord,
    pub pre_snapshot: Option<SnapshotRecord<TargetId>>,
    pub run: RunRecord<TargetId>,
    pub post_snapshot: Option<SnapshotRecord<TargetId>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HarnessObservedBundle<TargetId = String> {
    pub core: HarnessCoreBundle<TargetId>,
    pub diagnostics: DiagnosticsRecord,
    pub explanations: Vec<ExplanationRecord<TargetId>>,
    pub provenance: Vec<ProvenanceRecord<TargetId>>,
    pub events: Vec<EventRecord<TargetId>>,
    pub performance: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HarnessTimelineBundle<TargetId = String> {
    pub core: HarnessCoreBundle<TargetId>,
    pub events: Vec<EventRecord<TargetId>>,
    pub event_streams: Vec<EventStreamRecord<TargetId>>,
    pub performance: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HarnessError<AdapterError> {
    UnsupportedExecutionMode(ExecutionMode),
    UnsupportedDiagnosticsLevel(DiagnosticsLevel),
    UnsupportedCaptureDepth(CaptureDepth),
    UnsupportedComparisonMode(ComparisonMode),
    UnsupportedClockDomain(ClockDomain),
    UnsupportedExecutionPhase(ExecutionPhase),
    UnsupportedWorkBudget,
    UnsupportedReplay,
    Adapter(AdapterError),
}

impl<AdapterError: fmt::Display> fmt::Display for HarnessError<AdapterError> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedExecutionMode(mode) => {
                write!(f, "unsupported execution mode: {mode:?}")
            }
            Self::UnsupportedDiagnosticsLevel(level) => {
                write!(f, "unsupported diagnostics level: {level:?}")
            }
            Self::UnsupportedCaptureDepth(depth) => {
                write!(f, "unsupported capture depth: {depth:?}")
            }
            Self::UnsupportedComparisonMode(mode) => {
                write!(f, "unsupported comparison mode: {mode:?}")
            }
            Self::UnsupportedClockDomain(domain) => {
                write!(f, "unsupported clock domain: {domain:?}")
            }
            Self::UnsupportedExecutionPhase(phase) => {
                write!(f, "unsupported execution phase: {phase:?}")
            }
            Self::UnsupportedWorkBudget => write!(f, "unsupported work budget"),
            Self::UnsupportedReplay => write!(f, "unsupported replay"),
            Self::Adapter(error) => write!(f, "{error}"),
        }
    }
}

impl<AdapterError: fmt::Debug + fmt::Display> std::error::Error for HarnessError<AdapterError> {}

pub struct HarnessRunner<A> {
    adapter: A,
}

impl<A> HarnessRunner<A> {
    pub fn new(adapter: A) -> Self {
        Self { adapter }
    }

    pub fn adapter(&self) -> &A {
        &self.adapter
    }
}

struct LoadedHarnessRun<A: HarnessAdapter> {
    runtime: A::Runtime,
    core: HarnessCoreBundle<A::TargetId>,
}

impl<A> HarnessRunner<A>
where
    A: HarnessAdapter,
{
    fn execute_loaded(
        &self,
        fixture: &ScenarioFixture<A::Fixture>,
        mutation_batch: Option<&MutationBatch<A::Mutation>>,
        request: &ExecutionRequest<A::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<LoadedHarnessRun<A>, HarnessError<A::Error>> {
        self.validate_execution_profile(request, profile)?;

        let mut runtime = self
            .adapter
            .create_runtime()
            .map_err(HarnessError::Adapter)?;
        self.adapter
            .load_fixture(&mut runtime, fixture)
            .map_err(HarnessError::Adapter)?;

        let scenario = self.adapter.scenario_record(fixture);
        let pre_snapshot = if request.capture_pre_snapshot {
            Some(
                self.adapter
                    .capture_snapshot(&runtime, fixture, request, profile)
                    .map_err(HarnessError::Adapter)?,
            )
        } else {
            None
        };

        if let Some(batch) = mutation_batch {
            self.adapter
                .apply_mutation_batch(&mut runtime, batch)
                .map_err(HarnessError::Adapter)?;
        }

        let run = self
            .adapter
            .execute(&mut runtime, fixture, request, profile)
            .map_err(HarnessError::Adapter)?;

        let post_snapshot = if request.capture_post_snapshot {
            Some(
                self.adapter
                    .capture_snapshot(&runtime, fixture, request, profile)
                    .map_err(HarnessError::Adapter)?,
            )
        } else {
            None
        };

        Ok(LoadedHarnessRun {
            runtime,
            core: HarnessCoreBundle {
                scenario,
                pre_snapshot,
                run,
                post_snapshot,
            },
        })
    }

    fn validate_execution_profile(
        &self,
        request: &ExecutionRequest<A::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<(), HarnessError<A::Error>> {
        let capabilities = self.adapter.capabilities();
        if !capabilities.supports_execution_mode(profile.execution_mode) {
            return Err(HarnessError::UnsupportedExecutionMode(
                profile.execution_mode,
            ));
        }
        if !capabilities.supports_diagnostics_level(profile.diagnostics_level) {
            return Err(HarnessError::UnsupportedDiagnosticsLevel(
                profile.diagnostics_level,
            ));
        }
        if !capabilities.supports_capture_depth(profile.capture_depth) {
            return Err(HarnessError::UnsupportedCaptureDepth(profile.capture_depth));
        }
        if !capabilities.supports_clock_domain(profile.clock_domain) {
            return Err(HarnessError::UnsupportedClockDomain(profile.clock_domain));
        }
        if let Some(phase) = profile
            .workload
            .as_ref()
            .and_then(|workload| workload.phase)
        {
            if !capabilities.supports_execution_phase(phase) {
                return Err(HarnessError::UnsupportedExecutionPhase(phase));
            }
        }
        if let Some(phase) = request
            .feed_batch
            .as_ref()
            .and_then(|feed_batch| feed_batch.phase)
        {
            if !capabilities.supports_execution_phase(phase) {
                return Err(HarnessError::UnsupportedExecutionPhase(phase));
            }
        }
        if (profile.workload.is_some() || profile.work_budget.is_some())
            && !matches!(
                capabilities.workload_budget_support,
                AdapterSupport::Supported
            )
        {
            return Err(HarnessError::UnsupportedWorkBudget);
        }
        Ok(())
    }

    pub fn execute_core(
        &self,
        fixture: &ScenarioFixture<A::Fixture>,
        mutation_batch: Option<&MutationBatch<A::Mutation>>,
        request: &ExecutionRequest<A::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<HarnessCoreBundle<A::TargetId>, HarnessError<A::Error>> {
        Ok(self.execute_loaded(fixture, mutation_batch, request, profile)?.core)
    }

    pub fn compare_runs(
        &self,
        left: &RunRecord<A::TargetId>,
        right: &RunRecord<A::TargetId>,
        profile: &ComparisonProfile,
    ) -> Result<ComparisonRecord, HarnessError<A::Error>>
    where
        A::TargetId: std::fmt::Debug + PartialEq,
    {
        let capabilities = self.adapter.capabilities();
        if !capabilities.supports_comparison_mode(profile.mode) {
            return Err(HarnessError::UnsupportedComparisonMode(profile.mode));
        }
        Ok(crate::comparison::compare_run_records(left, right, profile))
    }
}

impl<A> HarnessRunner<A>
where
    A: HarnessAdapter + EventHarnessAdapter,
{
    pub fn execute_with_events(
        &self,
        fixture: &ScenarioFixture<A::Fixture>,
        mutation_batch: Option<&MutationBatch<A::Mutation>>,
        request: &ExecutionRequest<A::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<HarnessTimelineBundle<A::TargetId>, HarnessError<A::Error>> {
        let LoadedHarnessRun { runtime, core } =
            self.execute_loaded(fixture, mutation_batch, request, profile)?;
        let events = self
            .adapter
            .capture_events(&runtime, fixture, request, profile)
            .map_err(HarnessError::Adapter)?;
        Ok(HarnessTimelineBundle {
            core,
            events,
            event_streams: Vec::new(),
            performance: None,
        })
    }
}

impl<A> HarnessRunner<A>
where
    A: HarnessAdapter + EventStreamHarnessAdapter + PerformanceHarnessAdapter,
{
    pub fn execute_streamed(
        &self,
        fixture: &ScenarioFixture<A::Fixture>,
        mutation_batch: Option<&MutationBatch<A::Mutation>>,
        request: &ExecutionRequest<A::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<HarnessTimelineBundle<A::TargetId>, HarnessError<A::Error>> {
        let LoadedHarnessRun { runtime, core } =
            self.execute_loaded(fixture, mutation_batch, request, profile)?;
        let event_streams = self
            .adapter
            .capture_event_streams(&runtime, fixture, request, profile)
            .map_err(HarnessError::Adapter)?;
        let performance = Some(
            self.adapter
                .capture_performance(&runtime, fixture, profile)
                .map_err(HarnessError::Adapter)?,
        );
        Ok(HarnessTimelineBundle {
            core,
            events: Vec::new(),
            event_streams,
            performance,
        })
    }
}

impl<A> HarnessRunner<A>
where
    A: HarnessAdapter + ReplayHarnessAdapter,
{
    pub fn execute_replay(
        &self,
        fixture: &ScenarioFixture<A::Fixture>,
        mutation_batch: Option<&MutationBatch<A::Mutation>>,
        replay: &ReplayRequest<A::TargetId>,
    ) -> Result<ReplayRecord<A::TargetId>, HarnessError<A::Error>> {
        let capabilities = self.adapter.capabilities();
        if !matches!(capabilities.replay_support, AdapterSupport::Supported) {
            return Err(HarnessError::UnsupportedReplay);
        }
        let mut runtime = self
            .adapter
            .create_runtime()
            .map_err(HarnessError::Adapter)?;
        self.adapter
            .load_fixture(&mut runtime, fixture)
            .map_err(HarnessError::Adapter)?;
        if let Some(batch) = mutation_batch {
            self.adapter
                .apply_mutation_batch(&mut runtime, batch)
                .map_err(HarnessError::Adapter)?;
        }
        self.adapter
            .capture_replay(&runtime, fixture, replay)
            .map_err(HarnessError::Adapter)
    }
}

impl<A> HarnessRunner<A>
where
    A: HarnessAdapter
        + DiagnosticsHarnessAdapter
        + ExplanationHarnessAdapter
        + ProvenanceHarnessAdapter,
{
    pub fn execute_observed(
        &self,
        fixture: &ScenarioFixture<A::Fixture>,
        mutation_batch: Option<&MutationBatch<A::Mutation>>,
        request: &ExecutionRequest<A::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<HarnessObservedBundle<A::TargetId>, HarnessError<A::Error>> {
        let LoadedHarnessRun { runtime, core } =
            self.execute_loaded(fixture, mutation_batch, request, profile)?;

        let diagnostics = self
            .adapter
            .capture_diagnostics(&runtime, fixture, profile)
            .map_err(HarnessError::Adapter)?;
        let explanations = self
            .adapter
            .capture_explanations(&runtime, fixture, request, profile)
            .map_err(HarnessError::Adapter)?;
        let provenance = self
            .adapter
            .capture_provenance(&runtime, fixture, request, profile)
            .map_err(HarnessError::Adapter)?;

        Ok(HarnessObservedBundle {
            core,
            diagnostics,
            explanations,
            provenance,
            events: Vec::new(),
            performance: None,
        })
    }
}
