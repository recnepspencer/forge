use crate::{config::WorthServerQueryHandoffConfig, WorthServerRequestContextFacade};

use super::{run_query_dependency_audit, WorthServerQueryDependencyAuditReceipt};

#[derive(Clone, Debug)]
pub struct WorthServerQueryDependencyAuditFacade {
    request_contexts: WorthServerRequestContextFacade,
    query_handoff_config: WorthServerQueryHandoffConfig,
}

impl WorthServerQueryDependencyAuditFacade {
    pub(crate) fn new(
        request_contexts: WorthServerRequestContextFacade,
        query_handoff_config: WorthServerQueryHandoffConfig,
    ) -> Self {
        Self {
            request_contexts,
            query_handoff_config,
        }
    }

    pub fn run(&self) -> WorthServerQueryDependencyAuditReceipt {
        run_query_dependency_audit(&self.request_contexts, &self.query_handoff_config)
    }
}
