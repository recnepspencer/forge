use std::sync::Arc;

use crate::domain_computation::provider_session::graph_provider::bounded_step::provider_anchor::WorthQueryGraphProviderAnchor;
use crate::domain_computation::provider_session::graph_provider::bounded_step::WorthQueryGraphProviderStepArtifactContext;

use super::managed_graph_execution::WorthQueryManagedGraphExecution;
use super::{
    WorthQueryActiveWorkflowGraphExecution, WorthQueryManagedGraphCallRequest,
    WorthQueryRunningWorkflowRun,
};
use crate::domain_computation::WorthQueryGraphProviderCall;

struct WorthQueryBoundWorkflowGraphStart {
    running: WorthQueryRunningWorkflowRun,
    anchor: Arc<WorthQueryGraphProviderAnchor>,
    call: WorthQueryGraphProviderCall,
}

struct WorthQueryReadyWorkflowGraphStart {
    bound: WorthQueryBoundWorkflowGraphStart,
    contract: super::step_contract_admission::WorthQueryAdmittedManagedStepContract,
    artifact_context: Option<WorthQueryGraphProviderStepArtifactContext>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryWorkflowGraphExecutionStartFailureKind {
    StageResourcesUnavailable,
    ProviderBinding,
    MissingInstalledProvider,
    ProviderSupportMismatch,
    ProviderStart,
    InvalidStepContract,
    StepContract(super::WorthQueryManagedStepContractDenialKind),
    ArtifactAuthority,
}

pub struct WorthQueryWorkflowGraphExecutionStartFailure {
    kind: WorthQueryWorkflowGraphExecutionStartFailureKind,
    detail: Arc<str>,
    running: WorthQueryRunningWorkflowRun,
}

impl WorthQueryWorkflowGraphExecutionStartFailure {
    pub const fn kind(&self) -> WorthQueryWorkflowGraphExecutionStartFailureKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn into_running(self) -> WorthQueryRunningWorkflowRun {
        self.running
    }
}

impl std::fmt::Debug for WorthQueryWorkflowGraphExecutionStartFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryWorkflowGraphExecutionStartFailure")
            .field("kind", &self.kind)
            .field("detail", &self.detail)
            .field("run_identity", &self.running.identity())
            .finish()
    }
}

pub(super) fn begin(
    running: WorthQueryRunningWorkflowRun,
    stage_identity: &str,
    graph_authority: &worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority,
    request: WorthQueryManagedGraphCallRequest,
) -> Result<WorthQueryActiveWorkflowGraphExecution, WorthQueryWorkflowGraphExecutionStartFailure> {
    WorthQueryBoundWorkflowGraphStart::bind(running, stage_identity, graph_authority, request)?
        .validate_contract(stage_identity)?
        .start_provider()
}

impl WorthQueryBoundWorkflowGraphStart {
    fn bind(
        running: WorthQueryRunningWorkflowRun,
        stage_identity: &str,
        graph_authority: &worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority,
        request: WorthQueryManagedGraphCallRequest,
    ) -> Result<Self, WorthQueryWorkflowGraphExecutionStartFailure> {
        let Some(anchor) =
            graph_authority.retain_provider_anchor::<WorthQueryGraphProviderAnchor>()
        else {
            return Err(start_failure(
                WorthQueryWorkflowGraphExecutionStartFailureKind::MissingInstalledProvider,
                "installed graph authority does not retain the exact execution-owned provider anchor",
                running,
            ));
        };
        if running.stage_graph_resource_support(stage_identity, graph_authority.role())
            != Some(anchor.resource_support())
        {
            return Err(start_failure(
                WorthQueryWorkflowGraphExecutionStartFailureKind::ProviderSupportMismatch,
                "admitted stage graph support was not minted from the installed provider's exact support authority",
                running,
            ));
        }
        let call = match running.mint_stage_graph_provider_call(
            stage_identity,
            graph_authority,
            request,
        ) {
            Ok(call) => call,
            Err("workflow-stage-resources-unavailable") => {
                return Err(start_failure(
                    WorthQueryWorkflowGraphExecutionStartFailureKind::StageResourcesUnavailable,
                    "workflow stage has no admitted graph resources",
                    running,
                ))
            }
            Err(detail) => {
                return Err(start_failure(
                    WorthQueryWorkflowGraphExecutionStartFailureKind::ProviderBinding,
                    detail,
                    running,
                ))
            }
        };
        Ok(Self {
            running,
            anchor,
            call,
        })
    }

    fn validate_contract(
        self,
        stage_identity: &str,
    ) -> Result<WorthQueryReadyWorkflowGraphStart, WorthQueryWorkflowGraphExecutionStartFailure>
    {
        let contract = match self.call.resource_envelope().bounded_step_contract() {
            Ok(contract) => contract,
            Err(detail) => {
                return Err(start_failure(
                    WorthQueryWorkflowGraphExecutionStartFailureKind::InvalidStepContract,
                    detail,
                    self.running,
                ))
            }
        };
        let contract = match super::step_contract_admission::admit_managed_step_contract(
            contract,
            self.running.bridge_basis().step_contract(),
        ) {
            Ok(contract) => contract,
            Err(denial) => return Err(step_contract_failure(denial, self.running)),
        };
        let artifact_authority = match self.running.artifacts.production_authority(stage_identity) {
            Ok(authority) => authority,
            Err(denial) => {
                return Err(start_failure(
                    WorthQueryWorkflowGraphExecutionStartFailureKind::ArtifactAuthority,
                    denial.detail(),
                    self.running,
                ))
            }
        };
        let artifact_context = artifact_authority.map(|authority| {
            WorthQueryGraphProviderStepArtifactContext::new(
                authority,
                self.running.provider_artifact_occurrences(),
            )
        });
        Ok(WorthQueryReadyWorkflowGraphStart {
            bound: self,
            contract,
            artifact_context,
        })
    }
}

impl WorthQueryReadyWorkflowGraphStart {
    fn start_provider(
        mut self,
    ) -> Result<WorthQueryActiveWorkflowGraphExecution, WorthQueryWorkflowGraphExecutionStartFailure>
    {
        self.bound.running.provider_work_mut().begin_step_call();
        let execution = match self.bound.anchor.begin(&self.bound.call) {
            Ok(execution) => execution,
            Err(failure) => {
                self.bound.running.provider_work_mut().abandon();
                return Err(start_failure(
                    WorthQueryWorkflowGraphExecutionStartFailureKind::ProviderStart,
                    failure.detail(),
                    self.bound.running,
                ));
            }
        };
        Ok(WorthQueryActiveWorkflowGraphExecution::new(
            self.bound.running,
            WorthQueryManagedGraphExecution::new(
                self.bound.call,
                execution,
                self.bound.anchor,
                self.contract,
                self.artifact_context,
            ),
        ))
    }
}

fn start_failure(
    kind: WorthQueryWorkflowGraphExecutionStartFailureKind,
    detail: impl Into<Arc<str>>,
    running: WorthQueryRunningWorkflowRun,
) -> WorthQueryWorkflowGraphExecutionStartFailure {
    WorthQueryWorkflowGraphExecutionStartFailure {
        kind,
        detail: detail.into(),
        running,
    }
}

fn step_contract_failure(
    denial: super::step_contract_admission::WorthQueryManagedStepContractDenial,
    running: WorthQueryRunningWorkflowRun,
) -> WorthQueryWorkflowGraphExecutionStartFailure {
    WorthQueryWorkflowGraphExecutionStartFailure {
        kind: WorthQueryWorkflowGraphExecutionStartFailureKind::StepContract(denial.kind()),
        detail: Arc::from(denial.detail()),
        running,
    }
}
