use super::topology_record_constructors::{
    base_container, edge, entity, face, half_edge, loop_record, shell, vertex,
    HalfEdgeRecordConstruction,
};
use super::{seed_parameter_denial, TopologySeedRecipeDenial};
use crate::brep::topology_graph::TopologyView;
use crate::workload_platform::topology_seed::TopologySeedCleanFailReasonCode;

pub(crate) fn single_face_loop(
    edge_count: usize,
) -> Result<TopologyView, TopologySeedRecipeDenial> {
    if !(3..=64).contains(&edge_count) {
        return Err(seed_parameter_denial(
            TopologySeedCleanFailReasonCode::SingleFaceLoopEdgeCountOutOfRange,
            "single-face loop seeds admit 3..=64 boundary edges",
        ));
    }
    Ok(sheet_loop_view(40_000, "single face loop", edge_count))
}

fn sheet_loop_view(base: u64, label: &str, edge_count: usize) -> TopologyView {
    let mut topology = base_container(base, label);
    let region_id = topology.regions[0].entity_id;
    let shell_id = entity(base + 4);
    let face_id = entity(base + 5);
    let loop_id = entity(base + 6);
    topology
        .shells
        .push(shell(format!("{label} shell"), shell_id, region_id));
    topology.regions[0].shell_ids.push(shell_id);
    topology.shells[0].face_ids.push(face_id);

    let vertex_ids = (0..edge_count)
        .map(|index| entity(base + 10 + index as u64))
        .collect::<Vec<_>>();
    let edge_ids = (0..edge_count)
        .map(|index| entity(base + 100 + index as u64))
        .collect::<Vec<_>>();
    let half_edge_ids = (0..edge_count)
        .map(|index| entity(base + 200 + index as u64))
        .collect::<Vec<_>>();

    for index in 0..edge_count {
        topology
            .vertices
            .push(vertex(format!("{label} vertex {index}"), vertex_ids[index]));
        topology
            .edges
            .push(edge(format!("{label} edge {index}"), edge_ids[index]));
        topology
            .half_edges
            .push(half_edge(HalfEdgeRecordConstruction {
                label: format!("{label} half-edge {index}"),
                id: half_edge_ids[index],
                loop_id: Some(loop_id),
                wire_id: None,
                next_id: Some(half_edge_ids[(index + 1) % edge_count]),
                prev_id: Some(half_edge_ids[(index + edge_count - 1) % edge_count]),
                radial_next_id: Some(half_edge_ids[index]),
                edge_id: edge_ids[index],
                origin_id: vertex_ids[index],
                target_id: vertex_ids[(index + 1) % edge_count],
                face_id: Some(face_id),
            }));
    }

    topology.faces.push(face(
        format!("{label} face"),
        face_id,
        Some(shell_id),
        loop_id,
        half_edge_ids.clone(),
    ));
    topology.loops.push(loop_record(
        format!("{label} loop"),
        loop_id,
        face_id,
        half_edge_ids,
    ));
    topology
}
