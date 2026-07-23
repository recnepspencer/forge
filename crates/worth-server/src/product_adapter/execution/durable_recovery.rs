use crate::{
    WorthServerAdmission, WorthServerDurableProductMutationConclusion,
    WorthServerDurableProductMutationRecoveryHandle, WorthServerProductOperationExecutionBoundary,
    WorthServerProductOperationSurfaceDenial, WorthServerProductOperationSurfaceDenialCode,
    WorthServerProductOperationSurfaceDenialFacts,
};

use super::WorthServerProductOperationRuntime;

impl WorthServerProductOperationRuntime {
    pub fn resolve_durable_mutation(
        &self,
        admission: &WorthServerAdmission,
        recovery: &WorthServerDurableProductMutationRecoveryHandle,
    ) -> Result<WorthServerDurableProductMutationConclusion, WorthServerProductOperationSurfaceDenial>
    {
        let (_, declaration) = self.resolve_declaration(recovery.operation_name())?;
        let Some(contract) = declaration.durable_mutation_contract() else {
            return Err(WorthServerProductOperationSurfaceDenial::new(
                WorthServerProductOperationSurfaceDenialCode::InvalidDeclaration,
                format!(
                    "product operation `{}` does not declare durable recovery",
                    recovery.operation_name()
                ),
            ));
        };
        crate::durable_product_mutation::admit_durable_product_recovery(
            &self.operation_registry,
            admission,
            declaration,
        )?;
        let workspace_target = admission.request_context().workspace_target();
        if recovery.tenant_id() != workspace_target.tenant_id()
            || recovery.workspace_id() != workspace_target.workspace_id()
            || recovery.authority_scope() != contract.authority_scope()
        {
            return Err(WorthServerProductOperationSurfaceDenial::new(
                WorthServerProductOperationSurfaceDenialCode::RequestDenied,
                "durable product recovery handle is outside the admitted product scope".to_string(),
            ));
        }
        let executor = self
            .adapter_registry
            .resolve_durable_executor(recovery.operation_name())
            .ok_or_else(|| {
                WorthServerProductOperationSurfaceDenial::new(
                    WorthServerProductOperationSurfaceDenialCode::InvalidDeclaration,
                    "validated durable product declaration lost its installed executor".to_string(),
                )
            })?;
        self.counters.increment_durable_product_recovery_attempts();
        let conclusion = executor.resolve(recovery);
        match &conclusion {
            WorthServerDurableProductMutationConclusion::Committed(completion)
            | WorthServerDurableProductMutationConclusion::PreviouslyCommitted(completion) => {
                if !completion.matches_recovery(recovery, declaration.result_contract()) {
                    self.counters.increment_durable_product_recovery_failed();
                    return Err(WorthServerProductOperationSurfaceDenial::new(
                        WorthServerProductOperationSurfaceDenialCode::InvalidDurableCompletion,
                        "durable product recovery returned a completion outside the admitted recovery authority"
                            .to_string(),
                    )
                    .with_facts(
                        WorthServerProductOperationSurfaceDenialFacts::default()
                            .with_execution_boundary(
                                WorthServerProductOperationExecutionBoundary::DurableExecutorAttempted,
                            ),
                    ));
                }
                self.counters.increment_durable_product_recovery_resolved();
                self.counters.record_product_result_artifact(
                    completion.success().result_artifact().body().byte_len(),
                );
            }
            _ => self.counters.increment_durable_product_recovery_failed(),
        }
        Ok(conclusion)
    }
}
