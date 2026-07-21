#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiExecutablePlanEquivalenceDenial {
    ForeignHostSession,
    MissingPredecessorProof,
    PredecessorArtifactMismatch,
    PredecessorPlanMismatch,
}
