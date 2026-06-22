use std::collections::BTreeMap;

use crate::memory_workspace::ForgeQueryWorkspaceError;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ForgeQueryMutationMetadataKey {
    text: String,
}

impl ForgeQueryMutationMetadataKey {
    pub fn new(key: impl Into<String>) -> Result<Self, ForgeQueryWorkspaceError> {
        let text = key.into();
        if text.trim().is_empty() {
            return Err(ForgeQueryWorkspaceError::new(
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
pub struct ForgeQueryMutationMetadataValue {
    text: String,
}

impl ForgeQueryMutationMetadataValue {
    fn from_text(value: impl Into<String>) -> Result<Self, ForgeQueryWorkspaceError> {
        let text = value.into();
        if text.trim().is_empty() {
            return Err(ForgeQueryWorkspaceError::new(
                "mutation metadata value may not be empty",
            ));
        }
        Ok(Self { text })
    }

    pub fn native_digest_text(&self) -> &str {
        &self.text
    }
}

impl std::fmt::Display for ForgeQueryMutationMetadataValue {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.native_digest_text())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ForgeQueryMutationMetadata {
    entries: BTreeMap<String, ForgeQueryMutationMetadataValue>,
}

impl ForgeQueryMutationMetadata {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn get(
        &self,
        key: &ForgeQueryMutationMetadataKey,
    ) -> Option<&ForgeQueryMutationMetadataValue> {
        self.entries.get(key.as_str())
    }

    pub fn entries(
        &self,
    ) -> impl Iterator<
        Item = (
            ForgeQueryMutationMetadataKey,
            &ForgeQueryMutationMetadataValue,
        ),
    > + '_ {
        self.entries
            .iter()
            .map(|(key, value)| (ForgeQueryMutationMetadataKey { text: key.clone() }, value))
    }

    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), ForgeQueryWorkspaceError> {
        self.insert_text(key, value)
    }

    pub(crate) fn insert_text(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), ForgeQueryWorkspaceError> {
        let key = ForgeQueryMutationMetadataKey::new(key)?;
        let key = key.as_str().to_string();
        if self.entries.contains_key(&key) {
            return Err(ForgeQueryWorkspaceError::new(format!(
                "mutation metadata `{key}` may only be declared once per mutation"
            )));
        }
        let value = ForgeQueryMutationMetadataValue::from_text(value)?;
        self.entries.insert(key, value);
        Ok(())
    }
}
