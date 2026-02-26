//! Provenance tracking for configuration values.
//!
//! DOMAIN: Tracking where configuration values came from across the cascade.

use serde::{Deserialize, Serialize};

/// The scope or level of the configuration cascade that provided a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConfigScope {
    /// The base session default, usually system-wide or user-wide.
    SessionDefault,
    /// An override applied to the entire loaded model.
    ModelOverride,
    /// An override applied by a specific feature (e.g., inside a FeatureContract).
    FeatureOverride,
    /// An override applied to a specific sub-operation (e.g., boolean intersection).
    OperationOverride,
}

impl Default for ConfigScope {
    fn default() -> Self {
        Self::SessionDefault
    }
}

/// The origin of a configuration value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigSource {
    /// Which level of the cascade provided this value.
    pub scope: ConfigScope,
    /// Optional handle to the specific entity that set the override.
    /// E.g., "FeatureHandle(0xAF32)" or "Imported_Bolt_4"
    pub origin: Option<String>,
}

impl ConfigSource {
    /// Create a new ConfigSource.
    pub fn new(scope: ConfigScope, origin: Option<String>) -> Self {
        Self { scope, origin }
    }

    /// Create a Default session source without a specific origin.
    pub fn session() -> Self {
        Self {
            scope: ConfigScope::SessionDefault,
            origin: None,
        }
    }
}

/// Stores the source of every field in the `KernelConfig`.
///
/// Handled internally by `resolve_config()` to provide debug
/// introspectability into why a certain tolerance was chosen.
#[derive(Debug, Clone)]
pub struct ConfigProvenance {
    /// A map from the fully qualified field name (e.g., "tolerance.spatial_tolerance")
    /// to its source.
    sources: std::collections::BTreeMap<&'static str, ConfigSource>,
}

impl ConfigProvenance {
    /// Create a new, empty provenance tracker.
    pub fn new() -> Self {
        Self {
            sources: std::collections::BTreeMap::new(),
        }
    }

    /// Record the source for a specific field.
    pub fn record(&mut self, field: &'static str, source: ConfigSource) {
        self.sources.insert(field, source);
    }

    /// Query which scope set a given field, including the specific entity.
    pub fn source_of(&self, field: &str) -> Option<&ConfigSource> {
        self.sources.get(field)
    }

    /// Get all recorded provenances.
    pub fn all_sources(&self) -> &std::collections::BTreeMap<&'static str, ConfigSource> {
        &self.sources
    }
}

impl Default for ConfigProvenance {
    fn default() -> Self {
        Self::new()
    }
}
