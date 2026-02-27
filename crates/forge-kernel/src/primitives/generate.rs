//! Primitive shape generation logic.
//!
//! DOMAIN: Dispatches `PrimitiveParams` → plane generators → `make_convex_solid`.
//! All convex planar primitives share the same mesh-building pipeline;
//! only the plane set differs.
//!
//! DEPENDENCIES: forge-geom (shapes), mesh_builder (make_convex_solid)

use forge_core::KernelError;

use crate::core::config::resolve::ResolvedConfig;
use crate::engine::traits::FeatureOutput;

use super::PrimitiveParams;

/// Generate a convex primitive solid from its parameters and center.
///
/// Dispatches to the appropriate plane generator in `forge_geom::primitives::shapes`,
/// then builds a closed halfedge mesh via `make_convex_solid`.
pub fn generate_primitive(
    params: &PrimitiveParams,
    center: [f64; 3],
    config: &ResolvedConfig,
) -> Result<FeatureOutput, KernelError> {
    let result = match params {
        PrimitiveParams::Cube { size } => {
            crate::mesh_builder::make_cube(center, *size, config)?
        }
        PrimitiveParams::Block { half_extents } => {
            crate::mesh_builder::make_block(center, *half_extents, config)?
        }
        PrimitiveParams::Tetrahedron { scale } => {
            crate::mesh_builder::make_tetrahedron(center, *scale, config)?
        }
        PrimitiveParams::Dodecahedron { scale } => {
            crate::mesh_builder::make_dodecahedron(center, *scale, config)?
        }
        PrimitiveParams::Prism { sides, radius, height } => {
            crate::mesh_builder::make_prism(center, *sides, *radius, *height, config)?
        }
        PrimitiveParams::Pyramid { sides, radius, height } => {
            crate::mesh_builder::make_pyramid(center, *sides, *radius, *height, config)?
        }
        PrimitiveParams::Wedge { dimensions } => {
            crate::mesh_builder::make_wedge(center, *dimensions, config)?
        }
    };

    let (topo, geom, brep) = result.into_parts();

    Ok(FeatureOutput {
        topology: topo,
        geometry: geom,
        brep,
    })
}
