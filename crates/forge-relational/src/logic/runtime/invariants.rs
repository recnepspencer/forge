use super::{
    ComplexityContract, InvariantCheckResult, InvariantClass, InvariantExecutionPoint,
    RelationalRuntime, StorageInvariantReport, COMPLEXITY_CONTRACTS,
};

impl RelationalRuntime {
    pub fn complexity_contracts(&self) -> &'static [ComplexityContract] {
        COMPLEXITY_CONTRACTS
    }

    pub fn invariants(&self, class: InvariantClass) -> StorageInvariantReport {
        StorageInvariantReport {
            violations: self
                .run_invariants_for_state(
                    &self.current_state(),
                    self.current_version_id(),
                    InvariantExecutionPoint::MutationSensitive,
                    false,
                    None,
                )
                .into_iter()
                .filter(|result| result.class == class)
                .flat_map(|result| result.violations)
                .collect(),
        }
    }

    pub fn run_invariants(
        &self,
        execution_point: InvariantExecutionPoint,
        include_harness_heavy: bool,
    ) -> Vec<InvariantCheckResult> {
        self.run_invariants_for_state(
            &self.current_state(),
            self.current_version_id(),
            execution_point,
            include_harness_heavy,
            None,
        )
    }

    #[cfg(test)]
    pub(crate) fn remove_commit_envelope_for_test(
        &mut self,
        commit_id: crate::history::data::CommitId,
    ) -> bool {
        let Some(envelope) = self.history.commit_envelopes.remove(&commit_id) else {
            return false;
        };
        self.history
            .patch_stream_index
            .remove(&envelope.patch.position);
        true
    }

    #[cfg(test)]
    pub(crate) fn tamper_commit_patch_for_test(
        &mut self,
        commit_id: crate::history::data::CommitId,
        mutate: impl FnOnce(&mut crate::publication::data::diff::RelationalPatchRecord),
    ) -> bool {
        let Some(envelope) = self.history.commit_envelopes.get_mut(&commit_id) else {
            return false;
        };
        mutate(&mut envelope.patch);
        true
    }
}
