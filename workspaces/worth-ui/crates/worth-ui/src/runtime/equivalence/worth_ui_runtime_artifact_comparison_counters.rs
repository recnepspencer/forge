#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiRuntimeArtifactComparisonCounters {
    artifact_comparisons: usize,
    impact_narrowing_attempts: usize,
    plan_lowering_attempts: usize,
}

impl WorthUiRuntimeArtifactComparisonCounters {
    pub(crate) fn record_artifact_comparison(&mut self) {
        self.artifact_comparisons += 1;
    }

    pub fn artifact_comparisons(self) -> usize {
        self.artifact_comparisons
    }

    pub fn impact_narrowing_attempts(self) -> usize {
        self.impact_narrowing_attempts
    }

    pub fn plan_lowering_attempts(self) -> usize {
        self.plan_lowering_attempts
    }
}
