#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SettingValidationPosture {
    SchemaChecked,
    PlatformValidated,
    ExternalAuthorityValidated,
}

impl SettingValidationPosture {
    pub fn schema_checked() -> Self {
        Self::SchemaChecked
    }

    pub fn platform_validated() -> Self {
        Self::PlatformValidated
    }

    pub fn external_authority_validated() -> Self {
        Self::ExternalAuthorityValidated
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::SchemaChecked => "schema_checked",
            Self::PlatformValidated => "platform_validated",
            Self::ExternalAuthorityValidated => "external_authority_validated",
        }
    }
}
