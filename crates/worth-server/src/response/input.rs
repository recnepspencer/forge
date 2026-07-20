use crate::{
    WorthServerDenial, WorthServerQueryHandoff, WorthServerQueryHandoffDenial,
    WorthServerRequestContextDenial,
};

#[derive(Debug)]
pub enum WorthServerResponseInput {
    QueryHandoffSuccess(Box<WorthServerQueryHandoff>),
    RequestContextDenied(Box<WorthServerRequestContextDenial>),
    MiddlewareDenied(Box<WorthServerDenial>),
    QueryHandoffDenied(Box<WorthServerQueryHandoffDenial>),
}

impl WorthServerResponseInput {
    pub fn query_handoff_success(handoff: WorthServerQueryHandoff) -> Self {
        Self::QueryHandoffSuccess(Box::new(handoff))
    }

    pub fn request_context_denied(denial: WorthServerRequestContextDenial) -> Self {
        Self::RequestContextDenied(Box::new(denial))
    }

    pub fn middleware_denied(denial: WorthServerDenial) -> Self {
        Self::MiddlewareDenied(Box::new(denial))
    }

    pub fn query_handoff_denied(denial: WorthServerQueryHandoffDenial) -> Self {
        Self::QueryHandoffDenied(Box::new(denial))
    }
}
