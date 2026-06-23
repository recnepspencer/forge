#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessScopeExpectation {
    MilestoneSevenDeclarationCandidateInput,
    QueryAccessRequirementCandidateInput,
    FutureExecutionReceiptExpectation,
    DeletionOnlyResidue,
    CertificationOnlyBoundary,
    NonGraphReadBoundary,
}
