mod euler_genus;
mod loop_wiring;
mod radial_edge;
mod reference_integrity;
mod shell_closure;
mod shared;
mod vertex_disk;

use forge_core::KernelError;

use crate::projection::data::ProjectedTopology;

pub use loop_wiring::validate_projected_loop_wiring;
pub use radial_edge::validate_projected_radial_edge;
pub use euler_genus::validate_projected_per_component_euler;
pub use reference_integrity::{
    validate_projected_acyclic_containment,
    validate_projected_bidirectional_links,
    validate_projected_face_has_at_least_one_loop,
    validate_projected_hierarchy,
    validate_projected_inner_outer_loop_consistency,
    validate_projected_no_dangling_refs,
    validate_projected_no_orphan_half_edges,
    validate_projected_single_owner_per_loop,
};
pub use shell_closure::{
    validate_projected_broken_boundary, validate_projected_face_adjacency,
    validate_projected_laminar_edges, validate_projected_manifold_edges,
    validate_projected_orientation_consistency,
    validate_projected_shell_closure, validate_projected_shell_consistency,
};
pub use vertex_disk::{
    validate_projected_disk_closure, validate_projected_no_cross_disk_coedges,
    validate_projected_vertex_disk, validate_projected_vertex_disk_partition,
    validate_projected_vertex_outgoing,
};

pub fn validate_projected_topology_baseline(
    topology: &ProjectedTopology,
) -> Result<(), KernelError> {
    validate_projected_loop_wiring(topology)?;
    validate_projected_radial_edge(topology)?;
    reference_integrity::validate_projected_reference_integrity(topology)?;
    vertex_disk::validate_projected_vertex_disk(topology)?;
    shell_closure::validate_projected_shell_closure(topology)?;
    Ok(())
}
