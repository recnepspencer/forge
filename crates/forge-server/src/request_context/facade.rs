use forge_proof::TransitionReadiness;

use crate::config::ForgeServerRequestContextConfig;

use super::{
    resolution::{resolve_request_context, ForgeServerRequestContextResolution},
    ForgeServerRequestContextInput,
};

#[derive(Clone, Debug)]
pub struct ForgeServerRequestContextFacade {
    config: ForgeServerRequestContextConfig,
}

impl ForgeServerRequestContextFacade {
    pub(crate) fn new(config: ForgeServerRequestContextConfig) -> Self {
        Self { config }
    }

    pub fn resolve(
        &self,
        input: ForgeServerRequestContextInput,
    ) -> ForgeServerRequestContextResolution {
        resolve_request_context(&self.config, input)
    }

    pub fn review(
        &self,
        input: ForgeServerRequestContextInput,
    ) -> TransitionReadiness<
        super::ForgeServerResolvedRequestContext,
        super::ForgeServerRequestContextDenial,
        super::ForgeServerRequestContextDeferred,
        super::ForgeServerRequestContextStale,
        super::ForgeServerRequestContextRebindRequired,
        super::ForgeServerRequestContextFailure,
    > {
        self.resolve(input)
    }

    pub fn default_diagnostics_profile(&self) -> crate::request_context::DiagnosticRichnessProfile {
        self.config.default_diagnostics_profile()
    }
}
