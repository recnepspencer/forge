use crate::{
    declaration_intake::ForgeServerDirectDeclarationIntakeFacade,
    forge_native::{
        declaration::ForgeServerForgeNativeDeclarationFacade, ForgeServerForgeNativeDirectFacade,
    },
    ForgeServerAdmission, ForgeServerQueryHandoffFacade, ForgeServerResolvedRequestContext,
    ForgeServerResponseFacade,
};

#[derive(Clone, Debug)]
pub struct ForgeServerForgeNativePreparedSession {
    admission: ForgeServerAdmission,
    declaration_intake: ForgeServerDirectDeclarationIntakeFacade,
    query_handoff: ForgeServerQueryHandoffFacade,
    responses: ForgeServerResponseFacade,
}

impl ForgeServerForgeNativePreparedSession {
    pub(crate) fn new(
        admission: ForgeServerAdmission,
        declaration_intake: ForgeServerDirectDeclarationIntakeFacade,
        query_handoff: ForgeServerQueryHandoffFacade,
        responses: ForgeServerResponseFacade,
    ) -> Self {
        Self {
            admission,
            declaration_intake,
            query_handoff,
            responses,
        }
    }

    pub fn resolved_request_context(&self) -> &ForgeServerResolvedRequestContext {
        self.admission.resolved_request_context()
    }

    pub fn admission(&self) -> &ForgeServerAdmission {
        &self.admission
    }

    pub fn declarations(&self) -> ForgeServerForgeNativeDeclarationFacade {
        ForgeServerForgeNativeDeclarationFacade::new(
            self.admission.clone(),
            self.declaration_intake.clone(),
        )
    }

    pub fn direct(&self) -> ForgeServerForgeNativeDirectFacade {
        ForgeServerForgeNativeDirectFacade::new(
            self.admission.clone(),
            self.declaration_intake.clone(),
            self.query_handoff.clone(),
            self.responses.clone(),
        )
    }

    pub fn into_session(self) -> ForgeServerForgeNativeSession {
        ForgeServerForgeNativeSession::new(
            self.admission,
            self.declaration_intake,
            self.query_handoff,
            self.responses,
        )
    }
}

#[derive(Clone, Debug)]
pub struct ForgeServerForgeNativeSession {
    admission: ForgeServerAdmission,
    declaration_intake: ForgeServerDirectDeclarationIntakeFacade,
    query_handoff: ForgeServerQueryHandoffFacade,
    responses: ForgeServerResponseFacade,
}

impl ForgeServerForgeNativeSession {
    pub(crate) fn new(
        admission: ForgeServerAdmission,
        declaration_intake: ForgeServerDirectDeclarationIntakeFacade,
        query_handoff: ForgeServerQueryHandoffFacade,
        responses: ForgeServerResponseFacade,
    ) -> Self {
        Self {
            admission,
            declaration_intake,
            query_handoff,
            responses,
        }
    }

    pub fn resolved_request_context(&self) -> &ForgeServerResolvedRequestContext {
        self.admission.resolved_request_context()
    }

    pub fn admission(&self) -> &ForgeServerAdmission {
        &self.admission
    }

    pub fn declarations(&self) -> ForgeServerForgeNativeDeclarationFacade {
        ForgeServerForgeNativeDeclarationFacade::new(
            self.admission.clone(),
            self.declaration_intake.clone(),
        )
    }

    pub fn direct(&self) -> ForgeServerForgeNativeDirectFacade {
        ForgeServerForgeNativeDirectFacade::new(
            self.admission.clone(),
            self.declaration_intake.clone(),
            self.query_handoff.clone(),
            self.responses.clone(),
        )
    }
}
