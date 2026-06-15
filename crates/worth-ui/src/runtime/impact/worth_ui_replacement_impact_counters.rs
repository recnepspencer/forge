#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiReplacementImpactCounters {
    artifact_comparisons_consumed: usize,
    impact_classifications_attempted: usize,
    dependency_metadata_reads: usize,
    impact_metadata_lookups: usize,
    broad_replacement_denials: usize,
    plan_lowering_attempts: usize,
}

impl WorthUiReplacementImpactCounters {
    pub(crate) fn record_artifact_comparison_consumed(&mut self) {
        self.artifact_comparisons_consumed += 1;
    }

    pub(crate) fn record_impact_classification_attempted(&mut self) {
        self.impact_classifications_attempted += 1;
    }

    pub(crate) fn record_dependency_metadata_read(&mut self) {
        self.dependency_metadata_reads += 1;
    }

    pub(crate) fn record_impact_metadata_lookups(&mut self, lookup_count: usize) {
        self.impact_metadata_lookups += lookup_count;
    }

    pub(crate) fn record_broad_replacement_denial(&mut self) {
        self.broad_replacement_denials += 1;
    }

    pub fn artifact_comparisons_consumed(self) -> usize {
        self.artifact_comparisons_consumed
    }

    pub fn impact_classifications_attempted(self) -> usize {
        self.impact_classifications_attempted
    }

    pub fn dependency_metadata_reads(self) -> usize {
        self.dependency_metadata_reads
    }

    pub fn impact_metadata_lookups(self) -> usize {
        self.impact_metadata_lookups
    }

    pub fn broad_replacement_denials(self) -> usize {
        self.broad_replacement_denials
    }

    pub fn plan_lowering_attempts(self) -> usize {
        self.plan_lowering_attempts
    }
}
