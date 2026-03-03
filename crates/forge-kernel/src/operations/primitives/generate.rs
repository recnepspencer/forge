//! Primitive shape generation logic.
//!
//! DOMAIN: Dispatches `PrimitiveParams` → plane generators → `make_convex_solid`.
//! All convex planar primitives share the same mesh-building pipeline;
//! only the plane set differs.
//!
//! DEPENDENCIES: forge-geom (shapes), mesh_builder (make_convex_solid)

use forge_core::KernelError;

use crate::context::scope::OperationScope;
use crate::engine::facade::SolidEnvelope;

use super::PrimitiveParams;

/// Generate a convex primitive solid from its parameters and center.
///
/// Dispatches to the appropriate plane generator in `forge_geom::primitives::shapes`,
/// then builds a closed halfedge mesh via `make_convex_solid`.
pub fn generate_primitive(
    params: &PrimitiveParams,
    center: [f64; 3],
    scope: &mut OperationScope<'_>,
) -> Result<SolidEnvelope, KernelError> {
    match params {
        PrimitiveParams::Cube { size } => {
            super::make_cube(center, *size, scope)
        }
        PrimitiveParams::Block { half_extents } => {
            super::make_block(center, *half_extents, scope)
        }
        PrimitiveParams::Tetrahedron { scale } => {
            super::make_tetrahedron(center, *scale, scope)
        }
        PrimitiveParams::Dodecahedron { scale } => {
            super::make_dodecahedron(center, *scale, scope)
        }
        PrimitiveParams::Prism { sides, radius, height } => {
            super::make_prism(center, *sides, *radius, *height, scope)
        }
        PrimitiveParams::Pyramid { sides, radius, height } => {
            super::make_pyramid(center, *sides, *radius, *height, scope)
        }
        PrimitiveParams::Wedge { dimensions } => {
            super::make_wedge(center, *dimensions, scope)
        }
    }
}
