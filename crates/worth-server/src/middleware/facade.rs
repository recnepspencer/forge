use crate::config::WorthServerMiddlewareConfig;

use super::{progression::admit_pipeline_input, WorthServerPipelineInput};

#[derive(Clone, Debug)]
pub struct WorthServerMiddlewareFacade {
    config: WorthServerMiddlewareConfig,
}

impl WorthServerMiddlewareFacade {
    pub(crate) fn new(config: WorthServerMiddlewareConfig) -> Self {
        Self { config }
    }

    pub fn admit(&self, input: WorthServerPipelineInput) -> super::WorthServerAdmissionOutcome {
        admit_pipeline_input(&self.config, input)
    }
}
