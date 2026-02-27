//! Primitive shape feature.
//!
//! DOMAIN: Parameterized convex primitive generation (cube, tetrahedron,
//! dodecahedron). All variants share the same internal pipeline:
//! plane generation → BSP → halfedge mesh.
//!
//! INVARIANTS:
//! - Output satisfies Euler's formula (V - E + F = 2)
//! - All faces have closed halfedge loops
//! - Every edge has a twin (manifold)
//!
//! DEPENDENCIES: forge-geom (shapes, BSP), mesh_builder (make_convex_solid),
//! pipeline (FeatureContract, Feature)

mod contract;
mod generate;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use forge_core::KernelError;
use forge_signal::handles::NodeId;

use crate::core::config::resolve::ResolvedConfig;
use crate::engine::traits::{Feature, FeatureOutput};

pub use contract::PrimitiveInputs;

/// Which convex primitive to generate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrimitiveKind {
    Cube,
    Tetrahedron,
    Dodecahedron,
}

impl PrimitiveKind {
    /// Machine-readable name for tracing and serialization.
    pub fn as_str(&self) -> &'static str {
        match self {
            PrimitiveKind::Cube => "cube",
            PrimitiveKind::Tetrahedron => "tetrahedron",
            PrimitiveKind::Dodecahedron => "dodecahedron",
        }
    }
}

/// A parameterized convex primitive feature.
///
/// Generates a closed convex solid from analytic planes via BSP intersection.
/// All variants use the same algorithm (`make_convex_solid`); only the
/// plane generator differs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MakePrimitiveFeature {
    name: String,
    kind: PrimitiveKind,
    center: [f64; 3],
    size: f64,
}

impl MakePrimitiveFeature {
    /// Create a new primitive feature.
    pub fn new(name: &str, kind: PrimitiveKind, center: [f64; 3], size: f64) -> Self {
        Self {
            name: name.to_string(),
            kind,
            center,
            size,
        }
    }

    /// Convenience constructor for a cube.
    pub fn cube(name: &str, center: [f64; 3], size: f64) -> Self {
        Self::new(name, PrimitiveKind::Cube, center, size)
    }

    /// Convenience constructor for a tetrahedron.
    pub fn tetrahedron(name: &str, center: [f64; 3], scale: f64) -> Self {
        Self::new(name, PrimitiveKind::Tetrahedron, center, scale)
    }

    /// Convenience constructor for a dodecahedron.
    pub fn dodecahedron(name: &str, center: [f64; 3], scale: f64) -> Self {
        Self::new(name, PrimitiveKind::Dodecahedron, center, scale)
    }

    /// The kind of primitive this feature generates.
    pub fn kind(&self) -> PrimitiveKind {
        self.kind
    }
}

// ── Feature impl ─────────────────────────────────────────────────────────

impl Feature for MakePrimitiveFeature {
    type Inputs = PrimitiveInputs;

    fn parse_inputs(
        &self,
        _raw: &HashMap<NodeId, FeatureOutput>,
    ) -> Result<PrimitiveInputs, KernelError> {
        Ok(PrimitiveInputs)
    }

    fn execute_typed(
        &self,
        _inputs: &PrimitiveInputs,
        config: &ResolvedConfig,
    ) -> Result<FeatureOutput, KernelError> {
        generate::generate_primitive(self.kind, self.center, self.size, config)
    }

    fn dependencies(&self) -> Vec<NodeId> {
        vec![]
    }

    fn name(&self) -> &str {
        &self.name
    }
}
