use super::ForgeServerPipelineIntent;
use crate::ForgeServerResolvedRequestContext;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerAdmission {
    resolved_request_context: ForgeServerResolvedRequestContext,
    prepared_query_handoff_intent: ForgeServerPreparedQueryHandoffIntent,
}

impl ForgeServerAdmission {
    pub(crate) fn new(
        resolved_request_context: ForgeServerResolvedRequestContext,
        prepared_query_handoff_intent: ForgeServerPreparedQueryHandoffIntent,
    ) -> Self {
        Self {
            resolved_request_context,
            prepared_query_handoff_intent,
        }
    }

    pub fn request_context(&self) -> &crate::ForgeServerRequestContext {
        self.resolved_request_context.request_context()
    }

    pub fn resolved_request_context(&self) -> &ForgeServerResolvedRequestContext {
        &self.resolved_request_context
    }

    pub fn query_handoff_intent(&self) -> &ForgeServerPreparedQueryHandoffIntent {
        &self.prepared_query_handoff_intent
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerPreparedQueryHandoffKind {
    QueryRead,
    QueryMutation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerPreparedQueryHandoffIntent {
    kind: ForgeServerPreparedQueryHandoffKind,
    operation_name: String,
}

impl ForgeServerPreparedQueryHandoffIntent {
    pub(crate) fn from_pipeline_intent(intent: ForgeServerPipelineIntent) -> Self {
        match intent {
            ForgeServerPipelineIntent::QueryRead { operation_name } => Self {
                kind: ForgeServerPreparedQueryHandoffKind::QueryRead,
                operation_name,
            },
            ForgeServerPipelineIntent::QueryMutation { operation_name } => Self {
                kind: ForgeServerPreparedQueryHandoffKind::QueryMutation,
                operation_name,
            },
        }
    }

    pub fn kind(&self) -> ForgeServerPreparedQueryHandoffKind {
        self.kind
    }

    pub fn operation_name(&self) -> &str {
        &self.operation_name
    }
}
