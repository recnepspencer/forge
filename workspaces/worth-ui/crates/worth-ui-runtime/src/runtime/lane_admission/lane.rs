#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthUiExecutionLane {
    OrdinaryWidgetShell,
    VirtualizedData,
    CanvasSpatial,
    RealtimeOverlayHud,
    QueryBound,
    CommandSurface,
    StyleToken,
    DiagnosticsProjection,
    LaneBoundary,
    EguiBoundary,
    RenderResource,
    SpecialCaseExtension,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthUiLaneCostRegime {
    LocalTraversal,
    WindowedTraversal,
    SpatialIndexTraversal,
    FrameSynchronizedTraversal,
    QueryRuntimeBacked,
    BoundaryOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthUiLaneFailureMode {
    LocalWidgetFailure,
    WindowInvalidationFailure,
    SpatialHitTestFailure,
    RealtimeFrameMiss,
    QuerySupportDenial,
    BoundaryAdmissionFailure,
}

impl WorthUiExecutionLane {
    pub(crate) fn canonical_tag(self) -> u64 {
        match self {
            Self::OrdinaryWidgetShell => 1,
            Self::VirtualizedData => 2,
            Self::CanvasSpatial => 3,
            Self::RealtimeOverlayHud => 4,
            Self::QueryBound => 5,
            Self::CommandSurface => 6,
            Self::StyleToken => 7,
            Self::DiagnosticsProjection => 8,
            Self::LaneBoundary => 9,
            Self::EguiBoundary => 10,
            Self::RenderResource => 11,
            Self::SpecialCaseExtension => 12,
        }
    }
}

impl WorthUiLaneCostRegime {
    pub(crate) fn canonical_tag(self) -> u64 {
        match self {
            Self::LocalTraversal => 1,
            Self::WindowedTraversal => 2,
            Self::SpatialIndexTraversal => 3,
            Self::FrameSynchronizedTraversal => 4,
            Self::QueryRuntimeBacked => 5,
            Self::BoundaryOnly => 6,
        }
    }
}

impl WorthUiLaneFailureMode {
    pub(crate) fn canonical_tag(self) -> u64 {
        match self {
            Self::LocalWidgetFailure => 1,
            Self::WindowInvalidationFailure => 2,
            Self::SpatialHitTestFailure => 3,
            Self::RealtimeFrameMiss => 4,
            Self::QuerySupportDenial => 5,
            Self::BoundaryAdmissionFailure => 6,
        }
    }
}
