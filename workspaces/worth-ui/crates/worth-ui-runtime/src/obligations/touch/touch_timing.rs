#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiGraphTouchTiming {
    PreMutation,
    PostMutation,
    ReactiveObservation,
    DiagnosticProjection,
    ReplayEvaluation,
}
