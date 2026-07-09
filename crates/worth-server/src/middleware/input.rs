use super::WorthServerPipelineIntent;
use crate::WorthServerResolvedRequestContext;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerPipelineInput {
    resolved_request_context: WorthServerResolvedRequestContext,
    pipeline_intent: WorthServerPipelineIntent,
}

impl WorthServerPipelineInput {
    pub fn new(
        resolved_request_context: WorthServerResolvedRequestContext,
        pipeline_intent: WorthServerPipelineIntent,
    ) -> Self {
        Self {
            resolved_request_context,
            pipeline_intent,
        }
    }

    pub(crate) fn resolved_request_context(&self) -> &WorthServerResolvedRequestContext {
        &self.resolved_request_context
    }

    pub(crate) fn pipeline_intent(&self) -> &WorthServerPipelineIntent {
        &self.pipeline_intent
    }

    pub(crate) fn into_parts(
        self,
    ) -> (WorthServerResolvedRequestContext, WorthServerPipelineIntent) {
        (self.resolved_request_context, self.pipeline_intent)
    }
}
