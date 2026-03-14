use crate::publication::cdc::data::{
    SubscriberCheckpoint, SubscriberRecoveryDecision, SubscriberResumeRequest,
};
use crate::publication::patch::data::PatchStreamPosition;
use crate::replay::data::CanonicalCommitEnvelope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriberRecoveryPlan {
    pub(crate) request: SubscriberResumeRequest,
    pub(crate) decision: SubscriberRecoveryDecision,
    pub(crate) latest_available_checkpoint: Option<SubscriberCheckpoint>,
    pub(crate) start_after_position: Option<PatchStreamPosition>,
    pub(crate) source_envelopes: Vec<CanonicalCommitEnvelope>,
}
