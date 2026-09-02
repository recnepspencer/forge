use super::ComponentBasisDependencyClass;

/// Runtime World destinations used when one exact pin dependency changes
/// meaning. Release is explicit so a transfer cannot be mistaken for a fresh
/// acquisition or a component-owner lease operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ComponentBasisObligationTransferDestination {
    ProductBranchHead,
    RetainedCompositeHistory,
    AdmittedObservation,
    ActivePublicationAttempt,
    ProductUnpublishedOwnerEffects,
    HistoricalInspection,
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

impl ComponentBasisObligationTransferDestination {
    pub(crate) const fn dependency_class(self) -> Option<ComponentBasisDependencyClass> {
        match self {
            Self::ProductBranchHead => Some(ComponentBasisDependencyClass::ProductBranchHead),
            Self::RetainedCompositeHistory => {
                Some(ComponentBasisDependencyClass::RetainedCompositeHistory)
            }
            Self::AdmittedObservation => Some(ComponentBasisDependencyClass::AdmittedObservation),
            Self::ActivePublicationAttempt => {
                Some(ComponentBasisDependencyClass::ActivePublicationAttempt)
            }
            Self::ProductUnpublishedOwnerEffects => {
                Some(ComponentBasisDependencyClass::ProductUnpublishedOwnerEffects)
            }
            Self::HistoricalInspection => Some(ComponentBasisDependencyClass::HistoricalInspection),
            Self::Release => None,
        }
    }
}
