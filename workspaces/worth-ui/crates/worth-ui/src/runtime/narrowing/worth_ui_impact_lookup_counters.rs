#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiImpactLookupCounters {
    impact_classifications_consumed: usize,
    dependency_metadata_reads: usize,
    module_impact_lookups: usize,
    subtree_impact_lookups: usize,
    runtime_hook_lookups: usize,
    subtree_digest_lookups: usize,
    full_artifact_scans: usize,
    plan_lowering_attempts: usize,
}

impl WorthUiImpactLookupCounters {
    pub(crate) fn record_impact_classification_consumed(&mut self) {
        self.impact_classifications_consumed += 1;
    }

    pub(crate) fn record_dependency_metadata_read(&mut self) {
        self.dependency_metadata_reads += 1;
    }

    pub(crate) fn record_module_impact_lookup(&mut self) {
        self.module_impact_lookups += 1;
    }

    pub(crate) fn record_subtree_impact_lookup(&mut self) {
        self.subtree_impact_lookups += 1;
    }

    pub(crate) fn record_runtime_hook_lookup(&mut self) {
        self.runtime_hook_lookups += 1;
    }

    pub(crate) fn record_subtree_digest_lookup(&mut self) {
        self.subtree_digest_lookups += 1;
    }

    #[cfg(test)]
    pub(crate) fn record_full_artifact_scan_for_test(&mut self) {
        self.full_artifact_scans += 1;
    }

    pub fn impact_classifications_consumed(self) -> usize {
        self.impact_classifications_consumed
    }

    pub fn dependency_metadata_reads(self) -> usize {
        self.dependency_metadata_reads
    }

    pub fn module_impact_lookups(self) -> usize {
        self.module_impact_lookups
    }

    pub fn subtree_impact_lookups(self) -> usize {
        self.subtree_impact_lookups
    }

    pub fn runtime_hook_lookups(self) -> usize {
        self.runtime_hook_lookups
    }

    pub fn subtree_digest_lookups(self) -> usize {
        self.subtree_digest_lookups
    }

    pub fn full_artifact_scans(self) -> usize {
        self.full_artifact_scans
    }

    pub fn plan_lowering_attempts(self) -> usize {
        self.plan_lowering_attempts
    }
}
