use std::collections::BTreeMap;

use serde::Serialize;
use serde_json::Value;

use crate::memory_workspace::ForgeQueryWorkspaceError;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ForgeQueryMutationMetadata {
    entries: BTreeMap<String, Value>,
}

impl ForgeQueryMutationMetadata {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.entries.get(key)
    }

    pub fn entries(&self) -> &BTreeMap<String, Value> {
        &self.entries
    }

    pub(crate) fn insert<T: Serialize>(
        &mut self,
        key: impl Into<String>,
        value: T,
    ) -> Result<(), ForgeQueryWorkspaceError> {
        let key = key.into();
        if key.trim().is_empty() {
            return Err(ForgeQueryWorkspaceError::new(
                "mutation metadata key may not be empty",
            ));
        }
        if self.entries.contains_key(&key) {
            return Err(ForgeQueryWorkspaceError::new(format!(
                "mutation metadata `{key}` may only be declared once per mutation"
            )));
        }
        let value = serde_json::to_value(value).map_err(|error| {
            ForgeQueryWorkspaceError::new(format!(
                "mutation metadata `{key}` could not serialize into retained evidence: {error}"
            ))
        })?;
        self.entries.insert(key, value);
        Ok(())
    }
}
