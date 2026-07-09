use crate::comparison::{ComparisonProfile, ComparisonRecord};
use crate::runtime::{HarnessAdapter, HarnessError, HarnessRunner};
use crate::scenario::{ExecutionProfile, ExecutionRequest, MutationBatch, ScenarioFixture};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParityError<AdapterError> {
    Runner(HarnessError<AdapterError>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParityResult {
    pub baseline_profile: String,
    pub candidate_profile: String,
    pub comparison: ComparisonRecord,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParityReport {
    pub matched: bool,
    pub results: Vec<ParityResult>,
}

pub struct ParitySuite<A, FixtureData, MutationData, TargetId> {
    runner: HarnessRunner<A>,
    fixture: ScenarioFixture<FixtureData>,
    mutation_batch: Option<MutationBatch<MutationData>>,
    request: ExecutionRequest<TargetId>,
    baseline_profile: ExecutionProfile,
    candidate_profiles: Vec<ExecutionProfile>,
    comparison_profile: ComparisonProfile,
}

impl<A, FixtureData, MutationData, TargetId> ParitySuite<A, FixtureData, MutationData, TargetId> {
    pub fn new(
        adapter: A,
        fixture: ScenarioFixture<FixtureData>,
        request: ExecutionRequest<TargetId>,
        baseline_profile: ExecutionProfile,
    ) -> Self {
        Self {
            runner: HarnessRunner::new(adapter),
            fixture,
            mutation_batch: None,
            request,
            baseline_profile,
            candidate_profiles: Vec::new(),
            comparison_profile: ComparisonProfile::default(),
        }
    }

    pub fn mutate(mut self, mutation_batch: MutationBatch<MutationData>) -> Self {
        self.mutation_batch = Some(mutation_batch);
        self
    }

    pub fn candidate(mut self, profile: ExecutionProfile) -> Self {
        self.candidate_profiles.push(profile);
        self
    }

    pub fn candidates<I>(mut self, profiles: I) -> Self
    where
        I: IntoIterator<Item = ExecutionProfile>,
    {
        self.candidate_profiles.extend(profiles);
        self
    }

    pub fn comparison_profile(mut self, comparison_profile: ComparisonProfile) -> Self {
        self.comparison_profile = comparison_profile;
        self
    }
}

impl<A, FixtureData, MutationData, TargetId> ParitySuite<A, FixtureData, MutationData, TargetId>
where
    A: HarnessAdapter<Fixture = FixtureData, Mutation = MutationData, TargetId = TargetId>,
    TargetId: std::fmt::Debug + PartialEq,
{
    pub fn compare(self) -> Result<ParityReport, ParityError<A::Error>> {
        let Self {
            runner,
            fixture,
            mutation_batch,
            request,
            baseline_profile,
            candidate_profiles,
            comparison_profile,
        } = self;
        let baseline_bundle = runner
            .execute_core(
                &fixture,
                mutation_batch.as_ref(),
                &request,
                &baseline_profile,
            )
            .map_err(ParityError::Runner)?;

        let mut results = Vec::new();
        for candidate in candidate_profiles {
            let candidate_bundle = runner
                .execute_core(&fixture, mutation_batch.as_ref(), &request, &candidate)
                .map_err(ParityError::Runner)?;
            let comparison = runner
                .compare_runs(
                    &baseline_bundle.run,
                    &candidate_bundle.run,
                    &comparison_profile,
                )
                .map_err(ParityError::Runner)?;
            results.push(ParityResult {
                baseline_profile: baseline_profile.name.clone(),
                candidate_profile: candidate.name,
                comparison,
            });
        }

        Ok(ParityReport {
            matched: results.iter().all(|result| result.comparison.matched),
            results,
        })
    }
}

pub fn parity_suite<A, FixtureData, MutationData, TargetId>(
    adapter: A,
    fixture: ScenarioFixture<FixtureData>,
    request: ExecutionRequest<TargetId>,
    baseline_profile: ExecutionProfile,
) -> ParitySuite<A, FixtureData, MutationData, TargetId> {
    ParitySuite::new(adapter, fixture, request, baseline_profile)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::capture::DiagnosticsLevel;
    use crate::runtime::{CaptureDepth, HarnessCapabilities};
    use crate::scenario::{ExecutionProfile, ExecutionRequest, ScenarioPlan};
    use crate::tooling::AdapterDouble;

    use super::parity_suite;

    #[test]
    fn parity_suite_compares_candidate_profiles_against_baseline() {
        let mut capabilities = HarnessCapabilities::default();
        capabilities
            .diagnostics_levels
            .insert(DiagnosticsLevel::Operational);
        capabilities.capture_depths.insert(CaptureDepth::Standard);

        let fixture = ScenarioPlan::new("fixture", json!({ "fixture": true })).compile();
        let request = ExecutionRequest::target("request", "target".to_string());

        let report = parity_suite(
            AdapterDouble::new("double", capabilities),
            fixture,
            request,
            ExecutionProfile::serial("baseline"),
        )
        .candidates([ExecutionProfile::serial("candidate")])
        .compare()
        .unwrap();

        assert!(report.matched);
        assert_eq!(report.results.len(), 1);
    }
}
