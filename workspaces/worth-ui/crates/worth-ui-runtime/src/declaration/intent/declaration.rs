use crate::capability::{UiIntentDefinitionSlot, UiSemanticInteractionFamily};

use super::UiIntentDeclarationIdentity;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct UiCanonicalIntentDeclaration {
    identity: UiIntentDeclarationIdentity,
    definition: UiIntentDefinitionSlot,
    interaction: UiSemanticInteractionFamily,
    payload: Box<[super::UiResolvedIntentPayloadBinding]>,
}

impl UiCanonicalIntentDeclaration {
    pub(crate) const fn new(
        identity: UiIntentDeclarationIdentity,
        definition: UiIntentDefinitionSlot,
        interaction: UiSemanticInteractionFamily,
        payload: Box<[super::UiResolvedIntentPayloadBinding]>,
    ) -> Self {
        Self {
            identity,
            definition,
            interaction,
            payload,
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

    pub(crate) fn payload(&self) -> &[super::UiResolvedIntentPayloadBinding] {
        &self.payload
    }
}
