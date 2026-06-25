#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiLiveViewStateFactId(String);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewStateValue {
    Text(String),
    Boolean(bool),
    Number(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewStateValueKind {
    Text,
    Boolean,
    Number,
    Unsupported(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewStateAccess {
    ReadWrite,
    ReadOnly,
}

impl WorthUiLiveViewStateFactId {
    pub fn new(raw: impl Into<String>) -> Result<Self, String> {
        let raw = raw.into();
        if raw.trim().is_empty() || raw.chars().any(char::is_whitespace) {
            Err(raw)
        } else {
            Ok(Self(raw))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl WorthUiLiveViewStateValue {
    pub fn text(value: impl Into<String>) -> Self {
        Self::Text(value.into())
    }

    pub fn value_kind(&self) -> WorthUiLiveViewStateValueKind {
        match self {
            Self::Text(_) => WorthUiLiveViewStateValueKind::Text,
            Self::Boolean(_) => WorthUiLiveViewStateValueKind::Boolean,
            Self::Number(_) => WorthUiLiveViewStateValueKind::Number,
        }
    }

    pub fn as_display_text(&self) -> String {
        match self {
            Self::Text(value) | Self::Number(value) => value.clone(),
            Self::Boolean(value) => value.to_string(),
        }
    }

    pub(crate) fn digest_token(&self) -> String {
        format!("{}:{}", self.value_kind().token(), self.as_display_text())
    }
}

impl WorthUiLiveViewStateValueKind {
    pub fn token(&self) -> &str {
        match self {
            Self::Text => "text",
            Self::Boolean => "boolean",
            Self::Number => "number",
            Self::Unsupported(value) => value.as_str(),
        }
    }

    pub(crate) fn is_supported(&self) -> bool {
        !matches!(self, Self::Unsupported(_))
    }
}

impl WorthUiLiveViewStateAccess {
    pub fn token(self) -> &'static str {
        match self {
            Self::ReadWrite => "read_write",
            Self::ReadOnly => "read_only",
        }
    }
}
