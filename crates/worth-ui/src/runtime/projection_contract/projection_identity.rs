#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiProjectionIdentity {
    raw: String,
}

impl WorthUiProjectionIdentity {
    pub(crate) fn runtime(raw: impl Into<String>) -> Self {
        Self { raw: raw.into() }
    }

    pub fn as_str(&self) -> &str {
        &self.raw
    }
}
