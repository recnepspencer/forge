use crate::{
    ForgeServerDirectDeclaration, ForgeServerDirectDeclarationDenial,
    ForgeServerPreparedDirectDeclaration,
};

use super::{ForgeServerForgeNativeDirectFacade, ForgeServerForgeNativeProductFacade};

impl ForgeServerForgeNativeDirectFacade {
    pub fn product(&self) -> ForgeServerForgeNativeProductFacade {
        ForgeServerForgeNativeProductFacade::new(self.clone())
    }

    pub(super) fn prepare_declaration(
        &self,
        declaration: ForgeServerDirectDeclaration,
    ) -> Result<ForgeServerPreparedDirectDeclaration, ForgeServerDirectDeclarationDenial> {
        self.declaration_intake
            .prepare(self.admission.clone(), declaration)
    }
}
