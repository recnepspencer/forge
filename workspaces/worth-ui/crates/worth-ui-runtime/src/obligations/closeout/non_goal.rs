#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiObligationCloseoutNonGoal {
    MeasurementExecution,
    QueryExecution,
    IntentExecution,
    ServiceExecution,
    RebindExecution,
    RendererLocalLegality,
}
