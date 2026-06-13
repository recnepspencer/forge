#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthUiPlanNodeInputFamily {
    ComponentInvocation,
    ChildRange,
    Command,
    TokenStyle,
    LayoutRegion,
    QueryViewBinding,
    Accessibility,
    DiagnosticsRef,
    LanePartitionRef,
    EguiBoundaryRef,
    RenderResourceRef,
}
