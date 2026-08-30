use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlatformPulseCommandScopeInspection {
    Application,
    Surface,
    ActiveRegion,
    FocusedControl,
    ActivePortal,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlatformPulseCommandLossReasonInspection {
    LowerScopePrecedence,
    LowerDeclaredPriority,
    LowerSpecificity,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseCommandLosingCandidateInspection {
    command: String,
    reason: PlatformPulseCommandLossReasonInspection,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseCommandTransitionInspection {
    winner: String,
    scope: PlatformPulseCommandScopeInspection,
    losing_candidate: Option<PlatformPulseCommandLosingCandidateInspection>,
    revision: u64,
}

impl PlatformPulseCommandTransitionInspection {
    pub fn from_inspection(
        summary: worth_ui::facade::inspection::UiCommandWonInspectionSummary,
    ) -> Self {
        Self {
            winner: summary.command().to_owned(),
            scope: map_scope(summary.scope()),
            losing_candidate: summary.losers().first().map(|loser| {
                PlatformPulseCommandLosingCandidateInspection {
                    command: loser.command().to_owned(),
                    reason: map_loss_reason(loser.reason()),
                }
            }),
            revision: summary.source().revision(),
        }
    }

    pub fn winner(&self) -> &str {
        &self.winner
    }

    pub const fn scope(&self) -> PlatformPulseCommandScopeInspection {
        self.scope
    }

    pub fn losing_candidate(&self) -> Option<&PlatformPulseCommandLosingCandidateInspection> {
        self.losing_candidate.as_ref()
    }

    pub const fn revision(&self) -> u64 {
        self.revision
    }
}

impl PlatformPulseCommandLosingCandidateInspection {
    pub fn command(&self) -> &str {
        &self.command
    }

    pub const fn reason(&self) -> PlatformPulseCommandLossReasonInspection {
        self.reason
    }
}

const fn map_scope(
    scope: worth_ui::facade::inspection::UiCommandRouteScopeInspection,
) -> PlatformPulseCommandScopeInspection {
    use worth_ui::facade::inspection::UiCommandRouteScopeInspection as Scope;
    match scope {
        Scope::Application => PlatformPulseCommandScopeInspection::Application,
        Scope::Surface => PlatformPulseCommandScopeInspection::Surface,
        Scope::ActiveRegion => PlatformPulseCommandScopeInspection::ActiveRegion,
        Scope::FocusedControl => PlatformPulseCommandScopeInspection::FocusedControl,
        Scope::ActivePortal => PlatformPulseCommandScopeInspection::ActivePortal,
    }
}

const fn map_loss_reason(
    reason: worth_ui::facade::inspection::UiCommandRouteLossInspectionReason,
) -> PlatformPulseCommandLossReasonInspection {
    use worth_ui::facade::inspection::UiCommandRouteLossInspectionReason as Reason;
    match reason {
        Reason::LowerScopePrecedence => {
            PlatformPulseCommandLossReasonInspection::LowerScopePrecedence
        }
        Reason::LowerDeclaredPriority => {
            PlatformPulseCommandLossReasonInspection::LowerDeclaredPriority
        }
        Reason::LowerSpecificity => PlatformPulseCommandLossReasonInspection::LowerSpecificity,
    }
}
