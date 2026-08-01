/// Sealed source authority for entering Worth UI intent routing.
///
/// The only constructible 3.14 source consumes a semantic interaction already
/// issued by the mounted interaction subsystem. Later source families must add
/// owner-issued variants here rather than counterfeiting mounted evidence.
pub struct UiIntentRouteSource {
    source: UiIntentRouteSourceKind,
}

enum UiIntentRouteSourceKind {
    MountedInteraction(super::super::UiSemanticInteraction),
}

impl UiIntentRouteSource {
    pub fn mounted_interaction(interaction: super::super::UiSemanticInteraction) -> Self {
        Self {
            source: UiIntentRouteSourceKind::MountedInteraction(interaction),
        }
    }

    pub(crate) fn into_mounted_interaction(self) -> super::super::UiSemanticInteraction {
        match self.source {
            UiIntentRouteSourceKind::MountedInteraction(interaction) => interaction,
        }
    }

    pub(crate) fn evidence_input(&self) -> worth_ui_inspection::UiIntentInteractionEvidenceInput {
        match &self.source {
            UiIntentRouteSourceKind::MountedInteraction(interaction) => {
                super::super::semantic_evidence_input(interaction)
            }
        }
    }
}
