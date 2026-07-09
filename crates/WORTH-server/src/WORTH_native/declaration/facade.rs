use crate::{
    declaration_intake::WorthServerDirectDeclarationIntakeFacade, WorthServerAdmission,
    WorthServerDirectDeclaration, WorthServerDirectDeclarationDenial,
    WorthServerPreparedDirectDeclaration,
};

#[derive(Clone, Debug)]
pub struct WorthServerWorthNativeDeclarationFacade {
    admission: WorthServerAdmission,
    declaration_intake: WorthServerDirectDeclarationIntakeFacade,
}

impl WorthServerWorthNativeDeclarationFacade {
    pub(crate) fn new(
        admission: WorthServerAdmission,
        declaration_intake: WorthServerDirectDeclarationIntakeFacade,
    ) -> Self {
        Self {
            admission,
            declaration_intake,
        }
    }

    pub fn read(
        &self,
        declaration: WorthServerDirectDeclaration,
    ) -> Result<WorthServerPreparedDirectDeclaration, WorthServerDirectDeclarationDenial> {
        self.declaration_intake
            .prepare(self.admission.clone(), declaration)
    }
}
