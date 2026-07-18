pub(super) enum UiAllocationReplanCommitMode<'a> {
    Ordinary(&'a crate::graph::UiAdmittedReplanNeighborhoodSet),
    Viewport(Box<crate::runtime::UiViewportResizeCommitBasis>),
    DurableResize {
        selection: &'a crate::graph::UiAdmittedReplanNeighborhoodSet,
        basis: crate::runtime::UiResizeAllocationPlanningBasis,
    },
}

impl UiAllocationReplanCommitMode<'_> {
    pub(super) fn selection(&self) -> &crate::graph::UiAdmittedReplanNeighborhoodSet {
        match self {
            Self::Ordinary(selection) => selection,
            Self::Viewport(basis) => basis.selection(),
            Self::DurableResize { selection, .. } => selection,
        }
    }

    pub(super) fn durable_resize(
        &self,
    ) -> Option<&crate::runtime::UiResizeAllocationPlanningBasis> {
        match self {
            Self::DurableResize { basis, .. } => Some(basis),
            _ => None,
        }
    }
}
