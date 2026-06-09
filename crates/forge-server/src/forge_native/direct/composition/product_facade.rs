use crate::{
    ForgeServerAdmittedDirectDeclaration, ForgeServerDirectDeclaration,
    ForgeServerDirectDeclarationDenial, ForgeServerForgeNativeDirectFacade,
};

use super::ForgeServerDirectProductFlow;

#[derive(Clone, Debug)]
pub struct ForgeServerForgeNativeProductFacade {
    direct: ForgeServerForgeNativeDirectFacade,
}

impl ForgeServerForgeNativeProductFacade {
    pub(crate) fn new(direct: ForgeServerForgeNativeDirectFacade) -> Self {
        Self { direct }
    }

    pub fn named_read(
        &self,
        operation_name: &str,
    ) -> Result<ForgeServerDirectProductFlow, ForgeServerDirectDeclarationDenial> {
        self.read(ForgeServerDirectDeclaration::named_read(operation_name))
    }

    pub fn read(
        &self,
        declaration: ForgeServerDirectDeclaration,
    ) -> Result<ForgeServerDirectProductFlow, ForgeServerDirectDeclarationDenial> {
        let declaration = self.direct.prepare_declaration(declaration)?.admit()?;
        Ok(ForgeServerDirectProductFlow::new(
            self.direct.clone(),
            declaration,
        ))
    }

    pub fn from_admitted(
        &self,
        declaration: ForgeServerAdmittedDirectDeclaration,
    ) -> ForgeServerDirectProductFlow {
        ForgeServerDirectProductFlow::new(self.direct.clone(), declaration)
    }
}
