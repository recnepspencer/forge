use std::fmt;

use crate::replay::{ReplayRecord, ReplayRequest};
use crate::runtime::{
    DiagnosticsHarnessAdapter, EventHarnessAdapter, EventStreamHarnessAdapter,
    ExplanationHarnessAdapter, HarnessAdapter, HarnessError, HarnessObservedBundle, HarnessRunner,
    HarnessTimelineBundle, PerformanceHarnessAdapter, ProvenanceHarnessAdapter,
    ReplayHarnessAdapter,
};
use crate::scenario::{ExecutionProfile, ExecutionRequest, MutationBatch, ScenarioFixture};
use crate::workload::WorkBudget;

pub struct ProfileCatalog;

impl ProfileCatalog {
    pub fn operational(name: impl Into<String>) -> ExecutionProfile {
        ExecutionProfile::operational(name)
    }

    pub fn development(name: impl Into<String>) -> ExecutionProfile {
        ExecutionProfile::development(name)
    }

    pub fn forensic(name: impl Into<String>) -> ExecutionProfile {
        ExecutionProfile::forensic(name)
    }

    pub fn replay(name: impl Into<String>) -> ExecutionProfile {
        ExecutionProfile::replay(name)
    }

    pub fn frame(name: impl Into<String>, frame_budget_micros: u64) -> ExecutionProfile {
        ExecutionProfile::frame_budget(name, frame_budget_micros)
    }

    pub fn throughput(
        name: impl Into<String>,
        max_operations: u64,
        max_duration_millis: u64,
    ) -> ExecutionProfile {
        ExecutionProfile::operational(name).with_work_budget(WorkBudget {
            max_operations: Some(max_operations),
            max_duration_millis: Some(max_duration_millis),
            frame_budget_micros: None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BenchError<AdapterError> {
    Runner(HarnessError<AdapterError>),
}

impl<AdapterError: fmt::Display> fmt::Display for BenchError<AdapterError> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Runner(error) => write!(f, "{error}"),
        }
    }
}

impl<AdapterError: fmt::Debug + fmt::Display> std::error::Error for BenchError<AdapterError> {}

pub struct HarnessBench<A, FixtureData, MutationData, TargetId> {
    runner: HarnessRunner<A>,
    fixture: ScenarioFixture<FixtureData>,
    mutation_batch: Option<MutationBatch<MutationData>>,
    request: ExecutionRequest<TargetId>,
}

impl<A, FixtureData, MutationData, TargetId> HarnessBench<A, FixtureData, MutationData, TargetId> {
    pub fn new(
        adapter: A,
        fixture: ScenarioFixture<FixtureData>,
        request: ExecutionRequest<TargetId>,
    ) -> Self {
        Self {
            runner: HarnessRunner::new(adapter),
            fixture,
            mutation_batch: None,
            request,
        }
    }

    pub fn mutate(mut self, mutation_batch: MutationBatch<MutationData>) -> Self {
        self.mutation_batch = Some(mutation_batch);
        self
    }
}

impl<A, FixtureData, MutationData, TargetId> HarnessBench<A, FixtureData, MutationData, TargetId>
where
    A: HarnessAdapter<Fixture = FixtureData, Mutation = MutationData, TargetId = TargetId>,
{
    pub fn run(
        &self,
        profile: &ExecutionProfile,
    ) -> Result<crate::runtime::HarnessCoreBundle<TargetId>, BenchError<A::Error>> {
        self.runner
            .execute_core(
                &self.fixture,
                self.mutation_batch.as_ref(),
                &self.request,
                profile,
            )
            .map_err(BenchError::Runner)
    }
}

impl<A, FixtureData, MutationData, TargetId> HarnessBench<A, FixtureData, MutationData, TargetId>
where
    A: HarnessAdapter<Fixture = FixtureData, Mutation = MutationData, TargetId = TargetId>
        + DiagnosticsHarnessAdapter
        + ExplanationHarnessAdapter
        + ProvenanceHarnessAdapter,
{
    pub fn observe(
        &self,
        profile: &ExecutionProfile,
    ) -> Result<HarnessObservedBundle<TargetId>, BenchError<A::Error>> {
        self.runner
            .execute_observed(
                &self.fixture,
                self.mutation_batch.as_ref(),
                &self.request,
                profile,
            )
            .map_err(BenchError::Runner)
    }
}

impl<A, FixtureData, MutationData, TargetId> HarnessBench<A, FixtureData, MutationData, TargetId>
where
    A: HarnessAdapter<Fixture = FixtureData, Mutation = MutationData, TargetId = TargetId>
        + EventHarnessAdapter,
{
    pub fn events(
        &self,
        profile: &ExecutionProfile,
    ) -> Result<HarnessTimelineBundle<TargetId>, BenchError<A::Error>> {
        self.runner
            .execute_with_events(
                &self.fixture,
                self.mutation_batch.as_ref(),
                &self.request,
                profile,
            )
            .map_err(BenchError::Runner)
    }
}

impl<A, FixtureData, MutationData, TargetId> HarnessBench<A, FixtureData, MutationData, TargetId>
where
    A: HarnessAdapter<Fixture = FixtureData, Mutation = MutationData, TargetId = TargetId>
        + EventStreamHarnessAdapter
        + PerformanceHarnessAdapter,
{
    pub fn stream(
        &self,
        profile: &ExecutionProfile,
    ) -> Result<HarnessTimelineBundle<TargetId>, BenchError<A::Error>> {
        self.runner
            .execute_streamed(
                &self.fixture,
                self.mutation_batch.as_ref(),
                &self.request,
                profile,
            )
            .map_err(BenchError::Runner)
    }
}

impl<A, FixtureData, MutationData, TargetId> HarnessBench<A, FixtureData, MutationData, TargetId>
where
    A: HarnessAdapter<Fixture = FixtureData, Mutation = MutationData, TargetId = TargetId>
        + ReplayHarnessAdapter,
{
    pub fn replay(
        &self,
        replay: &ReplayRequest<TargetId>,
    ) -> Result<ReplayRecord<TargetId>, BenchError<A::Error>> {
        self.runner
            .execute_replay(&self.fixture, self.mutation_batch.as_ref(), replay)
            .map_err(BenchError::Runner)
    }
}

pub fn bench<A, FixtureData, MutationData, TargetId>(
    adapter: A,
    fixture: ScenarioFixture<FixtureData>,
    request: ExecutionRequest<TargetId>,
) -> HarnessBench<A, FixtureData, MutationData, TargetId> {
    HarnessBench::new(adapter, fixture, request)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::capture::{DiagnosticsLevel, ExecutionMode};
    use crate::replay::ReplayRequest;
    use crate::runtime::{AdapterSupport, CaptureDepth, HarnessCapabilities};
    use crate::scenario::{ExecutionRequest, MutationBatch, ScenarioPlan};
    use crate::timeline::{ClockDomain, ExecutionPhase, FeedBatch};

    use super::{bench, ProfileCatalog};
    use crate::tooling::AdapterDouble;

    #[test]
    fn harness_bench_runs_core_flow_fluently() {
        let mut capabilities = HarnessCapabilities::default();
        capabilities.execution_modes.insert(ExecutionMode::Serial);
        capabilities
            .diagnostics_levels
            .insert(DiagnosticsLevel::Operational);
        capabilities.capture_depths.insert(CaptureDepth::Standard);
        capabilities.clock_domains.insert(ClockDomain::Logical);

        let fixture = ScenarioPlan::new("fixture", json!({ "fixture": true }))
            .input("source")
            .observe("target")
            .compile();
        let request = ExecutionRequest::target("pull", "target".to_string());
        let profile = ProfileCatalog::operational("operational");
        let batch = MutationBatch::new("change")
            .operation(json!({ "set": 1 }))
            .meta("cause", "test");

        let bundle = bench(AdapterDouble::new("double", capabilities), fixture, request)
            .mutate(batch)
            .run(&profile)
            .unwrap();

        assert_eq!(bundle.run.profile_name, "operational");
    }

    #[test]
    fn harness_bench_supports_stream_and_replay_paths() {
        let mut capabilities = HarnessCapabilities::default();
        capabilities.execution_modes.insert(ExecutionMode::Serial);
        capabilities
            .diagnostics_levels
            .insert(DiagnosticsLevel::Operational);
        capabilities.capture_depths.insert(CaptureDepth::Standard);
        capabilities.clock_domains.insert(ClockDomain::Logical);
        capabilities
            .execution_phases
            .insert(ExecutionPhase::Evaluate);
        capabilities.workload_budget_support = AdapterSupport::Supported;
        capabilities.replay_support = AdapterSupport::Supported;

        let fixture = ScenarioPlan::new("fixture", json!({ "fixture": true })).compile();
        let request = ExecutionRequest::target("pull", "target".to_string())
            .with_feed_batch(FeedBatch::new("feed", 1, 1).with_phase(ExecutionPhase::Evaluate));
        let profile = ProfileCatalog::frame("frame", 1_000);

        let source_run = bench(AdapterDouble::new("double", capabilities.clone()), fixture.clone(), request.clone())
            .run(&ProfileCatalog::operational("operational"))
            .unwrap()
            .run;

        let replay = ReplayRequest {
            name: "replay".to_string(),
            source_run,
            request: request.clone(),
            profile: ProfileCatalog::replay("replay"),
        };

        let streamed = bench(AdapterDouble::new("double", capabilities.clone()), fixture.clone(), request)
            .stream(&profile)
            .unwrap();
        assert_eq!(streamed.event_streams.len(), 1);

        let replay_record = bench(
            AdapterDouble::new("double", capabilities),
            fixture,
            ExecutionRequest::target("ignored", "target".to_string()),
        )
        .replay(&replay)
        .unwrap();
        assert_eq!(replay_record.replay_name, "replay");
    }
}
