use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    declaration_intake::ForgeServerDirectDeclarationIntakeFacade, CompatHttpSurfaceRoot,
    ForgeServerMiddlewareFacade, ForgeServerQueryHandoffFacade, ForgeServerRequestContextFacade,
    ForgeServerResponseFacade,
};

use super::mutation_execution::ForgeServerStoredCompatibilityMutation;

#[derive(Clone, Debug)]
pub struct ForgeServerCompatibilityFacade {
    pub(super) root: CompatHttpSurfaceRoot,
    pub(super) request_contexts: ForgeServerRequestContextFacade,
    pub(super) middleware: ForgeServerMiddlewareFacade,
    pub(super) declaration_intake: ForgeServerDirectDeclarationIntakeFacade,
    pub(super) query_handoff: ForgeServerQueryHandoffFacade,
    pub(super) responses: ForgeServerResponseFacade,
    pub(super) idempotency_store:
        Arc<Mutex<HashMap<String, ForgeServerStoredCompatibilityMutation>>>,
}

impl ForgeServerCompatibilityFacade {
    pub(crate) fn new(
        root: CompatHttpSurfaceRoot,
        request_contexts: ForgeServerRequestContextFacade,
        middleware: ForgeServerMiddlewareFacade,
        declaration_intake: ForgeServerDirectDeclarationIntakeFacade,
        query_handoff: ForgeServerQueryHandoffFacade,
        responses: ForgeServerResponseFacade,
        idempotency_store: Arc<Mutex<HashMap<String, ForgeServerStoredCompatibilityMutation>>>,
    ) -> Self {
        Self {
            root,
            request_contexts,
            middleware,
            declaration_intake,
            query_handoff,
            responses,
            idempotency_store,
        }
    }

    pub fn root(&self) -> &CompatHttpSurfaceRoot {
        &self.root
    }
}
