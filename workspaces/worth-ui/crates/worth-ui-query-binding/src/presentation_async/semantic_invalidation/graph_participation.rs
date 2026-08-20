use std::sync::Arc;

use worth_query::facade::domain;

#[derive(Clone, Copy, Debug)]
pub(super) struct WorthUiPresentationSemanticGraph;

pub(super) struct WorthUiPresentationGraphProvider;

pub(super) fn presentation_graph_definition(
) -> domain::WorthQueryGraphParticipationDefinition<WorthUiPresentationSemanticGraph> {
    domain::WorthQueryGraphParticipationDefinition::new(
        "presentation",
        domain::WorthQueryGraphParticipationContract {
            observation: domain::WorthQueryGraphObservationPosture::Snapshot,
            projection: domain::WorthQueryGraphProjectionPosture::NativeProjection,
            mutation: domain::WorthQueryGraphMutationPosture::NotRequired,
            identity: domain::WorthQueryGraphIdentityPosture::EvolvingLineage,
            locality: domain::WorthQueryGraphLocalityPosture::InProcess,
            budget: domain::WorthQueryGraphBudgetPosture::ConstantAdmission,
            commit: domain::WorthQueryGraphCommitPosture::ReadOnly,
            failure: domain::WorthQueryGraphFailureTopology::Local,
        },
    )
}

pub(super) struct WorthUiPresentationGraphExecution {
    advanced: bool,
    observation: Option<WorthUiPresentationGraphObservation>,
}

struct WorthUiPresentationGraphObservation {
    operation: Arc<str>,
    binding: Arc<str>,
    basis: Arc<str>,
    snapshot: Arc<str>,
    query: Arc<str>,
}

impl WorthUiPresentationGraphObservation {
    fn into_artifact(self) -> Arc<str> {
        Arc::from(format!(
            "worth-ui-presentation-semantic-observation-v2:{}:{}:{}:{}:{}",
            self.operation, self.binding, self.basis, self.snapshot, self.query,
        ))
    }
}

impl domain::WorthQueryGraphProviderExecution for WorthUiPresentationGraphExecution {
    fn advance(
        &mut self,
        step: &mut domain::WorthQueryGraphProviderStep,
    ) -> Result<
        domain::WorthQueryGraphProviderStepDisposition,
        domain::WorthQueryGraphProviderFailure,
    > {
        if self.advanced {
            return Err(domain::WorthQueryGraphProviderFailure::new(
                "WUI presentation graph execution advanced after completion",
            ));
        }
        self.advanced = true;
        let observation = self.observation.take().ok_or_else(|| {
            domain::WorthQueryGraphProviderFailure::new(
                "WUI presentation graph observation was not retained",
            )
        })?;
        let artifact = step.perform_work_unit(|| Ok(observation.into_artifact()))?;
        domain::WorthQueryGraphProviderStepDisposition::complete(artifact)
            .map_err(domain::WorthQueryGraphProviderFailure::new)
    }

    fn dispose(&mut self) -> Result<(), domain::WorthQueryGraphProviderFailure> {
        Ok(())
    }
}

impl domain::WorthQueryGraphParticipationProvider<WorthUiPresentationSemanticGraph>
    for WorthUiPresentationGraphProvider
{
    type Execution = WorthUiPresentationGraphExecution;

    fn execution_resource_support(&self) -> domain::WorthQueryExecutionResourceSupport {
        crate::installed_domain::execution_resources::operation_execution_resource_support()
    }

    fn begin(
        &self,
        call: &domain::WorthQueryGraphProviderCall,
        start: &mut domain::WorthQueryGraphProviderExecutionStart,
    ) -> Result<
        domain::WorthQueryCooperativeGraphProviderExecution<Self::Execution>,
        domain::WorthQueryGraphProviderFailure,
    > {
        start
            .admit_cooperative_execution(|| WorthUiPresentationGraphExecution {
                advanced: false,
                observation: Some(WorthUiPresentationGraphObservation {
                    operation: Arc::from(call.operation_identity()),
                    binding: Arc::from(call.binding_identity()),
                    basis: Arc::from(call.basis_identity()),
                    snapshot: Arc::from(call.snapshot_identity()),
                    query: Arc::from(call.canonical_query_digest()),
                }),
            })
            .map_err(|denial| domain::WorthQueryGraphProviderFailure::new(denial.detail()))
    }
}
