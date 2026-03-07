mod builder;
mod queries;
mod signature;
mod validation;

pub use builder::ProjectionBuilder;
pub use queries::ProjectedTopologyQueries;
pub use signature::compute_projected_topology_hash;
pub use validation::{
    validate_projected_broken_boundary, validate_projected_face_adjacency,
    validate_projected_loop_wiring, validate_projected_manifold_edges,
    validate_projected_radial_edge, validate_projected_shell_closure,
    validate_projected_topology_baseline,
};
