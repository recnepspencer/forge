#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiFrameworkTransitionPlanningDenial {
    FrameEpochAssignmentMismatch,
    DurableResizeIdentityMissing,
    DurableResizeExtentMissing,
    DragResizeIdentityMissing,
    DragResizeExtentMissing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiFrameworkTransitionExecutionDenial {
    ActiveApplicationGenerationChanged,
    ActiveFrameEpochChanged,
    SourceOrderAuthorityChanged,
    DurableSemanticStateMissing,
}
