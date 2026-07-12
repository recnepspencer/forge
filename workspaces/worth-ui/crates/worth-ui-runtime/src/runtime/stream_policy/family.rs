/// Closed ingress taxonomy. Hosts and Query adapters classify into this enum;
/// they never choose a cadence policy themselves.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiAllocationStreamFamily {
    TextInput,
    QueryProjection,
    ResizePreview,
    DurableResize,
    ViewportObservation,
    ScrollExtentObservation,
    PortalAnchorObservation,
    HostMeasurementReplacement,
}

impl UiAllocationStreamFamily {
    pub const ALL: [Self; 8] = [
        Self::TextInput,
        Self::QueryProjection,
        Self::HostMeasurementReplacement,
        Self::ViewportObservation,
        Self::DurableResize,
        Self::ResizePreview,
        Self::ScrollExtentObservation,
        Self::PortalAnchorObservation,
    ];

    pub const fn canonical_order(self) -> u8 {
        match self {
            Self::TextInput => 0,
            Self::QueryProjection => 1,
            Self::HostMeasurementReplacement => 2,
            Self::ViewportObservation => 3,
            Self::DurableResize => 4,
            Self::ResizePreview => 5,
            Self::ScrollExtentObservation => 6,
            Self::PortalAnchorObservation => 7,
        }
    }
}
