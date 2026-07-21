#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiPlanRegionStoreDenial {
    HandleCapacity(crate::runtime::WorthUiHandleCapacityExhaustion),
    MissingLinkedRegion,
    DuplicateRegionIdentity,
    OrdinaryMeaningFamilyMismatch,
    SpatialMeaningFamilyMismatch,
    RealtimeMeaningFamilyMismatch,
    QueryBindingFactsMismatch,
    DuplicateChildTarget,
    OverlappingChildTarget,
    CyclicRegionDependency,
    OwnerManifestMismatch,
    IncompleteSuccessor,
}

impl From<crate::runtime::WorthUiHandleCapacityExhaustion> for WorthUiPlanRegionStoreDenial {
    fn from(value: crate::runtime::WorthUiHandleCapacityExhaustion) -> Self {
        Self::HandleCapacity(value)
    }
}
