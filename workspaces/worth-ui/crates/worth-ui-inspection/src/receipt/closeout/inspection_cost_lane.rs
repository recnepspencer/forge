#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionCostLane {
    IndexedLookup,
    NoBroadScan,
    BudgetOmissionTracked,
    MaterializationTracked,
    TraversalDenialsExplicit,
}
