use super::ComponentBasisDependencyClass;

/// Runtime World destinations used when one exact pin dependency changes
/// meaning. Release is explicit so a transfer cannot be mistaken for a fresh
/// acquisition or a component-owner lease operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ComponentBasisObligationTransferDestination {
    ProductBranchHead,
    RetainedCompositeHistory,
    ProductUnpublishedOwnerEffects,
    Release,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ComponentBasisObligationTransfer {
    from: ComponentBasisDependencyClass,
    to: ComponentBasisObligationTransferDestination,
}

impl ComponentBasisObligationTransfer {
    pub(crate) const fn new(
        from: ComponentBasisDependencyClass,
        to: ComponentBasisObligationTransferDestination,
    ) -> Self {
        Self { from, to }
    }

    pub(crate) const fn from(self) -> ComponentBasisDependencyClass {
        self.from
    }

    pub(crate) const fn to(self) -> ComponentBasisObligationTransferDestination {
        self.to
    }
}
