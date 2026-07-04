#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionAiHarnessLane {
    Inspect,
    ExpandEvidenceRef,
    CiteForeignEvidence,
    SupportReport,
    ClosureReport,
}
