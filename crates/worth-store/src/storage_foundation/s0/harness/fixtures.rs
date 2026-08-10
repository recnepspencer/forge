use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum S1ForbiddenShortcut {
    OverclaimedPhysicalPosture,
    BackendTierMismatch,
    UnmappedDeferredGuarantee,
    MissingMilestonePhysicalStatusRow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum S1CompileTimeBoundaryFixture {
    PlatformGradeClaimConstructorPrivate,
    PlatformGradeEvidenceWitnessConstructorPrivate,
    PhysicalDebtCannotPromoteToPlatform,
    BackendDeclarationRequiresTier,
    NonPlatformBackendRequiresForbiddenClaims,
    PhysicalDebtRequiresSequenceMapping,
    S1HandoffRequiresAcceptedDigests,
}

impl S1CompileTimeBoundaryFixture {
    pub fn required_by_s0() -> Vec<Self> {
        vec![
            Self::PlatformGradeClaimConstructorPrivate,
            Self::PlatformGradeEvidenceWitnessConstructorPrivate,
            Self::PhysicalDebtCannotPromoteToPlatform,
            Self::BackendDeclarationRequiresTier,
            Self::NonPlatformBackendRequiresForbiddenClaims,
            Self::PhysicalDebtRequiresSequenceMapping,
            Self::S1HandoffRequiresAcceptedDigests,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum S1CompileTimeBoundaryStatus {
    Present,
    MissingS0Debt,
}
