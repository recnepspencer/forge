//! JSON schema types for versioned model serialization.
//!
//! DOMAIN: Data shapes for the JSON serialization envelope.
//! DEPENDENCIES: `serde`, `forge-kernel` (FeatureTree)

use forge_kernel::engine::facade::FeatureTree;
use forge_kernel::registry::facade::NativeFeature;
use serde::{Deserialize, Serialize};

/// Current schema version for forward compatibility.
pub const SCHEMA_VERSION: u32 = 1;

/// Versioned envelope for serialized models.
///
/// Every JSON file starts with this wrapper so that future versions
/// can detect and migrate older schemas automatically.
#[derive(Debug, Serialize, Deserialize)]
pub struct VersionedModel {
    /// Schema version (monotonically increasing).
    pub(crate) version: u32,
    /// The feature tree payload.
    pub(crate) tree: FeatureTree<NativeFeature>,
}

impl VersionedModel {
    /// Wrap a feature tree with the current schema version.
    pub fn wrap(tree: FeatureTree<NativeFeature>) -> Self {
        Self {
            version: SCHEMA_VERSION,
            tree,
        }
    }

    /// The schema version of this model.
    pub fn get_version(&self) -> u32 {
        self.version
    }

    /// Consume and return the inner feature tree.
    pub fn into_tree(self) -> FeatureTree<NativeFeature> {
        self.tree
    }
}
