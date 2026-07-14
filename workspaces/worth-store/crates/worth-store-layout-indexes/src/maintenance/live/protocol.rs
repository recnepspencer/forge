#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexPublicationProtocol {
    LsmManifestReplacement,
    CopyOnWriteRootSwap,
    DeferredUntilRebuild,
    MaterializeOnDemand,
    AdvisoryObservation,
    VerificationObservation,
    MigrationCutover,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexMaintenanceFailureOutcome {
    AwaitingExactPublication,
    FamilyBindingMismatch,
    SecurityScopeMismatch,
    PhysicalPublicationAuthorityMismatch,
    MutationSourceMaterializationMismatch,
    MutationShapeMismatch,
    InPlaceReachableOverwriteUnsupported,
}
