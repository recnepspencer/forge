use std::sync::Arc;

use worth_runtime_bridge::facade::{
    AdmittedBridgeAsyncRequestIdentity, BridgeAsyncRequestIdentity,
    BridgeAsyncRequestRuntimeIdentity, BridgeAsyncSourceDeclarationIdentity,
    BridgeMixedCauseAsyncResultTransition,
};

use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};

use super::WorthQueryRuntimeAsyncResultState;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryAsyncSourceBindingErrorKind {
    MissingLiveSubscription,
    MissingBinding,
    ForeignBridgeRuntime,
    ForeignDeclaration,
    ForeignRequest,
    PredecessorMismatch,
    InadmissibleDisposition,
    IllegalResultTransition,
    ProjectionFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryAsyncSourceBindingError {
    kind: WorthQueryAsyncSourceBindingErrorKind,
    detail: Arc<str>,
}

impl WorthQueryAsyncSourceBindingError {
    pub(super) fn new(
        kind: WorthQueryAsyncSourceBindingErrorKind,
        detail: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> WorthQueryAsyncSourceBindingErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        self.detail.as_ref()
    }
}

#[must_use]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryAsyncResultTransitionBatch {
    binding_identity: WorthQueryEvidenceIdentity,
    states: Vec<WorthQueryRuntimeAsyncResultState>,
    suppressed_duplicate_count: usize,
}

impl WorthQueryAsyncResultTransitionBatch {
    pub(super) fn admitted(
        binding_identity: WorthQueryEvidenceIdentity,
        states: Vec<WorthQueryRuntimeAsyncResultState>,
        suppressed_duplicate_count: usize,
    ) -> Self {
        Self {
            binding_identity,
            states,
            suppressed_duplicate_count,
        }
    }

    pub fn binding_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.binding_identity
    }

    pub fn states(&self) -> &[WorthQueryRuntimeAsyncResultState] {
        &self.states
    }

    pub fn suppressed_duplicate_count(&self) -> usize {
        self.suppressed_duplicate_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WorthQueryRuntimeAsyncSourceBinding {
    binding_identity: WorthQueryEvidenceIdentity,
    bridge_runtime_identity: BridgeAsyncRequestRuntimeIdentity,
    declaration_identity: BridgeAsyncSourceDeclarationIdentity,
    current_request_identity: BridgeAsyncRequestIdentity,
    truth_view_basis_digest: Arc<str>,
    request_generation: u64,
    request_attempt: u64,
}

impl WorthQueryRuntimeAsyncSourceBinding {
    pub(super) fn admit(view_name: &str, request: &AdmittedBridgeAsyncRequestIdentity) -> Self {
        let declaration_identity = request.lowered().declaration_identity();
        let current_request_identity = request.request_identity();
        let binding_identity =
            worth_query_evidence_identity(WorthQueryEvidenceScope::RuntimeStateSnapshot)
                .field_shape(
                    WorthQueryEvidenceTag::new("identity_family"),
                    "worth_query_async_source_binding_v1",
                )
                .field_shape(WorthQueryEvidenceTag::new("live_target"), view_name)
                .field_shape(
                    WorthQueryEvidenceTag::new("bridge_declaration"),
                    request.lowered().declaration_identity_for_reporting(),
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("bridge_request"),
                    request.request_identity_for_reporting(),
                )
                .seal();
        Self {
            binding_identity,
            bridge_runtime_identity: request.bridge_runtime_identity(),
            declaration_identity: declaration_identity.clone(),
            current_request_identity: current_request_identity.clone(),
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

    pub(super) fn binding_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.binding_identity
    }

    pub(super) fn validate_and_advance(
        &mut self,
        transition: &BridgeMixedCauseAsyncResultTransition,
    ) -> Result<(), WorthQueryAsyncSourceBindingError> {
        if transition.bridge_runtime_identity() != self.bridge_runtime_identity {
            return Err(binding_error(
                WorthQueryAsyncSourceBindingErrorKind::ForeignBridgeRuntime,
                transition,
            ));
        }
        if transition.declaration_identity_reference() != &self.declaration_identity {
            return Err(binding_error(
                WorthQueryAsyncSourceBindingErrorKind::ForeignDeclaration,
                transition,
            ));
        }
        match transition.predecessor_request_identity_reference() {
            Some(predecessor) if predecessor != &self.current_request_identity => {
                return Err(binding_error(
                    WorthQueryAsyncSourceBindingErrorKind::PredecessorMismatch,
                    transition,
                ));
            }
            Some(_) => {
                self.current_request_identity = transition.request_identity_reference().clone();
                self.truth_view_basis_digest =
                    Arc::from(transition.truth_view_basis_digest().to_owned());
                self.request_generation = transition.request_generation();
                self.request_attempt = transition.request_attempt();
            }
            None if transition.request_identity_reference() != &self.current_request_identity => {
                return Err(binding_error(
                    WorthQueryAsyncSourceBindingErrorKind::ForeignRequest,
                    transition,
                ));
            }
            None => {}
        }
        Ok(())
    }
}

fn binding_error(
    kind: WorthQueryAsyncSourceBindingErrorKind,
    transition: &BridgeMixedCauseAsyncResultTransition,
) -> WorthQueryAsyncSourceBindingError {
    WorthQueryAsyncSourceBindingError::new(
        kind,
        format!(
            "bridge async transition `{}` for request `{}` did not match the live binding",
            transition.source_identity(),
            transition.request_identity(),
        ),
    )
}
