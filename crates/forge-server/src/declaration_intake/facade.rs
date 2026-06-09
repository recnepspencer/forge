use crate::{config::ForgeServerQueryHandoffConfig, ForgeServerAdmission};

use super::{progression::prepare_direct_declaration, ForgeServerDirectDeclaration};

#[derive(Clone, Debug)]
pub(crate) struct ForgeServerDirectDeclarationIntakeFacade {
    config: ForgeServerQueryHandoffConfig,
}

impl ForgeServerDirectDeclarationIntakeFacade {
    pub(crate) fn new(config: ForgeServerQueryHandoffConfig) -> Self {
        Self { config }
    }

    pub(crate) fn prepare(
        &self,
        admission: ForgeServerAdmission,
        declaration: ForgeServerDirectDeclaration,
    ) -> Result<
        super::ForgeServerPreparedDirectDeclaration,
        super::ForgeServerDirectDeclarationDenial,
    > {
        prepare_direct_declaration(&self.config, admission, declaration)
    }
}
