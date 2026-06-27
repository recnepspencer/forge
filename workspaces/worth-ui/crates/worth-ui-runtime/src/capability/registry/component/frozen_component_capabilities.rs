use crate::capability::ComponentId;

use super::{ComponentAcceptedRegistrationProof, ComponentDescriptor};

/// Canonical frozen component capability index.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenComponentCapabilities {
    descriptors: Vec<ComponentDescriptor>,
}

impl FrozenComponentCapabilities {
    pub(crate) fn empty() -> Self {
        Self {
            descriptors: Vec::new(),
        }
    }

    pub(crate) fn from_accepted_descriptors(
        mut descriptors: Vec<ComponentDescriptor>,
        accepted_components: &ComponentAcceptedRegistrationProof,
    ) -> Self {
        descriptors.retain(|descriptor| accepted_components.admits(descriptor));
        descriptors.sort_by(|left, right| left.id().cmp(right.id()));
        Self { descriptors }
    }

    pub fn is_empty(&self) -> bool {
        self.descriptors.is_empty()
    }

    pub fn len(&self) -> usize {
        self.descriptors.len()
    }

    pub fn descriptors(&self) -> &[ComponentDescriptor] {
        &self.descriptors
    }

    pub fn get(&self, id: &ComponentId) -> Option<&ComponentDescriptor> {
        self.descriptors
            .binary_search_by(|descriptor| descriptor.id().cmp(id))
            .ok()
            .map(|index| &self.descriptors[index])
    }

    pub(crate) fn digest_basis(&self) -> u64 {
        self.descriptors
            .iter()
            .fold(0x6c62_272e_07bb_0142, fold_component_descriptor)
    }
}

fn fold_component_descriptor(accumulator: u64, descriptor: &ComponentDescriptor) -> u64 {
    let with_id = fold_bytes(accumulator, descriptor.id().as_str().as_bytes());
    let with_prop_schema = fold_optional_str(
        with_id,
        descriptor
            .prop_schema()
            .map(|prop_schema| prop_schema.digest_basis()),
    );
    let with_child_policy = fold_bytes(
        with_prop_schema,
        descriptor.child_policy().as_str().as_bytes(),
    );
    let with_state = fold_optional_str(
        with_child_policy,
        descriptor
            .state_ownership()
            .map(|state_ownership| state_ownership.as_str().to_owned()),
    );
    let with_accessibility = fold_bytes(with_state, descriptor.accessibility().as_str().as_bytes());
    let with_focus = fold_bytes(with_accessibility, descriptor.focus().as_str().as_bytes());
    let with_tokens = descriptor.theme_token_dependencies().iter().fold(
        fold_bytes(with_focus, b"theme_token_dependencies"),
        |basis, token_id| fold_list_item(basis, token_id.as_str()),
    );
    let with_commands = descriptor.command_binding_slots().iter().fold(
        fold_bytes(with_tokens, b"command_binding_slots"),
        |basis, command_id| fold_list_item(basis, command_id.as_str()),
    );
    fold_bytes(
        with_commands,
        descriptor.execution_lane().as_str().as_bytes(),
    )
}

fn fold_list_item(accumulator: u64, value: &str) -> u64 {
    fold_bytes(fold_bytes(accumulator, b"item"), value.as_bytes())
}

fn fold_optional_str(accumulator: u64, value: Option<String>) -> u64 {
    match value {
        Some(value) => fold_bytes(fold_bytes(accumulator, b"some"), value.as_bytes()),
        None => fold_bytes(accumulator, b"none"),
    }
}

fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::capability::{
        CommandId, ComponentChildPolicy, ComponentDescriptor, ComponentId, ComponentPropSchema,
        ComponentStateOwnership,
    };

    use super::{ComponentAcceptedRegistrationProof, FrozenComponentCapabilities};

    #[test]
    fn dependency_list_boundaries_affect_digest_basis() {
        let combined = freeze_component(
            component_descriptor("workspace.component.editor")
                .with_command_binding_slot(command_id("workspace.command.ab"))
                .with_command_binding_slot(command_id("workspace.command.c")),
        );
        let split = freeze_component(
            component_descriptor("workspace.component.editor")
                .with_command_binding_slot(command_id("workspace.command.a"))
                .with_command_binding_slot(command_id("workspace.command.bc")),
        );

        assert_ne!(combined.descriptors(), split.descriptors());
        assert_ne!(combined.digest_basis(), split.digest_basis());
    }

    fn freeze_component(descriptor: ComponentDescriptor) -> FrozenComponentCapabilities {
        let mut accepted_identity_texts = BTreeSet::new();
        accepted_identity_texts.insert(descriptor.id().as_str().to_owned());
        FrozenComponentCapabilities::from_accepted_descriptors(
            vec![descriptor],
            &ComponentAcceptedRegistrationProof::from_identity_texts(accepted_identity_texts),
        )
    }

    fn component_descriptor(id: &str) -> ComponentDescriptor {
        ComponentDescriptor::new(
            component_id(id),
            ComponentPropSchema::named(format!("{id}.props")),
            ComponentChildPolicy::no_children(),
            ComponentStateOwnership::runtime_owned(),
        )
    }

    fn component_id(raw_text: &str) -> ComponentId {
        ComponentId::new(raw_text).expect("valid component id")
    }

    fn command_id(raw_text: &str) -> CommandId {
        CommandId::new(raw_text).expect("valid command id")
    }
}
