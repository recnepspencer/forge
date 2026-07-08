#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum UiMeasurementCoordinateSpace {
    Viewport,
    Window,
    GraphNodeLocal,
    HostSurface,
    PortalLayer,
}

impl UiMeasurementCoordinateSpace {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Viewport => "viewport",
            Self::Window => "window",
            Self::GraphNodeLocal => "graph_node_local",
            Self::HostSurface => "host_surface",
            Self::PortalLayer => "portal_layer",
        }
    }
}
