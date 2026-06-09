use crate::{
    ForgeServerDenial, ForgeServerQueryHandoff, ForgeServerQueryHandoffDenial,
    ForgeServerRequestContextDenial,
};

#[derive(Debug)]
pub enum ForgeServerResponseInput {
    QueryHandoffSuccess(ForgeServerQueryHandoff),
    RequestContextDenied(ForgeServerRequestContextDenial),
    MiddlewareDenied(ForgeServerDenial),
    QueryHandoffDenied(ForgeServerQueryHandoffDenial),
}

impl ForgeServerResponseInput {
    pub fn query_handoff_success(handoff: ForgeServerQueryHandoff) -> Self {
        Self::QueryHandoffSuccess(handoff)
    }

    pub fn request_context_denied(denial: ForgeServerRequestContextDenial) -> Self {
        Self::RequestContextDenied(denial)
    }

    pub fn middleware_denied(denial: ForgeServerDenial) -> Self {
        Self::MiddlewareDenied(denial)
    }

    pub fn query_handoff_denied(denial: ForgeServerQueryHandoffDenial) -> Self {
        Self::QueryHandoffDenied(denial)
    }
}
