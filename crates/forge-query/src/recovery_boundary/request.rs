use super::explanation::ForgeQueryRecoveryExplanation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryRecoveryRequestKind {
    CheckSupport,
    CorrectHandle,
    CorrectWorld,
    EscalateFailure,
    GatherAvailability,
    InspectCheckedLane,
    InspectProofLane,
    NarrowInput,
    RebindContext,
    RefreshBasis,
    RepairDeclarationMeaning,
    RetryLater,
    ReviewContributionIntent,
    UseExplicitHandoff,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryRecoveryRequest {
    CheckSupport {
        explanation: ForgeQueryRecoveryExplanation,
    },
    CorrectHandle {
        explanation: ForgeQueryRecoveryExplanation,
    },
    CorrectWorld {
        explanation: ForgeQueryRecoveryExplanation,
    },
    EscalateFailure {
        explanation: ForgeQueryRecoveryExplanation,
    },
    GatherAvailability {
        explanation: ForgeQueryRecoveryExplanation,
    },
    InspectCheckedLane {
        explanation: ForgeQueryRecoveryExplanation,
    },
    InspectProofLane {
        explanation: ForgeQueryRecoveryExplanation,
    },
    NarrowInput {
        explanation: ForgeQueryRecoveryExplanation,
    },
    RebindContext {
        explanation: ForgeQueryRecoveryExplanation,
    },
    RefreshBasis {
        explanation: ForgeQueryRecoveryExplanation,
    },
    RepairDeclarationMeaning {
        explanation: ForgeQueryRecoveryExplanation,
    },
    RetryLater {
        explanation: ForgeQueryRecoveryExplanation,
    },
    ReviewContributionIntent {
        explanation: ForgeQueryRecoveryExplanation,
    },
    UseExplicitHandoff {
        explanation: ForgeQueryRecoveryExplanation,
    },
}

impl ForgeQueryRecoveryRequest {
    pub(crate) fn new(
        kind: ForgeQueryRecoveryRequestKind,
        explanation: ForgeQueryRecoveryExplanation,
    ) -> Self {
        match kind {
            ForgeQueryRecoveryRequestKind::CheckSupport => Self::CheckSupport { explanation },
            ForgeQueryRecoveryRequestKind::CorrectHandle => Self::CorrectHandle { explanation },
            ForgeQueryRecoveryRequestKind::CorrectWorld => Self::CorrectWorld { explanation },
            ForgeQueryRecoveryRequestKind::EscalateFailure => Self::EscalateFailure { explanation },
            ForgeQueryRecoveryRequestKind::GatherAvailability => {
                Self::GatherAvailability { explanation }
            }
            ForgeQueryRecoveryRequestKind::InspectCheckedLane => {
                Self::InspectCheckedLane { explanation }
            }
            ForgeQueryRecoveryRequestKind::InspectProofLane => {
                Self::InspectProofLane { explanation }
            }
            ForgeQueryRecoveryRequestKind::NarrowInput => Self::NarrowInput { explanation },
            ForgeQueryRecoveryRequestKind::RebindContext => Self::RebindContext { explanation },
            ForgeQueryRecoveryRequestKind::RefreshBasis => Self::RefreshBasis { explanation },
            ForgeQueryRecoveryRequestKind::RepairDeclarationMeaning => {
                Self::RepairDeclarationMeaning { explanation }
            }
            ForgeQueryRecoveryRequestKind::RetryLater => Self::RetryLater { explanation },
            ForgeQueryRecoveryRequestKind::ReviewContributionIntent => {
                Self::ReviewContributionIntent { explanation }
            }
            ForgeQueryRecoveryRequestKind::UseExplicitHandoff => {
                Self::UseExplicitHandoff { explanation }
            }
        }
    }

    pub fn kind(&self) -> ForgeQueryRecoveryRequestKind {
        match self {
            Self::CheckSupport { .. } => ForgeQueryRecoveryRequestKind::CheckSupport,
            Self::CorrectHandle { .. } => ForgeQueryRecoveryRequestKind::CorrectHandle,
            Self::CorrectWorld { .. } => ForgeQueryRecoveryRequestKind::CorrectWorld,
            Self::EscalateFailure { .. } => ForgeQueryRecoveryRequestKind::EscalateFailure,
            Self::GatherAvailability { .. } => ForgeQueryRecoveryRequestKind::GatherAvailability,
            Self::InspectCheckedLane { .. } => ForgeQueryRecoveryRequestKind::InspectCheckedLane,
            Self::InspectProofLane { .. } => ForgeQueryRecoveryRequestKind::InspectProofLane,
            Self::NarrowInput { .. } => ForgeQueryRecoveryRequestKind::NarrowInput,
            Self::RebindContext { .. } => ForgeQueryRecoveryRequestKind::RebindContext,
            Self::RefreshBasis { .. } => ForgeQueryRecoveryRequestKind::RefreshBasis,
            Self::RepairDeclarationMeaning { .. } => {
                ForgeQueryRecoveryRequestKind::RepairDeclarationMeaning
            }
            Self::RetryLater { .. } => ForgeQueryRecoveryRequestKind::RetryLater,
            Self::ReviewContributionIntent { .. } => {
                ForgeQueryRecoveryRequestKind::ReviewContributionIntent
            }
            Self::UseExplicitHandoff { .. } => ForgeQueryRecoveryRequestKind::UseExplicitHandoff,
        }
    }

    pub fn explanation(&self) -> &ForgeQueryRecoveryExplanation {
        match self {
            Self::CheckSupport { explanation }
            | Self::CorrectHandle { explanation }
            | Self::CorrectWorld { explanation }
            | Self::EscalateFailure { explanation }
            | Self::GatherAvailability { explanation }
            | Self::InspectCheckedLane { explanation }
            | Self::InspectProofLane { explanation }
            | Self::NarrowInput { explanation }
            | Self::RebindContext { explanation }
            | Self::RefreshBasis { explanation }
            | Self::RepairDeclarationMeaning { explanation }
            | Self::RetryLater { explanation }
            | Self::ReviewContributionIntent { explanation }
            | Self::UseExplicitHandoff { explanation } => explanation,
        }
    }
}
