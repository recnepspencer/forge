#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BindingError {
    EmptyBindingSlot,
    EmptyMetadataKey,
    UnsupportedMetadataKey { key: String },
    ForbiddenMetadataKey { key: String },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BindingFailureClass {
    InvalidAtom,
    UnsupportedMetadata,
    ForbiddenMetadata,
}

impl BindingError {
    pub fn failure_class(&self) -> BindingFailureClass {
        match self {
            Self::EmptyBindingSlot | Self::EmptyMetadataKey => BindingFailureClass::InvalidAtom,
            Self::UnsupportedMetadataKey { .. } => BindingFailureClass::UnsupportedMetadata,
            Self::ForbiddenMetadataKey { .. } => BindingFailureClass::ForbiddenMetadata,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum NonIdentityBindingMetadataKey {
    RouteName,
    HostComponentName,
    UiLabel,
    DebugLabel,
    HostAttachmentHandle,
}

impl NonIdentityBindingMetadataKey {
    fn parse(raw: &str) -> Result<Self, BindingError> {
        let normalized = raw.trim();
        if normalized.is_empty() {
            return Err(BindingError::EmptyMetadataKey);
        }

        match normalized {
            "route" | "route_name" => Ok(Self::RouteName),
            "component" | "component_name" | "controller" | "controller_name" => {
                Ok(Self::HostComponentName)
            }
            "ui_label" | "label" => Ok(Self::UiLabel),
            "debug_label" | "debug" => Ok(Self::DebugLabel),
            "host_attachment_handle" | "attachment_handle" => Ok(Self::HostAttachmentHandle),
            "filter" | "hidden_filter" | "branch" | "basis" | "policy" | "tenant"
            | "result_shape" | "execution_hint" => Err(BindingError::ForbiddenMetadataKey {
                key: normalized.to_string(),
            }),
            _ => Err(BindingError::UnsupportedMetadataKey {
                key: normalized.to_string(),
            }),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RouteName => "route",
            Self::HostComponentName => "component",
            Self::UiLabel => "ui_label",
            Self::DebugLabel => "debug_label",
            Self::HostAttachmentHandle => "host_attachment_handle",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NonIdentityBindingMetadata {
    key: NonIdentityBindingMetadataKey,
    value: String,
}

impl NonIdentityBindingMetadata {
    pub fn new(key: impl AsRef<str>, value: impl Into<String>) -> Result<Self, BindingError> {
        Ok(Self {
            key: NonIdentityBindingMetadataKey::parse(key.as_ref())?,
            value: value.into(),
        })
    }

    pub fn key(&self) -> &str {
        self.key.as_str()
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
