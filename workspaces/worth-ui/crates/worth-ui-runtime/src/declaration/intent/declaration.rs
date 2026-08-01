use crate::capability::{UiIntentDefinitionSlot, UiSemanticInteractionFamily};

use super::UiIntentDeclarationIdentity;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct UiCanonicalIntentDeclaration {
    identity: UiIntentDeclarationIdentity,
    definition: UiIntentDefinitionSlot,
    interaction: UiSemanticInteractionFamily,
    payload: Box<[super::UiResolvedIntentPayloadBinding]>,
    operability: super::UiResolvedIntentOperabilityContract,
    confirmation: super::UiResolvedIntentConfirmationContract,
    concurrency: super::UiIntentConcurrencyScope,
    consequences: super::UiResolvedIntentConsequenceContract,
}

impl UiCanonicalIntentDeclaration {
    pub(crate) const fn new(
        identity: UiIntentDeclarationIdentity,
        definition: UiIntentDefinitionSlot,
        interaction: UiSemanticInteractionFamily,
        payload: Box<[super::UiResolvedIntentPayloadBinding]>,
        operability: super::UiResolvedIntentOperabilityContract,
        confirmation: super::UiResolvedIntentConfirmationContract,
        concurrency: super::UiIntentConcurrencyScope,
        consequences: super::UiResolvedIntentConsequenceContract,
    ) -> Self {
        Self {
            identity,
            definition,
            interaction,
            payload,
            operability,
            confirmation,
            concurrency,
            consequences,
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

    pub(crate) const fn operability(&self) -> &super::UiResolvedIntentOperabilityContract {
        &self.operability
    }

    pub(crate) const fn confirmation(&self) -> &super::UiResolvedIntentConfirmationContract {
        &self.confirmation
    }

    pub(crate) const fn concurrency(&self) -> super::UiIntentConcurrencyScope {
        self.concurrency
    }

    pub(crate) const fn consequences(&self) -> &super::UiResolvedIntentConsequenceContract {
        &self.consequences
    }
}
