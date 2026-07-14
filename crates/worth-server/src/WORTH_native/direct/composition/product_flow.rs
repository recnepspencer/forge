use crate::{
    WorthServerAdmittedDirectDeclaration, WorthServerDirectDeclaration,
    WorthServerDirectInspectionOutcome, WorthServerDirectProjectionOutcome,
    WorthServerDirectProjectionRequest, WorthServerDirectReadOutcome,
    WorthServerDirectStateOutcome, WorthServerWorthNativeDirectFacade,
};

use super::{WorthServerDirectDeclarationSnapshot, WorthServerDirectRetainedPosture};

#[derive(Debug)]
pub struct WorthServerDirectProductFlow {
    direct: WorthServerWorthNativeDirectFacade,
    declaration: WorthServerAdmittedDirectDeclaration,
    declaration_snapshot: WorthServerDirectDeclarationSnapshot,
}

impl WorthServerDirectProductFlow {
    pub(crate) fn new(
        direct: WorthServerWorthNativeDirectFacade,
        declaration: WorthServerAdmittedDirectDeclaration,
    ) -> Self {
        let declaration_snapshot =
            WorthServerDirectDeclarationSnapshot::from_admitted(&declaration);
        Self {
            direct,
            declaration,
            declaration_snapshot,
        }
    }

    pub fn declaration(&self) -> &WorthServerDirectDeclaration {
        self.declaration.declaration()
    }

    pub fn admitted_declaration(&self) -> &WorthServerAdmittedDirectDeclaration {
        &self.declaration
    }

    pub fn declaration_snapshot(&self) -> &WorthServerDirectDeclarationSnapshot {
        &self.declaration_snapshot
    }

    pub fn read(&self) -> WorthServerDirectReadOutcome {
        self.direct.read(&self.declaration)
    }

    pub fn state(&self) -> WorthServerDirectStateOutcome {
        self.direct.state(&self.declaration)
    }

    pub fn inspect(&self) -> WorthServerDirectInspectionOutcome {
        self.direct.inspect(&self.declaration)
    }

    pub fn project(
        &self,
        request: &WorthServerDirectProjectionRequest,
    ) -> WorthServerDirectProjectionOutcome {
        self.direct.project(&self.declaration, request)
    }

    pub fn product_retained_posture(
        &self,
    ) -> worth_proof::TransitionOutcome<
        WorthServerDirectRetainedPosture,
        crate::WorthServerQueryHandoffDenial,
        crate::WorthServerQueryHandoffDeferred,
        crate::WorthServerQueryHandoffStale,
        crate::WorthServerQueryHandoffRebindRequired,
        crate::WorthServerQueryHandoffFailure,
    > {
        match self.state() {
            worth_proof::TransitionOutcome::Success(state) => {
                worth_proof::TransitionOutcome::Success(WorthServerDirectRetainedPosture::new(
                    self.declaration_snapshot.clone(),
                    state,
                ))
            }
            worth_proof::TransitionOutcome::Denied(denial) => {
                worth_proof::TransitionOutcome::Denied(denial)
            }
            worth_proof::TransitionOutcome::Deferred(deferred) => {
                worth_proof::TransitionOutcome::Deferred(deferred)
            }
            worth_proof::TransitionOutcome::Stale(stale) => {
                worth_proof::TransitionOutcome::Stale(stale)
            }
            worth_proof::TransitionOutcome::RebindRequired(rebind) => {
                worth_proof::TransitionOutcome::RebindRequired(rebind)
            }
            worth_proof::TransitionOutcome::Failed(failure) => {
                worth_proof::TransitionOutcome::Failed(failure)
            }
        }
    }
}
