use crate::capability::{UiIntentDefinitionSlot, UiSemanticInteractionFamily};

use super::UiIntentDeclarationIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiCanonicalIntentDeclaration {
    identity: UiIntentDeclarationIdentity,
    definition: UiIntentDefinitionSlot,
    interaction: UiSemanticInteractionFamily,
}

impl UiCanonicalIntentDeclaration {
    pub(crate) const fn new(
        identity: UiIntentDeclarationIdentity,
        definition: UiIntentDefinitionSlot,
        interaction: UiSemanticInteractionFamily,
    ) -> Self {
        Self {
            identity,
            definition,
            interaction,
        }
    }

    pub(crate) const fn identity(&self) -> &UiIntentDeclarationIdentity {
        &self.identity
    }

    pub(crate) const fn definition(&self) -> UiIntentDefinitionSlot {
        self.definition
    }

    pub(crate) const fn interaction(&self) -> UiSemanticInteractionFamily {
        self.interaction
    }
}
