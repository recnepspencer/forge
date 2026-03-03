//! Mesh building operations — topology construction from ConvexCell data.
//!
//! DOMAIN: Generic "ConvexCell → halfedge topology" conversion. Builds faces,
//! loops, halfedge chains, and stitches twin pointers. Consumed by primitives,
//! booleans, and any operation that creates meshes from BSP output.

pub mod cell_to_mesh;
pub mod containment;
