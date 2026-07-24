use worth_query::facade::domain;

use super::provider::ArtifactProbe;
use crate::suite::installed_operation_fixture::{GeometryDomain, ReadFamily, WorkflowRead};

#[derive(Clone)]
pub struct ArtifactWorkflowExecutor {
    probe: ArtifactProbe,
}

impl ArtifactWorkflowExecutor {
    pub fn new(probe: ArtifactProbe) -> Self {
        Self { probe }
    }

    fn execute_producer(
        &self,
        input: domain::WorthQueryWorkflowValue,
        context: &domain::WorthQueryWorkflowStageExecutionContext<'_>,
        workspace: &mut domain::WorthQueryWorkflowStageWorkspace<'_>,
    ) -> Result<
        domain::WorthQueryWorkflowStageMaterial,
        domain::WorthQueryWorkflowStageExecutorFailure,
    > {
        let mode = match input {
            domain::WorthQueryWorkflowValue::Text(mode) => mode,
            _ => return Err(failure("producer expected a text scenario")),
        };
        let admission = production_admission(context)?;
        if mode == "panic-during-projection" {
            let _ = workspace.register_artifact(admission, self.probe.panic_during_projection());
            unreachable!("provider projection panic must unwind registration");
        }
        if mode == "reject-provider" {
            let denial = workspace
                .register_artifact(admission, self.probe.foreign(b"foreign"))
                .expect_err("foreign provider must be rejected");
            self.probe.observe_denial(denial.kind());
            return Err(failure("foreign provider rejected"));
        }
        let handle = workspace
            .register_artifact(admission, self.probe.candidate(b"canonical-candidates"))
            .map_err(artifact_failure)?;
        if mode == "cancel" {
            let disposed = handle.cancel();
            if disposed.disposition() == domain::WorthQueryArtifactDisposition::Cancelled
                && disposed.provider_disposed()
            {
                self.probe.observe_cancellation();
            }
            return Err(failure("artifact cancelled by producer"));
        }
        if mode == "panic-after-production" {
            panic!("declared artifact handoff panic");
        }
        if mode == "fail-after-production" {
            return Err(failure("declared failure after artifact production"));
        }
        let handle = if mode == "replace" {
            let replacement = workspace
                .replace_artifact(
                    handle,
                    production_admission(context)?,
                    self.probe.candidate(b"canonical-candidates"),
                )
                .map_err(|stop| artifact_failure(stop.into_parts().0))?;
            if replacement.prior().disposition() == domain::WorthQueryArtifactDisposition::Replaced
                && replacement.prior().provider_disposed()
            {
                self.probe.observe_replacement();
            }
            replacement.into_replacement()
        } else {
            handle
        };
        Ok(domain::WorthQueryWorkflowStageMaterial::new(
            domain::WorthQueryWorkflowValue::installed_artifact(handle),
        ))
    }

    fn execute_consumer(
        &self,
        input: domain::WorthQueryWorkflowValue,
        context: &domain::WorthQueryWorkflowStageExecutionContext<'_>,
    ) -> Result<
        domain::WorthQueryWorkflowStageMaterial,
        domain::WorthQueryWorkflowStageExecutorFailure,
    > {
        let transferred = input
            .into_transferred_artifact()
            .map_err(|_| failure("consumer expected a transferred artifact"))?;
        let view = transferred
            .borrow(format!("{}-projection", context.stage().identity()))
            .map_err(artifact_failure)?;
        if view.semantic_projection().bytes() != b"canonical-candidates" {
            return Err(failure("consumer observed a non-canonical projection"));
        }
        self.probe.observe_borrow();
        drop(view);
        drop(transferred);
        Ok(
            domain::WorthQueryWorkflowStageMaterial::new(domain::WorthQueryWorkflowValue::Text(
                context.stage().identity().to_owned(),
            ))
            .with_result_state(domain::WorthQueryOperationResultState::Ready),
        )
    }
}

impl domain::WorthQueryDomainWorkflowStageExecutor<GeometryDomain, WorkflowRead, ReadFamily>
    for ArtifactWorkflowExecutor
{
    const LOWERING_FAMILY: &'static str = "artifact-workflow-v1";
    const DETERMINISTIC: bool = true;
    const IDEMPOTENT_STAGE_RETRY: bool = true;
    const EXECUTION_COST: domain::WorthQueryOperationCostClass =
        domain::WorthQueryOperationCostClass::DeclaredWidth;
    const RESULT_WIDTH_COST: domain::WorthQueryOperationCostClass =
        domain::WorthQueryOperationCostClass::DeclaredWidth;
    const REPLAY_COMPARATOR_FAMILY: Option<&'static str> = Some("artifact-workflow-exact-v1");

    fn execute_stage(
        &self,
        input: domain::WorthQueryWorkflowValue,
        context: &domain::WorthQueryWorkflowStageExecutionContext<'_>,
        workspace: &mut domain::WorthQueryWorkflowStageWorkspace<'_>,
    ) -> Result<
        domain::WorthQueryWorkflowStageMaterial,
        domain::WorthQueryWorkflowStageExecutorFailure,
    > {
        match context.stage().identity() {
            "produce" => self.execute_producer(input, context, workspace),
            "consume" | "observe-a" | "observe-b" => self.execute_consumer(input, context),
            _ => Err(failure("unknown artifact workflow stage")),
        }
    }
}

impl domain::WorthQueryDomainReplaySemanticComparator<GeometryDomain, WorkflowRead, ReadFamily>
    for ArtifactWorkflowExecutor
{
    fn compare_replay_semantics(
        &self,
        original: &domain::WorthQueryWorkflowTraceSemantics,
        replay: &domain::WorthQueryWorkflowTraceSemantics,
        noise: domain::WorthQueryReplayNoiseContract,
    ) -> domain::WorthQueryReplayComparison {
        domain::compare_exact_workflow_traces(original, replay, noise)
    }
}

fn production_admission(
    context: &domain::WorthQueryWorkflowStageExecutionContext<'_>,
) -> Result<
    domain::WorthQueryArtifactProductionAdmission,
    domain::WorthQueryWorkflowStageExecutorFailure,
> {
    context
        .admit_artifact_production(domain::WorthQueryArtifactProductionEvidence::new(
            "artifact-workflow-provenance",
            "artifact-workflow-dependency",
        ))
        .map_err(artifact_failure)
}

fn artifact_failure(
    denial: domain::WorthQueryArtifactDenial,
) -> domain::WorthQueryWorkflowStageExecutorFailure {
    failure(format!("{:?}: {}", denial.kind(), denial.detail()))
}

fn failure(detail: impl Into<String>) -> domain::WorthQueryWorkflowStageExecutorFailure {
    domain::WorthQueryWorkflowStageExecutorFailure::new(
        domain::WorthQueryOperationFailureClass::Dependency,
        detail,
    )
}
