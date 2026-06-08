use super::ForgeServerPipelineIntent;
use crate::ForgeServerResolvedRequestContext;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerPipelineInput {
    resolved_request_context: ForgeServerResolvedRequestContext,
    pipeline_intent: ForgeServerPipelineIntent,
}

impl ForgeServerPipelineInput {
    pub fn new(
        resolved_request_context: ForgeServerResolvedRequestContext,
        pipeline_intent: ForgeServerPipelineIntent,
    ) -> Self {
        Self {
            resolved_request_context,
            pipeline_intent,
        }
    }

    pub(crate) fn resolved_request_context(&self) -> &ForgeServerResolvedRequestContext {
        &self.resolved_request_context
    }

    pub(crate) fn pipeline_intent(&self) -> &ForgeServerPipelineIntent {
        &self.pipeline_intent
    }

    pub(crate) fn into_parts(
        self,
    ) -> (ForgeServerResolvedRequestContext, ForgeServerPipelineIntent) {
        (self.resolved_request_context, self.pipeline_intent)
    }
}
