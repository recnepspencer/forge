use super::*;

pub(super) type CapturedContext = (String, String, String, Option<String>, String, u64);

pub(super) struct CapturingCompute(pub(super) Arc<Mutex<Option<CapturedContext>>>);

impl domain::WorthQueryConditionalNodeComputeProvider<GeometryDomain, ReadVertex, ReadFamily>
    for CapturingCompute
{
    type SemanticContract = ();

    fn semantic_contract(&self) -> Self::SemanticContract {}

    fn execution_resource_support(&self) -> domain::WorthQueryExecutionResourceSupport {
        crate::suite::installed_operation_fixture::execution_resource_support()
    }

    fn compute(
        &self,
        context: &domain::WorthQueryConditionalComputeContext,
    ) -> Result<worth_signal::facade::NodeEvaluationResult, String> {
        *self.0.lock().unwrap() = Some((
            context.operation_identity().to_string(),
            context.binding_identity().to_string(),
            context.basis_identity().to_string(),
            context.workflow_run_identity().map(str::to_string),
            context.snapshot_identity().to_string(),
            context.attempt(),
        ));
        Ok(worth_signal::facade::NodeEvaluationResult::from_version(
            worth_signal::facade::AspectVersion::from_updates([(
                worth_signal::facade::Aspect::new(0),
                1,
            )]),
        ))
    }
}
