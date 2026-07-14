use crate::runtime::{
    DiagnosticsHarnessAdapter, ExplanationHarnessAdapter, HarnessAdapter, HarnessCoreBundle,
    HarnessDiagnosedBundle, HarnessError, HarnessObservedBundle, HarnessRunner,
    ProvenanceHarnessAdapter,
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

impl<A, FixtureData, MutationData, TargetId> RunMatrix<A, FixtureData, MutationData, TargetId>
where
    A: HarnessAdapter<Fixture = FixtureData, Mutation = MutationData, TargetId = TargetId>
        + DiagnosticsHarnessAdapter,
    TargetId: PartialEq,
{
    pub fn diagnose(self) -> Result<Vec<HarnessDiagnosedBundle<TargetId>>, HarnessError<A::Error>> {
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
                runner.execute_diagnosed(&fixture, mutation_batch.as_ref(), &request, profile)
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
    use crate::capture::RecordSchemaVersion;
    use crate::identity::{diagnostics_id, run_id, scenario_id};
    use crate::runtime::{
        CaptureDepth, DiagnosticsHarnessAdapter, HarnessAdapter, HarnessCapabilities,
    };
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

    #[test]
    fn run_matrix_can_capture_diagnostics_without_full_observed_bundle_support() {
        struct DiagnosticsDouble(AdapterDouble);

        impl HarnessAdapter for DiagnosticsDouble {
            type Runtime = <AdapterDouble as HarnessAdapter>::Runtime;
            type Fixture = <AdapterDouble as HarnessAdapter>::Fixture;
            type Mutation = <AdapterDouble as HarnessAdapter>::Mutation;
            type TargetId = <AdapterDouble as HarnessAdapter>::TargetId;
            type Error = <AdapterDouble as HarnessAdapter>::Error;

            fn adapter_name(&self) -> &'static str {
                self.0.adapter_name()
            }

            fn capabilities(&self) -> HarnessCapabilities {
                self.0.capabilities()
            }

            fn create_runtime(&self) -> Result<Self::Runtime, Self::Error> {
                self.0.create_runtime()
            }

            fn prepare_runtime(
                &self,
                runtime: &mut Self::Runtime,
                profile: &ExecutionProfile,
            ) -> Result<(), Self::Error> {
                self.0.prepare_runtime(runtime, profile)
            }

            fn load_fixture(
                &self,
                runtime: &mut Self::Runtime,
                fixture: &crate::scenario::ScenarioFixture<Self::Fixture>,
            ) -> Result<(), Self::Error> {
                self.0.load_fixture(runtime, fixture)
            }

            fn apply_mutation_batch(
                &self,
                runtime: &mut Self::Runtime,
                batch: &crate::scenario::MutationBatch<Self::Mutation>,
            ) -> Result<(), Self::Error> {
                self.0.apply_mutation_batch(runtime, batch)
            }

            fn execute(
                &self,
                runtime: &mut Self::Runtime,
                fixture: &crate::scenario::ScenarioFixture<Self::Fixture>,
                request: &crate::scenario::ExecutionRequest<Self::TargetId>,
                profile: &ExecutionProfile,
            ) -> Result<crate::capture::RunRecord<Self::TargetId>, Self::Error> {
                self.0.execute(runtime, fixture, request, profile)
            }

            fn capture_snapshot(
                &self,
                runtime: &Self::Runtime,
                fixture: &crate::scenario::ScenarioFixture<Self::Fixture>,
                request: &crate::scenario::ExecutionRequest<Self::TargetId>,
                profile: &ExecutionProfile,
            ) -> Result<crate::capture::SnapshotRecord<Self::TargetId>, Self::Error> {
                self.0.capture_snapshot(runtime, fixture, request, profile)
            }
        }

        impl DiagnosticsHarnessAdapter for DiagnosticsDouble {
            fn capture_diagnostics(
                &self,
                _runtime: &Self::Runtime,
                fixture: &crate::scenario::ScenarioFixture<Self::Fixture>,
                profile: &ExecutionProfile,
            ) -> Result<crate::capture::DiagnosticsRecord, Self::Error> {
                let scenario = scenario_id(&fixture.name);
                let run_id = run_id(&scenario, &profile.name, "request");
                Ok(crate::capture::DiagnosticsRecord {
                    schema_version: RecordSchemaVersion::V1,
                    diagnostics_id: diagnostics_id(&run_id),
                    run_id,
                    adapter_name: self.adapter_name().to_string(),
                    profile_name: profile.name.clone(),
                    level: profile.diagnostics_level,
                    time_marker: profile.time_marker.clone(),
                    summary: json!({ "diagnosed": true }),
                    attachments: Vec::new(),
                    extensions: Default::default(),
                })
            }
        }

        let mut capabilities = HarnessCapabilities::default();
        capabilities
            .diagnostics_levels
            .insert(DiagnosticsLevel::Operational);
        capabilities.capture_depths.insert(CaptureDepth::Standard);

        let fixture = ScenarioPlan::new("fixture", json!({ "fixture": true })).compile();
        let request = ExecutionRequest::target("request", "target".to_string());
        let bundles = run_matrix(
            DiagnosticsDouble(AdapterDouble::new("double", capabilities)),
            fixture,
            request,
        )
        .profiles([
            ExecutionProfile::serial("serial"),
            ExecutionProfile::operational("operational"),
        ])
        .diagnose()
        .unwrap();

        assert_eq!(bundles.len(), 2);
        assert_eq!(bundles[0].core.run.profile_name, "serial");
        assert_eq!(bundles[1].core.run.profile_name, "operational");
        assert_eq!(
            bundles[0].diagnostics.as_ref().unwrap().summary["diagnosed"],
            json!(true)
        );
    }
}
