use super::IconDescriptor;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct IconKey {
    projection_basis: String,
}

impl IconKey {
    pub(crate) fn from_descriptor(descriptor: &IconDescriptor) -> Self {
        Self {
            projection_basis: icon_projection_basis(descriptor),
        }
    }

    pub fn projection_basis(&self) -> &str {
        &self.projection_basis
    }
}

fn icon_projection_basis(descriptor: &IconDescriptor) -> String {
    [
        length_prefixed(descriptor.id().as_str()),
        descriptor.family().digest_basis(),
        descriptor
            .source()
            .map(|source| source.digest_basis())
            .unwrap_or_else(|| "source:none".to_string()),
        descriptor.theme_posture().digest_basis().to_string(),
        descriptor
            .accessibility_posture()
            .digest_basis()
            .to_string(),
    ]
    .join("|")
}

fn length_prefixed(value: &str) -> String {
    format!("{}:{value}", value.len())
}
