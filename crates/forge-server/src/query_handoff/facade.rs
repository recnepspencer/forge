use crate::config::ForgeServerQueryHandoffConfig;

use super::{
    progression::prepare_query_handoff, ForgeServerQueryHandoffInput,
    ForgeServerQueryHandoffOutcome,
};

#[derive(Clone, Debug)]
pub struct ForgeServerQueryHandoffFacade {
    config: ForgeServerQueryHandoffConfig,
}

impl ForgeServerQueryHandoffFacade {
    pub(crate) fn new(config: ForgeServerQueryHandoffConfig) -> Self {
        Self { config }
    }

    pub fn prepare(&self, input: ForgeServerQueryHandoffInput) -> ForgeServerQueryHandoffOutcome {
        prepare_query_handoff(&self.config, input)
    }
}
