use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    declaration_intake::ForgeServerDirectDeclarationIntakeFacade, CompatHttpSurfaceRoot,
    ForgeServerMiddlewareFacade, ForgeServerOperatorEvidenceFacade, ForgeServerQueryHandoffFacade,
    ForgeServerRequestContextFacade, ForgeServerResponseFacade,
};

use super::{
    mutation_execution::ForgeServerStoredCompatibilityMutation,
    upload_execution::ForgeServerStoredBinaryIngress,
};

#[derive(Clone, Debug)]
pub struct ForgeServerCompatibilityFacade {
    pub(super) root: CompatHttpSurfaceRoot,
    pub(super) request_contexts: ForgeServerRequestContextFacade,
    pub(super) middleware: ForgeServerMiddlewareFacade,
    pub(super) declaration_intake: ForgeServerDirectDeclarationIntakeFacade,
    pub(super) query_handoff: ForgeServerQueryHandoffFacade,
    pub(super) responses: ForgeServerResponseFacade,
    pub(super) operator_evidence: ForgeServerOperatorEvidenceFacade,
    pub(super) idempotency_store:
        Arc<Mutex<HashMap<String, ForgeServerStoredCompatibilityMutation>>>,
    pub(super) binary_ingress_store: Arc<Mutex<HashMap<String, ForgeServerStoredBinaryIngress>>>,
}

impl ForgeServerCompatibilityFacade {
    pub(crate) fn new(
        root: CompatHttpSurfaceRoot,
        request_contexts: ForgeServerRequestContextFacade,
        middleware: ForgeServerMiddlewareFacade,
        declaration_intake: ForgeServerDirectDeclarationIntakeFacade,
        query_handoff: ForgeServerQueryHandoffFacade,
        responses: ForgeServerResponseFacade,
        operator_evidence: ForgeServerOperatorEvidenceFacade,
        idempotency_store: Arc<Mutex<HashMap<String, ForgeServerStoredCompatibilityMutation>>>,
        binary_ingress_store: Arc<Mutex<HashMap<String, ForgeServerStoredBinaryIngress>>>,
    ) -> Self {
        Self {
            root,
            request_contexts,
            middleware,
            declaration_intake,
            query_handoff,
            responses,
            operator_evidence,
            idempotency_store,
            binary_ingress_store,
        }
    }

    pub fn root(&self) -> &CompatHttpSurfaceRoot {
        &self.root
    }
}
