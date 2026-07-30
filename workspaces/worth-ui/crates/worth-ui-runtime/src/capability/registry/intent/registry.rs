use crate::capability::{
    CapabilitySupportKind, RegistrationCandidate, INTENT_DEFINITION_FAMILY_NAME,
};

use super::semantic_digest::UiIntentSemanticDigest;
use super::{IntentDefinitionAcceptedRegistrationProof, IntentDefinitionDescriptor};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiIntentDefinitionRegistrationError {
    DuplicateIdentity { identity: super::UiIntentId },
}

impl core::fmt::Display for UiIntentDefinitionRegistrationError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DuplicateIdentity { identity } => {
                write!(
                    formatter,
                    "intent definition `{identity}` is already registered"
                )
            }
        }
    }
}

impl std::error::Error for UiIntentDefinitionRegistrationError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct IntentDefinitionRegistry {
    definitions: Vec<IntentDefinitionDescriptor>,
}

impl IntentDefinitionRegistry {
    pub(crate) fn empty() -> Self {
        Self {
            definitions: Vec::new(),
        }
    }

    pub(crate) fn push(
        &mut self,
        descriptor: IntentDefinitionDescriptor,
    ) -> Result<RegistrationCandidate, UiIntentDefinitionRegistrationError> {
        if self
            .definitions
            .iter()
            .any(|existing| existing.id() == descriptor.id())
        {
            return Err(UiIntentDefinitionRegistrationError::DuplicateIdentity {
                identity: descriptor.id(),
            });
        }
        let candidate = RegistrationCandidate::new(
            INTENT_DEFINITION_FAMILY_NAME,
            descriptor.id().as_str(),
            CapabilitySupportKind::Admitted,
        );
        self.definitions.push(descriptor);
        Ok(candidate)
    }

    pub(crate) fn freeze(
        self,
        accepted: &IntentDefinitionAcceptedRegistrationProof,
    ) -> FrozenIntentDefinitionCapabilities {
        FrozenIntentDefinitionCapabilities::from_accepted(self.definitions, accepted)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenIntentDefinitionCapabilities {
    definitions: Vec<IntentDefinitionDescriptor>,
}

impl FrozenIntentDefinitionCapabilities {
    #[cfg(test)]
    pub(crate) fn empty() -> Self {
        Self {
            definitions: Vec::new(),
        }
    }

    pub(crate) fn from_accepted(
        mut definitions: Vec<IntentDefinitionDescriptor>,
        accepted: &IntentDefinitionAcceptedRegistrationProof,
    ) -> Self {
        definitions.retain(|definition| accepted.admits(definition));
        definitions.sort_by_key(IntentDefinitionDescriptor::id);
        Self { definitions }
    }

    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }

    pub fn len(&self) -> usize {
        self.definitions.len()
    }

    pub fn definitions(&self) -> &[IntentDefinitionDescriptor] {
        &self.definitions
    }

    pub fn get(&self, id: &super::UiIntentId) -> Option<&IntentDefinitionDescriptor> {
        self.definitions
            .binary_search_by_key(id, IntentDefinitionDescriptor::id)
            .ok()
            .map(|index| &self.definitions[index])
    }

    pub(crate) fn digest_basis(&self) -> u64 {
        let mut digest = UiIntentSemanticDigest::new(0x6614_6d3a_8cb9_104f)
            .usize("definition-count", self.definitions.len());
        for definition in &self.definitions {
            let payload = definition.payload_schema();
            let outcome = definition.product_outcome_schema();
            let interactions = definition.accepted_interactions();
            digest = digest
                .field("definition", &[])
                .field("intent-id", definition.id().as_str().as_bytes())
                .field("payload-schema-id", payload.stable_identity().as_bytes())
                .u16("payload-schema-version", payload.version())
                .field("outcome-schema-id", outcome.stable_identity().as_bytes())
                .u16("outcome-schema-version", outcome.version())
                .field(
                    "execution-destination",
                    definition.execution_destination().digest_basis().as_bytes(),
                )
                .usize("interaction-count", interactions.len());
            for interaction in interactions {
                digest = digest.field("interaction", interaction.digest_basis().as_bytes());
            }
        }
        digest.finish()
    }
}
