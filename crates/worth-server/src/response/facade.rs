use crate::config::WorthServerResponseConfig;

use super::{
    WorthServerResponseEnvelope, WorthServerResponseInput, WorthServerResponsePlan,
    WorthServerResponseTransform,
};

#[derive(Clone, Debug)]
pub struct WorthServerResponseFacade {
    config: WorthServerResponseConfig,
}

impl WorthServerResponseFacade {
    pub(crate) fn new(config: WorthServerResponseConfig) -> Self {
        Self { config }
    }

    pub fn plan(
        &self,
        input: WorthServerResponseInput,
        transform: WorthServerResponseTransform,
    ) -> WorthServerResponsePlan {
        WorthServerResponsePlan::new(&self.config, input, Some(transform))
    }

    pub fn shape(
        &self,
        input: WorthServerResponseInput,
        transform: WorthServerResponseTransform,
    ) -> WorthServerResponseEnvelope {
        self.plan(input, transform).materialize()
    }

    pub fn shape_with_defaults(
        &self,
        input: WorthServerResponseInput,
    ) -> WorthServerResponseEnvelope {
        WorthServerResponsePlan::new(&self.config, input, None).materialize()
    }
}
