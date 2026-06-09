use crate::config::ForgeServerMiddlewareConfig;

use super::{progression::admit_pipeline_input, ForgeServerPipelineInput};

#[derive(Clone, Debug)]
pub struct ForgeServerMiddlewareFacade {
    config: ForgeServerMiddlewareConfig,
}

impl ForgeServerMiddlewareFacade {
    pub(crate) fn new(config: ForgeServerMiddlewareConfig) -> Self {
        Self { config }
    }

    pub fn admit(&self, input: ForgeServerPipelineInput) -> super::ForgeServerAdmissionOutcome {
        admit_pipeline_input(&self.config, input)
    }
}
