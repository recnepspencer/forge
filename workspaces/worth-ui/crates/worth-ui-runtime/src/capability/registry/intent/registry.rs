use crate::capability::{
    CapabilitySupportKind, RegistrationCandidate, INTENT_DEFINITION_FAMILY_NAME,
};

use super::semantic_digest::UiIntentSemanticDigest;
use super::{
    IntentDefinitionAcceptedRegistrationProof, IntentDefinitionDescriptor,
    UiIntentPayloadSchemaViolation, UiRegisteredIntentDefinition,
    UiRegisteredIntentPayloadProjector,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiIntentDefinitionRegistrationError {
    DuplicateIdentity {
        identity: super::UiIntentId,
    },
    InvalidPayloadSchema {
        identity: super::UiIntentId,
        violation: UiIntentPayloadSchemaViolation,
    },
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
            Self::InvalidPayloadSchema {
                identity,
                violation,
            } => write!(
                formatter,
                "intent definition `{identity}` has an invalid payload schema: {violation:?}"
            ),
        }
    }
}

impl std::error::Error for UiIntentDefinitionRegistrationError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct IntentDefinitionRegistry {
    definitions: Vec<UiRegisteredIntentDefinition>,
}

impl IntentDefinitionRegistry {
    pub(crate) fn empty() -> Self {
        Self {
            definitions: Vec::new(),
        }
    }

    pub(crate) fn push(
        &mut self,
        definition: UiRegisteredIntentDefinition,
    ) -> Result<RegistrationCandidate, UiIntentDefinitionRegistrationError> {
        let descriptor = definition.descriptor();
        descriptor
            .payload_fields()
            .validate()
            .map_err(
                |violation| UiIntentDefinitionRegistrationError::InvalidPayloadSchema {
                    identity: descriptor.id(),
                    violation,
                },
            )?;
        if self
            .definitions
            .iter()
            .any(|existing| existing.descriptor().id() == descriptor.id())
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
        self.definitions.push(definition);
        Ok(candidate)
    }

    pub(crate) fn freeze(
        self,
        accepted: &IntentDefinitionAcceptedRegistrationProof,
    ) -> FrozenIntentDefinitionCapabilities {
        FrozenIntentDefinitionCapabilities::from_accepted(self.definitions, accepted)
    }
}

pub struct FrozenIntentDefinitionCapabilities {
    definitions: Vec<IntentDefinitionDescriptor>,
    projectors: Vec<std::sync::Arc<dyn UiRegisteredIntentPayloadProjector>>,
}

impl Clone for FrozenIntentDefinitionCapabilities {
    fn clone(&self) -> Self {
        Self {
            definitions: self.definitions.clone(),
            projectors: self.projectors.clone(),
        }
    }
}

impl core::fmt::Debug for FrozenIntentDefinitionCapabilities {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("FrozenIntentDefinitionCapabilities")
            .field("definitions", &self.definitions)
            .finish()
    }
}

impl PartialEq for FrozenIntentDefinitionCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.definitions == other.definitions
    }
}

impl Eq for FrozenIntentDefinitionCapabilities {}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct UiIntentDefinitionSlot(usize);

pub(crate) struct UiResolvedIntentDefinition<'registry> {
    slot: UiIntentDefinitionSlot,
    descriptor: &'registry IntentDefinitionDescriptor,
}

impl FrozenIntentDefinitionCapabilities {
    #[cfg(test)]
    pub(crate) fn empty() -> Self {
        Self {
            definitions: Vec::new(),
            projectors: Vec::new(),
        }
    }

    pub(crate) fn from_accepted(
        definitions: Vec<UiRegisteredIntentDefinition>,
        accepted: &IntentDefinitionAcceptedRegistrationProof,
    ) -> Self {
        let mut definitions = definitions
            .into_iter()
            .filter(|definition| accepted.admits(definition.descriptor()))
            .collect::<Vec<_>>();
        definitions.sort_by_key(|definition| definition.descriptor().id());
        let (descriptors, projectors): (Vec<_>, Vec<_>) = definitions
            .into_iter()
            .map(UiRegisteredIntentDefinition::into_parts)
            .unzip();
        Self {
            definitions: descriptors,
            projectors,
        }
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

    pub(crate) fn resolve_stable_text(
        &self,
        stable_text: &str,
    ) -> Option<UiResolvedIntentDefinition<'_>> {
        self.definitions
            .binary_search_by(|definition| definition.id().as_str().cmp(stable_text))
            .ok()
            .map(|index| UiResolvedIntentDefinition {
                slot: UiIntentDefinitionSlot(index),
                descriptor: &self.definitions[index],
            })
    }

    pub(crate) fn definition_at(
        &self,
        slot: UiIntentDefinitionSlot,
    ) -> &IntentDefinitionDescriptor {
        &self.definitions[slot.0]
    }

    pub(crate) fn projector_at(
        &self,
        slot: UiIntentDefinitionSlot,
    ) -> &dyn UiRegisteredIntentPayloadProjector {
        self.projectors[slot.0].as_ref()
    }

    pub(crate) fn digest_basis(&self) -> u64 {
        let mut digest = UiIntentSemanticDigest::new(0x6614_6d3a_8cb9_104f)
            .usize("definition-count", self.definitions.len());
        for definition in &self.definitions {
            let payload = definition.payload_schema();
            let payload_fields = definition.payload_fields();
            let outcome = definition.product_outcome_schema();
            let interactions = definition.accepted_interactions();
            digest = digest
                .field("definition", &[])
                .field("intent-id", definition.id().as_str().as_bytes())
                .field("payload-schema-id", payload.stable_identity().as_bytes())
                .u16("payload-schema-version", payload.version())
                .usize("payload-field-count", payload_fields.len())
                .field("outcome-schema-id", outcome.stable_identity().as_bytes())
                .u16("outcome-schema-version", outcome.version())
                .field(
                    "execution-destination",
                    definition.execution_destination().digest_basis().as_bytes(),
                )
                .usize("interaction-count", interactions.len());
            for field in payload_fields.fields() {
                digest = digest
                    .usize("payload-field-slot", usize::from(field.slot()))
                    .field("payload-field-name", field.stable_name().as_bytes())
                    .field("payload-field-kind", field.kind().digest_basis().as_bytes())
                    .usize("payload-field-byte-budget", field.byte_budget());
            }
            for interaction in interactions {
                digest = digest.field("interaction", interaction.digest_basis().as_bytes());
            }
        }
        digest.finish()
    }
}

impl UiResolvedIntentDefinition<'_> {
    pub(crate) const fn slot(&self) -> UiIntentDefinitionSlot {
        self.slot
    }

    pub(crate) const fn descriptor(&self) -> &IntentDefinitionDescriptor {
        self.descriptor
    }
}
