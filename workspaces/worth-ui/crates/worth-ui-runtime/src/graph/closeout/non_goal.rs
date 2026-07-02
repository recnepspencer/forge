#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum UiGraphCloseoutNonGoal {
    QueryExecution,
    TouchedObligationSelection,
    HostObservation,
    MeasurementRuntimeTruth,
    InteractionRuntimeTruth,
    SideTopologiesOutsideGraphAuthority,
}
