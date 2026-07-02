#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionAdmissionPosture {
    Denied,
    Unsupported,
    WrongWorld,
    Deferred,
    DiagnosticOnly,
    WrongQueryBasis,
    WrongHostCapability,
    Stale,
    Ambiguous,
    RebindRequired,
    BudgetExceeded,
    Admitted,
    AdmittedWithAdvisory,
}
