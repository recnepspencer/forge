use crate::{
    WorthServerDenial, WorthServerQueryHandoff, WorthServerQueryHandoffDenial,
    WorthServerRequestContextDenial,
};

#[derive(Debug)]
pub enum WorthServerResponseInput {
    QueryHandoffSuccess(WorthServerQueryHandoff),
    RequestContextDenied(WorthServerRequestContextDenial),
    MiddlewareDenied(WorthServerDenial),
    QueryHandoffDenied(WorthServerQueryHandoffDenial),
}

impl WorthServerResponseInput {
    pub fn query_handoff_success(handoff: WorthServerQueryHandoff) -> Self {
        Self::QueryHandoffSuccess(handoff)
    }

    pub fn request_context_denied(denial: WorthServerRequestContextDenial) -> Self {
        Self::RequestContextDenied(denial)
    }

    pub fn middleware_denied(denial: WorthServerDenial) -> Self {
        Self::MiddlewareDenied(denial)
    }

    pub fn query_handoff_denied(denial: WorthServerQueryHandoffDenial) -> Self {
        Self::QueryHandoffDenied(denial)
    }
}
