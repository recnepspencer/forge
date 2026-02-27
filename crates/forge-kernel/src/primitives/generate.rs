//! Primitive shape generation logic.
//!
//! DOMAIN: Dispatches `PrimitiveKind` → plane generators → `make_convex_solid`.
//! All convex planar primitives share the same mesh-building pipeline;
//! only the plane set differs.
//!
//! DEPENDENCIES: forge-geom (shapes), mesh_builder (make_convex_solid)

use forge_core::KernelError;

use crate::core::config::resolve::ResolvedConfig;
use crate::engine::traits::FeatureOutput;

use super::PrimitiveKind;

/// Generate a convex primitive solid from its kind, center, and size.
///
/// Dispatches to the appropriate plane generator in `forge_geom::primitives::shapes`,
/// then builds a closed halfedge mesh via `make_convex_solid`.
pub fn generate_primitive(
    kind: PrimitiveKind,
    center: [f64; 3],
    size: f64,
    _config: &ResolvedConfig,
) -> Result<FeatureOutput, KernelError> {
    let planes = match kind {
        PrimitiveKind::Cube => crate::geom_facade::shapes::cube(center, size / 2.0),
        PrimitiveKind::Tetrahedron => crate::geom_facade::shapes::tetrahedron(center, size),
        PrimitiveKind::Dodecahedron => crate::geom_facade::shapes::dodecahedron(center, size),
    };

    let result = crate::mesh_builder::make_convex_solid(planes)?;
    let (topo, geom, brep) = result.into_parts();

    Ok(FeatureOutput {
        topology: topo,
        geometry: geom,
        brep,
    })
}
