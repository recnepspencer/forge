pub(crate) enum UiResolvedAllocationCommitPlan<'a> {
    Ordinary,
    Viewport(UiViewportResolvedFramePlan<'a>),
    ResizePreview(&'a crate::runtime::UiNarrowedAllocationFramePlan),
    DurableResize(&'a crate::runtime::UiNarrowedAllocationFramePlan),
    DragResize(&'a crate::runtime::UiNarrowedAllocationFramePlan),
}

/// Move-only proof that stream-policy resolution assigned this exact narrowed
/// frame to the viewport-derived commit lane.
///
/// ```compile_fail
/// use worth_ui_runtime::facade::runtime_handoff::UiViewportResolvedFramePlan;
/// ```
pub(crate) struct UiViewportResolvedFramePlan<'a> {
    plan: &'a crate::runtime::UiNarrowedAllocationFramePlan,
}

impl<'a> UiResolvedAllocationCommitPlan<'a> {
    pub(crate) fn classify(plan: &'a crate::runtime::UiNarrowedAllocationFramePlan) -> Self {
        match plan.policy().commit_lane() {
            crate::runtime::stream_policy::UiAllocationResolvedCommitLane::Ordinary => {
                Self::Ordinary
            }
            crate::runtime::stream_policy::UiAllocationResolvedCommitLane::ViewportDerived => {
                Self::Viewport(UiViewportResolvedFramePlan { plan })
            }
            crate::runtime::stream_policy::UiAllocationResolvedCommitLane::ResizePreview => {
                Self::ResizePreview(plan)
            }
            crate::runtime::stream_policy::UiAllocationResolvedCommitLane::DurableResize => {
                Self::DurableResize(plan)
            }
            crate::runtime::stream_policy::UiAllocationResolvedCommitLane::DragResize => {
                Self::DragResize(plan)
            }
        }
    }
}

impl<'a> UiViewportResolvedFramePlan<'a> {
    pub(super) fn plan(&self) -> &'a crate::runtime::UiNarrowedAllocationFramePlan {
        self.plan
    }
}
