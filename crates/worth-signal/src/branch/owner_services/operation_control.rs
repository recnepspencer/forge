/// Named owner progression seams available only to deterministic test control.
///
/// Later phases place parks and faults at these real boundaries. The feature
/// never creates another engine or changes an unarmed operation's semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SignalOwnerOperationBoundary {
    OwnerLifecycleAdmission,
    BranchRegistryLookup,
    BranchRegistryReservation,
    ExactBasisPreflight,
    TargetCellAdmission,
    BeforeCanonicalMovement,
    AfterCanonicalMovement,
    ForkSourceCapture,
    ForkDestinationInstallation,
    OutcomeConstruction,
    OwnerCloseBatch,
}
