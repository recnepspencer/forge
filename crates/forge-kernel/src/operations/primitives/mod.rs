//! Primitive shape generation and mesh construction.
//!
//! DOMAIN: Parameterized convex primitive generation (cube, block, tetrahedron,
//! dodecahedron, prism, pyramid, wedge) and the BSP → halfedge mesh pipeline.
//!
//! INVARIANTS:
//! - Output satisfies Euler's formula (V - E + F = 2)
//! - All faces have closed halfedge loops
//! - Every edge has a twin (manifold)
//! - All inputs validated (no NaN, Inf, or ≤0 dimensions)
//!
//! DEPENDENCIES: forge-geom (shapes, BSP, ConvexCell, Plane),
//!               forge-topo (arena, operators), geometry (GeometryStore),
//!               configuration (ResolvedConfig)

mod contract;
mod eval;
mod generate;
#[cfg(test)]
mod tests;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use forge_core::KernelError;
use forge_signal::facade::NodeId;

use crate::context::scope::OperationScope;
use crate::engine::facade::{Feature, FeatureOutput};

pub use contract::PrimitiveInputs;

// Re-export mesh construction API (used by boolean test helpers and integration tests)
pub use eval::{
    build_halfedge_mesh, make_block, make_convex_solid, make_cube, make_dodecahedron,
    make_prism, make_pyramid, make_tetrahedron, make_wedge, MeshBuildResult,
};

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

    /// Convenience constructor from origin + dimensions (schema-native parameters).
    pub fn block_from_origin(name: &str, origin: [f64; 3], dimensions: [f64; 3]) -> Self {
        let center = [
            origin[0] + dimensions[0] / 2.0,
            origin[1] + dimensions[1] / 2.0,
            origin[2] + dimensions[2] / 2.0,
        ];
        let size = dimensions[0];
        Self::cube(name, center, size)
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
        scope: &mut OperationScope<'_>,
    ) -> Result<FeatureOutput, KernelError> {
        generate::generate_primitive(&self.params, self.center, scope)
    }

    fn dependencies(&self) -> Vec<NodeId> {
        vec![]
    }

    fn name(&self) -> &str {
        &self.name
    }
}
