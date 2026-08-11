use serde::Serialize;

use super::super::PlacementObservationScopeClass;
use worth_store_contracts::PlacementArtifactFamily;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct RecallCoalescingKey {
    artifact_family: PlacementArtifactFamily,
    scope_class: PlacementObservationScopeClass,
    scope_key: String,
}

impl RecallCoalescingKey {
    pub(crate) fn new(
        artifact_family: PlacementArtifactFamily,
        scope_class: PlacementObservationScopeClass,
        scope_key: impl Into<String>,
    ) -> Self {
        Self {
            artifact_family,
            scope_class,
            scope_key: scope_key.into(),
        }
    }

    pub fn artifact_family(&self) -> PlacementArtifactFamily {
        self.artifact_family
    }

    pub fn scope_class(&self) -> PlacementObservationScopeClass {
        self.scope_class
    }

    pub fn scope_key(&self) -> &str {
        &self.scope_key
    }
}
