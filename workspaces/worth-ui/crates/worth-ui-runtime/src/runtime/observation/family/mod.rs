#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiObservationFamily {
    AuthoredSource,
    HostViewport,
    HostDeviceScale,
    Measurement,
    Query,
    IntentPosture,
    CommittedScrollExtent,
    CommittedPortalAnchor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiObservationOwner {
    SourceIngress,
    HostViewport,
    HostDeviceScale,
    MeasurementExchange,
    QueryBinding,
    IntentRuntime,
    ScrollRuntimeState,
    PortalRuntimeState,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiObservationDuplicatePolicy {
    Reject,
    OwnerEquivalentMayCoalesce,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiObservationLossPolicy {
    Lossless,
    OwnerDeclaredLoss,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiObservationResetPolicy {
    NoReset,
    OwnerIssuedReset,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiObservationCoalescingPolicy {
    Forbidden,
    OwnerEquivalentOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiObservationFamilyDefinition {
    family: UiObservationFamily,
    owner: UiObservationOwner,
    framework_rank: u8,
    duplicate: UiObservationDuplicatePolicy,
    loss: UiObservationLossPolicy,
    reset: UiObservationResetPolicy,
    coalescing: UiObservationCoalescingPolicy,
}

const fn definition(
    family: UiObservationFamily,
    owner: UiObservationOwner,
    framework_rank: u8,
    duplicate: UiObservationDuplicatePolicy,
    loss: UiObservationLossPolicy,
    reset: UiObservationResetPolicy,
    coalescing: UiObservationCoalescingPolicy,
) -> UiObservationFamilyDefinition {
    UiObservationFamilyDefinition {
        family,
        owner,
        framework_rank,
        duplicate,
        loss,
        reset,
        coalescing,
    }
}

impl UiObservationFamily {
    pub const fn produced_fact_contract(self) -> crate::fact_contract::UiProducedFactContract {
        crate::fact_contract::UiProducedFactContract::for_owner(match self {
            Self::AuthoredSource => crate::fact_contract::UiProducedFactOwner::SourceIngress,
            Self::HostViewport => crate::fact_contract::UiProducedFactOwner::HostViewport,
            Self::HostDeviceScale => crate::fact_contract::UiProducedFactOwner::HostDeviceScale,
            Self::Measurement => crate::fact_contract::UiProducedFactOwner::MeasurementExchange,
            Self::Query => crate::fact_contract::UiProducedFactOwner::QueryBinding,
            Self::IntentPosture => crate::fact_contract::UiProducedFactOwner::IntentRuntime,
            Self::CommittedScrollExtent => {
                crate::fact_contract::UiProducedFactOwner::ScrollRuntimeState
            }
            Self::CommittedPortalAnchor => {
                crate::fact_contract::UiProducedFactOwner::PortalRuntimeState
            }
        })
    }

    pub const fn definition(self) -> UiObservationFamilyDefinition {
        match self {
            Self::AuthoredSource => definition(
                self,
                UiObservationOwner::SourceIngress,
                0,
                UiObservationDuplicatePolicy::Reject,
                UiObservationLossPolicy::Lossless,
                UiObservationResetPolicy::NoReset,
                UiObservationCoalescingPolicy::Forbidden,
            ),
            Self::HostViewport => {
                host_latest_value_definition(self, UiObservationOwner::HostViewport, 1)
            }
            Self::HostDeviceScale => {
                host_latest_value_definition(self, UiObservationOwner::HostDeviceScale, 2)
            }
            Self::Measurement => definition(
                self,
                UiObservationOwner::MeasurementExchange,
                3,
                UiObservationDuplicatePolicy::Reject,
                UiObservationLossPolicy::Lossless,
                UiObservationResetPolicy::NoReset,
                UiObservationCoalescingPolicy::Forbidden,
            ),
            Self::Query => definition(
                self,
                UiObservationOwner::QueryBinding,
                4,
                UiObservationDuplicatePolicy::Reject,
                UiObservationLossPolicy::OwnerDeclaredLoss,
                UiObservationResetPolicy::OwnerIssuedReset,
                UiObservationCoalescingPolicy::Forbidden,
            ),
            Self::IntentPosture => definition(
                self,
                UiObservationOwner::IntentRuntime,
                5,
                UiObservationDuplicatePolicy::Reject,
                UiObservationLossPolicy::Lossless,
                UiObservationResetPolicy::NoReset,
                UiObservationCoalescingPolicy::Forbidden,
            ),
            Self::CommittedScrollExtent => definition(
                self,
                UiObservationOwner::ScrollRuntimeState,
                6,
                UiObservationDuplicatePolicy::OwnerEquivalentMayCoalesce,
                UiObservationLossPolicy::Lossless,
                UiObservationResetPolicy::NoReset,
                UiObservationCoalescingPolicy::OwnerEquivalentOnly,
            ),
            Self::CommittedPortalAnchor => definition(
                self,
                UiObservationOwner::PortalRuntimeState,
                7,
                UiObservationDuplicatePolicy::OwnerEquivalentMayCoalesce,
                UiObservationLossPolicy::Lossless,
                UiObservationResetPolicy::NoReset,
                UiObservationCoalescingPolicy::OwnerEquivalentOnly,
            ),
        }
    }
}

const fn host_latest_value_definition(
    family: UiObservationFamily,
    owner: UiObservationOwner,
    framework_rank: u8,
) -> UiObservationFamilyDefinition {
    definition(
        family,
        owner,
        framework_rank,
        UiObservationDuplicatePolicy::OwnerEquivalentMayCoalesce,
        UiObservationLossPolicy::OwnerDeclaredLoss,
        UiObservationResetPolicy::NoReset,
        UiObservationCoalescingPolicy::OwnerEquivalentOnly,
    )
}

impl UiObservationFamilyDefinition {
    pub const fn family(self) -> UiObservationFamily {
        self.family
    }

    pub const fn owner(self) -> UiObservationOwner {
        self.owner
    }

    pub const fn framework_rank(self) -> u8 {
        self.framework_rank
    }

    pub const fn duplicate_policy(self) -> UiObservationDuplicatePolicy {
        self.duplicate
    }

    pub const fn loss_policy(self) -> UiObservationLossPolicy {
        self.loss
    }

    pub const fn reset_policy(self) -> UiObservationResetPolicy {
        self.reset
    }

    pub const fn coalescing_policy(self) -> UiObservationCoalescingPolicy {
        self.coalescing
    }
}
