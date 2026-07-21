#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiRuntimeArtifactComparisonOutcome {
    EquivalentNoOp,
    MeaningfullyDifferent,
}
