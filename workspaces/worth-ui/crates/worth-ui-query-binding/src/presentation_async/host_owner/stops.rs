#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPresentationAdmissionStop {
    RuntimeAdmission,
    RuntimeObservation,
    SemanticExecution,
    QuerySupersession,
    UnexpectedPendingPosture,
    UnexpectedSupersessionPosture,
    SemanticRetirement,
    MissingPerformedFrontier,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiPresentationCleanupProgress {
    pub cause: WorthUiPresentationAdmissionStop,
    pub stopped_at: WorthUiPresentationRuntimeCleanupStop,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPresentationSettlementStop {
    SemanticExecution,
    QueryCompletion,
    QuerySupersession,
    QueryObservation,
    UnexpectedQueryPosture,
    SemanticRetirement,
    QueryClose,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPresentationRuntimeCleanupStop {
    Query,
    Semantic,
}
