//! Primitive shape feature.
//!
//! DOMAIN: Parameterized convex primitive generation (cube, block, tetrahedron,
//! dodecahedron, prism, pyramid, wedge). All variants share the same internal
//! pipeline: plane generation → BSP → halfedge mesh.
//!
//! INVARIANTS:
//! - Output satisfies Euler's formula (V - E + F = 2)
//! - All faces have closed halfedge loops
//! - Every edge has a twin (manifold)
//! - All inputs validated (no NaN, Inf, or ≤0 dimensions)
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

/// Shape-specific parameters for each primitive variant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrimitiveParams {
    /// Uniform cube: side length.
    Cube { size: f64 },

    /// Non-uniform axis-aligned box: independent half-extents [hx, hy, hz].
    Block { half_extents: [f64; 3] },

    /// Regular tetrahedron: scale factor.
    Tetrahedron { scale: f64 },

    /// Regular dodecahedron: scale factor.
    Dodecahedron { scale: f64 },

    /// Regular n-gon extrusion along Z.
    Prism {
        sides: u32,
        radius: f64,
        height: f64,
    },

    /// Regular n-gon base with apex above center.
    Pyramid {
        sides: u32,
        radius: f64,
        height: f64,
    },

    /// Triangular cross-section extrusion: [width, depth, height].
    Wedge { dimensions: [f64; 3] },
}

/// A parameterized convex primitive feature.
///
/// Generates a closed convex solid from analytic planes via BSP intersection.
/// All variants use the same algorithm (`make_convex_solid`); only the
/// plane generator differs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MakePrimitiveFeature {
    name: String,
    params: PrimitiveParams,
    center: [f64; 3],
}

impl MakePrimitiveFeature {
    /// Create a new primitive feature.
    pub fn new(name: &str, params: PrimitiveParams, center: [f64; 3]) -> Self {
        Self {
            name: name.to_string(),
            params,
            center,
        }
    }

    /// Convenience constructor for a cube.
    pub fn cube(name: &str, center: [f64; 3], size: f64) -> Self {
        Self::new(name, PrimitiveParams::Cube { size }, center)
    }

    /// Convenience constructor for a non-uniform box.
    pub fn block(name: &str, center: [f64; 3], half_extents: [f64; 3]) -> Self {
        Self::new(name, PrimitiveParams::Block { half_extents }, center)
    }

    /// Convenience constructor for a tetrahedron.
    pub fn tetrahedron(name: &str, center: [f64; 3], scale: f64) -> Self {
        Self::new(name, PrimitiveParams::Tetrahedron { scale }, center)
    }

    /// Convenience constructor for a dodecahedron.
    pub fn dodecahedron(name: &str, center: [f64; 3], scale: f64) -> Self {
        Self::new(name, PrimitiveParams::Dodecahedron { scale }, center)
    }

    /// Convenience constructor for a regular prism.
    pub fn prism(name: &str, center: [f64; 3], sides: u32, radius: f64, height: f64) -> Self {
        Self::new(
            name,
            PrimitiveParams::Prism {
                sides,
                radius,
                height,
            },
            center,
        )
    }

    /// Convenience constructor for a regular pyramid.
    pub fn pyramid(name: &str, center: [f64; 3], sides: u32, radius: f64, height: f64) -> Self {
        Self::new(
            name,
            PrimitiveParams::Pyramid {
                sides,
                radius,
                height,
            },
            center,
        )
    }

    /// Convenience constructor for a wedge.
    pub fn wedge(name: &str, center: [f64; 3], dimensions: [f64; 3]) -> Self {
        Self::new(name, PrimitiveParams::Wedge { dimensions }, center)
    }

    /// The kind of primitive this feature generates (machine-readable label).
    pub fn kind_str(&self) -> &'static str {
        match &self.params {
            PrimitiveParams::Cube { .. } => "cube",
            PrimitiveParams::Block { .. } => "block",
            PrimitiveParams::Tetrahedron { .. } => "tetrahedron",
            PrimitiveParams::Dodecahedron { .. } => "dodecahedron",
            PrimitiveParams::Prism { .. } => "prism",
            PrimitiveParams::Pyramid { .. } => "pyramid",
            PrimitiveParams::Wedge { .. } => "wedge",
        }
    }

    /// The shape-specific parameters.
    pub fn params(&self) -> &PrimitiveParams {
        &self.params
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
        generate::generate_primitive(&self.params, self.center, config)
    }

    fn dependencies(&self) -> Vec<NodeId> {
        vec![]
    }

    fn name(&self) -> &str {
        &self.name
    }
}
