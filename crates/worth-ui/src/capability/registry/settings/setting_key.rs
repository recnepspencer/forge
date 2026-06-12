use super::SettingDescriptor;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SettingKey {
    configuration_basis: String,
}

impl SettingKey {
    pub(crate) fn from_descriptor(descriptor: &SettingDescriptor) -> Self {
        Self::new(format!(
            "{}|{}|{}|{}|{}|{}|{}|{}",
            length_prefixed(descriptor.id().as_str()),
            descriptor
                .scope()
                .map(|scope| scope.digest_basis())
                .unwrap_or("none"),
            descriptor
                .value_schema()
                .map(|schema| schema.digest_basis())
                .unwrap_or_else(|| "none".to_string()),
            descriptor
                .default_posture()
                .map(|posture| posture.digest_basis())
                .unwrap_or_else(|| "none".to_string()),
            descriptor
                .validation_posture()
                .map(|posture| posture.digest_basis())
                .unwrap_or("none"),
            descriptor
                .migration_posture()
                .map(|posture| posture.digest_basis())
                .unwrap_or("none"),
            descriptor
                .editor_hint()
                .map(|hint| hint.digest_basis())
                .unwrap_or_else(|| "none".to_string()),
            descriptor
                .ownership_metadata()
                .map(|metadata| metadata.digest_basis())
                .unwrap_or("none")
        ))
    }

    fn new(configuration_basis: impl Into<String>) -> Self {
        Self {
            configuration_basis: configuration_basis.into(),
        }
    }

    pub fn configuration_basis(&self) -> &str {
        &self.configuration_basis
    }
}

fn length_prefixed(value: &str) -> String {
    format!("{}:{value}", value.len())
}
