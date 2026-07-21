#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiExecutablePlanDecisionKind {
    ExactSemanticNoOp,
    BoundedChangedRegions,
    RebuildRequired,
    Denied,
}
