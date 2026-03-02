//! The serializable feature record for the registry.
//!
//! DOMAIN: Wraps a `FeatureKind` (the variant-specific data) with shared
//! identity fields (name, dependencies). Implements `FeatureRegistry` so
//! that `FeatureTree<NativeFeature>` can evaluate features generically.
//!
//! INVARIANTS:
//! - `name` and `dependencies` are always populated at construction time
//! - `FeatureRegistry` delegates `execute_via_pipeline` to `self.kind`

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use forge_core::envelope::OperationResult;
use forge_core::KernelError;
use forge_signal::facade::NodeId;

use crate::configuration::facade::KernelConfig;
use crate::engine::facade::{Feature, FeatureOutput, FeatureRegistry};
use crate::operations::boolean::{BooleanFeature, BooleanOp};
use crate::operations::primitives::MakePrimitiveFeature;

use super::catalog::FeatureKind;

/// The full serializable feature record.
///
/// Identity fields (name, deps) are stored once. The `kind` field
/// carries the variant-specific data. Adding a new feature = adding
/// a variant to `FeatureKind` via the `feature_catalog!` macro.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeFeature {
    name: String,
    dependencies: Vec<NodeId>,
    kind: FeatureKind,
}

impl NativeFeature {
    // ── Constructors ─────────────────────────────────────────────────────

    /// Create a primitive feature (cube, block, tetrahedron, etc.).
    pub fn primitive(name: &str, feature: MakePrimitiveFeature) -> Self {
        Self {
            name: name.to_string(),
            dependencies: feature.dependencies(),
            kind: FeatureKind::MakePrimitive(feature),
        }
    }

    /// Create a boolean feature (union, subtraction, intersection).
    pub fn boolean(name: &str, op: BooleanOp, target: NodeId, tool: NodeId) -> Self {
        let feature = BooleanFeature::new(name, op, target, tool);
        Self {
            name: name.to_string(),
            dependencies: feature.dependencies(),
            kind: FeatureKind::Boolean(feature),
        }
    }

    // ── Accessors ────────────────────────────────────────────────────────

    /// The variant-specific feature data.
    pub fn kind(&self) -> &FeatureKind {
        &self.kind
    }
}

// ── FeatureRegistry impl ─────────────────────────────────────────────────────

impl FeatureRegistry for NativeFeature {
    fn execute_via_pipeline(
        &self,
        inputs: &HashMap<NodeId, FeatureOutput>,
        session_config: &KernelConfig,
    ) -> Result<OperationResult<FeatureOutput>, KernelError> {
        self.kind.execute_via_pipeline(inputs, session_config)
    }

    fn dependencies(&self) -> Vec<NodeId> {
        self.dependencies.clone()
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for NativeFeature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
