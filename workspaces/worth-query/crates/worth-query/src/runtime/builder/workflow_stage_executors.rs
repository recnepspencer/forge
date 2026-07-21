use super::WorthQueryRuntimeBuilder;

impl WorthQueryRuntimeBuilder {
    pub fn workflow_stage_executor<D: 'static, O, F: 'static, E>(
        mut self,
        _domain: D,
        _operation: O,
        _family: F,
        executor: E,
    ) -> Self
    where
        O: 'static
            + crate::domain_installation::WorthQueryExecutableDomainOperation<
                D,
                F,
                Execution = crate::domain_installation::WorthQueryWorkflowOperation,
            >,
        E: crate::domain_installation::WorthQueryDomainWorkflowStageExecutor<D, O, F>,
    {
        self.pending_workflow_stage_executors = self
            .pending_workflow_stage_executors
            .register::<D, O, F, E>(executor);
        self
    }

    pub fn replayable_workflow_stage_executor<D: 'static, O, F: 'static, E>(
        mut self,
        _domain: D,
        _operation: O,
        _family: F,
        executor: E,
    ) -> Self
    where
        O: 'static
            + crate::domain_installation::WorthQueryExecutableDomainOperation<
                D,
                F,
                Execution = crate::domain_installation::WorthQueryWorkflowOperation,
            >,
        E: crate::domain_installation::WorthQueryDomainWorkflowStageExecutor<D, O, F>
            + crate::domain_installation::WorthQueryDomainReplaySemanticComparator<D, O, F>,
    {
        self.pending_workflow_stage_executors = self
            .pending_workflow_stage_executors
            .register_replayable::<D, O, F, E>(executor);
        self
    }
}
