use super::{
    UiAuthoredChangedFact, UiCommittedPortalAnchorChangedFact, UiCommittedScrollExtentChangedFact,
    UiHostDeviceScaleChangedFact, UiHostViewportChangedFact, UiMeasurementChangedFact,
    UiProducedFactFamily, UiQueryChangedFact,
};

pub enum UiProducedFact {
    AuthoredSource(UiAuthoredChangedFact),
    HostViewport(UiHostViewportChangedFact),
    HostDeviceScale(UiHostDeviceScaleChangedFact),
    Measurement(UiMeasurementChangedFact),
    Query(UiQueryChangedFact),
    CommittedScrollExtent(UiCommittedScrollExtentChangedFact),
    CommittedPortalAnchor(UiCommittedPortalAnchorChangedFact),
}

impl UiProducedFact {
    pub const fn family(&self) -> UiProducedFactFamily {
        match self {
            Self::AuthoredSource(_) => UiProducedFactFamily::AuthoredSource,
            Self::HostViewport(_) => UiProducedFactFamily::HostViewport,
            Self::HostDeviceScale(_) => UiProducedFactFamily::HostDeviceScale,
            Self::Measurement(_) => UiProducedFactFamily::Measurement,
            Self::Query(_) => UiProducedFactFamily::Query,
            Self::CommittedScrollExtent(_) => UiProducedFactFamily::CommittedScrollExtent,
            Self::CommittedPortalAnchor(_) => UiProducedFactFamily::CommittedPortalAnchor,
        }
    }

    pub fn authored_source(&self) -> Option<&UiAuthoredChangedFact> {
        match self {
            Self::AuthoredSource(fact) => Some(fact),
            _ => None,
        }
    }

    pub fn query(&self) -> Option<&UiQueryChangedFact> {
        match self {
            Self::Query(fact) => Some(fact),
            _ => None,
        }
    }
}
