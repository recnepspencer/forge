use worth_ui::facade::intent::{UiIntentProductOutcome, UiIntentSchema};

/// The product-authored transitions in Pulse's one real Portal story.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlatformPulsePortalStoryTransition {
    OpenedByAdmittedIntent,
    ClosedByAdmittedIntent,
}

/// Product-owned attribution for the one real Portal story shipped by Pulse.
///
/// This does not infer runtime state. It recognizes only the typed open and
/// close outcomes declared by the Pulse product; Portal and Focus truth remain
/// owned by their runtime services and mounted publication receipts.
pub fn platform_pulse_portal_story_transition(
    outcome: UiIntentSchema,
) -> Option<PlatformPulsePortalStoryTransition> {
    if outcome == crate::intent::PlatformPulseOpenPortalOutcome::SCHEMA {
        Some(PlatformPulsePortalStoryTransition::OpenedByAdmittedIntent)
    } else if outcome == crate::intent::PlatformPulseClosePortalOutcome::SCHEMA {
        Some(PlatformPulsePortalStoryTransition::ClosedByAdmittedIntent)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn portal_story_classifies_only_its_typed_product_outcomes() {
        assert_eq!(
            platform_pulse_portal_story_transition(
                crate::intent::PlatformPulseOpenPortalOutcome::SCHEMA
            ),
            Some(PlatformPulsePortalStoryTransition::OpenedByAdmittedIntent)
        );
        assert_eq!(
            platform_pulse_portal_story_transition(
                crate::intent::PlatformPulseClosePortalOutcome::SCHEMA
            ),
            Some(PlatformPulsePortalStoryTransition::ClosedByAdmittedIntent)
        );
        assert_eq!(
            platform_pulse_portal_story_transition(
                crate::intent::PlatformPulseActionOutcome::SCHEMA
            ),
            None
        );
    }
}
