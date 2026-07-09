use worth_proof::TransitionReadiness;

use crate::config::WorthServerRequestContextConfig;

use super::{
    resolution::{resolve_request_context, WorthServerRequestContextResolution},
    WorthServerRequestContextInput,
};

#[derive(Clone, Debug)]
pub struct WorthServerRequestContextFacade {
    config: WorthServerRequestContextConfig,
}

impl WorthServerRequestContextFacade {
    pub(crate) fn new(config: WorthServerRequestContextConfig) -> Self {
        Self { config }
    }

    pub fn resolve(
        &self,
        input: WorthServerRequestContextInput,
    ) -> WorthServerRequestContextResolution {
        resolve_request_context(&self.config, input)
    }

    pub fn review(
        &self,
        input: WorthServerRequestContextInput,
    ) -> TransitionReadiness<
        super::WorthServerResolvedRequestContext,
        super::WorthServerRequestContextDenial,
        super::WorthServerRequestContextDeferred,
        super::WorthServerRequestContextStale,
        super::WorthServerRequestContextRebindRequired,
        super::WorthServerRequestContextFailure,
    > {
        self.resolve(input)
    }

    pub fn default_diagnostics_profile(&self) -> crate::request_context::DiagnosticRichnessProfile {
        self.config.default_diagnostics_profile()
    }
}
