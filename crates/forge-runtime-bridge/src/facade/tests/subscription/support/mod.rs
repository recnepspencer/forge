#![allow(dead_code, unused_imports)]

pub(crate) use crate::facade::{
    BridgeSignalStrategyKind, BridgeSubscriptionAdmissionRejectionKind,
    BridgeSubscriptionBasisKind, BridgeSubscriptionBasisRequest,
    BridgeSubscriptionBasisResolutionFailureKind, BridgeSubscriptionConsumerBackpressurePosture,
    BridgeSubscriptionConsumerContractFamily, BridgeSubscriptionConsumerDiagnosticsRetention,
    BridgeSubscriptionConsumerPacingCapability, BridgeSubscriptionDeclarationFamilyKind,
    BridgeSubscriptionDeliveryContentDigest, BridgeSubscriptionDeliveryContentOmissionReason,
    BridgeSubscriptionDeliveryDensityPosture, BridgeSubscriptionDeliveryFamilyKind,
    BridgeSubscriptionDeliveryIntentClass, BridgeSubscriptionDeliveryMemberClass,
    BridgeSubscriptionDeliveryMemberInput, BridgeSubscriptionPreviewWorkInput,
    BridgeSubscriptionPreviewWorkTrace, NormalizedSubscriptionSliceIntent,
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

pub(crate) fn runtime(policy: BridgeRuntimePolicy) -> crate::facade::RuntimeBridge {
    super::super::runtime(policy)
}

mod delivery;
mod preview;
mod runtime_sources;
mod subscriptions;

pub(crate) use delivery::*;
pub(crate) use preview::*;
pub(crate) use runtime_sources::*;
pub(crate) use subscriptions::*;
