#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiObligationCheckKind {
    BlockingInvariant,
    PrerequisiteRequirement,
    CapabilityGapScreen,
    WorldGate,
    AdvisoryCheck,
    DiagnosticOnlyCheck,
    DeferredBackstop,
}
