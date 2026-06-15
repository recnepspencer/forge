use super::{closed_solid_recipes, sheet_recipes, wire_recipes};
use crate::brep::topology_graph::TopologyView;

pub(crate) fn self_intersecting_loop_topology_view() -> TopologyView {
    sheet_recipes::single_face_loop(4)
        .expect("self-intersecting loop uses an admitted topology loop before spatial denial")
}

pub(crate) fn non_manifold_wire_topology_view() -> TopologyView {
    let mut topology = wire_recipes::wire_chain_view(80_000, "non manifold wire", 3);
    let extra = closed_solid_recipes::tetrahedron_topology_view();
    topology.faces.extend(extra.faces.into_iter().take(1));
    topology
}

pub(crate) fn thin_wall_local_basis_topology_view() -> TopologyView {
    sheet_recipes::single_face_loop(64)
        .expect("thin wall local-basis seed uses an admitted topology loop before spatial denial")
}

pub(crate) fn orientation_inconsistency_topology_view() -> TopologyView {
    sheet_recipes::single_face_loop(5).expect(
        "orientation-inconsistency seed uses an admitted topology loop before spatial denial",
    )
}
