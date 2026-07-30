use crate::capability::{
    CapabilitySupportKind, RegistrationCandidate, INTENT_DEFINITION_FAMILY_NAME,
};

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
        self.definitions
            .iter()
            .fold(0x6614_6d3a_8cb9_104f, |basis, definition| {
                let with_id = fold_bytes(basis, definition.id().as_str().as_bytes());
                let with_payload = fold_bytes(
                    with_id,
                    definition.payload_schema().digest_basis().as_bytes(),
                );
                let with_outcome = fold_bytes(
                    with_payload,
                    definition
                        .product_outcome_schema()
                        .digest_basis()
                        .as_bytes(),
                );
                let with_destination = fold_bytes(
                    with_outcome,
                    definition.execution_destination().digest_basis().as_bytes(),
                );
                definition
                    .accepted_interactions()
                    .iter()
                    .fold(with_destination, |basis, interaction| {
                        fold_bytes(basis, format!("{interaction:?}").as_bytes())
                    })
            })
    }
}

fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}
