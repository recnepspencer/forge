/// Move-only authority proving viewport policy and locality preflight ran over
/// the exact plan and selection submitted to receipt commitment.
pub(crate) struct UiViewportResizeCommitBasis {
    plan: crate::runtime::UiNarrowedAllocationFramePlan,
    selection: crate::graph::UiAdmittedReplanNeighborhoodSet,
}

impl UiViewportResizeCommitBasis {
    pub(in crate::runtime) fn admit(
        plan: crate::runtime::UiNarrowedAllocationFramePlan,
        selection: crate::graph::UiAdmittedReplanNeighborhoodSet,
    ) -> Result<Self, super::UiViewportResizeDenial> {
        let selected = selection.ordered_neighborhoods().len();
        let maximum = plan.policy().budget().max_committed_receipts();
        if selected > usize::from(maximum) {
            return Err(super::UiViewportResizeDenial::ReceiptBudgetExceeded {
                selected: selected as u16,
                maximum,
            });
        }
        Ok(Self { plan, selection })
    }

    pub(crate) fn plan(&self) -> &crate::runtime::UiNarrowedAllocationFramePlan {
        &self.plan
    }

    pub(crate) fn selection(&self) -> &crate::graph::UiAdmittedReplanNeighborhoodSet {
        &self.selection
    }
}
