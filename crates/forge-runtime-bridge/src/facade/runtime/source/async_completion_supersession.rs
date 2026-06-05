use super::*;
use crate::source::{
    BridgeAsyncClassifiedDeniedCompletion, BridgeAsyncCompletionSupersessionClassificationRequest,
    BridgeAsyncCompletionSupersessionRejection,
};

impl RuntimeBridge {
    /// Classifies one stale or superseded denied async completion into one
    /// explicit bridge stale-causality family.
    pub fn classify_async_completion_supersession(
        &self,
        request: BridgeAsyncCompletionSupersessionClassificationRequest,
    ) -> Result<BridgeAsyncClassifiedDeniedCompletion, BridgeAsyncCompletionSupersessionRejection>
    {
        let _ = self;
        BridgeAsyncClassifiedDeniedCompletion::classify(request)
    }
}
