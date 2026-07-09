use crate::{
    WorthServerDirectDeclaration, WorthServerDirectDeclarationDenial,
    WorthServerPreparedDirectDeclaration,
};

use super::{WorthServerWorthNativeDirectFacade, WorthServerWorthNativeProductFacade};

impl WorthServerWorthNativeDirectFacade {
    pub fn product(&self) -> WorthServerWorthNativeProductFacade {
        WorthServerWorthNativeProductFacade::new(self.clone())
    }

    pub(super) fn prepare_declaration(
        &self,
        declaration: WorthServerDirectDeclaration,
    ) -> Result<WorthServerPreparedDirectDeclaration, WorthServerDirectDeclarationDenial> {
        self.declaration_intake
            .prepare(self.admission.clone(), declaration)
    }
}
