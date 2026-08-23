use worth_signal::facade::{
    AspectMask, AsyncNodeCapabilityDeclaration, AsyncNodePayloadContract,
    AsyncNodePayloadContractId, NodeId, ResourceCancellationPolicyDeclaration,
    ResourceRetryPolicyDeclaration, ResourceSupersessionPolicyDeclaration,
    ResourceTimeoutPolicyDeclaration, TemporalDuration,
};

use super::aspects::UiNativePhysicalSignalAspect;

pub(crate) const PHYSICAL_SIGNAL_ROUTE_CAPACITY: usize = 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiNativePhysicalSignalOperation {
    AtlasUpload,
    PresentationReadback,
    Recovery,
}

impl UiNativePhysicalSignalOperation {
    pub(crate) const fn from_index(index: usize) -> Self {
        match index {
            0 => Self::AtlasUpload,
            1 => Self::PresentationReadback,
            2 => Self::Recovery,
            _ => panic!("physical Signal operation index is bounded"),
        }
    }

    pub(crate) const fn index(self) -> usize {
        match self {
            Self::AtlasUpload => 0,
            Self::PresentationReadback => 1,
            Self::Recovery => 2,
        }
    }

    pub(crate) const fn partition(self) -> &'static str {
        match self {
            Self::AtlasUpload => "atlas-upload",
            Self::PresentationReadback => "presentation-readback",
            Self::Recovery => "recovery",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiNativePhysicalSignalResourceDeclaration {
    operation: UiNativePhysicalSignalOperation,
}

impl UiNativePhysicalSignalResourceDeclaration {
    pub(crate) const fn new(operation: UiNativePhysicalSignalOperation) -> Self {
        Self { operation }
    }

    pub(crate) fn reads(self) -> AspectMask {
        match self.operation {
            UiNativePhysicalSignalOperation::AtlasUpload => AspectMask::from([
                UiNativePhysicalSignalAspect::HostLineage.signal_aspect(),
                UiNativePhysicalSignalAspect::WorkIdentity.signal_aspect(),
                UiNativePhysicalSignalAspect::Demand.signal_aspect(),
                UiNativePhysicalSignalAspect::Target.signal_aspect(),
                UiNativePhysicalSignalAspect::Submission.signal_aspect(),
            ]),
            UiNativePhysicalSignalOperation::PresentationReadback => AspectMask::from([
                UiNativePhysicalSignalAspect::HostLineage.signal_aspect(),
                UiNativePhysicalSignalAspect::WorkIdentity.signal_aspect(),
                UiNativePhysicalSignalAspect::Target.signal_aspect(),
                UiNativePhysicalSignalAspect::Submission.signal_aspect(),
            ]),
            UiNativePhysicalSignalOperation::Recovery => AspectMask::from([
                UiNativePhysicalSignalAspect::HostLineage.signal_aspect(),
                UiNativePhysicalSignalAspect::WorkIdentity.signal_aspect(),
                UiNativePhysicalSignalAspect::Submission.signal_aspect(),
                UiNativePhysicalSignalAspect::Recovery.signal_aspect(),
            ]),
        }
    }

    pub(crate) const fn payload_contract_id(self) -> u64 {
        match self.operation {
            UiNativePhysicalSignalOperation::AtlasUpload => 0x574f_5254_485f_4154,
            UiNativePhysicalSignalOperation::PresentationReadback => 0x574f_5254_485f_5052,
            UiNativePhysicalSignalOperation::Recovery => 0x574f_5254_485f_5243,
        }
    }

    pub(crate) fn capability(self, node: NodeId) -> AsyncNodeCapabilityDeclaration {
        AsyncNodeCapabilityDeclaration::new(
            node,
            AsyncNodePayloadContract::new(AsyncNodePayloadContractId::new(
                self.payload_contract_id(),
            )),
        )
        .with_retry_policy(ResourceRetryPolicyDeclaration::FixedDelay {
            delay: TemporalDuration::temporal_duration(1)
                .expect("physical retry delay is positive"),
        })
        .with_retry_max_attempts(2)
        .with_timeout_policy(ResourceTimeoutPolicyDeclaration::PerAttemptTimeout {
            timeout: TemporalDuration::temporal_duration(8)
                .expect("physical attempt timeout is positive"),
        })
        .with_cancellation_policy(
            ResourceCancellationPolicyDeclaration::BestEffortHostSignalAndRuntimeDenial,
        )
        .with_supersession_policy(
            ResourceSupersessionPolicyDeclaration::OverlappingGenerationRetainsOldHostWork,
        )
    }
}
