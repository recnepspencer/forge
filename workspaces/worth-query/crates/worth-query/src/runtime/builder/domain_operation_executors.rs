use super::WorthQueryRuntimeBuilder;

impl WorthQueryRuntimeBuilder {
    pub fn domain_operation_executor<D: 'static, O, F: 'static, E>(
        mut self,
        _domain: D,
        _operation: O,
        _family: F,
        executor: E,
    ) -> Self
    where
        O: crate::domain_installation::WorthQueryExecutableDomainOperation<
            D,
            F,
            Execution = crate::domain_installation::WorthQueryDirectOperation,
        >,
        E: crate::domain_installation::WorthQueryDomainOperationExecutor<D, O, F>,
    {
        self.pending_domain_operation_executors = self
            .pending_domain_operation_executors
            .register::<D, O, F, E>(executor);
        self
    }
}
