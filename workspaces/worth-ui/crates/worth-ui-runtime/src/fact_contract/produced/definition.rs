#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiProducedFactOwner {
    SourceIngress,
    HostViewport,
    HostDeviceScale,
    MeasurementExchange,
    QueryBinding,
    IntentRuntime,
    ScrollRuntimeState,
    PortalRuntimeState,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiProducedFactFamily {
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
pub enum UiProducedFactResetPosture {
    NoReset,
    OwnerIssuedReset,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiProducedFactContract {
    owner: UiProducedFactOwner,
    family: UiProducedFactFamily,
    reset: UiProducedFactResetPosture,
}

impl UiProducedFactContract {
    pub const fn for_owner(owner: UiProducedFactOwner) -> Self {
        match owner {
            UiProducedFactOwner::SourceIngress => {
                Self::new(owner, UiProducedFactFamily::AuthoredSource, false)
            }
            UiProducedFactOwner::HostViewport => {
                Self::new(owner, UiProducedFactFamily::HostViewport, false)
            }
            UiProducedFactOwner::HostDeviceScale => {
                Self::new(owner, UiProducedFactFamily::HostDeviceScale, false)
            }
            UiProducedFactOwner::MeasurementExchange => {
                Self::new(owner, UiProducedFactFamily::Measurement, false)
            }
            UiProducedFactOwner::QueryBinding => {
                Self::new(owner, UiProducedFactFamily::Query, true)
            }
            UiProducedFactOwner::IntentRuntime => {
                Self::new(owner, UiProducedFactFamily::IntentPosture, false)
            }
            UiProducedFactOwner::ScrollRuntimeState => {
                Self::new(owner, UiProducedFactFamily::CommittedScrollExtent, false)
            }
            UiProducedFactOwner::PortalRuntimeState => {
                Self::new(owner, UiProducedFactFamily::CommittedPortalAnchor, false)
            }
        }
    }

    const fn new(
        owner: UiProducedFactOwner,
        family: UiProducedFactFamily,
        owner_issued_reset: bool,
    ) -> Self {
        Self {
            owner,
            family,
            reset: if owner_issued_reset {
                UiProducedFactResetPosture::OwnerIssuedReset
            } else {
                UiProducedFactResetPosture::NoReset
            },
        }
    }

    pub const fn owner(self) -> UiProducedFactOwner {
        self.owner
    }

    pub const fn family(self) -> UiProducedFactFamily {
        self.family
    }

    pub const fn reset_posture(self) -> UiProducedFactResetPosture {
        self.reset
    }
}
