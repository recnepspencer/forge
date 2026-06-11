use super::CommandProjectionDescriptor;

/// Stable projection key derived from command-spine view posture.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandProjectionKey {
    projection_basis: String,
}

impl CommandProjectionKey {
    pub(crate) fn from_descriptor(descriptor: &CommandProjectionDescriptor) -> Self {
        Self {
            projection_basis: projection_basis(descriptor),
        }
    }

    pub fn projection_basis(&self) -> &str {
        &self.projection_basis
    }
}

fn projection_basis(descriptor: &CommandProjectionDescriptor) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        descriptor.id().as_str(),
        descriptor.surface().digest_basis(),
        command_reference_basis(descriptor),
        category_basis(descriptor),
        grouping_basis(descriptor),
        descriptor.ordering().digest_basis(),
        descriptor.shortcut_visibility().digest_basis(),
        descriptor.readiness_display_policy().digest_basis(),
        descriptor.icon_label_policy().digest_basis(),
        descriptor.overflow_behavior().digest_basis(),
        descriptor
            .mosaic_scope()
            .map(|scope| scope.digest_basis())
            .unwrap_or_else(|| "none".to_owned()),
        meaning_override_basis(descriptor)
    )
}

fn command_reference_basis(descriptor: &CommandProjectionDescriptor) -> String {
    descriptor
        .command_references()
        .iter()
        .map(|reference| length_prefixed(&reference.digest_basis()))
        .collect::<Vec<_>>()
        .join("")
}

fn category_basis(descriptor: &CommandProjectionDescriptor) -> String {
    let mut categories = descriptor
        .eligible_categories()
        .iter()
        .map(|category| category.as_str())
        .collect::<Vec<_>>();
    categories.sort();
    categories.dedup();
    categories
        .into_iter()
        .map(length_prefixed)
        .collect::<Vec<_>>()
        .join("")
}

fn grouping_basis(descriptor: &CommandProjectionDescriptor) -> String {
    descriptor
        .groupings()
        .iter()
        .map(|grouping| length_prefixed(&grouping.digest_basis()))
        .collect::<Vec<_>>()
        .join("")
}

fn meaning_override_basis(descriptor: &CommandProjectionDescriptor) -> String {
    descriptor
        .meaning_overrides()
        .iter()
        .map(|override_kind| length_prefixed(override_kind.digest_basis()))
        .collect::<Vec<_>>()
        .join("")
}

fn length_prefixed(value: &str) -> String {
    format!("{}:{value}", value.len())
}
