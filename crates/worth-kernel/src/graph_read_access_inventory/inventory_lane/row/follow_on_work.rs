#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessFollowOnWork {
    MilestoneSevenDeclaration,
    MilestoneEightAccessPlanAdoption,
    DeletionOnlyCleanup,
    CertificationOnly,
    OutOfScope,
}
