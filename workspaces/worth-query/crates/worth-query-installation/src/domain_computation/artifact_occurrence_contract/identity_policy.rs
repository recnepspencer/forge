#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryArtifactOccurrenceIdentityPolicy {
    IndependentPerExecution,
    DomainMintedIndependent,
}
