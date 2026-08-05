use std::sync::Arc;

use worth_relational::facade::runtime::RelationalExecutionBasisLease;
use worth_runtime_bridge::facade::BridgeBoundExecutionBasis;

use super::provider_work::WorthQueryManagedProviderWorkLedger;
use super::run_identity::WorthQueryManagedRunIdentity;
use super::{
    WorthQueryDirectRunTerminal, WorthQueryManagedGraphCallRequest, WorthQueryManagedRunCounters,
    WorthQueryManagedRunDenial, WorthQueryManagedRunDenialKind, WorthQueryManagedRunTerminalKind,
    WorthQueryManagedSafePointFailure, WorthQueryManagedSafePointObservation,
};
use crate::domain_computation::{
    WorthQueryDirectExecutionResourceAttempt, WorthQueryExecutionBoundOperationAuthority,
    WorthQueryExecutionResourceAttemptEvidence, WorthQueryGraphCallBindingDenial,
    WorthQueryGraphProviderCallRequest,
};

pub struct WorthQueryAdmittedDirectRun {
    logical_run_identity: Arc<str>,
    identity: Arc<str>,
    resource_attempt: WorthQueryDirectExecutionResourceAttempt,
    bridge_basis: BridgeBoundExecutionBasis,
    relational_basis: RelationalExecutionBasisLease,
    counters: WorthQueryManagedRunCounters,
}

impl WorthQueryAdmittedDirectRun {
    pub(crate) fn new(
        operation: &WorthQueryExecutionBoundOperationAuthority,
        resource_attempt: WorthQueryDirectExecutionResourceAttempt,
        bridge_basis: BridgeBoundExecutionBasis,
        relational_basis: RelationalExecutionBasisLease,
        counters: WorthQueryManagedRunCounters,
    ) -> Self {
        let identity = WorthQueryManagedRunIdentity::initial(
            "direct",
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

    pub fn start(self) -> WorthQueryRunningDirectRun {
        let provider_work = WorthQueryManagedProviderWorkLedger::new(
            self.resource_attempt.provider_session().identity(),
        );
        WorthQueryRunningDirectRun {
            logical_run_identity: self.logical_run_identity,
            identity: self.identity,
            resource_attempt: self.resource_attempt,
            bridge_basis: self.bridge_basis,
            relational_basis: self.relational_basis,
            counters: self.counters,
            provider_work,
        }
    }
}

pub struct WorthQueryRunningDirectRun {
    pub(super) logical_run_identity: Arc<str>,
    pub(super) identity: Arc<str>,
    pub(super) resource_attempt: WorthQueryDirectExecutionResourceAttempt,
    pub(super) bridge_basis: BridgeBoundExecutionBasis,
    pub(super) relational_basis: RelationalExecutionBasisLease,
    pub(super) counters: WorthQueryManagedRunCounters,
    pub(super) provider_work: WorthQueryManagedProviderWorkLedger,
}

impl WorthQueryRunningDirectRun {
    pub(crate) fn graph_work_affinity(
        &self,
    ) -> Option<crate::domain_computation::operation_binding::WorthQueryApplicationGraphWorkAffinity>
    {
        self.resource_attempt
            .binding_authority()
            .graph_work_affinity()
    }

    pub(crate) fn mutation_resource_release_expectation(&self) -> (&str, usize) {
        (
            self.resource_attempt.resources().identity(),
            self.resource_attempt.retained_capacity_reservation_count(),
        )
    }

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

    pub fn begin_graph_execution(
        self,
        graph_authority: &worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority,
        request: WorthQueryManagedGraphCallRequest,
    ) -> Result<
        super::WorthQueryActiveDirectGraphExecution,
        super::WorthQueryDirectGraphExecutionStartFailure,
    > {
        super::direct_graph_execution_start::begin(self, graph_authority, request)
    }

    pub(super) fn mint_graph_provider_call(
        &self,
        graph_authority: &worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority,
        request: WorthQueryManagedGraphCallRequest,
    ) -> Result<
        crate::domain_computation::WorthQueryGraphProviderCall,
        WorthQueryGraphCallBindingDenial,
    > {
        let request =
            WorthQueryGraphProviderCallRequest::direct(request.kind(), request.scope_identity())
                .bind_execution_snapshot(self.execution_snapshot_reference());
        self.resource_attempt
            .provider_session()
            .bind_graph_provider_call(
                graph_authority,
                request,
                self.resource_attempt.evidence(),
                self.resource_attempt.resources().shared_envelope(),
            )
    }

    pub(crate) fn execution_snapshot_reference(&self) -> String {
        let parts = self
            .bridge_basis
            .observation()
            .snapshot_identity()
            .relational_snapshot_parts()
            .expect("managed Relational run admission validates typed snapshot identity");
        format!(
            "worth-query-managed-snapshot|runtime={}|snapshot={}|version={}",
            self.relational_basis.identity().runtime_instance_id(),
            parts.snapshot_id(),
            parts.version_id(),
        )
    }

    pub(crate) fn bind_convergence_candidate_evidence(
        &self,
        output_occurrence_identity: &str,
    ) -> Result<
        crate::domain_computation::WorthQueryDomainEvidenceExecutionBinding,
        crate::domain_computation::WorthQueryDomainEvidenceBindingDenial,
    > {
        self.resource_attempt
            .provider_session()
            .bind_direct_domain_evidence(
                &self.execution_snapshot_reference(),
                output_occurrence_identity,
            )
    }

    pub fn completed(
        self,
    ) -> Result<WorthQueryDirectRunTerminal, WorthQueryDirectRunCompletionRejection> {
        if self.provider_work.has_uncertainty() {
            return Err(WorthQueryDirectRunCompletionRejection {
                denial: WorthQueryManagedRunDenial::new(
                    WorthQueryManagedRunDenialKind::UnverifiedProviderWork,
                    "provider work must be receipt-bound before a managed run can claim completion",
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

    pub(super) fn graph_resource_support(
        &self,
        role: &str,
    ) -> Option<
        &worth_query_admission::facade::resource_admission::WorthQueryExecutionResourceSupport,
    > {
        self.resource_attempt
            .resources()
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
        self,
        kind: WorthQueryManagedRunTerminalKind,
    ) -> WorthQueryDirectRunTerminal {
        let (provider_work, provider_cleanup) = self.provider_work.into_terminal_parts();
        WorthQueryDirectRunTerminal {
            logical_run_identity: self.logical_run_identity,
            identity: self.identity,
            kind,
            resource_attempt: self.resource_attempt,
            bridge_basis: self.bridge_basis,
            relational_basis: self.relational_basis,
            counters: self.counters,
            provider_work,
            provider_cleanup,
        }
    }

    pub(crate) fn terminate_for_convergence(
        self,
        kind: WorthQueryManagedRunTerminalKind,
    ) -> WorthQueryDirectRunTerminal {
        self.terminal(kind)
    }
}

pub struct WorthQueryDirectRunCompletionRejection {
    denial: WorthQueryManagedRunDenial,
    running: WorthQueryRunningDirectRun,
}

impl WorthQueryDirectRunCompletionRejection {
    pub fn denial(&self) -> &WorthQueryManagedRunDenial {
        &self.denial
    }

    pub fn into_running(self) -> WorthQueryRunningDirectRun {
        self.running
    }
}

impl std::fmt::Debug for WorthQueryDirectRunCompletionRejection {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryDirectRunCompletionRejection")
            .field("denial", &self.denial)
            .field("run_identity", &self.running.identity())
            .finish()
    }
}
