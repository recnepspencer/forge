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
pub enum AspectRegistrationIdTag {}
pub enum SignalInvalidationScopeTag {}
pub enum TruthDeltaSurfaceIdentityTag {}
pub enum TruthViewSelectorIdentityTag {}
pub enum HistoricalEvaluationDeclarationIdentityTag {}
pub enum RouteIdentityTag {}
pub enum InvalidationIdentityTag {}
pub enum SubscriptionSliceIdentityTag {}
pub enum ContinuityIdentityTag {}
pub enum HistoricalEvaluationRecordIdentityTag {}
pub enum HistoricalEvaluationDecisionLogIdentityTag {}
pub enum HistoricalEvaluationArtifactIdentityTag {}
pub enum HistoricalEvaluationFailureIdentityTag {}
pub enum WorkloadIdentityTag {}
pub enum BulkPlanningIdentityTag {}
pub enum BulkAdmissionProfileIdentityTag {}
pub enum ReducedPublicationIdentityTag {}
pub enum ReducedRoutingTargetIdentityTag {}
pub enum ReducedTruthViewIdentityTag {}
pub enum ReducedContinuityIdentityTag {}
pub enum ReducedFallbackIdentityTag {}
pub enum RoutingPacketIdentityTag {}
pub enum TruthViewPacketIdentityTag {}
pub enum ContinuityPacketIdentityTag {}
pub enum FallbackPacketIdentityTag {}
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
