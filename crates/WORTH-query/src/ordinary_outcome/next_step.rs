#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryOrdinaryNextStep {
    CheckSupport,
    CorrectHandle,
    CorrectWorld,
    EscalateFailure,
    GatherAvailability,
    InspectCheckedLane,
    InspectProofLane,
    NarrowInput,
    RebindContext,
    RefreshBasis,
    RetryLater,
    UseExplicitHandoff,
}
