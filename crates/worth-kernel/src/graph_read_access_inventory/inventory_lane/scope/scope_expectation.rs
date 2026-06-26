#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessScopeExpectation {
    MilestoneSevenDeclarationCandidateInput,
    PreviewDeclarationCandidateInput,
    BranchDeclarationCandidateInput,
    QueryAccessRequirementCandidateInput,
    FutureExecutionReceiptExpectation,
    DeletionOnlyResidue,
    CertificationOnlyBoundary,
    NonGraphReadBoundary,
}
