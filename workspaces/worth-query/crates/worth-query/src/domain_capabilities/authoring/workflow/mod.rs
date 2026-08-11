mod inspection;
mod merge;
mod mutation;
mod posture;
mod target_binding;
mod writeback;

use crate::domain_capabilities::payloads::{
    WorthQueryWorkflowContributionPayload, WorthQueryWorkflowContributionPosture,
    WorthQueryWorkflowLoweringSemantics, WorthQueryWorkflowRuntimeSemantics,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryWorkflowContributionAuthoring {
    pub(super) payload: WorthQueryWorkflowContributionPayload,
}

impl WorthQueryWorkflowContributionAuthoring {
    fn new(
        posture: WorthQueryWorkflowContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            payload: WorthQueryWorkflowContributionPayload::new(posture, semantic_code, detail),
        }
    }

    fn with_runtime_semantics(
        posture: WorthQueryWorkflowContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        runtime_semantics: WorthQueryWorkflowRuntimeSemantics,
    ) -> Self {
        Self {
            payload: WorthQueryWorkflowContributionPayload::with_runtime_semantics(
                posture,
                semantic_code,
                detail,
                Some(runtime_semantics),
            ),
        }
    }

    fn with_runtime_and_lowering_semantics(
        posture: WorthQueryWorkflowContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        runtime_semantics: WorthQueryWorkflowRuntimeSemantics,
        lowering_semantics: WorthQueryWorkflowLoweringSemantics,
    ) -> Self {
        Self {
            payload: WorthQueryWorkflowContributionPayload::with_runtime_and_lowering_semantics(
                posture,
                semantic_code,
                detail,
                Some(runtime_semantics),
                Some(lowering_semantics),
            ),
        }
    }
}
