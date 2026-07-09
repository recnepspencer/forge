use crate::config::WorthServerQueryHandoffConfig;

use super::{
    progression::prepare_query_handoff, WorthServerQueryHandoffInput,
    WorthServerQueryHandoffOutcome,
};

#[derive(Clone, Debug)]
pub struct WorthServerQueryHandoffFacade {
    config: WorthServerQueryHandoffConfig,
}

impl WorthServerQueryHandoffFacade {
    pub(crate) fn new(config: WorthServerQueryHandoffConfig) -> Self {
        Self { config }
    }

    pub(crate) fn config(&self) -> &WorthServerQueryHandoffConfig {
        &self.config
    }

    pub fn prepare(&self, input: WorthServerQueryHandoffInput) -> WorthServerQueryHandoffOutcome {
        prepare_query_handoff(&self.config, input)
    }
}
