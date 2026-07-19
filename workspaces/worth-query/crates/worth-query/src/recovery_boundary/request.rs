use super::explanation::WorthQueryRecoveryExplanation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryRecoveryRequestKind {
    CheckSupport,
    CorrectHandle,
    CorrectWorld,
    EscalateFailure,
    GatherAvailability,
    InspectCheckedLane,
    InspectProofLane,
    InspectSourceConflict,
    NarrowInput,
    RebindContext,
    RebuildSupportState,
    RefreshBasis,
    RepairDeclarationMeaning,
    RetryLater,
    ReviewContributionIntent,
    UseExplicitHandoff,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryRecoveryRequest {
    CheckSupport {
        explanation: WorthQueryRecoveryExplanation,
    },
    CorrectHandle {
        explanation: WorthQueryRecoveryExplanation,
    },
    CorrectWorld {
        explanation: WorthQueryRecoveryExplanation,
    },
    EscalateFailure {
        explanation: WorthQueryRecoveryExplanation,
    },
    GatherAvailability {
        explanation: WorthQueryRecoveryExplanation,
    },
    InspectCheckedLane {
        explanation: WorthQueryRecoveryExplanation,
    },
    InspectProofLane {
        explanation: WorthQueryRecoveryExplanation,
    },
    InspectSourceConflict {
        explanation: WorthQueryRecoveryExplanation,
    },
    NarrowInput {
        explanation: WorthQueryRecoveryExplanation,
    },
    RebindContext {
        explanation: WorthQueryRecoveryExplanation,
    },
    RebuildSupportState {
        explanation: WorthQueryRecoveryExplanation,
    },
    RefreshBasis {
        explanation: WorthQueryRecoveryExplanation,
    },
    RepairDeclarationMeaning {
        explanation: WorthQueryRecoveryExplanation,
    },
    RetryLater {
        explanation: WorthQueryRecoveryExplanation,
    },
    ReviewContributionIntent {
        explanation: WorthQueryRecoveryExplanation,
    },
    UseExplicitHandoff {
        explanation: WorthQueryRecoveryExplanation,
    },
}

impl WorthQueryRecoveryRequest {
    pub(crate) fn new(
        kind: WorthQueryRecoveryRequestKind,
        explanation: WorthQueryRecoveryExplanation,
    ) -> Self {
        match kind {
            WorthQueryRecoveryRequestKind::CheckSupport => Self::CheckSupport { explanation },
            WorthQueryRecoveryRequestKind::CorrectHandle => Self::CorrectHandle { explanation },
            WorthQueryRecoveryRequestKind::CorrectWorld => Self::CorrectWorld { explanation },
            WorthQueryRecoveryRequestKind::EscalateFailure => Self::EscalateFailure { explanation },
            WorthQueryRecoveryRequestKind::GatherAvailability => {
                Self::GatherAvailability { explanation }
            }
            WorthQueryRecoveryRequestKind::InspectCheckedLane => {
                Self::InspectCheckedLane { explanation }
            }
            WorthQueryRecoveryRequestKind::InspectProofLane => {
                Self::InspectProofLane { explanation }
            }
            WorthQueryRecoveryRequestKind::InspectSourceConflict => {
                Self::InspectSourceConflict { explanation }
            }
            WorthQueryRecoveryRequestKind::NarrowInput => Self::NarrowInput { explanation },
            WorthQueryRecoveryRequestKind::RebindContext => Self::RebindContext { explanation },
            WorthQueryRecoveryRequestKind::RebuildSupportState => {
                Self::RebuildSupportState { explanation }
            }
            WorthQueryRecoveryRequestKind::RefreshBasis => Self::RefreshBasis { explanation },
            WorthQueryRecoveryRequestKind::RepairDeclarationMeaning => {
                Self::RepairDeclarationMeaning { explanation }
            }
            WorthQueryRecoveryRequestKind::RetryLater => Self::RetryLater { explanation },
            WorthQueryRecoveryRequestKind::ReviewContributionIntent => {
                Self::ReviewContributionIntent { explanation }
            }
            WorthQueryRecoveryRequestKind::UseExplicitHandoff => {
                Self::UseExplicitHandoff { explanation }
            }
        }
    }

    pub fn kind(&self) -> WorthQueryRecoveryRequestKind {
        match self {
            Self::CheckSupport { .. } => WorthQueryRecoveryRequestKind::CheckSupport,
            Self::CorrectHandle { .. } => WorthQueryRecoveryRequestKind::CorrectHandle,
            Self::CorrectWorld { .. } => WorthQueryRecoveryRequestKind::CorrectWorld,
            Self::EscalateFailure { .. } => WorthQueryRecoveryRequestKind::EscalateFailure,
            Self::GatherAvailability { .. } => WorthQueryRecoveryRequestKind::GatherAvailability,
            Self::InspectCheckedLane { .. } => WorthQueryRecoveryRequestKind::InspectCheckedLane,
            Self::InspectProofLane { .. } => WorthQueryRecoveryRequestKind::InspectProofLane,
            Self::InspectSourceConflict { .. } => {
                WorthQueryRecoveryRequestKind::InspectSourceConflict
            }
            Self::NarrowInput { .. } => WorthQueryRecoveryRequestKind::NarrowInput,
            Self::RebindContext { .. } => WorthQueryRecoveryRequestKind::RebindContext,
            Self::RebuildSupportState { .. } => WorthQueryRecoveryRequestKind::RebuildSupportState,
            Self::RefreshBasis { .. } => WorthQueryRecoveryRequestKind::RefreshBasis,
            Self::RepairDeclarationMeaning { .. } => {
                WorthQueryRecoveryRequestKind::RepairDeclarationMeaning
            }
            Self::RetryLater { .. } => WorthQueryRecoveryRequestKind::RetryLater,
            Self::ReviewContributionIntent { .. } => {
                WorthQueryRecoveryRequestKind::ReviewContributionIntent
            }
            Self::UseExplicitHandoff { .. } => WorthQueryRecoveryRequestKind::UseExplicitHandoff,
        }
    }

    pub fn explanation(&self) -> &WorthQueryRecoveryExplanation {
        match self {
            Self::CheckSupport { explanation }
            | Self::CorrectHandle { explanation }
            | Self::CorrectWorld { explanation }
            | Self::EscalateFailure { explanation }
            | Self::GatherAvailability { explanation }
            | Self::InspectCheckedLane { explanation }
            | Self::InspectProofLane { explanation }
            | Self::InspectSourceConflict { explanation }
            | Self::NarrowInput { explanation }
            | Self::RebindContext { explanation }
            | Self::RebuildSupportState { explanation }
            | Self::RefreshBasis { explanation }
            | Self::RepairDeclarationMeaning { explanation }
            | Self::RetryLater { explanation }
            | Self::ReviewContributionIntent { explanation }
            | Self::UseExplicitHandoff { explanation } => explanation,
        }
    }
}
