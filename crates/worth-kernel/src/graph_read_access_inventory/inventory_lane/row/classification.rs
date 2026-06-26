#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessClassification {
    QueryDeclarationCandidate,
    DeletionTarget,
    CappedResidue,
    CertificationOnlySupport,
    QueryAccessCapabilityGap,
    OutOfScopeNonGraphRead,
}
