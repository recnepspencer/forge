use serde_json::Value;

use crate::comparison::{ComparisonProfile, ComparisonRecord};
use crate::runtime::{DiagnosticsHarnessAdapter, HarnessAdapter, HarnessError, HarnessRunner};
use crate::scenario::{ExecutionProfile, ExecutionRequest, MutationBatch, ScenarioFixture};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CertificationMatrixError<AdapterError> {
    Runner(HarnessError<AdapterError>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CertificationMatrixCase {
    pub baseline_profile: String,
    pub candidate_profile: String,
    pub comparison: ComparisonRecord,
    pub diagnostics_summary: Option<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CertificationMatrixReport {
    pub baseline_profile: String,
    pub baseline_diagnostics_summary: Option<Value>,
    pub matched: bool,
    pub cases: Vec<CertificationMatrixCase>,
}

pub struct CertificationMatrix<A, FixtureData, MutationData, TargetId> {
    runner: HarnessRunner<A>,
    fixture: ScenarioFixture<FixtureData>,
    mutation_batch: Option<MutationBatch<MutationData>>,
    request: ExecutionRequest<TargetId>,
    baseline_profile: ExecutionProfile,
    candidate_profiles: Vec<ExecutionProfile>,
    comparison_profile: ComparisonProfile,
}

impl<A, FixtureData, MutationData, TargetId>
    CertificationMatrix<A, FixtureData, MutationData, TargetId>
{
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

impl<A, FixtureData, MutationData, TargetId>
    CertificationMatrix<A, FixtureData, MutationData, TargetId>
where
    A: HarnessAdapter<Fixture = FixtureData, Mutation = MutationData, TargetId = TargetId>
        + DiagnosticsHarnessAdapter,
    TargetId: std::fmt::Debug + PartialEq,
{
    pub fn certify(self) -> Result<CertificationMatrixReport, CertificationMatrixError<A::Error>> {
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
            .execute_diagnosed(
                &fixture,
                mutation_batch.as_ref(),
                &request,
                &baseline_profile,
            )
            .map_err(CertificationMatrixError::Runner)?;

        let mut cases = Vec::new();
        for candidate in candidate_profiles {
            let candidate_bundle = runner
                .execute_diagnosed(&fixture, mutation_batch.as_ref(), &request, &candidate)
                .map_err(CertificationMatrixError::Runner)?;
            let comparison = runner
                .compare_runs(
                    &baseline_bundle.core.run,
                    &candidate_bundle.core.run,
                    &comparison_profile,
                )
                .map_err(CertificationMatrixError::Runner)?;
            cases.push(CertificationMatrixCase {
                baseline_profile: baseline_profile.name.clone(),
                candidate_profile: candidate.name,
                comparison,
                diagnostics_summary: candidate_bundle
                    .diagnostics
                    .map(|diagnostics| diagnostics.summary),
            });
        }

        Ok(CertificationMatrixReport {
            baseline_profile: baseline_profile.name,
            baseline_diagnostics_summary: baseline_bundle
                .diagnostics
                .map(|diagnostics| diagnostics.summary),
            matched: cases.iter().all(|case| case.comparison.matched),
            cases,
        })
    }
}

pub fn certification_matrix<A, FixtureData, MutationData, TargetId>(
    adapter: A,
    fixture: ScenarioFixture<FixtureData>,
    request: ExecutionRequest<TargetId>,
    baseline_profile: ExecutionProfile,
) -> CertificationMatrix<A, FixtureData, MutationData, TargetId> {
    CertificationMatrix::new(adapter, fixture, request, baseline_profile)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::capture::DiagnosticsLevel;
    use crate::runtime::{CaptureDepth, HarnessCapabilities};
    use crate::scenario::{ExecutionProfile, ExecutionRequest, ScenarioPlan};
    use crate::tooling::{certification_matrix, AdapterDouble};

    #[test]
    fn certification_matrix_compares_profiles_and_captures_diagnostics() {
        let mut capabilities = HarnessCapabilities::default();
        capabilities
            .diagnostics_levels
            .insert(DiagnosticsLevel::Operational);
        capabilities.capture_depths.insert(CaptureDepth::Standard);

        let fixture = ScenarioPlan::new("fixture", json!({ "fixture": true })).compile();
        let request = ExecutionRequest::target("request", "target".to_string());

        let report = certification_matrix(
            AdapterDouble::new("double", capabilities),
            fixture,
            request,
            ExecutionProfile::serial("baseline"),
        )
        .candidates([
            ExecutionProfile::serial("candidate-a"),
            ExecutionProfile::operational("candidate-b"),
        ])
        .certify()
        .unwrap();

        assert!(report.matched);
        assert_eq!(report.baseline_profile, "baseline");
        assert_eq!(report.cases.len(), 2);
        assert!(report.baseline_diagnostics_summary.is_some());
        assert!(report
            .cases
            .iter()
            .all(|case| case.diagnostics_summary.is_some()));
    }
}
