use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use crate::execution_digest::hash_parts;

use super::graph_provider::{
    WorthQueryGraphCallBindingDenial, WorthQueryGraphCommitCall, WorthQueryGraphCommitCallRequest,
    WorthQueryGraphProviderCall, WorthQueryGraphProviderCallRequest,
};
use super::WorthQueryExecutionResourceAttemptEvidence;
use crate::domain_computation::operation_binding::WorthQueryExecutionBoundOperationAuthority;

static NEXT_PROVIDER_SESSION: AtomicU64 = AtomicU64::new(1);

#[derive(Debug)]
pub struct WorthQueryExecutionProviderSession {
    identity: Arc<str>,
    attempt_identity: Arc<str>,
    binding_authority: Arc<WorthQueryExecutionBoundOperationAuthority>,
}

impl WorthQueryExecutionProviderSession {
    pub(super) fn mint(
        attempt_identity: &str,
        binding_authority: &WorthQueryExecutionBoundOperationAuthority,
    ) -> Self {
        let ordinal = NEXT_PROVIDER_SESSION.fetch_add(1, Ordering::Relaxed);
        let identity = Arc::<str>::from(hash_parts(&[
            "worth_query_execution_provider_session_v1".into(),
            format!("attempt:{attempt_identity}"),
            format!("ordinal:{ordinal}"),
        ]));
        Self {
            identity,
            attempt_identity: Arc::from(attempt_identity),
            binding_authority: Arc::new(binding_authority.clone()),
        }
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn attempt_identity(&self) -> &str {
        &self.attempt_identity
    }

    pub(super) fn retain_binding_authority(
        &self,
    ) -> Arc<WorthQueryExecutionBoundOperationAuthority> {
        Arc::clone(&self.binding_authority)
    }

    pub fn bind_direct_domain_evidence(
        &self,
        execution_snapshot_identity: &str,
        output_occurrence_identity: &str,
    ) -> Result<
        crate::domain_computation::WorthQueryDomainEvidenceExecutionBinding,
        crate::domain_computation::WorthQueryDomainEvidenceBindingDenial,
    > {
        crate::domain_computation::WorthQueryDomainEvidenceExecutionBinding::direct(
            &self.binding_authority,
            execution_snapshot_identity,
            output_occurrence_identity,
        )
    }

    pub fn bind_workflow_stage_domain_evidence(
        &self,
        run_identity: &str,
        stage_identity: &str,
        execution_snapshot_identity: &str,
        output_occurrence_identity: &str,
    ) -> Result<
        crate::domain_computation::WorthQueryDomainEvidenceExecutionBinding,
        crate::domain_computation::WorthQueryDomainEvidenceBindingDenial,
    > {
        crate::domain_computation::WorthQueryDomainEvidenceExecutionBinding::workflow_stage(
            &self.binding_authority,
            run_identity,
            stage_identity,
            execution_snapshot_identity,
            output_occurrence_identity,
        )
    }

    pub fn bind_graph_provider_call(
        &self,
        graph_authority: &worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority,
        request: WorthQueryGraphProviderCallRequest,
        execution_resources: &WorthQueryExecutionResourceAttemptEvidence,
        resource_envelope: Arc<
            worth_query_installation::facade::WorthQueryExecutionResourceEnvelope,
        >,
    ) -> Result<WorthQueryGraphProviderCall, WorthQueryGraphCallBindingDenial> {
        if !self.binding_authority.admits_graph_call(
            request.stage_identity(),
            graph_authority,
            request.kind(),
        ) {
            return Err(WorthQueryGraphCallBindingDenial::BoundOperationAuthorityMismatch);
        }
        let spec = request.into_spec(&self.binding_authority, graph_authority);
        WorthQueryGraphProviderCall::mint(self, spec, execution_resources, resource_envelope)
    }

    pub fn bind_graph_commit_call(
        &self,
        graph_authorities: &[&worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority],
        request: WorthQueryGraphCommitCallRequest,
        execution_resources: &WorthQueryExecutionResourceAttemptEvidence,
        resource_envelope: Arc<
            worth_query_installation::facade::WorthQueryExecutionResourceEnvelope,
        >,
    ) -> Result<WorthQueryGraphCommitCall, WorthQueryGraphCallBindingDenial> {
        if !self
            .binding_authority
            .admits_commit_call(request.stage_identity(), graph_authorities)
        {
            return Err(WorthQueryGraphCallBindingDenial::BoundOperationAuthorityMismatch);
        }
        let spec = request.into_spec(&self.binding_authority, graph_authorities);
        WorthQueryGraphCommitCall::mint(self, spec, execution_resources, resource_envelope)
    }
}
