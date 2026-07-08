#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiMeasurementNeighborhoodClassHint {
    LocalIntrinsicContentDependency,
    ContainerAvailableSpaceDependency,
    ViewportDependency,
    ScrollContainerDependency,
    PortalAnchorDependency,
}
