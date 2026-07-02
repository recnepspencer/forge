#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionSupportReason {
    BelongsArchitecturallyNotYetAdmitted,
    DiagnosticOnly,
    SubsystemSupportTruthConflict,
    TargetOutsideInspectionBoundary,
    WrongWorld,
}
