use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use worth_proof::{TransitionOutcome, TransitionReadiness};

use crate::{
    declaration_intake::WorthServerDirectDeclarationIntakeFacade,
    operation_registry::WorthServerOperationRegistry,
    product_operation_contract::WorthServerStoredProductOperation,
    worth_native::{
        denial::{
            WorthServerWorthNativeDeferred, WorthServerWorthNativeFailure,
            WorthServerWorthNativeRebindRequired, WorthServerWorthNativeSessionDenial,
            WorthServerWorthNativeSessionDenialCode, WorthServerWorthNativeStale,
        },
        input::{RawWorthServerWorthNativeBranchTarget, WorthServerWorthNativeSessionInput},
        WorthServerWorthNativePreparedSession, WorthServerWorthNativeSession,
        WorthServerWorthNativeSurfaceRoot,
    },
    WorthServerMiddlewareFacade, WorthServerPipelineInput, WorthServerPipelineIntent,
    WorthServerProductAdapterRegistry, WorthServerProductSessionRegistry,
    WorthServerQueryHandoffFacade, WorthServerRequestContextFacade, WorthServerRequestContextInput,
    WorthServerResponseFacade, WorthServerSurfaceFamily, WorthServerTransportClass,
};

pub type WorthServerWorthNativePreparationOutcome = TransitionOutcome<
    WorthServerWorthNativePreparedSession,
    WorthServerWorthNativeSessionDenial,
    WorthServerWorthNativeDeferred,
    WorthServerWorthNativeStale,
    WorthServerWorthNativeRebindRequired,
    WorthServerWorthNativeFailure,
>;

pub type WorthServerWorthNativeSessionOutcome = TransitionOutcome<
    WorthServerWorthNativeSession,
    WorthServerWorthNativeSessionDenial,
    WorthServerWorthNativeDeferred,
    WorthServerWorthNativeStale,
    WorthServerWorthNativeRebindRequired,
    WorthServerWorthNativeFailure,
>;

#[derive(Clone, Debug)]
pub struct WorthServerWorthNativeFacade {
    root: WorthServerWorthNativeSurfaceRoot,
    operation_registry: WorthServerOperationRegistry,
    product_adapter_registry: WorthServerProductAdapterRegistry,
    product_session_registry: WorthServerProductSessionRegistry,
    product_operation_retry_store: Arc<Mutex<HashMap<String, WorthServerStoredProductOperation>>>,
    counters: Arc<crate::diagnostics::WorthServerCounters>,
    request_contexts: WorthServerRequestContextFacade,
    middleware: WorthServerMiddlewareFacade,
    declaration_intake: WorthServerDirectDeclarationIntakeFacade,
    query_handoff: WorthServerQueryHandoffFacade,
    responses: WorthServerResponseFacade,
}

pub(crate) struct WorthServerWorthNativeFacadeParts {
    pub(crate) root: WorthServerWorthNativeSurfaceRoot,
    pub(crate) operation_registry: WorthServerOperationRegistry,
    pub(crate) product_adapter_registry: WorthServerProductAdapterRegistry,
    pub(crate) product_session_registry: WorthServerProductSessionRegistry,
    pub(crate) product_operation_retry_store:
        Arc<Mutex<HashMap<String, WorthServerStoredProductOperation>>>,
    pub(crate) counters: Arc<crate::diagnostics::WorthServerCounters>,
    pub(crate) request_contexts: WorthServerRequestContextFacade,
    pub(crate) middleware: WorthServerMiddlewareFacade,
    pub(crate) declaration_intake: WorthServerDirectDeclarationIntakeFacade,
    pub(crate) query_handoff: WorthServerQueryHandoffFacade,
    pub(crate) responses: WorthServerResponseFacade,
}

impl WorthServerWorthNativeFacade {
    pub(crate) fn new(parts: WorthServerWorthNativeFacadeParts) -> Self {
        let WorthServerWorthNativeFacadeParts {
            root,
            operation_registry,
            product_adapter_registry,
            product_session_registry,
            product_operation_retry_store,
            counters,
            request_contexts,
            middleware,
            declaration_intake,
            query_handoff,
            responses,
        } = parts;
        Self {
            root,
            operation_registry,
            product_adapter_registry,
            product_session_registry,
            product_operation_retry_store,
            counters,
            request_contexts,
            middleware,
            declaration_intake,
            query_handoff,
            responses,
        }
    }

    pub fn root(&self) -> WorthServerWorthNativeSurfaceRoot {
        self.root
    }

    pub fn operation_registry(&self) -> &WorthServerOperationRegistry {
        &self.operation_registry
    }

    pub fn prepare_session(
        &self,
        input: WorthServerWorthNativeSessionInput,
    ) -> WorthServerWorthNativePreparationOutcome {
        if self.root.capabilities().is_absent() {
            return TransitionOutcome::denied(WorthServerWorthNativeSessionDenial::new(
                WorthServerWorthNativeSessionDenialCode::WorthNativeSurfaceAbsent,
                default_diagnostics_profile(&input, &self.request_contexts),
                "Worth-native surface family is not registered on this server",
            ));
        }
        if self.root.capabilities().is_disabled() {
            return TransitionOutcome::denied(WorthServerWorthNativeSessionDenial::new(
                WorthServerWorthNativeSessionDenialCode::WorthNativeSurfaceDisabled,
                default_diagnostics_profile(&input, &self.request_contexts),
                "Worth-native surface family is registered but disabled on this server",
            ));
        }

        let request_context_input = lower_request_context_input(input);
        let resolved_request_context = match self.request_contexts.resolve(request_context_input) {
            TransitionReadiness::Ready(resolved_request_context) => resolved_request_context,
            TransitionReadiness::Denied(denial) => return TransitionOutcome::denied(denial.into()),
            TransitionReadiness::Deferred(reason) => {
                return TransitionOutcome::deferred(
                    WorthServerWorthNativeDeferred::RequestContext(reason),
                );
            }
            TransitionReadiness::Stale(reason) => {
                return TransitionOutcome::stale(WorthServerWorthNativeStale::RequestContext(
                    reason,
                ));
            }
            TransitionReadiness::RebindRequired(reason) => {
                return TransitionOutcome::rebind_required(
                    WorthServerWorthNativeRebindRequired::RequestContext(reason),
                );
            }
            TransitionReadiness::Failed(reason) => {
                return TransitionOutcome::failed(WorthServerWorthNativeFailure::RequestContext(
                    reason,
                ));
            }
        };

        let admission_outcome = self.middleware.admit(WorthServerPipelineInput::new(
            resolved_request_context,
            WorthServerPipelineIntent::worth_native_session("worth_native.session"),
        ));
        match admission_outcome {
            TransitionOutcome::Success(admission) => {
                TransitionOutcome::success(WorthServerWorthNativePreparedSession::new(
                    admission,
                    super::WorthServerWorthNativeSessionServices::new(
                        super::WorthServerWorthNativeSessionServiceParts {
                            operation_registry: self.operation_registry.clone(),
                            product_adapter_registry: self.product_adapter_registry.clone(),
                            product_session_registry: self.product_session_registry.clone(),
                            product_operation_retry_store: self
                                .product_operation_retry_store
                                .clone(),
                            counters: self.counters.clone(),
                            query_handoff_config: self.query_handoff.config().clone(),
                            declaration_intake: self.declaration_intake.clone(),
                            query_handoff: self.query_handoff.clone(),
                            responses: self.responses.clone(),
                        },
                    ),
                ))
            }
            TransitionOutcome::Denied(denial) => TransitionOutcome::denied(denial.into()),
            TransitionOutcome::Deferred(reason) => {
                TransitionOutcome::deferred(WorthServerWorthNativeDeferred::Middleware(reason))
            }
            TransitionOutcome::Stale(reason) => {
                TransitionOutcome::stale(WorthServerWorthNativeStale::Middleware(reason))
            }
            TransitionOutcome::RebindRequired(reason) => TransitionOutcome::rebind_required(
                WorthServerWorthNativeRebindRequired::Middleware(reason),
            ),
            TransitionOutcome::Failed(reason) => {
                TransitionOutcome::failed(WorthServerWorthNativeFailure::Middleware(reason))
            }
        }
    }

    pub fn session(
        &self,
        input: WorthServerWorthNativeSessionInput,
    ) -> WorthServerWorthNativeSessionOutcome {
        self.prepare_session(input)
            .map_success(WorthServerWorthNativePreparedSession::into_session)
    }
}

fn lower_request_context_input(
    input: WorthServerWorthNativeSessionInput,
) -> WorthServerRequestContextInput {
    let mut builder = WorthServerRequestContextInput::builder()
        .with_surface_family(WorthServerSurfaceFamily::WorthNative)
        .with_transport_class(WorthServerTransportClass::WorthNativeInProcess)
        .with_authenticated_principal_id(input.authenticated_principal_id())
        .with_tenant_id(input.tenant_id())
        .with_workspace_id(input.workspace_id());

    builder = match input.branch_target() {
        RawWorthServerWorthNativeBranchTarget::Main => builder.with_main_branch(),
        RawWorthServerWorthNativeBranchTarget::Branch { branch_id } => {
            builder.with_branch_id(branch_id)
        }
        RawWorthServerWorthNativeBranchTarget::Preview { preview_id } => {
            builder.with_preview_id(preview_id)
        }
    };

    if let Some(diagnostics_profile) = input.diagnostics_profile() {
        builder = builder.with_diagnostics_profile(diagnostics_profile);
    }

    builder
        .build()
        .expect("Worth-native session input lowering must provide all fixed server-owned fields")
}

fn default_diagnostics_profile(
    input: &WorthServerWorthNativeSessionInput,
    request_contexts: &WorthServerRequestContextFacade,
) -> crate::request_context::DiagnosticRichnessProfile {
    input
        .diagnostics_profile()
        .unwrap_or(request_contexts.default_diagnostics_profile())
}
