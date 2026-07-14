use std::collections::BTreeMap;

use crate::memory_workspace::WorthQueryWorkspaceError;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryMutationMetadataKey {
    text: String,
}

impl WorthQueryMutationMetadataKey {
    pub fn new(key: impl Into<String>) -> Result<Self, WorthQueryWorkspaceError> {
        let text = key.into();
        if text.trim().is_empty() {
            return Err(WorthQueryWorkspaceError::new(
                "mutation metadata key may not be empty",
            ));
        }
        Ok(Self { text })
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthQueryMutationMetadataValue {
    text: String,
}

impl WorthQueryMutationMetadataValue {
    fn from_text(value: impl Into<String>) -> Result<Self, WorthQueryWorkspaceError> {
        let text = value.into();
        if text.trim().is_empty() {
            return Err(WorthQueryWorkspaceError::new(
                "mutation metadata value may not be empty",
            ));
        }
        Ok(Self { text })
    }

    pub fn terminal_digest_text(&self) -> &str {
        &self.text
    }
}

impl std::fmt::Display for WorthQueryMutationMetadataValue {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.terminal_digest_text())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WorthQueryMutationMetadata {
    entries: BTreeMap<WorthQueryMutationMetadataKey, WorthQueryMutationMetadataValue>,
}

impl WorthQueryMutationMetadata {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn get(
        &self,
        key: &WorthQueryMutationMetadataKey,
    ) -> Option<&WorthQueryMutationMetadataValue> {
        self.entries.get(key)
    }

    pub fn entries(
        &self,
    ) -> impl Iterator<
        Item = (
            WorthQueryMutationMetadataKey,
            &WorthQueryMutationMetadataValue,
        ),
    > + '_ {
        self.entries.iter().map(|(key, value)| (key.clone(), value))
    }

    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), WorthQueryWorkspaceError> {
        self.insert_text(key, value)
    }

    pub(crate) fn insert_text(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), WorthQueryWorkspaceError> {
        let key = WorthQueryMutationMetadataKey::new(key)?;
        if self.entries.contains_key(&key) {
            return Err(WorthQueryWorkspaceError::new(format!(
                "mutation metadata `{}` may only be declared once per mutation",
                key.as_str()
            )));
        }
        let value = WorthQueryMutationMetadataValue::from_text(value)?;
        self.entries.insert(key, value);
        Ok(())
    }
}
