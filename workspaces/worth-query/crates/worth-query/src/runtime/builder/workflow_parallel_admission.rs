use super::WorthQueryRuntimeBuilder;

impl WorthQueryRuntimeBuilder {
    pub fn workflow_parallel_admission_provider<D: 'static, O: 'static, F: 'static, P>(
        mut self,
        _domain: D,
        _operation: O,
        _family: F,
        provider: P,
    ) -> Self
    where
        P: crate::domain_installation::WorthQueryWorkflowParallelAdmissionProvider<D, O, F>,
    {
        self.pending_workflow_parallel_admission_providers = self
            .pending_workflow_parallel_admission_providers
            .register::<D, O, F, P>(provider);
        self
    }
}
