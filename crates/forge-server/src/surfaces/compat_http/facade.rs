use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    declaration_intake::ForgeServerDirectDeclarationIntakeFacade,
    product_operation_contract::ForgeServerStoredProductOperation, CompatHttpSurfaceRoot,
    ForgeServerMiddlewareFacade, ForgeServerOperationDenial, ForgeServerOperationFamily,
    ForgeServerOperationRegistry, ForgeServerOperatorEvidenceFacade,
    ForgeServerProductAdapterRegistry, ForgeServerProductSessionRegistry,
    ForgeServerQueryHandoffDenial, ForgeServerQueryHandoffDenialCode,
    ForgeServerQueryHandoffFacade, ForgeServerRequestContextFacade, ForgeServerResponseFacade,
    ForgeServerSurfaceFamily,
};

use super::{
    mutation_execution::ForgeServerStoredCompatibilityMutation,
    upload_execution::ForgeServerStoredBinaryIngress,
};

#[derive(Clone, Debug)]
pub struct ForgeServerCompatibilityFacade {
    pub(super) root: CompatHttpSurfaceRoot,
    pub(super) operation_registry: ForgeServerOperationRegistry,
    pub(super) product_adapter_registry: ForgeServerProductAdapterRegistry,
    pub(super) product_session_registry: ForgeServerProductSessionRegistry,
    pub(super) request_contexts: ForgeServerRequestContextFacade,
    pub(super) middleware: ForgeServerMiddlewareFacade,
    pub(super) declaration_intake: ForgeServerDirectDeclarationIntakeFacade,
    pub(super) query_handoff: ForgeServerQueryHandoffFacade,
    pub(super) responses: ForgeServerResponseFacade,
    pub(super) operator_evidence: ForgeServerOperatorEvidenceFacade,
    pub(super) idempotency_store:
        Arc<Mutex<HashMap<String, ForgeServerStoredCompatibilityMutation>>>,
    pub(super) product_operation_replay_store:
        Arc<Mutex<HashMap<String, ForgeServerStoredProductOperation>>>,
    pub(super) binary_ingress_store: Arc<Mutex<HashMap<String, ForgeServerStoredBinaryIngress>>>,
}

impl ForgeServerCompatibilityFacade {
    pub(crate) fn new(
        root: CompatHttpSurfaceRoot,
        operation_registry: ForgeServerOperationRegistry,
        product_adapter_registry: ForgeServerProductAdapterRegistry,
        product_session_registry: ForgeServerProductSessionRegistry,
        request_contexts: ForgeServerRequestContextFacade,
        middleware: ForgeServerMiddlewareFacade,
        declaration_intake: ForgeServerDirectDeclarationIntakeFacade,
        query_handoff: ForgeServerQueryHandoffFacade,
        responses: ForgeServerResponseFacade,
        operator_evidence: ForgeServerOperatorEvidenceFacade,
        idempotency_store: Arc<Mutex<HashMap<String, ForgeServerStoredCompatibilityMutation>>>,
        product_operation_replay_store: Arc<
            Mutex<HashMap<String, ForgeServerStoredProductOperation>>,
        >,
        binary_ingress_store: Arc<Mutex<HashMap<String, ForgeServerStoredBinaryIngress>>>,
    ) -> Self {
        Self {
            root,
            operation_registry,
            product_adapter_registry,
            product_session_registry,
            request_contexts,
            middleware,
            declaration_intake,
            query_handoff,
            responses,
            operator_evidence,
            idempotency_store,
            product_operation_replay_store,
            binary_ingress_store,
        }
    }

    pub fn root(&self) -> &CompatHttpSurfaceRoot {
        &self.root
    }

    pub fn operation_registry(&self) -> &ForgeServerOperationRegistry {
        &self.operation_registry
    }

    pub fn product_operations(&self) -> super::ForgeServerCompatibilityProductOperationFacade {
        super::ForgeServerCompatibilityProductOperationFacade::new(
            self.operation_registry.clone(),
            self.product_adapter_registry.clone(),
            self.query_handoff.config().clone(),
            self.product_session_registry.clone(),
            self.product_operation_replay_store.clone(),
        )
    }

    pub fn product_sessions(&self) -> super::ForgeServerCompatibilityProductSessionFacade {
        super::ForgeServerCompatibilityProductSessionFacade::new(
            self.operation_registry.clone(),
            self.product_session_registry.clone(),
        )
    }

    pub(super) fn admit_operation_family_for_query(
        &self,
        diagnostics_profile: forge_foundational::DiagnosticRichnessProfile,
        family: ForgeServerOperationFamily,
    ) -> Result<(), ForgeServerQueryHandoffDenial> {
        self.operation_registry
            .admit(ForgeServerSurfaceFamily::CompatHttp, family)
            .map(|_| ())
            .map_err(|denial| {
                let detail = denial.detail();
                let query_denial = ForgeServerQueryHandoffDenial::new(
                    map_operation_denial_to_query_code(&denial),
                    diagnostics_profile,
                    detail,
                );
                match denial {
                    ForgeServerOperationDenial::UnknownOperationName { operation_name, .. } => {
                        query_denial.with_facts(
                            crate::ForgeServerQueryHandoffDenialFacts::default()
                                .with_rejected_operation_name(operation_name),
                        )
                    }
                    _ => query_denial,
                }
            })
    }
}

pub(crate) fn map_operation_admission_denial(
    denial: crate::ForgeServerOperationAdmissionDenial,
) -> ForgeServerQueryHandoffDenial {
    let code = match denial.code() {
        crate::ForgeServerOperationAdmissionDenialCode::AuthorityDenied => {
            ForgeServerQueryHandoffDenialCode::AuthorityDenied
        }
        crate::ForgeServerOperationAdmissionDenialCode::AuthorizationDenied => {
            ForgeServerQueryHandoffDenialCode::AuthorizationDenied
        }
    };
    ForgeServerQueryHandoffDenial::new(code, denial.diagnostics_profile(), denial.detail())
}

fn map_operation_denial_to_query_code(
    denial: &ForgeServerOperationDenial,
) -> ForgeServerQueryHandoffDenialCode {
    match denial {
        ForgeServerOperationDenial::UnregisteredFamily { .. } => {
            ForgeServerQueryHandoffDenialCode::OperationFamilyNotRegistered
        }
        ForgeServerOperationDenial::DisabledFamily { .. } => {
            ForgeServerQueryHandoffDenialCode::OperationFamilyDisabled
        }
        ForgeServerOperationDenial::SurfaceFamilyNotExposed { .. } => {
            ForgeServerQueryHandoffDenialCode::OperationFamilyNotExposedOnSurface
        }
        ForgeServerOperationDenial::UnknownOperationName { .. } => {
            ForgeServerQueryHandoffDenialCode::UnknownOperationName
        }
    }
}
