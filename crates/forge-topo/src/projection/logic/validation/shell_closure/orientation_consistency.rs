use std::collections::BTreeSet;

use forge_core::KernelError;
use forge_spec::facade::SpecShellKind;

use crate::projection::data::{ProjectedHalfEdgeId, ProjectedTopology};

use super::super::shared::vf;

pub fn validate_projected_orientation_consistency(
    topology: &ProjectedTopology,
) -> Result<(), KernelError> {
    let mut checked = BTreeSet::new();

    for (index, half_edge) in topology.half_edges().iter().enumerate() {
        let half_edge_id = ProjectedHalfEdgeId::new(index as u32);
        let twin_id = half_edge.radial_next;

        if twin_id == half_edge_id {
            continue;
        }

        let canonical = (
            half_edge_id.raw().min(twin_id.raw()),
            half_edge_id.raw().max(twin_id.raw()),
        );
        if !checked.insert(canonical) {
            continue;
        }

        let twin = topology.half_edge(twin_id);
        if half_edge.face == twin.face {
            continue;
        }

        let shell_id = topology.face(half_edge.face).shell;
        if topology.face(twin.face).shell != shell_id {
            continue;
        }

        if !matches!(topology.shell(shell_id).kind, SpecShellKind::Solid(_)) {
            continue;
        }

        let destination = topology.half_edge(half_edge.next).origin;
        let twin_destination = topology.half_edge(twin.next).origin;
        let has_opposite_orientation =
            twin.origin == destination && twin_destination == half_edge.origin;

        if !has_opposite_orientation {
            return Err(vf(
                "projected_orientation_consistency",
                format!(
                    "Solid shell {} has adjacent halfedges {} ({}->{}) and {} ({}->{}) with matching, not opposite, orientation",
                    shell_id.raw(),
                    half_edge_id.raw(),
                    half_edge.origin.raw(),
                    destination.raw(),
                    twin_id.raw(),
                    twin.origin.raw(),
                    twin_destination.raw()
                ),
            ));
        }
    }

    Ok(())
}
