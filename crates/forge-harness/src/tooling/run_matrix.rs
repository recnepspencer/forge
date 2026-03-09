use crate::runtime::{
    DiagnosticsHarnessAdapter, ExplanationHarnessAdapter, HarnessAdapter, HarnessCoreBundle,
    HarnessError, HarnessObservedBundle, HarnessRunner, ProvenanceHarnessAdapter,
};
use crate::scenario::{ExecutionProfile, ExecutionRequest, MutationBatch, ScenarioFixture};

pub struct RunMatrix<A, FixtureData, MutationData, TargetId> {
    runner: HarnessRunner<A>,
    fixture: ScenarioFixture<FixtureData>,
    mutation_batch: Option<MutationBatch<MutationData>>,
    request: ExecutionRequest<TargetId>,
    profiles: Vec<ExecutionProfile>,
}

impl<A, FixtureData, MutationData, TargetId> RunMatrix<A, FixtureData, MutationData, TargetId> {
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
            profiles: Vec::new(),
        }
    }

    pub fn mutate(mut self, mutation_batch: MutationBatch<MutationData>) -> Self {
        self.mutation_batch = Some(mutation_batch);
        self
    }

    pub fn profile(mut self, profile: ExecutionProfile) -> Self {
        self.profiles.push(profile);
        self
    }

    pub fn profiles<I>(mut self, profiles: I) -> Self
    where
        I: IntoIterator<Item = ExecutionProfile>,
    {
        self.profiles.extend(profiles);
        self
    }

    pub fn against(self, profile: ExecutionProfile) -> Self {
        self.profile(profile)
    }
}

impl<A, FixtureData, MutationData, TargetId> RunMatrix<A, FixtureData, MutationData, TargetId>
where
    A: HarnessAdapter<Fixture = FixtureData, Mutation = MutationData, TargetId = TargetId>,
    TargetId: PartialEq,
{
    pub fn execute(self) -> Result<Vec<HarnessCoreBundle<TargetId>>, HarnessError<A::Error>> {
        let Self {
            runner,
            fixture,
            mutation_batch,
            request,
            profiles,
        } = self;
        profiles
            .iter()
            .map(|profile| {
                runner.execute_core(&fixture, mutation_batch.as_ref(), &request, profile)
            })
            .collect()
    }
}

pub fn run_matrix<A, FixtureData, MutationData, TargetId>(
    adapter: A,
    fixture: ScenarioFixture<FixtureData>,
    request: ExecutionRequest<TargetId>,
) -> RunMatrix<A, FixtureData, MutationData, TargetId> {
    RunMatrix::new(adapter, fixture, request)
}

impl<A, FixtureData, MutationData, TargetId> RunMatrix<A, FixtureData, MutationData, TargetId>
where
    A: HarnessAdapter<Fixture = FixtureData, Mutation = MutationData, TargetId = TargetId>
        + DiagnosticsHarnessAdapter
        + ExplanationHarnessAdapter
        + ProvenanceHarnessAdapter,
    TargetId: PartialEq,
{
    pub fn observe(self) -> Result<Vec<HarnessObservedBundle<TargetId>>, HarnessError<A::Error>> {
        let Self {
            runner,
            fixture,
            mutation_batch,
            request,
            profiles,
        } = self;
        profiles
            .iter()
            .map(|profile| {
                runner.execute_observed(&fixture, mutation_batch.as_ref(), &request, profile)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::capture::DiagnosticsLevel;
    use crate::runtime::{CaptureDepth, HarnessCapabilities};
    use crate::scenario::{ExecutionProfile, ExecutionRequest, ScenarioPlan};
    use crate::tooling::AdapterDouble;

    use super::run_matrix;

    #[test]
    fn run_matrix_executes_multiple_profiles() {
        let mut capabilities = HarnessCapabilities::default();
        capabilities
            .diagnostics_levels
            .insert(DiagnosticsLevel::Operational);
        capabilities.capture_depths.insert(CaptureDepth::Standard);

        let fixture = ScenarioPlan::new("fixture", json!({ "fixture": true })).compile();
        let request = ExecutionRequest::target("request", "target".to_string());
        let bundles = run_matrix(AdapterDouble::new("double", capabilities), fixture, request)
            .profiles([
                ExecutionProfile::serial("serial"),
                ExecutionProfile::operational("operational"),
            ])
            .execute()
            .unwrap();

        assert_eq!(bundles.len(), 2);
        assert_eq!(bundles[0].run.profile_name, "serial");
        assert_eq!(bundles[1].run.profile_name, "operational");
    }
}
