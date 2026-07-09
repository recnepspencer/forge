use crate::{
    WorthServerAdmittedDirectDeclaration, WorthServerDirectDeclaration,
    WorthServerDirectDeclarationDenial, WorthServerWorthNativeDirectFacade,
};

use super::WorthServerDirectProductFlow;

#[derive(Clone, Debug)]
pub struct WorthServerWorthNativeProductFacade {
    direct: WorthServerWorthNativeDirectFacade,
}

impl WorthServerWorthNativeProductFacade {
    pub(crate) fn new(direct: WorthServerWorthNativeDirectFacade) -> Self {
        Self { direct }
    }

    pub fn named_read(
        &self,
        operation_name: &str,
    ) -> Result<WorthServerDirectProductFlow, WorthServerDirectDeclarationDenial> {
        self.read(WorthServerDirectDeclaration::named_read(operation_name))
    }

    pub fn read(
        &self,
        declaration: WorthServerDirectDeclaration,
    ) -> Result<WorthServerDirectProductFlow, WorthServerDirectDeclarationDenial> {
        let declaration = self.direct.prepare_declaration(declaration)?.admit()?;
        Ok(WorthServerDirectProductFlow::new(
            self.direct.clone(),
            declaration,
        ))
    }

    pub fn from_admitted(
        &self,
        declaration: WorthServerAdmittedDirectDeclaration,
    ) -> WorthServerDirectProductFlow {
        WorthServerDirectProductFlow::new(self.direct.clone(), declaration)
    }
}
