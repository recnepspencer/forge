use std::sync::Arc;

use worth_query_installation::facade::WorthQueryExecutionResourceEnvelope;

use super::call_identity::WorthQueryGraphCallAuthorityIdentity;
use super::{
    WorthQueryBoundGraphExecutionReceipt, WorthQueryExecutionGraphReadProduct,
    WorthQueryGraphCallBindingDenial, WorthQueryGraphProviderCallKind,
    WorthQueryGraphProviderFailure, WorthQueryGraphProviderReceipt, WorthQueryGraphReadMaterial,
    WorthQueryGraphReceiptAdmissionDenial,
};
use crate::domain_computation::provider_session::{
    WorthQueryExecutionProviderSession, WorthQueryExecutionResourceAttemptEvidence,
};
use crate::execution_digest::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphCallScope {
    pub(super) scope_identity: Arc<str>,
    pub(super) operation_identity: Arc<str>,
    pub(super) binding_identity: Arc<str>,
}

impl WorthQueryGraphCallScope {
    pub fn new(
        scope_identity: impl Into<Arc<str>>,
        operation_identity: impl Into<Arc<str>>,
        binding_identity: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            scope_identity: scope_identity.into(),
            operation_identity: operation_identity.into(),
            binding_identity: binding_identity.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphCallReadBinding {
    graph_role: Arc<str>,
    canonical_query_digest: Arc<str>,
    basis_identity: Arc<str>,
    snapshot_identity: Arc<str>,
}

impl WorthQueryGraphCallReadBinding {
    pub fn new(
        graph_role: impl Into<Arc<str>>,
        canonical_query_digest: impl Into<Arc<str>>,
        basis_identity: impl Into<Arc<str>>,
        snapshot_identity: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            graph_role: graph_role.into(),
            canonical_query_digest: canonical_query_digest.into(),
            basis_identity: basis_identity.into(),
            snapshot_identity: snapshot_identity.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphProviderCallSpec {
    kind: WorthQueryGraphProviderCallKind,
    scope: WorthQueryGraphCallScope,
    read_binding: WorthQueryGraphCallReadBinding,
}

impl WorthQueryGraphProviderCallSpec {
    pub fn new(
        kind: WorthQueryGraphProviderCallKind,
        scope: WorthQueryGraphCallScope,
        read_binding: WorthQueryGraphCallReadBinding,
    ) -> Self {
        Self {
            kind,
            scope,
            read_binding,
        }
    }
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

    pub fn completed(
        &self,
        provider_receipt: impl Into<Arc<str>>,
    ) -> WorthQueryGraphProviderReceipt {
        WorthQueryGraphProviderReceipt::completed(self.authority_identity, provider_receipt)
    }

    pub fn projected(
        &self,
        provider_receipt: impl Into<Arc<str>>,
        material: WorthQueryGraphReadMaterial,
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

    pub(super) fn call_identity(&self) -> &str {
        &self.call_identity
    }

    pub(super) fn provider_session_identity(&self) -> &str {
        &self.provider_session_identity
    }
}
