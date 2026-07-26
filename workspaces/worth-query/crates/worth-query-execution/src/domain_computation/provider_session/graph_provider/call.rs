use std::sync::Arc;

use worth_query_installation::facade::WorthQueryExecutionResourceEnvelope;

use super::call_identity::WorthQueryGraphCallAuthorityIdentity;
use super::{
    WorthQueryBoundGraphExecutionReceipt, WorthQueryExecutionGraphReadProduct,
    WorthQueryExecutionGraphReadStreamEvidence, WorthQueryGraphCallBindingDenial,
    WorthQueryGraphProviderCallKind, WorthQueryGraphProviderFailure,
    WorthQueryGraphProviderReceipt, WorthQueryGraphReadMaterial,
    WorthQueryGraphReceiptAdmissionDenial, WorthQueryProviderWorkReport,
};
use crate::domain_computation::provider_session::{
    WorthQueryExecutionProviderSession, WorthQueryExecutionResourceAttemptEvidence,
};
use crate::execution_digest::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphProviderCallRequest {
    kind: WorthQueryGraphProviderCallKind,
    scope_identity: Arc<str>,
    stage_identity: Option<Arc<str>>,
    snapshot_identity: Option<Arc<str>>,
}

impl WorthQueryGraphProviderCallRequest {
    pub fn direct(
        kind: WorthQueryGraphProviderCallKind,
        scope_identity: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            kind,
            scope_identity: scope_identity.into(),
            stage_identity: None,
            snapshot_identity: None,
        }
    }

    pub fn workflow_stage(
        kind: WorthQueryGraphProviderCallKind,
        scope_identity: impl Into<Arc<str>>,
        stage_identity: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            kind,
            scope_identity: scope_identity.into(),
            stage_identity: Some(stage_identity.into()),
            snapshot_identity: None,
        }
    }

    /// Legacy operational snapshot binding retained until Phase 19 removes
    /// monolith-owned provider invocation. This request does not carry or mint
    /// managed-run authority.
    #[doc(hidden)]
    pub fn bind_execution_snapshot(mut self, snapshot_identity: impl Into<Arc<str>>) -> Self {
        self.snapshot_identity = Some(snapshot_identity.into());
        self
    }

    pub(in crate::domain_computation::provider_session) fn kind(
        &self,
    ) -> WorthQueryGraphProviderCallKind {
        self.kind
    }

    pub(in crate::domain_computation::provider_session) fn execution_snapshot_identity(
        &self,
    ) -> Option<&str> {
        self.snapshot_identity.as_deref()
    }

    pub(in crate::domain_computation::provider_session) fn stage_identity(&self) -> Option<&str> {
        self.stage_identity.as_deref()
    }

    pub(in crate::domain_computation::provider_session) fn into_spec(
        self,
        binding: &crate::domain_computation::operation_binding::WorthQueryExecutionBoundOperationAuthority,
        graph: &worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority,
    ) -> WorthQueryGraphProviderCallSpec {
        let scope = WorthQueryGraphCallScope {
            scope_identity: self.scope_identity,
            operation_identity: Arc::from(binding.operation_identity()),
            binding_identity: Arc::from(binding.binding_identity()),
            stage_identity: self.stage_identity,
        };
        WorthQueryGraphProviderCallSpec {
            kind: self.kind,
            scope,
            read_binding: WorthQueryGraphCallReadBinding {
                graph_role: Arc::from(graph.role()),
                canonical_query_digest: Arc::from(binding.canonical_query_digest()),
                basis_identity: Arc::from(binding.basis_identity()),
                snapshot_identity: self
                    .snapshot_identity
                    .expect("provider session validates managed execution basis"),
            },
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::domain_computation::provider_session) struct WorthQueryGraphCallScope {
    pub(super) scope_identity: Arc<str>,
    pub(super) operation_identity: Arc<str>,
    pub(super) binding_identity: Arc<str>,
    pub(super) stage_identity: Option<Arc<str>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::domain_computation::provider_session) struct WorthQueryGraphCallReadBinding {
    graph_role: Arc<str>,
    canonical_query_digest: Arc<str>,
    basis_identity: Arc<str>,
    snapshot_identity: Arc<str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::domain_computation::provider_session) struct WorthQueryGraphProviderCallSpec {
    kind: WorthQueryGraphProviderCallKind,
    scope: WorthQueryGraphCallScope,
    read_binding: WorthQueryGraphCallReadBinding,
}

#[derive(Clone, Debug)]
pub struct WorthQueryGraphProviderCall {
    authority_identity: WorthQueryGraphCallAuthorityIdentity,
    call_identity: Arc<str>,
    provider_session_identity: Arc<str>,
    spec: WorthQueryGraphProviderCallSpec,
    execution_resources: WorthQueryExecutionResourceAttemptEvidence,
    resource_envelope: Arc<WorthQueryExecutionResourceEnvelope>,
}

impl WorthQueryGraphProviderCall {
    pub(in crate::domain_computation::provider_session) fn mint(
        session: &WorthQueryExecutionProviderSession,
        spec: WorthQueryGraphProviderCallSpec,
        execution_resources: &WorthQueryExecutionResourceAttemptEvidence,
        resource_envelope: Arc<WorthQueryExecutionResourceEnvelope>,
    ) -> Result<Self, WorthQueryGraphCallBindingDenial> {
        if spec.kind == WorthQueryGraphProviderCallKind::CommitAdmission {
            return Err(WorthQueryGraphCallBindingDenial::CommitKindRequiresCommitCall);
        }
        if execution_resources.provider_session_identity() != session.identity()
            || execution_resources.provider_session_attempt_identity() != session.attempt_identity()
        {
            return Err(WorthQueryGraphCallBindingDenial::ForeignResourceAttempt);
        }
        let authority_identity = WorthQueryGraphCallAuthorityIdentity::mint();
        let call_identity = Arc::<str>::from(hash_parts(&[
            "worth_query_graph_provider_call_v2".into(),
            format!("authority:{}", authority_identity.as_u64()),
            format!("session:{}", session.identity()),
            format!("operation:{}", spec.scope.operation_identity),
            format!("binding:{}", spec.scope.binding_identity),
            format!("role:{}", spec.read_binding.graph_role),
            format!("query:{}", spec.read_binding.canonical_query_digest),
            format!("basis:{}", spec.read_binding.basis_identity),
            format!("snapshot:{}", spec.read_binding.snapshot_identity),
            format!("kind:{}", spec.kind.as_str()),
            format!("scope:{}", spec.scope.scope_identity),
            format!(
                "stage:{}",
                spec.scope.stage_identity.as_deref().unwrap_or("direct")
            ),
            format!("resources:{}", execution_resources.identity()),
        ]));
        Ok(Self {
            authority_identity,
            call_identity,
            provider_session_identity: Arc::from(session.identity()),
            spec,
            execution_resources: execution_resources.clone(),
            resource_envelope,
        })
    }

    pub fn operation_identity(&self) -> &str {
        &self.spec.scope.operation_identity
    }

    pub fn kind(&self) -> WorthQueryGraphProviderCallKind {
        self.spec.kind
    }

    pub fn scope_identity(&self) -> &str {
        &self.spec.scope.scope_identity
    }

    pub fn graph_role(&self) -> &str {
        &self.spec.read_binding.graph_role
    }

    pub fn binding_identity(&self) -> &str {
        &self.spec.scope.binding_identity
    }

    pub fn canonical_query_digest(&self) -> &str {
        &self.spec.read_binding.canonical_query_digest
    }

    pub fn basis_identity(&self) -> &str {
        &self.spec.read_binding.basis_identity
    }

    pub fn snapshot_identity(&self) -> &str {
        &self.spec.read_binding.snapshot_identity
    }

    pub fn execution_resources(&self) -> &WorthQueryExecutionResourceAttemptEvidence {
        &self.execution_resources
    }

    pub fn resource_envelope(&self) -> &WorthQueryExecutionResourceEnvelope {
        &self.resource_envelope
    }

    pub(crate) fn remint_for_readmission(
        &self,
        session: &WorthQueryExecutionProviderSession,
        execution_resources: &WorthQueryExecutionResourceAttemptEvidence,
    ) -> Result<Self, WorthQueryGraphCallBindingDenial> {
        if self.spec.scope.binding_identity.as_ref()
            != session.binding_authority().binding_identity()
        {
            return Err(WorthQueryGraphCallBindingDenial::BoundOperationAuthorityMismatch);
        }
        if execution_resources.admission_identity() != self.execution_resources.admission_identity()
            || execution_resources.request_identity() != self.execution_resources.request_identity()
            || execution_resources.strategy() != self.execution_resources.strategy()
            || execution_resources.envelope_identity()
                != self.execution_resources.envelope_identity()
            || execution_resources.support_snapshot_identity()
                != self.execution_resources.support_snapshot_identity()
        {
            return Err(WorthQueryGraphCallBindingDenial::ExecutionBasisMismatch);
        }
        Self::mint(
            session,
            self.spec.clone(),
            execution_resources,
            Arc::clone(&self.resource_envelope),
        )
    }

    pub(crate) fn stage_identity(&self) -> Option<&str> {
        self.spec.scope.stage_identity.as_deref()
    }

    pub(crate) fn completed(
        &self,
        provider_receipt: impl Into<Arc<str>>,
        work_report: WorthQueryProviderWorkReport,
    ) -> WorthQueryGraphProviderReceipt {
        WorthQueryGraphProviderReceipt::completed(
            self.authority_identity,
            provider_receipt,
            work_report,
        )
    }

    pub(crate) fn projected(
        &self,
        provider_receipt: impl Into<Arc<str>>,
        material: WorthQueryGraphReadMaterial,
        work_report: WorthQueryProviderWorkReport,
    ) -> Result<WorthQueryGraphProviderReceipt, WorthQueryGraphProviderFailure> {
        if self.kind() != WorthQueryGraphProviderCallKind::Project {
            return Err(WorthQueryGraphProviderFailure::new(
                WorthQueryGraphReceiptAdmissionDenial::UnexpectedProjection.detail(),
            ));
        }
        let product = Arc::new(WorthQueryExecutionGraphReadProduct::seal(self, material));
        Ok(WorthQueryGraphProviderReceipt::projected(
            self.authority_identity,
            provider_receipt,
            product,
            work_report,
        ))
    }

    pub(crate) fn streamed(
        &self,
        provider_receipt: impl Into<Arc<str>>,
        stream: WorthQueryExecutionGraphReadStreamEvidence,
        work_report: WorthQueryProviderWorkReport,
    ) -> Result<WorthQueryGraphProviderReceipt, WorthQueryGraphProviderFailure> {
        if self.kind() != WorthQueryGraphProviderCallKind::Project {
            return Err(WorthQueryGraphProviderFailure::new(
                WorthQueryGraphReceiptAdmissionDenial::UnexpectedProjection.detail(),
            ));
        }
        Ok(WorthQueryGraphProviderReceipt::streamed(
            self.authority_identity,
            provider_receipt,
            Arc::new(stream),
            work_report,
        ))
    }

    pub fn admit_receipt(
        &self,
        receipt: WorthQueryGraphProviderReceipt,
    ) -> Result<WorthQueryBoundGraphExecutionReceipt, WorthQueryGraphReceiptAdmissionDenial> {
        receipt.admit_graph_call(self)
    }

    pub(super) fn authority_identity(&self) -> WorthQueryGraphCallAuthorityIdentity {
        self.authority_identity
    }

    pub(crate) fn call_identity(&self) -> &str {
        &self.call_identity
    }

    pub(super) fn provider_session_identity(&self) -> &str {
        &self.provider_session_identity
    }
}
