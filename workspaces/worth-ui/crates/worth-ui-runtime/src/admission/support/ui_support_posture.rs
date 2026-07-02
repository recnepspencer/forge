use crate::admission::{UiAdmissionFamily, UiAdmissionWorld};
use crate::declaration::UiDeclarationSupportMilestoneExpectation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiSupportReason {
    TouchMeaningNotApplicable,
    MissingDeclarationSupportEvidence,
    TargetOutsideAdmissionBoundary,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiSupportPosture {
    Supported {
        family: UiAdmissionFamily,
        world: UiAdmissionWorld,
    },
    DiagnosticOnly {
        family: UiAdmissionFamily,
        world: UiAdmissionWorld,
    },
    Unsupported {
        family: UiAdmissionFamily,
        reason: UiSupportReason,
        world: UiAdmissionWorld,
    },
    WrongWorld {
        family: UiAdmissionFamily,
        expected: UiAdmissionWorld,
        observed: UiAdmissionWorld,
    },
    Deferred {
        family: UiAdmissionFamily,
        expected_in: UiDeclarationSupportMilestoneExpectation,
        world: UiAdmissionWorld,
    },
}

impl UiSupportPosture {
    pub const fn family(&self) -> UiAdmissionFamily {
        match self {
            Self::Supported { family, .. }
            | Self::DiagnosticOnly { family, .. }
            | Self::Unsupported { family, .. }
            | Self::WrongWorld { family, .. }
            | Self::Deferred { family, .. } => *family,
        }
    }
}
