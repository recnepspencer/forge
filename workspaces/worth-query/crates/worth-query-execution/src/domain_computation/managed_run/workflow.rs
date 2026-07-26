use std::sync::Arc;

use worth_relational::facade::runtime::RelationalExecutionBasisLease;
use worth_runtime_bridge::facade::BridgeBoundExecutionBasis;

use super::provider_work::WorthQueryManagedProviderWorkLedger;
use super::run_identity::WorthQueryManagedRunIdentity;
use super::{
    WorthQueryManagedGraphCallRequest, WorthQueryManagedProviderWorkEvidence,
    WorthQueryManagedRunCounters, WorthQueryManagedRunDenial, WorthQueryManagedRunDenialKind,
    WorthQueryManagedRunTerminalKind, WorthQueryManagedSafePointFailure,
    WorthQueryManagedSafePointObservation, WorthQueryManagedWorkflowArtifactAuthority,
    WorthQueryWorkflowRunCleanupOutcome,
};
use crate::domain_computation::artifact_owner::{
    WorthQueryArtifactOccurrenceLedger, WorthQueryWorkflowArtifactAuthority,
    WorthQueryWorkflowArtifactRegistryEvidence,
};
use crate::domain_computation::{
    WorthQueryArtifactDenial, WorthQueryExecutionBoundOperationAuthority,
    WorthQueryExecutionResourceAttemptEvidence, WorthQueryGraphProviderCallRequest,
    WorthQueryWorkflowExecutionResourceAttempt,
};

pub struct WorthQueryAdmittedWorkflowRun {
    logical_run_identity: Arc<str>,
    identity: Arc<str>,
    pub(super) resource_attempt: WorthQueryWorkflowExecutionResourceAttempt,
    pub(super) bridge_basis: BridgeBoundExecutionBasis,
    pub(super) relational_basis: RelationalExecutionBasisLease,
    counters: WorthQueryManagedRunCounters,
}

impl WorthQueryAdmittedWorkflowRun {
    pub(crate) fn new(
        operation: &WorthQueryExecutionBoundOperationAuthority,
        resource_attempt: WorthQueryWorkflowExecutionResourceAttempt,
        bridge_basis: BridgeBoundExecutionBasis,
        relational_basis: RelationalExecutionBasisLease,
        counters: WorthQueryManagedRunCounters,
    ) -> Self {
        let identity = WorthQueryManagedRunIdentity::initial(
            "workflow",
            operation,
            resource_attempt.attempt_identity().as_str(),
            &bridge_basis,
            &relational_basis,
        );
        let (logical_run_identity, identity) = identity.into_parts();
        Self {
            logical_run_identity,
            identity,
            resource_attempt,
            bridge_basis,
            relational_basis,
            counters,
        }
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn logical_run_identity(&self) -> &str {
        &self.logical_run_identity
    }

    pub fn counters(&self) -> &WorthQueryManagedRunCounters {
        &self.counters
    }

    pub(crate) fn belongs_to_operation(
        &self,
        operation: &WorthQueryExecutionBoundOperationAuthority,
    ) -> bool {
        self.resource_attempt.binding_authority().binding_identity() == operation.binding_identity()
    }

    pub fn start(
        self,
    ) -> Result<WorthQueryRunningWorkflowRun, WorthQueryWorkflowRunStartRejection> {
        let artifacts = match self.resource_attempt.bind_workflow_artifacts() {
            Ok(artifacts) => artifacts,
            Err(denial) => {
                return Err(WorthQueryWorkflowRunStartRejection {
                    denial,
                    admitted: self,
                });
            }
        };
        let provider_work = WorthQueryManagedProviderWorkLedger::new(
            self.resource_attempt.provider_session().identity(),
        );
        let provider_artifact_occurrences = Arc::new(WorthQueryArtifactOccurrenceLedger::default());
        Ok(WorthQueryRunningWorkflowRun {
            logical_run_identity: self.logical_run_identity,
            identity: self.identity,
            resource_attempt: self.resource_attempt,
            bridge_basis: self.bridge_basis,
            relational_basis: self.relational_basis,
            counters: self.counters,
            artifacts,
            provider_work,
            provider_artifact_occurrences,
        })
    }
}

pub struct WorthQueryWorkflowRunStartRejection {
    denial: WorthQueryArtifactDenial,
    admitted: WorthQueryAdmittedWorkflowRun,
}

impl WorthQueryWorkflowRunStartRejection {
    pub fn denial(&self) -> &WorthQueryArtifactDenial {
        &self.denial
    }

    pub fn into_admitted(self) -> WorthQueryAdmittedWorkflowRun {
        self.admitted
    }
}

impl std::fmt::Debug for WorthQueryWorkflowRunStartRejection {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryWorkflowRunStartRejection")
            .field("denial", &self.denial)
            .field("run_identity", &self.admitted.identity())
            .finish()
    }
}

pub struct WorthQueryRunningWorkflowRun {
    pub(super) logical_run_identity: Arc<str>,
    pub(super) identity: Arc<str>,
    pub(super) resource_attempt: WorthQueryWorkflowExecutionResourceAttempt,
    pub(super) bridge_basis: BridgeBoundExecutionBasis,
    pub(super) relational_basis: RelationalExecutionBasisLease,
    pub(super) counters: WorthQueryManagedRunCounters,
    pub(super) artifacts: WorthQueryWorkflowArtifactAuthority,
    pub(super) provider_work: WorthQueryManagedProviderWorkLedger,
    pub(super) provider_artifact_occurrences: Arc<WorthQueryArtifactOccurrenceLedger>,
}

impl WorthQueryRunningWorkflowRun {
    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn logical_run_identity(&self) -> &str {
        &self.logical_run_identity
    }

    pub fn evidence(&self) -> &WorthQueryExecutionResourceAttemptEvidence {
        self.resource_attempt.evidence()
    }

    pub fn observe_safe_point(
        &self,
    ) -> Result<WorthQueryManagedSafePointObservation, WorthQueryManagedSafePointFailure> {
        super::safe_point_observation::observe_managed_run_safe_point(
            &self.identity,
            &self.bridge_basis,
        )
    }

    pub fn artifacts(&self) -> WorthQueryManagedWorkflowArtifactAuthority<'_> {
        WorthQueryManagedWorkflowArtifactAuthority::new(&self.artifacts)
    }

    pub fn begin_stage_graph_execution(
        self,
        stage_identity: &str,
        graph_authority: &worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority,
        request: WorthQueryManagedGraphCallRequest,
    ) -> Result<
        super::WorthQueryActiveWorkflowGraphExecution,
        super::WorthQueryWorkflowGraphExecutionStartFailure,
    > {
        super::workflow_graph_execution_start::begin(self, stage_identity, graph_authority, request)
    }

    pub(super) fn mint_stage_graph_provider_call(
        &self,
        stage_identity: &str,
        graph_authority: &worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority,
        request: WorthQueryManagedGraphCallRequest,
    ) -> Result<crate::domain_computation::WorthQueryGraphProviderCall, &'static str> {
        let (resources, evidence) = self
            .resource_attempt
            .stage_resources_and_evidence(stage_identity)
            .ok_or("workflow-stage-resources-unavailable")?;
        let request = WorthQueryGraphProviderCallRequest::workflow_stage(
            request.kind(),
            request.scope_identity(),
            stage_identity,
        )
        .bind_execution_snapshot(self.execution_snapshot_reference());
        let call = self
            .resource_attempt
            .provider_session()
            .bind_graph_provider_call(
                graph_authority,
                request,
                &evidence,
                resources.shared_envelope(),
            )
            .map_err(|_| "workflow-provider-call-binding-denied")?;
        Ok(call)
    }

    pub(crate) fn execution_snapshot_reference(&self) -> String {
        let parts = self
            .bridge_basis
            .observation()
            .snapshot_identity()
            .relational_snapshot_parts()
            .expect("managed Relational workflow validates typed snapshot identity");
        format!(
            "worth-query-managed-snapshot|runtime={}|snapshot={}|version={}",
            self.relational_basis.identity().runtime_instance_id(),
            parts.snapshot_id(),
            parts.version_id(),
        )
    }

    pub(crate) fn bind_convergence_candidate_evidence(
        &self,
        stage_identity: &str,
        output_occurrence_identity: &str,
    ) -> Result<
        crate::domain_computation::WorthQueryDomainEvidenceExecutionBinding,
        crate::domain_computation::WorthQueryDomainEvidenceBindingDenial,
    > {
        self.resource_attempt
            .provider_session()
            .bind_workflow_stage_domain_evidence(
                self.logical_run_identity(),
                stage_identity,
                &self.execution_snapshot_reference(),
                output_occurrence_identity,
            )
    }

    pub fn completed(
        self,
    ) -> Result<WorthQueryWorkflowRunTerminal, WorthQueryWorkflowRunCompletionRejection> {
        if self.provider_work.has_uncertainty() {
            return Err(WorthQueryWorkflowRunCompletionRejection {
                denial: WorthQueryManagedRunDenial::new(
                    WorthQueryManagedRunDenialKind::UnverifiedProviderWork,
                    "workflow provider work must be receipt-bound before completion",
                    self.counters.clone(),
                ),
                running: self,
            });
        }
        Ok(self.terminal(WorthQueryManagedRunTerminalKind::Completed))
    }

    pub(super) fn provider_work_mut(&mut self) -> &mut WorthQueryManagedProviderWorkLedger {
        &mut self.provider_work
    }

    pub(super) fn provider_artifact_occurrences(&self) -> Arc<WorthQueryArtifactOccurrenceLedger> {
        Arc::clone(&self.provider_artifact_occurrences)
    }

    pub(super) fn stage_graph_resource_support(
        &self,
        stage_identity: &str,
        role: &str,
    ) -> Option<
        &worth_query_admission::facade::resource_admission::WorthQueryExecutionResourceSupport,
    > {
        self.resource_attempt
            .resources()
            .stage(stage_identity)?
            .support_snapshot()
            .graph_provider(role)
    }

    pub(super) fn bridge_basis(&self) -> &BridgeBoundExecutionBasis {
        &self.bridge_basis
    }

    pub(super) fn bridge_basis_mut(&mut self) -> &mut BridgeBoundExecutionBasis {
        &mut self.bridge_basis
    }

    pub(super) fn terminal(
        mut self,
        kind: WorthQueryManagedRunTerminalKind,
    ) -> WorthQueryWorkflowRunTerminal {
        let artifact_evidence_at_terminal = self.artifacts.registry().freeze_production();
        self.provider_work
            .settle_artifacts(self.provider_artifact_occurrences.snapshot());
        WorthQueryWorkflowRunTerminal {
            logical_run_identity: self.logical_run_identity,
            identity: self.identity,
            kind,
            resource_attempt: self.resource_attempt,
            bridge_basis: self.bridge_basis,
            relational_basis: self.relational_basis,
            artifacts: self.artifacts,
            artifact_evidence_at_terminal,
            counters: self.counters,
            provider_work: self.provider_work.into_evidence(),
        }
    }

    pub(crate) fn terminate_for_convergence(
        self,
        kind: WorthQueryManagedRunTerminalKind,
    ) -> WorthQueryWorkflowRunTerminal {
        self.terminal(kind)
    }
}

pub struct WorthQueryWorkflowRunCompletionRejection {
    denial: WorthQueryManagedRunDenial,
    running: WorthQueryRunningWorkflowRun,
}

impl WorthQueryWorkflowRunCompletionRejection {
    pub fn denial(&self) -> &WorthQueryManagedRunDenial {
        &self.denial
    }

    pub fn into_running(self) -> WorthQueryRunningWorkflowRun {
        self.running
    }
}

impl std::fmt::Debug for WorthQueryWorkflowRunCompletionRejection {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryWorkflowRunCompletionRejection")
            .field("denial", &self.denial)
            .field("run_identity", &self.running.identity())
            .finish()
    }
}

pub struct WorthQueryWorkflowRunTerminal {
    pub(super) logical_run_identity: Arc<str>,
    pub(super) identity: Arc<str>,
    pub(super) kind: WorthQueryManagedRunTerminalKind,
    pub(super) resource_attempt: WorthQueryWorkflowExecutionResourceAttempt,
    pub(super) bridge_basis: BridgeBoundExecutionBasis,
    pub(super) relational_basis: RelationalExecutionBasisLease,
    pub(super) artifacts: WorthQueryWorkflowArtifactAuthority,
    pub(super) artifact_evidence_at_terminal: WorthQueryWorkflowArtifactRegistryEvidence,
    pub(super) counters: WorthQueryManagedRunCounters,
    pub(super) provider_work: WorthQueryManagedProviderWorkEvidence,
}

impl WorthQueryWorkflowRunTerminal {
    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn logical_run_identity(&self) -> &str {
        &self.logical_run_identity
    }

    pub fn kind(&self) -> WorthQueryManagedRunTerminalKind {
        self.kind
    }

    pub fn provider_work(&self) -> &WorthQueryManagedProviderWorkEvidence {
        &self.provider_work
    }

    pub fn counters(&self) -> &WorthQueryManagedRunCounters {
        &self.counters
    }

    pub fn artifact_evidence(&self) -> WorthQueryWorkflowArtifactRegistryEvidence {
        self.artifact_evidence_at_terminal
    }

    pub fn cleanup(self) -> WorthQueryWorkflowRunCleanupOutcome {
        super::workflow_cleanup::cleanup_workflow_terminal(self)
    }
}
