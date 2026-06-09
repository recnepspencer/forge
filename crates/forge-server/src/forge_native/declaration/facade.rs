use crate::{
    declaration_intake::ForgeServerDirectDeclarationIntakeFacade, ForgeServerAdmission,
    ForgeServerDirectDeclaration, ForgeServerDirectDeclarationDenial,
    ForgeServerPreparedDirectDeclaration,
};

#[derive(Clone, Debug)]
pub struct ForgeServerForgeNativeDeclarationFacade {
    admission: ForgeServerAdmission,
    declaration_intake: ForgeServerDirectDeclarationIntakeFacade,
}

impl ForgeServerForgeNativeDeclarationFacade {
    pub(crate) fn new(
        admission: ForgeServerAdmission,
        declaration_intake: ForgeServerDirectDeclarationIntakeFacade,
    ) -> Self {
        Self {
            admission,
            declaration_intake,
        }
    }

    pub fn read(
        &self,
        declaration: ForgeServerDirectDeclaration,
    ) -> Result<ForgeServerPreparedDirectDeclaration, ForgeServerDirectDeclarationDenial> {
        self.declaration_intake
            .prepare(self.admission.clone(), declaration)
    }
}
