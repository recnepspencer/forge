use super::PluginSlotDescriptor;

/// Stable key for plugin slot admission semantics.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PluginSlotKey {
    admission_basis: String,
}

impl PluginSlotKey {
    pub(crate) fn from_descriptor(descriptor: &PluginSlotDescriptor) -> Self {
        Self {
            admission_basis: admission_basis(descriptor),
        }
    }

    pub fn admission_basis(&self) -> &str {
        &self.admission_basis
    }
}

fn admission_basis(descriptor: &PluginSlotDescriptor) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}",
        length_prefixed(descriptor.id().as_str()),
        allowed_family_basis(descriptor),
        descriptor
            .permission()
            .map(|permission| permission.digest_basis())
            .unwrap_or("none"),
        descriptor
            .ordering()
            .map(|ordering| ordering.digest_basis())
            .unwrap_or("none"),
        descriptor
            .diagnostics()
            .map(|diagnostics| diagnostics.digest_basis())
            .unwrap_or("none"),
        descriptor
            .support()
            .map(|support| support.digest_basis())
            .unwrap_or("none"),
        descriptor
            .contribution_reference()
            .map(|reference| length_prefixed(&reference.digest_basis()))
            .unwrap_or_else(|| "none".to_owned()),
        global_mutation_hook_basis(descriptor)
    )
}

fn allowed_family_basis(descriptor: &PluginSlotDescriptor) -> String {
    descriptor
        .allowed_families()
        .iter()
        .map(|family| length_prefixed(&family.digest_basis()))
        .collect::<Vec<_>>()
        .join("")
}

fn global_mutation_hook_basis(descriptor: &PluginSlotDescriptor) -> String {
    descriptor
        .global_mutation_hooks()
        .iter()
        .map(|hook| length_prefixed(hook.digest_basis()))
        .collect::<Vec<_>>()
        .join("")
}

fn length_prefixed(value: &str) -> String {
    format!("{}:{value}", value.len())
}
