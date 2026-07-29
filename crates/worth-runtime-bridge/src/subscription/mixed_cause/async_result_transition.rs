use std::sync::Arc;

use crate::source::{
    AdmittedBridgeAsyncCompletion, AdmittedBridgeAsyncRequestIdentity,
    BridgeAsyncClassifiedDeniedCompletion, BridgeAsyncCompletionState,
    BridgeAsyncCompletionSupersessionClass, BridgeAsyncForwardCausalityClass,
    BridgeAsyncRequestIdentity, BridgeAsyncRequestRuntimeIdentity, BridgeAsyncRetryLineage,
    BridgeAsyncRevalidationLineage, BridgeAsyncSourceDeclarationIdentity,
};

use super::ordering::BridgeMixedCauseDeniedKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BridgeMixedCauseAsyncResultCause {
    Completion(BridgeAsyncCompletionState),
    ClassifiedDenied {
        completion: BridgeAsyncCompletionState,
        supersession: BridgeAsyncCompletionSupersessionClass,
    },
    Retry(BridgeAsyncForwardCausalityClass),
    Revalidation(BridgeAsyncForwardCausalityClass),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BridgeMixedCauseAsyncResultDisposition {
    Ordered { ordinal: usize },
    DuplicateSuppressed,
    DeliveryDenied(BridgeMixedCauseDeniedKind),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeMixedCauseAsyncResultTransition {
    cause: BridgeMixedCauseAsyncResultCause,
    disposition: BridgeMixedCauseAsyncResultDisposition,
    source_identity: Arc<str>,
    source_digest: Arc<str>,
    request_identity: BridgeAsyncRequestIdentity,
    predecessor_request_identity: Option<BridgeAsyncRequestIdentity>,
    declaration_identity: BridgeAsyncSourceDeclarationIdentity,
    bridge_runtime_identity: BridgeAsyncRequestRuntimeIdentity,
    truth_view_basis_digest: Arc<str>,
    request_generation: u64,
    request_attempt: u64,
}

#[derive(Clone, Debug)]
pub(super) struct BridgeMixedCauseAsyncResultTransitionSeed {
    cause: BridgeMixedCauseAsyncResultCause,
    source_identity: Arc<str>,
    source_digest: Arc<str>,
    request_identity: BridgeAsyncRequestIdentity,
    predecessor_request_identity: Option<BridgeAsyncRequestIdentity>,
    declaration_identity: BridgeAsyncSourceDeclarationIdentity,
    bridge_runtime_identity: BridgeAsyncRequestRuntimeIdentity,
    truth_view_basis_digest: Arc<str>,
    request_generation: u64,
    request_attempt: u64,
}

impl BridgeMixedCauseAsyncResultTransitionSeed {
    pub(super) fn from_completion(completion: &AdmittedBridgeAsyncCompletion) -> Self {
        Self::from_request(
            BridgeMixedCauseAsyncResultCause::Completion(completion.state()),
            completion.completion_identity(),
            completion.digest(),
            completion.request_identity(),
            None,
        )
    }

    pub(super) fn from_classified_denied(denied: &BridgeAsyncClassifiedDeniedCompletion) -> Self {
        let completion = denied.denied_completion();
        Self::from_request(
            BridgeMixedCauseAsyncResultCause::ClassifiedDenied {
                completion: completion.state(),
                supersession: denied.supersession_class(),
            },
            completion.denial_identity(),
            denied.receipt().digest(),
            completion.request_identity(),
            None,
        )
    }

    pub(super) fn from_retry(lineage: &BridgeAsyncRetryLineage) -> Self {
        Self::from_request(
            BridgeMixedCauseAsyncResultCause::Retry(lineage.class()),
            lineage.causality_identity(),
            lineage.digest(),
            lineage.newer_request(),
            Some(lineage.prior_request().request_identity()),
        )
    }

    pub(super) fn from_revalidation(lineage: &BridgeAsyncRevalidationLineage) -> Self {
        Self::from_request(
            BridgeMixedCauseAsyncResultCause::Revalidation(lineage.class()),
            lineage.causality_identity(),
            lineage.digest(),
            lineage.newer_request(),
            Some(lineage.prior_request().request_identity()),
        )
    }

    fn from_request(
        cause: BridgeMixedCauseAsyncResultCause,
        source_identity: &str,
        source_digest: &str,
        request: &AdmittedBridgeAsyncRequestIdentity,
        predecessor_request_identity: Option<&BridgeAsyncRequestIdentity>,
    ) -> Self {
        Self {
            cause,
            source_identity: Arc::from(source_identity.to_owned()),
            source_digest: Arc::from(source_digest.to_owned()),
            request_identity: request.request_identity().clone(),
            predecessor_request_identity: predecessor_request_identity.cloned(),
            declaration_identity: request.lowered().declaration_identity().clone(),
            bridge_runtime_identity: request.bridge_runtime_identity(),
            truth_view_basis_digest: Arc::from(
                request
                    .basis_binding()
                    .truth_view_basis()
                    .digest()
                    .to_owned(),
            ),
            request_generation: request.request_handle().generation().get(),
            request_attempt: request.attempt().get(),
        }
    }

    pub(super) fn admit(
        &self,
        disposition: BridgeMixedCauseAsyncResultDisposition,
    ) -> BridgeMixedCauseAsyncResultTransition {
        BridgeMixedCauseAsyncResultTransition {
            cause: self.cause,
            disposition,
            source_identity: self.source_identity.clone(),
            source_digest: self.source_digest.clone(),
            request_identity: self.request_identity.clone(),
            predecessor_request_identity: self.predecessor_request_identity.clone(),
            declaration_identity: self.declaration_identity.clone(),
            bridge_runtime_identity: self.bridge_runtime_identity,
            truth_view_basis_digest: self.truth_view_basis_digest.clone(),
            request_generation: self.request_generation,
            request_attempt: self.request_attempt,
        }
    }
}

impl BridgeMixedCauseAsyncResultTransition {
    pub fn cause(&self) -> BridgeMixedCauseAsyncResultCause {
        self.cause
    }

    pub fn disposition(&self) -> BridgeMixedCauseAsyncResultDisposition {
        self.disposition
    }

    pub fn source_identity(&self) -> &str {
        self.source_identity.as_ref()
    }

    pub fn source_digest(&self) -> &str {
        self.source_digest.as_ref()
    }

    pub fn request_identity(&self) -> &str {
        self.request_identity.as_str()
    }

    pub fn predecessor_request_identity(&self) -> Option<&str> {
        self.predecessor_request_identity
            .as_ref()
            .map(BridgeAsyncRequestIdentity::as_str)
    }

    pub fn declaration_identity(&self) -> &str {
        self.declaration_identity.as_str()
    }

    pub fn request_identity_reference(&self) -> &BridgeAsyncRequestIdentity {
        &self.request_identity
    }

    pub fn predecessor_request_identity_reference(&self) -> Option<&BridgeAsyncRequestIdentity> {
        self.predecessor_request_identity.as_ref()
    }

    pub fn declaration_identity_reference(&self) -> &BridgeAsyncSourceDeclarationIdentity {
        &self.declaration_identity
    }

    pub fn bridge_runtime_identity(&self) -> BridgeAsyncRequestRuntimeIdentity {
        self.bridge_runtime_identity
    }

    pub fn truth_view_basis_digest(&self) -> &str {
        self.truth_view_basis_digest.as_ref()
    }

    pub fn request_generation(&self) -> u64 {
        self.request_generation
    }

    pub fn request_attempt(&self) -> u64 {
        self.request_attempt
    }
}
