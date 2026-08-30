use worth_ui::facade::inspection::{
    UiCommandRouteLossInspectionReason, UiCommandRouteScopeInspection,
    UiCommandWonInspectionSummary,
};

/// Pulse-private product copy derived from one bounded command-owner record.
/// It deliberately carries no command-routing authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlatformPulseCommandStory {
    winner: String,
    explanation: String,
}

impl PlatformPulseCommandStory {
    pub fn from_inspection(summary: &UiCommandWonInspectionSummary) -> Self {
        let winner = format!(
            "{} · {}",
            command_label(summary.command()),
            scope_label(summary.scope())
        );
        let explanation = summary.losers().first().map_or_else(
            || "No competing route in this context".to_owned(),
            |loser| {
                format!(
                    "Won over {} · {}",
                    command_label(loser.command()),
                    loss_label(loser.reason())
                )
            },
        );
        Self {
            winner: bounded(winner, 72),
            explanation: bounded(explanation, 96),
        }
    }

    pub fn winner(&self) -> &str {
        &self.winner
    }

    pub fn explanation(&self) -> &str {
        &self.explanation
    }
}

fn command_label(identity: &str) -> &str {
    identity
        .strip_prefix("platform.pulse.command.run.")
        .unwrap_or(identity)
}

const fn scope_label(scope: UiCommandRouteScopeInspection) -> &'static str {
    match scope {
        UiCommandRouteScopeInspection::Application => "application",
        UiCommandRouteScopeInspection::Surface => "surface",
        UiCommandRouteScopeInspection::ActiveRegion => "active region",
        UiCommandRouteScopeInspection::FocusedControl => "focused control",
        UiCommandRouteScopeInspection::ActivePortal => "active portal",
    }
}

const fn loss_label(reason: UiCommandRouteLossInspectionReason) -> &'static str {
    match reason {
        UiCommandRouteLossInspectionReason::LowerScopePrecedence => "lower context",
        UiCommandRouteLossInspectionReason::LowerDeclaredPriority => "lower priority",
        UiCommandRouteLossInspectionReason::LowerSpecificity => "less specific",
    }
}

fn bounded(mut value: String, maximum_chars: usize) -> String {
    if value.chars().count() > maximum_chars {
        value = value.chars().take(maximum_chars - 1).collect();
        value.push('…');
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;
    use worth_ui::facade::inspection::{
        UiCommandRouteLossInspection, UiRuntimeServiceInspectionCost,
        UiRuntimeServiceInspectionFamily, UiRuntimeServiceInspectionSource,
    };

    #[test]
    fn story_preserves_real_winner_and_first_bounded_loser() {
        let summary = UiCommandWonInspectionSummary::new(
            UiRuntimeServiceInspectionSource::new(
                UiRuntimeServiceInspectionFamily::CommandRouting,
                None,
                1,
            ),
            "platform.pulse.command.run.portal".to_owned(),
            UiCommandRouteScopeInspection::ActivePortal,
            vec![UiCommandRouteLossInspection::new(
                "platform.pulse.command.run.application".to_owned(),
                UiCommandRouteLossInspectionReason::LowerScopePrecedence,
            )]
            .into_boxed_slice(),
            UiRuntimeServiceInspectionCost::latest_record_with_projection(1, 1, 2),
        );
        let story = PlatformPulseCommandStory::from_inspection(&summary);
        assert_eq!(story.winner(), "portal · active portal");
        assert_eq!(story.explanation(), "Won over application · lower context");
    }
}
