use super::UiDeclarationSupportMilestoneExpectation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiDeclarationUnsupportedPosture {
    ArchitecturallyOwnedButNotYetAdmitted {
        expected_in: UiDeclarationSupportMilestoneExpectation,
    },
}

impl UiDeclarationUnsupportedPosture {
    pub const fn expected_in(self) -> UiDeclarationSupportMilestoneExpectation {
        match self {
            Self::ArchitecturallyOwnedButNotYetAdmitted { expected_in } => expected_in,
        }
    }
}
