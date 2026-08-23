use worth_query::facade::domain;

use super::installed_operation::{
    WorthUiPresentationAsyncDomainEntry, WorthUiPresentationAsyncOperation,
    WorthUiPresentationAsyncOperationFamily,
};

pub(super) struct WorthUiPresentationConditionalCompute {
    output_version: u64,
}

impl WorthUiPresentationConditionalCompute {
    pub(super) const fn new(output_version: u64) -> Self {
        Self { output_version }
    }
}

impl
    domain::WorthQueryConditionalNodeComputeProvider<
        WorthUiPresentationAsyncDomainEntry,
        WorthUiPresentationAsyncOperation,
        WorthUiPresentationAsyncOperationFamily,
    > for WorthUiPresentationConditionalCompute
{
    type SemanticContract = &'static str;

    fn semantic_contract(&self) -> Self::SemanticContract {
        "worth-ui-presentation-currentness-v1"
    }

    fn execution_resource_support(&self) -> domain::WorthQueryExecutionResourceSupport {
        crate::installed_domain::execution_resources::operation_execution_resource_support()
    }

    fn compute(
        &self,
        _context: &domain::WorthQueryConditionalComputeContext,
    ) -> Result<worth_signal::facade::NodeEvaluationResult, String> {
        Ok(worth_signal::facade::NodeEvaluationResult::from_version(
            worth_signal::facade::AspectVersion::from_updates([(
                worth_signal::facade::Aspect::new(0),
                self.output_version,
            )]),
        ))
    }
}
