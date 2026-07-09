use crate::{config::WorthServerQueryHandoffConfig, WorthServerAdmission};

use super::{progression::prepare_direct_declaration, WorthServerDirectDeclaration};

#[derive(Clone, Debug)]
pub(crate) struct WorthServerDirectDeclarationIntakeFacade {
    config: WorthServerQueryHandoffConfig,
}

impl WorthServerDirectDeclarationIntakeFacade {
    pub(crate) fn new(config: WorthServerQueryHandoffConfig) -> Self {
        Self { config }
    }

    pub(crate) fn prepare(
        &self,
        admission: WorthServerAdmission,
        declaration: WorthServerDirectDeclaration,
    ) -> Result<
        super::WorthServerPreparedDirectDeclaration,
        super::WorthServerDirectDeclarationDenial,
    > {
        prepare_direct_declaration(&self.config, admission, declaration)
    }
}
