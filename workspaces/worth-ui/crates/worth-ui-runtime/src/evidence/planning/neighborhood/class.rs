use crate::evidence::UiMeasurementNeighborhoodClassHint;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiAllocationNeighborhoodClass {
    LocalIntrinsicContent,
    ContainerPeerGroup,
    Viewport,
    ScrollContainer,
    PortalAnchor,
}

impl UiAllocationNeighborhoodClass {
    pub(crate) const fn from_measurement_hint(hint: UiMeasurementNeighborhoodClassHint) -> Self {
        match hint {
            UiMeasurementNeighborhoodClassHint::LocalIntrinsicContentDependency => {
                Self::LocalIntrinsicContent
            }
            UiMeasurementNeighborhoodClassHint::ContainerAvailableSpaceDependency => {
                Self::ContainerPeerGroup
            }
            UiMeasurementNeighborhoodClassHint::ViewportDependency => Self::Viewport,
            UiMeasurementNeighborhoodClassHint::ScrollContainerDependency => Self::ScrollContainer,
            UiMeasurementNeighborhoodClassHint::PortalAnchorDependency => Self::PortalAnchor,
        }
    }
}
