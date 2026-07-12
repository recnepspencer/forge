/// Move-only authority proving viewport policy and locality preflight ran over
/// the exact plan and selection submitted to receipt commitment.
pub(crate) struct UiViewportResizeCommitBasis<'a> {
    plan: super::UiViewportResolvedFramePlan<'a>,
    selection: crate::graph::UiAdmittedReplanNeighborhoodSet,
}

impl<'a> UiViewportResizeCommitBasis<'a> {
    pub(in crate::runtime) fn select(
        plan: super::UiViewportResolvedFramePlan<'a>,
        authority: &crate::runtime::invalidation_narrowing::UiAllocationInvalidationAuthority,
    ) -> Result<Self, crate::graph::UiReplanLocalityDenial> {
        let selection = crate::graph::select_replan_neighborhoods(plan.plan(), authority)?;
        Ok(Self { plan, selection })
    }

    pub(crate) fn plan(&self) -> &'a crate::runtime::UiNarrowedAllocationFramePlan {
        self.plan.plan()
    }

    pub(crate) fn selection(&self) -> &crate::graph::UiAdmittedReplanNeighborhoodSet {
        &self.selection
    }
}
