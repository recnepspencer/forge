mod builder;
mod queries;
mod signature;
mod validation;

pub use builder::ProjectionBuilder;
pub use queries::ProjectedTopologyQueries;
pub use signature::compute_projected_topology_hash;
pub use validation::{
    validate_projected_acyclic_containment, validate_projected_bidirectional_links,
    validate_projected_broken_boundary, validate_projected_disk_closure,
    validate_projected_edge_endpoints_match_loop_vertices, validate_projected_face_adjacency,
    validate_projected_face_has_at_least_one_loop,
    validate_projected_face_loop_membership_complete, validate_projected_hierarchy,
    validate_projected_inner_outer_loop_consistency, validate_projected_laminar_edges,
    validate_projected_loop_minimum_cardinality, validate_projected_loop_wiring,
    validate_projected_loops, validate_projected_manifold_edges,
    validate_projected_no_broken_radial_splices, validate_projected_no_cross_disk_coedges,
    validate_projected_no_dangling_refs, validate_projected_no_duplicate_coedges_in_loop,
    validate_projected_no_orphan_half_edges, validate_projected_orientation_consistency,
    validate_projected_per_component_euler, validate_projected_prev_consistency,
    validate_projected_radial_cycle_uniqueness, validate_projected_radial_edge,
    validate_projected_radial_edge_consistency, validate_projected_radial_neighbor_consistency,
    validate_projected_radial_rings, validate_projected_shell_closure,
    validate_projected_shell_consistency, validate_projected_single_owner_per_loop,
    validate_projected_topology_baseline, validate_projected_topology_structural,
    validate_projected_vertex_continuity, validate_projected_vertex_disk,
    validate_projected_vertex_disk_partition, validate_projected_vertex_outgoing,
};
