use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::sync::Arc;

use crate::clone_budget::CheapClone;

pub struct BridgeIdentity<Tag> {
    value: Arc<str>,
    payload: BridgeIdentityPayload,
    _tag: PhantomData<Tag>,
}

pub struct BridgeIdentityEvidence {
    value: Arc<str>,
    payload: BridgeIdentityEvidencePayload,
}

impl BridgeIdentityEvidence {
    pub fn as_str(&self) -> &str {
        self.value.as_ref()
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    pub fn from_external_authority(value: impl AsRef<str>) -> Self {
        Self {
            value: Arc::from(value.as_ref()),
            payload: BridgeIdentityEvidencePayload::ExternalAuthority,
        }
    }

    pub(crate) fn from_arc(value: &Arc<str>) -> Self {
        Self {
            value: Arc::clone(value),
            payload: BridgeIdentityEvidencePayload::ExternalAuthority,
        }
    }

    pub(crate) fn from_canonical_bridge_evidence(
        value: impl Into<Arc<str>>,
        scope: &'static str,
    ) -> Self {
        Self {
            value: value.into(),
            payload: BridgeIdentityEvidencePayload::CanonicalBridgeEvidence { scope },
        }
    }
}

impl Clone for BridgeIdentityEvidence {
    fn clone(&self) -> Self {
        Self {
            value: Arc::clone(&self.value),
            payload: self.payload.clone(),
        }
    }
}

impl PartialEq for BridgeIdentityEvidence {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.payload == other.payload
    }
}

impl Eq for BridgeIdentityEvidence {}

impl PartialOrd for BridgeIdentityEvidence {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BridgeIdentityEvidence {
    fn cmp(&self, other: &Self) -> Ordering {
        self.payload
            .cmp(&other.payload)
            .then_with(|| self.value.cmp(&other.value))
    }
}

impl Hash for BridgeIdentityEvidence {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.payload.hash(state);
        self.value.hash(state);
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum BridgeIdentityEvidencePayload {
    ExternalAuthority,
    CanonicalBridgeEvidence { scope: &'static str },
}

impl AsRef<str> for BridgeIdentityEvidence {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Debug for BridgeIdentityEvidence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("BridgeIdentityEvidence")
            .field(&"<opaque>")
            .finish()
    }
}

impl fmt::Display for BridgeIdentityEvidence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<Tag> BridgeIdentity<Tag> {
    pub(crate) fn new(value: impl Into<Arc<str>>) -> Self {
        Self {
            value: value.into(),
            payload: BridgeIdentityPayload::OpaqueText,
            _tag: PhantomData,
        }
    }

    pub(crate) fn with_payload(value: impl Into<Arc<str>>, payload: BridgeIdentityPayload) -> Self {
        Self {
            value: value.into(),
            payload,
            _tag: PhantomData,
        }
    }

    pub(crate) fn as_str(&self) -> &str {
        self.value.as_ref()
    }

    pub(crate) fn payload(&self) -> &BridgeIdentityPayload {
        &self.payload
    }

    pub fn evidence_identity(&self) -> BridgeIdentityEvidence {
        BridgeIdentityEvidence {
            value: Arc::clone(&self.value),
            payload: BridgeIdentityEvidencePayload::ExternalAuthority,
        }
    }
}

impl<Tag> Clone for BridgeIdentity<Tag> {
    fn clone(&self) -> Self {
        Self::with_payload(Arc::clone(&self.value), self.payload.clone())
    }
}

impl<Tag> CheapClone for BridgeIdentity<Tag> {}

impl<Tag> PartialEq for BridgeIdentity<Tag> {
    fn eq(&self, other: &Self) -> bool {
        match (&self.payload, &other.payload) {
            (BridgeIdentityPayload::OpaqueText, BridgeIdentityPayload::OpaqueText) => {
                self.value == other.value
            }
            _ => self.payload == other.payload,
        }
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
        match (&self.payload, &other.payload) {
            (BridgeIdentityPayload::OpaqueText, BridgeIdentityPayload::OpaqueText) => {
                self.value.cmp(&other.value)
            }
            _ => self.payload.cmp(&other.payload),
        }
    }
}

impl<Tag> Hash for BridgeIdentity<Tag> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self.payload {
            BridgeIdentityPayload::OpaqueText => self.value.hash(state),
            payload => payload.hash(state),
        }
    }
}

impl<Tag> fmt::Debug for BridgeIdentity<Tag> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("BridgeIdentity").field(&"<opaque>").finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum BridgeIdentityPayload {
    OpaqueText,
    RelationalBranch { branch_id: Arc<str> },
    RelationalCommit { commit_id: u64 },
    RelationalPatch { patch_position: u64 },
    RelationalSnapshot { snapshot_id: u64, version_id: u64 },
}

#[cfg(test)]
mod tests {
    use super::{BridgeIdentity, BridgeIdentityPayload, TruthCommitTag};

    #[test]
    fn debug_does_not_expose_identity_payload() {
        let identity = BridgeIdentity::<TruthCommitTag>::new("commit-1");

        let debug = format!("{identity:?}");

        assert!(!debug.contains("commit-1"));
        assert!(debug.contains("<opaque>"));
    }

    #[test]
    fn relational_truth_identity_can_store_typed_payload() {
        let identity = BridgeIdentity::<TruthCommitTag>::with_payload(
            "relational-commit:7",
            BridgeIdentityPayload::RelationalCommit { commit_id: 7 },
        );

        assert_eq!(
            identity.payload(),
            &BridgeIdentityPayload::RelationalCommit { commit_id: 7 }
        );
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
pub enum SubscriptionRetainedResumeBasisIdentityTag {}
pub enum SubscriptionRetainedTemporalResumeBasisIdentityTag {}
pub enum SubscriptionRetainedInflightAsyncResumeBasisIdentityTag {}
pub enum SubscriptionRetainedDeliveryResumeBasisIdentityTag {}
pub enum SubscriptionAdmittedResumeBasisIdentityTag {}
pub enum SubscriptionReplayReadinessIdentityTag {}
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
pub enum SubscriptionPreviewLifecycleResidueEnvelopeIdentityTag {}
pub enum SubscriptionPreviewDiscardResidueProofIdentityTag {}
pub enum SubscriptionPreviewPromotionRecordIdentityTag {}
pub enum SubscriptionPreviewLifecyclePromotionIdentityTag {}
pub enum SubscriptionPreviewAuthoritativeReadmissionIdentityTag {}
pub enum SubscriptionPreviewWorkTraceIdentityTag {}
pub enum SubscriptionPreviewWorkRecordIdentityTag {}
pub enum SubscriptionTemporalAdmissionIdentityTag {}
pub enum SubscriptionTemporalActivationReadyIdentityTag {}
pub enum SubscriptionTemporalWakeRoutingRequestIdentityTag {}
pub enum SubscriptionHistoricalTruthBasisIdentityTag {}
pub enum SubscriptionHistoricalPreviousValueEvidenceIdentityTag {}
pub enum SubscriptionHistoricalTemporalReplayBasisIdentityTag {}
pub enum SubscriptionHistoricalTemporalReplayRequestIdentityTag {}
pub enum SubscriptionHistoricalTemporalReadinessIdentityTag {}
pub enum SubscriptionPreviewTemporalAdmissionIdentityTag {}
pub enum SubscriptionPreviewTemporalActivationReadyIdentityTag {}
pub enum SubscriptionTemporalCauseRecordIdentityTag {}
pub enum SubscriptionTemporalDeliveryPlanIdentityTag {}
pub enum SubscriptionMixedCauseOrderingRequestIdentityTag {}
pub enum SubscriptionMixedCauseOrderedCauseIdentityTag {}
pub enum SubscriptionMixedCauseSuppressedCauseIdentityTag {}
pub enum SubscriptionMixedCauseDeniedCauseIdentityTag {}
pub enum SubscriptionMixedCauseOrderingIdentityTag {}
pub enum SubscriptionMixedCauseDeliveryWindowIdentityTag {}
pub enum SubscriptionSharedDeliveryPlanIdentityTag {}
pub enum SubscriptionSharedDeliveryLayoutIdentityTag {}
pub enum SubscriptionSharedDeliveryBundleDraftIdentityTag {}
pub enum SubscriptionSharedDeliveryBundleSealedIdentityTag {}
pub enum SubscriptionSharedDeliveryProjectionIdentityTag {}
pub enum SubscriptionSharedDeliveryAcknowledgementIdentityTag {}
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
pub enum TemporalBasisIdentityTag {}
pub enum TemporalCdcCursorIdentityTag {}
pub enum AsyncSourceDeclarationIdentityTag {}
pub enum AsyncSourceLoweringIdentityTag {}
pub enum AsyncRequestTruthViewBasisIdentityTag {}
pub enum AsyncRequestSubscriptionInstanceIdentityTag {}
pub enum AsyncRequestBasisBindingIdentityTag {}
pub enum AsyncRequestIdentityTag {}
pub enum AsyncInFlightRequestIdentityTag {}
pub enum AsyncCompletionEnvelopeIdentityTag {}
pub enum AsyncCompletionIdentityTag {}
pub enum AsyncCompletionReceiptIdentityTag {}
pub enum AsyncCompletionDenialIdentityTag {}
pub enum AsyncCompletionDenialReceiptIdentityTag {}
pub enum AsyncCompletionSupersessionIdentityTag {}
pub enum AsyncCompletionSupersessionReceiptIdentityTag {}
pub enum AsyncForwardCausalityIdentityTag {}
pub enum AsyncForwardCausalityReceiptIdentityTag {}
pub enum AsyncWritebackAdmissionIdentityTag {}
pub enum AsyncWritebackMapperOutputIdentityTag {}
pub enum AsyncWritebackStagedEffectIdentityTag {}
pub enum AsyncWritebackCommittedIdentityTag {}
pub enum AsyncWritebackNoopIdentityTag {}
pub enum AsyncWritebackRejectedIdentityTag {}
pub enum AsyncWritebackCausalityTransferReceiptIdentityTag {}
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
