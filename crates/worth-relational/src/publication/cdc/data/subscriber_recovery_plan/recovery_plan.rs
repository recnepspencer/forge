use crate::publication::cdc::data::{
    SubscriberCheckpoint, SubscriberContinuationAssessment, SubscriberRecoveryDecision,
    SubscriberResumeRequest,
};
use crate::publication::patch::data::PatchStreamPosition;
use crate::replay::data::CanonicalCommitEnvelope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriberRecoveryPlan {
    pub(crate) request: SubscriberResumeRequest,
    pub(crate) decision: SubscriberRecoveryDecision,
    pub(crate) latest_available_checkpoint: Option<SubscriberCheckpoint>,
    pub(crate) start_after_position: Option<PatchStreamPosition>,
    pub(crate) selected_envelopes: Vec<CanonicalCommitEnvelope>,
    pub(crate) continuation_assessment: SubscriberContinuationAssessment,
}

impl SubscriberRecoveryPlan {
    pub(crate) fn new(
        request: SubscriberResumeRequest,
        decision: SubscriberRecoveryDecision,
        latest_available_checkpoint: Option<SubscriberCheckpoint>,
        start_after_position: Option<PatchStreamPosition>,
        selected_envelopes: Vec<CanonicalCommitEnvelope>,
        continuation_assessment: SubscriberContinuationAssessment,
    ) -> Self {
        Self {
            request,
            decision,
            latest_available_checkpoint,
            start_after_position,
            selected_envelopes,
            continuation_assessment,
        }
    }
}
