#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthUiPlanNodeInputFamily {
    ComponentInvocation,
    ChildRange,
    Command,
    TokenStyle,
    StateSlot,
    LayoutRegion,
    QueryViewBinding,
    Accessibility,
    DiagnosticsRef,
    LanePartitionRef,
    RenderResourceRef,
    CanvasSpatial,
    RealtimeOverlay,
}
