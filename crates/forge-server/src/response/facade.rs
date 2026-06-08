use crate::config::ForgeServerResponseConfig;

use super::{
    ForgeServerResponseEnvelope, ForgeServerResponseInput, ForgeServerResponsePlan,
    ForgeServerResponseTransform,
};

#[derive(Clone, Debug)]
pub struct ForgeServerResponseFacade {
    config: ForgeServerResponseConfig,
}

impl ForgeServerResponseFacade {
    pub(crate) fn new(config: ForgeServerResponseConfig) -> Self {
        Self { config }
    }

    pub fn plan(
        &self,
        input: ForgeServerResponseInput,
        transform: ForgeServerResponseTransform,
    ) -> ForgeServerResponsePlan {
        ForgeServerResponsePlan::new(&self.config, input, Some(transform))
    }

    pub fn shape(
        &self,
        input: ForgeServerResponseInput,
        transform: ForgeServerResponseTransform,
    ) -> ForgeServerResponseEnvelope {
        self.plan(input, transform).materialize()
    }

    pub fn shape_with_defaults(
        &self,
        input: ForgeServerResponseInput,
    ) -> ForgeServerResponseEnvelope {
        ForgeServerResponsePlan::new(&self.config, input, None).materialize()
    }
}
