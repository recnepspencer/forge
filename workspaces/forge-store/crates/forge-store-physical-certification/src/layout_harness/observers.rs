#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutObserverLane {
    DeclarationInventoryObserver,
    CounterReceiptObserver,
    RecoveryOutcomeObserver,
    ReadmissionObserver,
    OfflineVerifierObserver,
    MultiArtifactTraceObserver,
}
