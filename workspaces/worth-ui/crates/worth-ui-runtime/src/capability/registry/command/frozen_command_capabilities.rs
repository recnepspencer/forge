use crate::capability::CommandId;

use super::{CommandAcceptedRegistrationProof, CommandDescriptor};

/// Canonical frozen command capability index.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenCommandCapabilities {
    descriptors: Vec<CommandDescriptor>,
}

impl FrozenCommandCapabilities {
    #[cfg(test)]
    pub(crate) fn empty() -> Self {
        Self {
            descriptors: Vec::new(),
        }
    }

    pub(crate) fn from_accepted_descriptors(
        mut descriptors: Vec<CommandDescriptor>,
        accepted_commands: &CommandAcceptedRegistrationProof,
    ) -> Self {
        descriptors.retain(|descriptor| accepted_commands.admits(descriptor));
        descriptors.sort_by(|left, right| left.id().cmp(right.id()));
        Self { descriptors }
    }

    pub fn is_empty(&self) -> bool {
        self.descriptors.is_empty()
    }

    pub fn len(&self) -> usize {
        self.descriptors.len()
    }

    pub fn descriptors(&self) -> &[CommandDescriptor] {
        &self.descriptors
    }

    pub fn get(&self, id: &CommandId) -> Option<&CommandDescriptor> {
        self.descriptors
            .binary_search_by(|descriptor| descriptor.id().cmp(id))
            .ok()
            .map(|index| &self.descriptors[index])
    }

    pub(crate) fn digest_basis(&self) -> u64 {
        self.descriptors
            .iter()
            .fold(0xcbf2_9ce4_8422_2325, fold_command_descriptor)
    }
}

fn fold_command_descriptor(accumulator: u64, descriptor: &CommandDescriptor) -> u64 {
    let with_id = fold_bytes(accumulator, descriptor.id().as_str().as_bytes());
    let with_label = fold_bytes(with_id, descriptor.label().as_bytes());
    let with_description = fold_optional_str(with_label, descriptor.description());
    let with_icon = fold_optional_str(
        with_description,
        descriptor.icon().map(|icon_id| icon_id.as_str()),
    );
    let with_shortcut = fold_optional_str(with_icon, descriptor.default_shortcut_reference());
    let with_category = fold_bytes(with_shortcut, descriptor.category().as_str().as_bytes());
    fold_optional_str(
        with_category,
        descriptor
            .projection_eligibility()
            .map(|projection_id| projection_id.as_str()),
    )
}

fn fold_optional_str(accumulator: u64, value: Option<&str>) -> u64 {
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
