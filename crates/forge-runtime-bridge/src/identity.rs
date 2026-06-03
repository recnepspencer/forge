use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::sync::Arc;

use crate::clone_budget::CheapClone;

pub struct BridgeIdentity<Tag> {
    value: Arc<str>,
    _tag: PhantomData<Tag>,
}

impl<Tag> BridgeIdentity<Tag> {
    pub fn new(value: impl Into<Arc<str>>) -> Self {
        Self {
            value: value.into(),
            _tag: PhantomData,
        }
    }

    pub fn as_str(&self) -> &str {
        self.value.as_ref()
    }
}

impl<Tag> Clone for BridgeIdentity<Tag> {
    fn clone(&self) -> Self {
        Self::new(Arc::clone(&self.value))
    }
}

impl<Tag> CheapClone for BridgeIdentity<Tag> {}

impl<Tag> PartialEq for BridgeIdentity<Tag> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<Tag> Eq for BridgeIdentity<Tag> {}

impl<Tag> PartialOrd for BridgeIdentity<Tag> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Tag> Ord for BridgeIdentity<Tag> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl<Tag> Hash for BridgeIdentity<Tag> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<Tag> fmt::Debug for BridgeIdentity<Tag> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("BridgeIdentity")
            .field(&self.value.as_ref())
            .finish()
    }
}

impl<Tag> fmt::Display for BridgeIdentity<Tag> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<Tag> PartialEq<&str> for BridgeIdentity<Tag> {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl<Tag> PartialEq<BridgeIdentity<Tag>> for &str {
    fn eq(&self, other: &BridgeIdentity<Tag>) -> bool {
        *self == other.as_str()
    }
}

pub enum TruthCommitTag {}
pub enum TruthPatchTag {}
pub enum TruthBranchTag {}
pub enum TruthSnapshotTag {}
pub enum CommittedPatchDigestTag {}
pub enum MappingIdTag {}
pub enum FrozenMappingRegistrationIdentityTag {}
pub enum AspectRegistrationIdTag {}
pub enum SignalInvalidationScopeTag {}
pub enum TruthDeltaSurfaceIdentityTag {}
pub enum TruthViewSelectorIdentityTag {}
pub enum HistoricalEvaluationDeclarationIdentityTag {}
pub enum SnapshotReadTargetIdentityTag {}
pub enum RouteIdentityTag {}
pub enum InvalidationIdentityTag {}
pub enum InvalidationTargetIdentityTag {}
pub enum SubscriptionSliceIdentityTag {}
pub enum SubscriptionDeclarationFamilyIdentityTag {}
pub enum SubscriptionDeclarationIdentityTag {}
pub enum SubscriptionSliceTargetIdentityTag {}
pub enum SubscriptionFamilyRegistryIdentityTag {}
pub enum SubscriptionBasisIdentityTag {}
pub enum SubscriptionSignalStrategyIdentityTag {}
pub enum AdmittedSubscriptionIdentityTag {}
pub enum SubscriptionLifecycleIdentityTag {}
pub enum SubscriptionReplayIdentityTag {}
pub enum SubscriptionDeliveryCostProfileIdentityTag {}
pub enum SubscriptionConsumerContractIdentityTag {}
pub enum SubscriptionActiveIdentityTag {}
pub enum SubscriptionDeliveryFamilyIdentityTag {}
pub enum SubscriptionDeliveryWindowOpenIdentityTag {}
pub enum SubscriptionDeliveryWindowIdentityTag {}
pub enum SubscriptionDeliveryMemberIdentityTag {}
pub enum SubscriptionDeliveryContentDigestTag {}
pub enum SubscriptionDeliveryDiagnosticsReferenceIdentityTag {}
pub enum SubscriptionDeliveryBufferLifecycleIdentityTag {}
pub enum SubscriptionSharingEligibilityIdentityTag {}
pub enum SubscriptionFanoutPlanIdentityTag {}
pub enum SubscriptionFanoutLayoutIdentityTag {}
pub enum SubscriptionFanoutConsumerBindingIdentityTag {}
pub enum SubscriptionFanoutDeliveryProjectionIdentityTag {}
pub enum SubscriptionFanoutDeliveryProjectionSetIdentityTag {}
pub enum SubscriptionRetainedDeliveryWindowSeedIdentityTag {}
pub enum SubscriptionRetainedDeliveryReplaySeedIdentityTag {}
pub enum SubscriptionFanoutProjectionValidationIdentityTag {}
pub enum SubscriptionDeliveryReplayReadinessIdentityTag {}
pub enum SubscriptionDeliveryReplayPlanIdentityTag {}
pub enum SubscriptionAcknowledgementFrontierIdentityTag {}
pub enum SubscriptionCheckpointReadyIdentityTag {}
pub enum SubscriptionCheckpointIdentityTag {}
pub enum SubscriptionResumeAdmissionIdentityTag {}
pub enum SubscriptionResumePlanIdentityTag {}
pub enum SubscriptionDuplicateReplayPolicyIdentityTag {}
pub enum SubscriptionContinuationIndexIdentityTag {}
pub enum SubscriptionContinuationCandidateIdentityTag {}
pub enum SubscriptionContinuationDecisionIdentityTag {}
pub enum SubscriptionContinuationChildIdentityTag {}
pub enum SubscriptionPreviewBasisIdentityTag {}
pub enum SubscriptionPreviewActiveIdentityTag {}
pub enum SubscriptionPreviewScopeIdentityTag {}
pub enum SubscriptionPreviewParentBasisIdentityTag {}
pub enum SubscriptionPreviewLifecycleIdentityTag {}
pub enum SubscriptionPreviewResidueScopeIdentityTag {}
pub enum SubscriptionPreviewResidueScopeIndexIdentityTag {}
pub enum SubscriptionPreviewResidueArtifactIdentityTag {}
pub enum SubscriptionPreviewDiscardResidueProofIdentityTag {}
pub enum SubscriptionPreviewPromotionRecordIdentityTag {}
pub enum SubscriptionPreviewWorkTraceIdentityTag {}
pub enum SubscriptionPreviewWorkRecordIdentityTag {}
pub enum ContinuityIdentityTag {}
pub enum HistoricalResolvedLineageIdentityTag {}
pub enum HistoricalResolvedRecordIdentityTag {}
pub enum HistoricalEvaluationRecordIdentityTag {}
pub enum HistoricalEvaluationDecisionLogIdentityTag {}
pub enum HistoricalEvaluationArtifactIdentityTag {}
pub enum HistoricalEvaluationFailureIdentityTag {}
pub enum WorkloadIdentityTag {}
pub enum BulkPlanningIdentityTag {}
pub enum BulkAdmissionProfileIdentityTag {}
pub enum BulkPacketRegionIdentityTag {}
pub enum BulkContinuityMemberIdentityTag {}
pub enum BulkTruthViewMemberIdentityTag {}
pub enum BulkWorkloadSegmentIdentityTag {}
pub enum ReducedPublicationIdentityTag {}
pub enum ReducedRoutingTargetIdentityTag {}
pub enum ReducedTruthViewIdentityTag {}
pub enum ReducedContinuityIdentityTag {}
pub enum ReducedWideningIdentityTag {}
pub enum RoutingPacketIdentityTag {}
pub enum TruthViewPacketIdentityTag {}
pub enum ContinuityPacketIdentityTag {}
pub enum WideningPacketIdentityTag {}
pub enum ReductionPacketIdentityTag {}
pub enum ChangeStreamDeclarationIdentityTag {}
pub enum StreamProtocolIdentityTag {}
pub enum ConsumerContractIdentityTag {}
pub enum StreamMemberIdentityTag {}
pub enum StreamPositionIdentityTag {}
pub enum StreamWindowIdentityTag {}
pub enum CheckpointTokenIdentityTag {}
pub enum StreamReplayRecordIdentityTag {}
pub enum BackpressureDecisionIdentityTag {}
pub enum SourceDeclarationIdentityTag {}
pub enum SourceContractIdentityTag {}
pub enum SourceMaterializationRecordIdentityTag {}
pub enum SourceFailureRecordIdentityTag {}
pub enum StructuralSchemaIdentityTag {}
pub enum StructuralEquivalenceContractIdentityTag {}
pub enum StructuralDeclarationIdentityTag {}
pub enum StructuralContractIdentityTag {}
pub enum StructuralTruthViewBasisIdentityTag {}
pub enum MergeDeclarationIdentityTag {}
pub enum MergeContractIdentityTag {}
pub enum MergeAuthorityBasisIdentityTag {}
pub enum MergeOntologyMappingIdentityTag {}
pub enum MergeParentOrderIdentityTag {}
pub enum MergeRecordIdentityTag {}
pub enum StructuralCandidateIdentityTag {}
pub enum StructuralFingerprintIdentityTag {}
pub enum StructuralRemapRecordIdentityTag {}
pub enum StructuralBranchComparisonRecordIdentityTag {}
pub enum SpeculativeSignalBranchIdentityTag {}
pub enum PreviewBranchBindingIdentityTag {}
pub enum PreviewSessionDeclarationIdentityTag {}
pub enum PreviewSessionIdentityTag {}
pub enum PreviewExecutionRecordIdentityTag {}
pub enum PreviewDiscardRecordIdentityTag {}
pub enum PreviewPromotionRecordIdentityTag {}
pub enum PromotionAdmissibilityProofIdentityTag {}
pub enum PreviewReuseEquivalenceIdentityTag {}
pub enum PolicyDeclarationIdentityTag {}
pub enum PolicyContractIdentityTag {}
pub enum LoweredExecutionPolicyIdentityTag {}
pub enum PolicyProvenanceIdentityTag {}
pub enum WritebackDeclarationIdentityTag {}
pub enum WritebackFamilyIdentityTag {}
pub enum WritebackStrategyIdentityTag {}
pub enum WritebackStrategyCoherenceIdentityTag {}
pub enum WritebackContractIdentityTag {}
pub enum WritebackAdmissionRecordIdentityTag {}
pub enum WritebackCausalityIdentityTag {}
pub enum WritebackMapperEnvelopeIdentityTag {}
pub enum WritebackMappedFamilyInputIdentityTag {}
pub enum WritebackEffectIdentityTag {}
pub enum WritebackIdempotenceIdentityTag {}
pub enum WritebackLoopPreventionIdentityTag {}
pub enum WritebackCandidateIdentityTag {}
pub enum WritebackMapperWitnessIdentityTag {}
pub enum WritebackMapperRecordIdentityTag {}
pub enum WritebackExecutionRecordIdentityTag {}
pub enum WritebackReplayRecordIdentityTag {}
