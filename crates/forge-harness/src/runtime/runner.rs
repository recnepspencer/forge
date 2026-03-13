use std::fmt;
use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::capture::{
    DiagnosticsLevel, DiagnosticsRecord, EventRecord, EventStreamRecord, ExecutionMode,
    ExplanationRecord, ProvenanceRecord, RunRecord, ScenarioRecord, SnapshotRecord,
};
use crate::comparison::{ComparisonMode, ComparisonProfile, ComparisonRecord};
use crate::replay::{ReplayRecord, ReplayRequest};
use crate::scenario::{
    CaptureMask, ExecutionProfile, ExecutionRequest, MutationBatch, ScenarioFixture,
};
use crate::timeline::{ClockDomain, ExecutionPhase};

use super::adapter::{
    DiagnosticsHarnessAdapter, EventHarnessAdapter, EventStreamHarnessAdapter,
    ExplanationHarnessAdapter, HarnessAdapter, HarnessAdapterAsync, PerformanceHarnessAdapter,
    ProvenanceHarnessAdapter, ReplayHarnessAdapter,
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
    pub diagnostics: Option<DiagnosticsRecord>,
    pub explanations: Vec<ExplanationRecord<TargetId>>,
    pub provenance: Vec<ProvenanceRecord<TargetId>>,
    pub events: Vec<EventRecord<TargetId>>,
    pub performance: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HarnessDiagnosedBundle<TargetId = String> {
    pub core: HarnessCoreBundle<TargetId>,
    pub diagnostics: Option<DiagnosticsRecord>,
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
    A::TargetId: PartialEq,
{
    fn capture_request(
        &self,
        request: &ExecutionRequest<A::TargetId>,
    ) -> ExecutionRequest<A::TargetId> {
        let mut filtered = request.clone();
        if let Some(included) = &request.capture.target_policy.included_targets {
            filtered.targets = request
                .targets
                .iter()
                .filter(|target| included.iter().any(|candidate| candidate == *target))
                .cloned()
                .collect();
        }
        filtered
    }

    fn minimal_run_record(
        &self,
        mut run: RunRecord<A::TargetId>,
        capture_mask: &CaptureMask,
    ) -> RunRecord<A::TargetId> {
        if !capture_mask.run_summary {
            run.summary = serde_json::json!({});
        }
        if !capture_mask.attachments {
            run.attachments.clear();
        }
        run
    }

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
            .prepare_runtime(&mut runtime, profile)
            .map_err(HarnessError::Adapter)?;
        self.adapter
            .load_fixture(&mut runtime, fixture)
            .map_err(HarnessError::Adapter)?;

        let scenario = self.adapter.scenario_record(fixture);
        let capture_request = self.capture_request(request);
        let pre_snapshot = if request.capture.mask.pre_snapshot {
            Some(
                self.adapter
                    .capture_snapshot(&runtime, fixture, &capture_request, profile)
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

        let run = self.minimal_run_record(
            self.adapter
                .execute(&mut runtime, fixture, request, profile)
                .map_err(HarnessError::Adapter)?,
            &request.capture.mask,
        );

        let post_snapshot = if request.capture.mask.post_snapshot {
            Some(
                self.adapter
                    .capture_snapshot(&runtime, fixture, &capture_request, profile)
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
        Ok(self
            .execute_loaded(fixture, mutation_batch, request, profile)?
            .core)
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
    A::TargetId: PartialEq,
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
        let events = if request.capture.mask.events {
            self.adapter
                .capture_events(&runtime, fixture, request, profile)
                .map_err(HarnessError::Adapter)?
        } else {
            Vec::new()
        };
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
    A::TargetId: PartialEq,
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
        let event_streams = if request.capture.mask.event_streams {
            self.adapter
                .capture_event_streams(&runtime, fixture, request, profile)
                .map_err(HarnessError::Adapter)?
        } else {
            Vec::new()
        };
        let performance = if request.capture.mask.performance {
            Some(
                self.adapter
                    .capture_performance(&runtime, fixture, profile)
                    .map_err(HarnessError::Adapter)?,
            )
        } else {
            None
        };
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
    A::TargetId: PartialEq,
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
        let mut record = self
            .adapter
            .capture_replay(&runtime, fixture, replay)
            .map_err(HarnessError::Adapter)?;
        if !replay.request.capture.mask.replay_artifacts {
            record.attachments.clear();
            record.summary = serde_json::json!({});
        }
        Ok(record)
    }
}

impl<A> HarnessRunner<A>
where
    A: HarnessAdapter + DiagnosticsHarnessAdapter,
    A::TargetId: PartialEq,
{
    pub fn execute_diagnosed(
        &self,
        fixture: &ScenarioFixture<A::Fixture>,
        mutation_batch: Option<&MutationBatch<A::Mutation>>,
        request: &ExecutionRequest<A::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<HarnessDiagnosedBundle<A::TargetId>, HarnessError<A::Error>> {
        let LoadedHarnessRun { runtime, core } =
            self.execute_loaded(fixture, mutation_batch, request, profile)?;
        let diagnostics = if request.capture.mask.diagnostics {
            Some(
                self.adapter
                    .capture_diagnostics(&runtime, fixture, profile)
                    .map_err(HarnessError::Adapter)?,
            )
        } else {
            None
        };
        Ok(HarnessDiagnosedBundle { core, diagnostics })
    }
}

impl<A> HarnessRunner<A>
where
    A: HarnessAdapter
        + DiagnosticsHarnessAdapter
        + ExplanationHarnessAdapter
        + ProvenanceHarnessAdapter,
    A::TargetId: PartialEq,
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

        let capture_request = self.capture_request(request);
        let diagnostics = if request.capture.mask.diagnostics {
            Some(
                self.adapter
                    .capture_diagnostics(&runtime, fixture, profile)
                    .map_err(HarnessError::Adapter)?,
            )
        } else {
            None
        };
        let explanations = if request.capture.mask.explanations {
            self.adapter
                .capture_explanations(&runtime, fixture, &capture_request, profile)
                .map_err(HarnessError::Adapter)?
        } else {
            Vec::new()
        };
        let provenance = if request.capture.mask.provenance {
            self.adapter
                .capture_provenance(&runtime, fixture, &capture_request, profile)
                .map_err(HarnessError::Adapter)?
        } else {
            Vec::new()
        };

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

pub struct AsyncHarnessRunner<A> {
    adapter: A,
}

impl<A> AsyncHarnessRunner<A> {
    pub fn new(adapter: A) -> Self {
        Self { adapter }
    }
}

impl<A> AsyncHarnessRunner<A>
where
    A: HarnessAdapterAsync,
    A::TargetId: PartialEq,
{
    pub fn execute_core_async<'a>(
        &'a self,
        fixture: &'a ScenarioFixture<A::Fixture>,
        mutation_batch: Option<&'a MutationBatch<A::Mutation>>,
        request: &'a ExecutionRequest<A::TargetId>,
        profile: &'a ExecutionProfile,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<HarnessCoreBundle<A::TargetId>, HarnessError<A::Error>>>
                + 'a,
        >,
    > {
        Box::pin(async move {
            let capabilities = self.adapter.capabilities();
            if !capabilities.supports_execution_mode(profile.execution_mode) {
                return Err(HarnessError::UnsupportedExecutionMode(
                    profile.execution_mode,
                ));
            }
            let mut runtime = self
                .adapter
                .create_runtime_async()
                .await
                .map_err(HarnessError::Adapter)?;
            self.adapter
                .load_fixture_async(&mut runtime, fixture)
                .await
                .map_err(HarnessError::Adapter)?;
            let scenario = self.adapter.scenario_record(fixture);
            let capture_request = request.clone();
            let pre_snapshot = if request.capture.mask.pre_snapshot {
                Some(
                    self.adapter
                        .capture_snapshot_async(&runtime, fixture, &capture_request, profile)
                        .await
                        .map_err(HarnessError::Adapter)?,
                )
            } else {
                None
            };
            if let Some(batch) = mutation_batch {
                self.adapter
                    .apply_mutation_batch_async(&mut runtime, batch)
                    .await
                    .map_err(HarnessError::Adapter)?;
            }
            let run = self
                .adapter
                .execute_async(&mut runtime, fixture, request, profile)
                .await
                .map_err(HarnessError::Adapter)?;
            let post_snapshot = if request.capture.mask.post_snapshot {
                Some(
                    self.adapter
                        .capture_snapshot_async(&runtime, fixture, &capture_request, profile)
                        .await
                        .map_err(HarnessError::Adapter)?,
                )
            } else {
                None
            };
            Ok(HarnessCoreBundle {
                scenario,
                pre_snapshot,
                run,
                post_snapshot,
            })
        })
    }
}
