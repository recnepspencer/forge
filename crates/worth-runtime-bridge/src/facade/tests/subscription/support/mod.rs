#![allow(dead_code, unused_imports)]

pub(crate) use crate::facade::{
    AdmittedBridgeAsyncRequestIdentity, AdmittedBridgeTemporalBasis,
    BridgeAsyncRequestAdmissionRequest, BridgeAsyncRequestTruthViewBasis,
    BridgeAsyncSourceDeclarationDraft, BridgeRequestKind, BridgeRetainedTemporalWakePosture,
    BridgeSignalStrategyKind, BridgeSubscriptionAdmissionRejectionKind,
    BridgeSubscriptionBasisKind, BridgeSubscriptionBasisRequest,
    BridgeSubscriptionBasisResolutionFailureKind, BridgeSubscriptionConsumerBackpressurePosture,
    BridgeSubscriptionConsumerContractFamily, BridgeSubscriptionConsumerDiagnosticsRetention,
    BridgeSubscriptionConsumerPacingCapability, BridgeSubscriptionDeclarationFamilyKind,
    BridgeSubscriptionDeliveryContentDigest, BridgeSubscriptionDeliveryContentOmissionReason,
    BridgeSubscriptionDeliveryDensityPosture, BridgeSubscriptionDeliveryFamilyKind,
    BridgeSubscriptionDeliveryIntentClass, BridgeSubscriptionDeliveryMemberClass,
    BridgeSubscriptionDeliveryMemberInput, BridgeSubscriptionPreviewWorkInput,
    BridgeSubscriptionPreviewWorkTrace, BridgeTemporalSignalBasis,
    BridgeTemporalSubscriptionFamilyKind, BridgeTemporalTruthViewBasis, BridgeTemporalWakeEvidence,
    NormalizedSubscriptionSliceIntent,
};
pub(crate) use crate::input::envelope::TruthBranchIdentity;
pub(crate) use crate::input::envelope::{
    BridgeCommittedPatchEnvelope, BridgeCommittedPatchEnvelopeIdentity, BridgeCommittedPatchItem,
    TruthCommitIdentity, TruthPatchIdentity,
};
pub(crate) use crate::mapping::SubscriptionSliceKind;
pub(crate) use crate::policy::BridgeRuntimePolicy;
pub(crate) use crate::snapshot::{SnapshotReadPacket, TruthSnapshotIdentity, TruthSnapshotReader};
pub(crate) use std::sync::Arc;
pub(crate) use worth_signal::facade::{
    NodeId, ResourceNodeDeclaration, ResourceNodeId, ResourceObservationPolicyDeclaration,
    ResourcePayloadContract, ResourcePayloadContractId,
};

pub(crate) fn runtime(policy: BridgeRuntimePolicy) -> crate::facade::RuntimeBridge {
    super::super::runtime(policy)
}

mod certification;
mod delivery;
mod preview;
mod preview_lifecycle;
mod resume_basis;
mod runtime_sources;
mod shared_delivery;
mod subscriptions;
mod temporal;

pub(crate) use certification::*;
pub(crate) use delivery::*;
pub(crate) use preview::*;
pub(crate) use preview_lifecycle::*;
pub(crate) use resume_basis::*;
pub(crate) use runtime_sources::*;
pub(crate) use shared_delivery::*;
pub(crate) use subscriptions::*;
pub(crate) use temporal::*;
