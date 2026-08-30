/// Sealed source authority for entering Worth UI intent routing.
///
/// The only constructible 3.14 source consumes a semantic interaction already
/// issued by the mounted interaction subsystem. Later source families must add
/// owner-issued variants here rather than counterfeiting mounted evidence.
pub struct UiIntentRouteSource {
    source: UiIntentRouteSourceKind,
}

pub(crate) enum UiIntentRouteSourceKind {
    MountedInteraction(super::super::UiSemanticInteraction),
    CommandRoute(crate::runtime::command_routing::UiCommandRouteReceipt),
}

impl UiIntentRouteSource {
    pub fn mounted_interaction(interaction: super::super::UiSemanticInteraction) -> Self {
        Self {
            source: UiIntentRouteSourceKind::MountedInteraction(interaction),
        }
    }

    pub fn command_route(receipt: crate::runtime::UiCommandRouteReceipt) -> Self {
        Self {
            source: UiIntentRouteSourceKind::CommandRoute(receipt),
        }
    }

    pub(crate) fn into_kind(self) -> UiIntentRouteSourceKind {
        self.source
    }

    pub(crate) fn evidence_input(
        &self,
    ) -> Option<worth_ui_inspection::UiIntentInteractionEvidenceInput> {
        match &self.source {
            UiIntentRouteSourceKind::MountedInteraction(interaction) => {
                Some(super::super::semantic_evidence_input(interaction))
            }
            UiIntentRouteSourceKind::CommandRoute(_) => None,
        }
    }
}

pub(crate) use UiIntentRouteSourceKind as UiIntentRouteSourceMaterial;
