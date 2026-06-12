use crate::{
    ForgeServerAdmittedDirectDeclaration, ForgeServerDirectDeclaration,
    ForgeServerDirectInspectionOutcome, ForgeServerDirectProjectionOutcome,
    ForgeServerDirectProjectionRequest, ForgeServerDirectReadOutcome,
    ForgeServerDirectStateOutcome, ForgeServerForgeNativeDirectFacade,
};

use super::{ForgeServerDirectDeclarationSnapshot, ForgeServerDirectRetainedPosture};

#[derive(Debug)]
pub struct ForgeServerDirectProductFlow {
    direct: ForgeServerForgeNativeDirectFacade,
    declaration: ForgeServerAdmittedDirectDeclaration,
    declaration_snapshot: ForgeServerDirectDeclarationSnapshot,
}

impl ForgeServerDirectProductFlow {
    pub(crate) fn new(
        direct: ForgeServerForgeNativeDirectFacade,
        declaration: ForgeServerAdmittedDirectDeclaration,
    ) -> Self {
        let declaration_snapshot =
            ForgeServerDirectDeclarationSnapshot::from_admitted(&declaration);
        Self {
            direct,
            declaration,
            declaration_snapshot,
        }
    }

    pub fn declaration(&self) -> &ForgeServerDirectDeclaration {
        self.declaration.declaration()
    }

    pub fn admitted_declaration(&self) -> &ForgeServerAdmittedDirectDeclaration {
        &self.declaration
    }

    pub fn declaration_snapshot(&self) -> &ForgeServerDirectDeclarationSnapshot {
        &self.declaration_snapshot
    }

    pub fn read(&self) -> ForgeServerDirectReadOutcome {
        self.direct.read(&self.declaration)
    }

    pub fn state(&self) -> ForgeServerDirectStateOutcome {
        self.direct.state(&self.declaration)
    }

    pub fn inspect(&self) -> ForgeServerDirectInspectionOutcome {
        self.direct.inspect(&self.declaration)
    }

    pub fn project(
        &self,
        request: &ForgeServerDirectProjectionRequest,
    ) -> ForgeServerDirectProjectionOutcome {
        self.direct.project(&self.declaration, request)
    }

    pub fn product_retained_posture(
        &self,
    ) -> forge_proof::TransitionOutcome<
        ForgeServerDirectRetainedPosture,
        crate::ForgeServerQueryHandoffDenial,
        crate::ForgeServerQueryHandoffDeferred,
        crate::ForgeServerQueryHandoffStale,
        crate::ForgeServerQueryHandoffRebindRequired,
        crate::ForgeServerQueryHandoffFailure,
    > {
        match self.state() {
            forge_proof::TransitionOutcome::Success(state) => {
                forge_proof::TransitionOutcome::Success(ForgeServerDirectRetainedPosture::new(
                    self.declaration_snapshot.clone(),
                    state,
                ))
            }
            forge_proof::TransitionOutcome::Denied(denial) => {
                forge_proof::TransitionOutcome::Denied(denial)
            }
            forge_proof::TransitionOutcome::Deferred(deferred) => {
                forge_proof::TransitionOutcome::Deferred(deferred)
            }
            forge_proof::TransitionOutcome::Stale(stale) => {
                forge_proof::TransitionOutcome::Stale(stale)
            }
            forge_proof::TransitionOutcome::RebindRequired(rebind) => {
                forge_proof::TransitionOutcome::RebindRequired(rebind)
            }
            forge_proof::TransitionOutcome::Failed(failure) => {
                forge_proof::TransitionOutcome::Failed(failure)
            }
        }
    }
}
