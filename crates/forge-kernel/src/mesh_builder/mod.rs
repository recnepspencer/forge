//! DOMAIN: ConvexCell → Halfedge Mesh Conversion
//!
//! Converts BSP `ConvexCell` output (face-vertex mesh with plane references)
//! into the `TopologyArena` halfedge mesh representation with associated
//! geometry stored in `GeometryStore`.
//!
//! INVARIANTS:
//! - Output topology satisfies Euler's formula (V - E + F = 2)
//! - Every face has a closed halfedge loop
//! - Every vertex position matches the ConvexCell source
//! - Every face plane is registered in the GeometryStore
//!
//! DEPENDENCIES: `forge-geom` (ConvexCell, Plane), `forge-topo` (arena, operators),
//!               `geometry_store` (GeometryStore)

mod eval;
#[cfg(test)]
mod tests;

pub use eval::{build_halfedge_mesh, make_cube, make_tetrahedron, make_dodecahedron, make_convex_solid, MeshBuildResult};
