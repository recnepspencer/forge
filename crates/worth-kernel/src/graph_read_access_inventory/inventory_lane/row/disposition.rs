#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessMilestoneSevenDisposition {
    DeclarationCandidate,
    CapabilityGap,
    DeletionOnly,
    CertificationOnly,
    OutOfScope,
}
