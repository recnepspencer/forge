use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use crate::execution_digest::hash_parts;

use super::graph_provider::{
    WorthQueryGraphCallBindingDenial, WorthQueryGraphCommitCall, WorthQueryGraphCommitCallSpec,
    WorthQueryGraphProviderCall, WorthQueryGraphProviderCallSpec,
};
use super::WorthQueryExecutionResourceAttemptEvidence;

static NEXT_PROVIDER_SESSION: AtomicU64 = AtomicU64::new(1);

#[derive(Debug)]
pub struct WorthQueryExecutionProviderSession {
    identity: Arc<str>,
    attempt_identity: Arc<str>,
}

impl WorthQueryExecutionProviderSession {
    pub(super) fn mint(attempt_identity: &str) -> Self {
        let ordinal = NEXT_PROVIDER_SESSION.fetch_add(1, Ordering::Relaxed);
        let identity = Arc::<str>::from(hash_parts(&[
            "worth_query_execution_provider_session_v1".into(),
            format!("attempt:{attempt_identity}"),
            format!("ordinal:{ordinal}"),
        ]));
        Self {
            identity,
            attempt_identity: Arc::from(attempt_identity),
        }
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn attempt_identity(&self) -> &str {
        &self.attempt_identity
    }

    pub fn bind_graph_provider_call(
        &self,
        spec: WorthQueryGraphProviderCallSpec,
        execution_resources: &WorthQueryExecutionResourceAttemptEvidence,
        resource_envelope: Arc<
            worth_query_installation::facade::WorthQueryExecutionResourceEnvelope,
        >,
    ) -> Result<WorthQueryGraphProviderCall, WorthQueryGraphCallBindingDenial> {
        WorthQueryGraphProviderCall::mint(self, spec, execution_resources, resource_envelope)
    }

    pub fn bind_graph_commit_call(
        &self,
        spec: WorthQueryGraphCommitCallSpec,
        execution_resources: &WorthQueryExecutionResourceAttemptEvidence,
        resource_envelope: Arc<
            worth_query_installation::facade::WorthQueryExecutionResourceEnvelope,
        >,
    ) -> Result<WorthQueryGraphCommitCall, WorthQueryGraphCallBindingDenial> {
        WorthQueryGraphCommitCall::mint(self, spec, execution_resources, resource_envelope)
    }
}
