use crate::identity::{
    AdmittedSubscriptionIdentityTag, BridgeIdentity,
    SubscriptionAcknowledgementFrontierIdentityTag, SubscriptionActiveIdentityTag,
    SubscriptionBasisIdentityTag, SubscriptionCheckpointIdentityTag,
    SubscriptionCheckpointReadyIdentityTag, SubscriptionConsumerContractIdentityTag,
    SubscriptionContinuationCandidateIdentityTag, SubscriptionContinuationChildIdentityTag,
    SubscriptionContinuationDecisionIdentityTag, SubscriptionContinuationIndexIdentityTag,
    SubscriptionDeclarationFamilyIdentityTag, SubscriptionDeclarationIdentityTag,
    SubscriptionDeliveryBufferLifecycleIdentityTag, SubscriptionDeliveryCostProfileIdentityTag,
    SubscriptionDeliveryDiagnosticsReferenceIdentityTag, SubscriptionDeliveryFamilyIdentityTag,
    SubscriptionDeliveryMemberIdentityTag, SubscriptionDeliveryReplayPlanIdentityTag,
    SubscriptionDeliveryReplayReadinessIdentityTag, SubscriptionDeliveryWindowIdentityTag,
    SubscriptionDeliveryWindowOpenIdentityTag, SubscriptionDuplicateReplayPolicyIdentityTag,
    SubscriptionFamilyRegistryIdentityTag, SubscriptionFanoutConsumerBindingIdentityTag,
    SubscriptionFanoutDeliveryProjectionIdentityTag,
    SubscriptionFanoutDeliveryProjectionSetIdentityTag, SubscriptionFanoutLayoutIdentityTag,
    SubscriptionFanoutPlanIdentityTag, SubscriptionFanoutProjectionValidationIdentityTag,
    SubscriptionLifecycleIdentityTag, SubscriptionPreviewActiveIdentityTag,
    SubscriptionPreviewBasisIdentityTag, SubscriptionPreviewDiscardResidueProofIdentityTag,
    SubscriptionPreviewLifecycleIdentityTag, SubscriptionPreviewParentBasisIdentityTag,
    SubscriptionPreviewResidueArtifactIdentityTag, SubscriptionPreviewResidueScopeIdentityTag,
    SubscriptionPreviewResidueScopeIndexIdentityTag, SubscriptionPreviewScopeIdentityTag,
    SubscriptionReplayIdentityTag, SubscriptionResumeAdmissionIdentityTag,
    SubscriptionResumePlanIdentityTag, SubscriptionRetainedDeliveryReplaySeedIdentityTag,
    SubscriptionRetainedDeliveryWindowSeedIdentityTag, SubscriptionSharingEligibilityIdentityTag,
    SubscriptionSignalStrategyIdentityTag,
};

pub type BridgeSubscriptionDeclarationFamilyIdentity =
    BridgeIdentity<SubscriptionDeclarationFamilyIdentityTag>;
pub type BridgeSubscriptionDeclarationIdentity = BridgeIdentity<SubscriptionDeclarationIdentityTag>;
pub type BridgeSubscriptionFamilyRegistryIdentity =
    BridgeIdentity<SubscriptionFamilyRegistryIdentityTag>;
pub type BridgeSubscriptionBasisIdentity = BridgeIdentity<SubscriptionBasisIdentityTag>;
pub type BridgeSignalStrategyIdentity = BridgeIdentity<SubscriptionSignalStrategyIdentityTag>;
pub type BridgeAdmittedSubscriptionIdentity = BridgeIdentity<AdmittedSubscriptionIdentityTag>;
pub type BridgeSubscriptionLifecycleIdentity = BridgeIdentity<SubscriptionLifecycleIdentityTag>;
pub type BridgeSubscriptionReplayIdentity = BridgeIdentity<SubscriptionReplayIdentityTag>;
pub type BridgeSubscriptionDeliveryCostProfileIdentity =
    BridgeIdentity<SubscriptionDeliveryCostProfileIdentityTag>;
pub type BridgeSubscriptionConsumerContractIdentity =
    BridgeIdentity<SubscriptionConsumerContractIdentityTag>;
pub type BridgeActiveSubscriptionIdentity = BridgeIdentity<SubscriptionActiveIdentityTag>;
pub type BridgeSubscriptionDeliveryFamilyIdentity =
    BridgeIdentity<SubscriptionDeliveryFamilyIdentityTag>;
pub type BridgeSubscriptionDeliveryWindowOpenIdentity =
    BridgeIdentity<SubscriptionDeliveryWindowOpenIdentityTag>;
pub type BridgeSubscriptionDeliveryWindowIdentity =
    BridgeIdentity<SubscriptionDeliveryWindowIdentityTag>;
pub type BridgeSubscriptionDeliveryMemberIdentity =
    BridgeIdentity<SubscriptionDeliveryMemberIdentityTag>;
pub type BridgeSubscriptionDeliveryDiagnosticsReferenceIdentity =
    BridgeIdentity<SubscriptionDeliveryDiagnosticsReferenceIdentityTag>;
pub type BridgeSubscriptionDeliveryBufferLifecycleIdentity =
    BridgeIdentity<SubscriptionDeliveryBufferLifecycleIdentityTag>;
pub type BridgeSubscriptionSharingEligibilityIdentity =
    BridgeIdentity<SubscriptionSharingEligibilityIdentityTag>;
pub type BridgeSubscriptionFanoutPlanIdentity = BridgeIdentity<SubscriptionFanoutPlanIdentityTag>;
pub type BridgeSubscriptionFanoutLayoutIdentity =
    BridgeIdentity<SubscriptionFanoutLayoutIdentityTag>;
pub type BridgeSubscriptionFanoutConsumerBindingIdentity =
    BridgeIdentity<SubscriptionFanoutConsumerBindingIdentityTag>;
pub type BridgeSubscriptionFanoutDeliveryProjectionIdentity =
    BridgeIdentity<SubscriptionFanoutDeliveryProjectionIdentityTag>;
pub type BridgeSubscriptionFanoutDeliveryProjectionSetIdentity =
    BridgeIdentity<SubscriptionFanoutDeliveryProjectionSetIdentityTag>;
pub type BridgeSubscriptionRetainedDeliveryWindowSeedIdentity =
    BridgeIdentity<SubscriptionRetainedDeliveryWindowSeedIdentityTag>;
pub type BridgeSubscriptionRetainedDeliveryReplaySeedIdentity =
    BridgeIdentity<SubscriptionRetainedDeliveryReplaySeedIdentityTag>;
pub type BridgeSubscriptionFanoutProjectionValidationIdentity =
    BridgeIdentity<SubscriptionFanoutProjectionValidationIdentityTag>;
pub type BridgeSubscriptionDeliveryReplayReadinessIdentity =
    BridgeIdentity<SubscriptionDeliveryReplayReadinessIdentityTag>;
pub type BridgeSubscriptionDeliveryReplayPlanIdentity =
    BridgeIdentity<SubscriptionDeliveryReplayPlanIdentityTag>;
pub type BridgeSubscriptionAcknowledgementFrontierIdentity =
    BridgeIdentity<SubscriptionAcknowledgementFrontierIdentityTag>;
pub type BridgeSubscriptionCheckpointReadyIdentity =
    BridgeIdentity<SubscriptionCheckpointReadyIdentityTag>;
pub type BridgeSubscriptionCheckpointIdentity = BridgeIdentity<SubscriptionCheckpointIdentityTag>;
pub type BridgeSubscriptionResumeAdmissionIdentity =
    BridgeIdentity<SubscriptionResumeAdmissionIdentityTag>;
pub type BridgeSubscriptionResumePlanIdentity = BridgeIdentity<SubscriptionResumePlanIdentityTag>;
pub type BridgeSubscriptionDuplicateReplayPolicyIdentity =
    BridgeIdentity<SubscriptionDuplicateReplayPolicyIdentityTag>;
pub type BridgeSubscriptionContinuationIndexIdentity =
    BridgeIdentity<SubscriptionContinuationIndexIdentityTag>;
pub type BridgeSubscriptionContinuationCandidateIdentity =
    BridgeIdentity<SubscriptionContinuationCandidateIdentityTag>;
pub type BridgeSubscriptionContinuationDecisionIdentity =
    BridgeIdentity<SubscriptionContinuationDecisionIdentityTag>;
pub type BridgeSubscriptionContinuationChildIdentity =
    BridgeIdentity<SubscriptionContinuationChildIdentityTag>;
pub type BridgeSubscriptionPreviewBasisIdentity =
    BridgeIdentity<SubscriptionPreviewBasisIdentityTag>;
pub type BridgePreviewActiveSubscriptionIdentity =
    BridgeIdentity<SubscriptionPreviewActiveIdentityTag>;
pub type BridgeSubscriptionPreviewScopeIdentity =
    BridgeIdentity<SubscriptionPreviewScopeIdentityTag>;
pub type BridgeSubscriptionPreviewParentBasisIdentity =
    BridgeIdentity<SubscriptionPreviewParentBasisIdentityTag>;
pub type BridgeSubscriptionPreviewLifecycleIdentity =
    BridgeIdentity<SubscriptionPreviewLifecycleIdentityTag>;
pub type BridgeSubscriptionPreviewResidueScopeIdentity =
    BridgeIdentity<SubscriptionPreviewResidueScopeIdentityTag>;
pub type BridgeSubscriptionPreviewResidueScopeIndexIdentity =
    BridgeIdentity<SubscriptionPreviewResidueScopeIndexIdentityTag>;
pub type BridgeSubscriptionPreviewResidueArtifactIdentity =
    BridgeIdentity<SubscriptionPreviewResidueArtifactIdentityTag>;
pub type BridgeSubscriptionPreviewDiscardResidueProofIdentity =
    BridgeIdentity<SubscriptionPreviewDiscardResidueProofIdentityTag>;
