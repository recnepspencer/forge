use crate::{config::ForgeServerQueryHandoffConfig, ForgeServerRequestContextFacade};

use super::{run_query_dependency_audit, ForgeServerQueryDependencyAuditReceipt};

#[derive(Clone, Debug)]
pub struct ForgeServerQueryDependencyAuditFacade {
    request_contexts: ForgeServerRequestContextFacade,
    query_handoff_config: ForgeServerQueryHandoffConfig,
}

impl ForgeServerQueryDependencyAuditFacade {
    pub(crate) fn new(
        request_contexts: ForgeServerRequestContextFacade,
        query_handoff_config: ForgeServerQueryHandoffConfig,
    ) -> Self {
        Self {
            request_contexts,
            query_handoff_config,
        }
    }

    pub fn run(&self) -> ForgeServerQueryDependencyAuditReceipt {
        run_query_dependency_audit(&self.request_contexts, &self.query_handoff_config)
    }
}
