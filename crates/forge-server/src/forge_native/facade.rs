use forge_proof::{TransitionOutcome, TransitionReadiness};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    declaration_intake::ForgeServerDirectDeclarationIntakeFacade,
    forge_native::{
        denial::{
            ForgeServerForgeNativeDeferred, ForgeServerForgeNativeFailure,
            ForgeServerForgeNativeRebindRequired, ForgeServerForgeNativeSessionDenial,
            ForgeServerForgeNativeSessionDenialCode, ForgeServerForgeNativeStale,
        },
        input::{ForgeServerForgeNativeSessionInput, RawForgeServerForgeNativeBranchTarget},
        ForgeServerForgeNativePreparedSession, ForgeServerForgeNativeSession,
        ForgeServerForgeNativeSurfaceRoot,
    },
    operation_registry::ForgeServerOperationRegistry,
    product_operation_contract::ForgeServerStoredProductOperation,
    ForgeServerMiddlewareFacade, ForgeServerPipelineInput, ForgeServerPipelineIntent,
    ForgeServerProductAdapterRegistry, ForgeServerProductSessionRegistry,
    ForgeServerQueryHandoffFacade, ForgeServerRequestContextFacade, ForgeServerRequestContextInput,
    ForgeServerResponseFacade, ForgeServerSurfaceFamily, ForgeServerTransportClass,
};

pub type ForgeServerForgeNativePreparationOutcome = TransitionOutcome<
    ForgeServerForgeNativePreparedSession,
    ForgeServerForgeNativeSessionDenial,
    ForgeServerForgeNativeDeferred,
    ForgeServerForgeNativeStale,
    ForgeServerForgeNativeRebindRequired,
    ForgeServerForgeNativeFailure,
>;

pub type ForgeServerForgeNativeSessionOutcome = TransitionOutcome<
    ForgeServerForgeNativeSession,
    ForgeServerForgeNativeSessionDenial,
    ForgeServerForgeNativeDeferred,
    ForgeServerForgeNativeStale,
    ForgeServerForgeNativeRebindRequired,
    ForgeServerForgeNativeFailure,
>;

#[derive(Clone, Debug)]
pub struct ForgeServerForgeNativeFacade {
    root: ForgeServerForgeNativeSurfaceRoot,
    operation_registry: ForgeServerOperationRegistry,
    product_adapter_registry: ForgeServerProductAdapterRegistry,
    product_session_registry: ForgeServerProductSessionRegistry,
    product_operation_replay_store: Arc<Mutex<HashMap<String, ForgeServerStoredProductOperation>>>,
    request_contexts: ForgeServerRequestContextFacade,
    middleware: ForgeServerMiddlewareFacade,
    declaration_intake: ForgeServerDirectDeclarationIntakeFacade,
    query_handoff: ForgeServerQueryHandoffFacade,
    responses: ForgeServerResponseFacade,
}

impl ForgeServerForgeNativeFacade {
    pub(crate) fn new(
        root: ForgeServerForgeNativeSurfaceRoot,
        operation_registry: ForgeServerOperationRegistry,
        product_adapter_registry: ForgeServerProductAdapterRegistry,
        product_session_registry: ForgeServerProductSessionRegistry,
        product_operation_replay_store: Arc<
            Mutex<HashMap<String, ForgeServerStoredProductOperation>>,
        >,
        request_contexts: ForgeServerRequestContextFacade,
        middleware: ForgeServerMiddlewareFacade,
        declaration_intake: ForgeServerDirectDeclarationIntakeFacade,
        query_handoff: ForgeServerQueryHandoffFacade,
        responses: ForgeServerResponseFacade,
    ) -> Self {
        Self {
            root,
            operation_registry,
            product_adapter_registry,
            product_session_registry,
            product_operation_replay_store,
            request_contexts,
            middleware,
            declaration_intake,
            query_handoff,
            responses,
        }
    }

    pub fn root(&self) -> ForgeServerForgeNativeSurfaceRoot {
        self.root
    }

    pub fn operation_registry(&self) -> &ForgeServerOperationRegistry {
        &self.operation_registry
    }

    pub fn prepare_session(
        &self,
        input: ForgeServerForgeNativeSessionInput,
    ) -> ForgeServerForgeNativePreparationOutcome {
        if self.root.capabilities().is_absent() {
            return TransitionOutcome::denied(ForgeServerForgeNativeSessionDenial::new(
                ForgeServerForgeNativeSessionDenialCode::ForgeNativeSurfaceAbsent,
                default_diagnostics_profile(&input, &self.request_contexts),
                "forge-native surface family is not registered on this server",
            ));
        }
        if self.root.capabilities().is_disabled() {
            return TransitionOutcome::denied(ForgeServerForgeNativeSessionDenial::new(
                ForgeServerForgeNativeSessionDenialCode::ForgeNativeSurfaceDisabled,
                default_diagnostics_profile(&input, &self.request_contexts),
                "forge-native surface family is registered but disabled on this server",
            ));
        }

        let request_context_input = lower_request_context_input(input);
        let resolved_request_context = match self.request_contexts.resolve(request_context_input) {
            TransitionReadiness::Ready(resolved_request_context) => resolved_request_context,
            TransitionReadiness::Denied(denial) => return TransitionOutcome::denied(denial.into()),
            TransitionReadiness::Deferred(reason) => {
                return TransitionOutcome::deferred(
                    ForgeServerForgeNativeDeferred::RequestContext(reason),
                );
            }
            TransitionReadiness::Stale(reason) => {
                return TransitionOutcome::stale(ForgeServerForgeNativeStale::RequestContext(
                    reason,
                ));
            }
            TransitionReadiness::RebindRequired(reason) => {
                return TransitionOutcome::rebind_required(
                    ForgeServerForgeNativeRebindRequired::RequestContext(reason),
                );
            }
            TransitionReadiness::Failed(reason) => {
                return TransitionOutcome::failed(ForgeServerForgeNativeFailure::RequestContext(
                    reason,
                ));
            }
        };

        let admission_outcome = self.middleware.admit(ForgeServerPipelineInput::new(
            resolved_request_context,
            ForgeServerPipelineIntent::forge_native_session("forge_native.session"),
        ));
        match admission_outcome {
            TransitionOutcome::Success(admission) => {
                TransitionOutcome::success(ForgeServerForgeNativePreparedSession::new(
                    admission,
                    self.operation_registry.clone(),
                    self.product_adapter_registry.clone(),
                    self.product_session_registry.clone(),
                    self.product_operation_replay_store.clone(),
                    self.query_handoff.config().clone(),
                    self.declaration_intake.clone(),
                    self.query_handoff.clone(),
                    self.responses.clone(),
                ))
            }
            TransitionOutcome::Denied(denial) => TransitionOutcome::denied(denial.into()),
            TransitionOutcome::Deferred(reason) => {
                TransitionOutcome::deferred(ForgeServerForgeNativeDeferred::Middleware(reason))
            }
            TransitionOutcome::Stale(reason) => {
                TransitionOutcome::stale(ForgeServerForgeNativeStale::Middleware(reason))
            }
            TransitionOutcome::RebindRequired(reason) => TransitionOutcome::rebind_required(
                ForgeServerForgeNativeRebindRequired::Middleware(reason),
            ),
            TransitionOutcome::Failed(reason) => {
                TransitionOutcome::failed(ForgeServerForgeNativeFailure::Middleware(reason))
            }
        }
    }

    pub fn session(
        &self,
        input: ForgeServerForgeNativeSessionInput,
    ) -> ForgeServerForgeNativeSessionOutcome {
        self.prepare_session(input)
            .map_success(ForgeServerForgeNativePreparedSession::into_session)
    }
}

fn lower_request_context_input(
    input: ForgeServerForgeNativeSessionInput,
) -> ForgeServerRequestContextInput {
    let mut builder = ForgeServerRequestContextInput::builder()
        .with_surface_family(ForgeServerSurfaceFamily::ForgeNative)
        .with_transport_class(ForgeServerTransportClass::ForgeNativeInProcess)
        .with_authenticated_principal_id(input.authenticated_principal_id())
        .with_tenant_id(input.tenant_id())
        .with_workspace_id(input.workspace_id());

    builder = match input.branch_target() {
        RawForgeServerForgeNativeBranchTarget::Main => builder.with_main_branch(),
        RawForgeServerForgeNativeBranchTarget::Branch { branch_id } => {
            builder.with_branch_id(branch_id)
        }
        RawForgeServerForgeNativeBranchTarget::Preview { preview_id } => {
            builder.with_preview_id(preview_id)
        }
    };

    if let Some(diagnostics_profile) = input.diagnostics_profile() {
        builder = builder.with_diagnostics_profile(diagnostics_profile);
    }

    builder
        .build()
        .expect("forge-native session input lowering must provide all fixed server-owned fields")
}

fn default_diagnostics_profile(
    input: &ForgeServerForgeNativeSessionInput,
    request_contexts: &ForgeServerRequestContextFacade,
) -> crate::request_context::DiagnosticRichnessProfile {
    input
        .diagnostics_profile()
        .unwrap_or(request_contexts.default_diagnostics_profile())
}
