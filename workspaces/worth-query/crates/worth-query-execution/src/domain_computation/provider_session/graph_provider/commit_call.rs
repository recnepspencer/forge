use std::any::TypeId;
use std::sync::Arc;

use worth_query_installation::facade::WorthQueryExecutionResourceEnvelope;

use super::call_identity::WorthQueryGraphCallAuthorityIdentity;
use super::{
    WorthQueryBoundGraphExecutionReceipt, WorthQueryGraphCallBindingDenial,
    WorthQueryGraphCallScope, WorthQueryGraphProviderReceipt,
    WorthQueryGraphReceiptAdmissionDenial,
};
use crate::domain_computation::provider_session::{
    WorthQueryExecutionProviderSession, WorthQueryExecutionResourceAttemptEvidence,
};
use crate::execution_digest::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphCommitCallSpec {
    scope: WorthQueryGraphCallScope,
    graph_roles: Vec<String>,
    commit_authority_identity: (u64, TypeId),
}

impl WorthQueryGraphCommitCallSpec {
    pub fn new(
        scope: WorthQueryGraphCallScope,
        graph_roles: impl IntoIterator<Item = String>,
        commit_authority_identity: (u64, TypeId),
    ) -> Self {
        let mut graph_roles: Vec<_> = graph_roles.into_iter().collect();
        graph_roles.sort();
        graph_roles.dedup();
        Self {
            scope,
            graph_roles,
            commit_authority_identity,
        }
    }
}

#[derive(Clone, Debug)]
pub struct WorthQueryGraphCommitCall {
    authority_identity: WorthQueryGraphCallAuthorityIdentity,
    call_identity: Arc<str>,
    provider_session_identity: Arc<str>,
    spec: WorthQueryGraphCommitCallSpec,
    execution_resources: WorthQueryExecutionResourceAttemptEvidence,
    resource_envelope: Arc<WorthQueryExecutionResourceEnvelope>,
}

impl WorthQueryGraphCommitCall {
    pub(in crate::domain_computation::provider_session) fn mint(
        session: &WorthQueryExecutionProviderSession,
        spec: WorthQueryGraphCommitCallSpec,
        execution_resources: &WorthQueryExecutionResourceAttemptEvidence,
        resource_envelope: Arc<WorthQueryExecutionResourceEnvelope>,
    ) -> Result<Self, WorthQueryGraphCallBindingDenial> {
        if execution_resources.provider_session_identity() != session.identity()
            || execution_resources.provider_session_attempt_identity() != session.attempt_identity()
        {
            return Err(WorthQueryGraphCallBindingDenial::ForeignResourceAttempt);
        }
        let authority_identity = WorthQueryGraphCallAuthorityIdentity::mint();
        let call_identity = Arc::<str>::from(hash_parts(&[
            "worth_query_graph_commit_call_v2".into(),
            format!("authority:{}", authority_identity.as_u64()),
            format!("session:{}", session.identity()),
            format!("operation:{}", spec.scope.operation_identity),
            format!("binding:{}", spec.scope.binding_identity),
            format!("roles:{}", spec.graph_roles.join(",")),
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

    pub fn graph_roles(&self) -> &[String] {
        &self.spec.graph_roles
    }

    pub fn binding_identity(&self) -> &str {
        &self.spec.scope.binding_identity
    }

    pub fn scope_identity(&self) -> &str {
        &self.spec.scope.scope_identity
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

    pub fn admit_receipt(
        &self,
        receipt: WorthQueryGraphProviderReceipt,
    ) -> Result<WorthQueryBoundGraphExecutionReceipt, WorthQueryGraphReceiptAdmissionDenial> {
        receipt.admit_commit_call(self)
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

    pub(super) fn commit_authority_identity(&self) -> (u64, TypeId) {
        self.spec.commit_authority_identity
    }
}
