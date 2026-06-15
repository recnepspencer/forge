use std::collections::BTreeMap;

use forge_relational::facade::identity::EntityId;

use super::topology_record_constructors::{
    base_container, edge, entity, face, half_edge, loop_record, shell, vertex,
    HalfEdgeRecordConstruction,
};
use super::{seed_parameter_denial, TopologySeedRecipeDenial};
use crate::brep::topology_graph::{TopologyHalfEdge, TopologyView};
use crate::workload_platform::topology_seed::TopologySeedCleanFailReasonCode;

pub(crate) fn cube_topology_view() -> TopologyView {
    closed_polyhedron(
        10_000,
        "cube",
        8,
        &[
            &[0, 1, 2, 3],
            &[4, 7, 6, 5],
            &[0, 4, 5, 1],
            &[1, 5, 6, 2],
            &[2, 6, 7, 3],
            &[3, 7, 4, 0],
        ],
    )
}

pub(crate) fn tetrahedron_topology_view() -> TopologyView {
    closed_polyhedron(
        20_000,
        "tetrahedron",
        4,
        &[&[0, 2, 1], &[0, 1, 3], &[1, 2, 3], &[2, 0, 3]],
    )
}

pub(crate) fn multi_face_shell_topology_view(
    face_count: usize,
) -> Result<TopologyView, TopologySeedRecipeDenial> {
    match face_count {
        4 => Ok(tetrahedron_topology_view()),
        6 => Ok(closed_bipyramid(30_000, "six face shell", 3)),
        8..=64 if face_count % 2 == 0 => {
            Ok(closed_bipyramid(30_000, "even face shell", face_count / 2))
        }
        5..=63 => Ok(closed_prism(
            40_000,
            "odd face shell",
            face_count.saturating_sub(2),
        )),
        _ => Err(seed_parameter_denial(
            TopologySeedCleanFailReasonCode::MultiFaceShellFaceCountOutOfRange,
            "multi-face shell seeds admit 4..=64 requested faces",
        )),
    }
}

fn closed_bipyramid(base: u64, label: &str, ring_count: usize) -> TopologyView {
    let top = ring_count;
    let bottom = ring_count + 1;
    let mut faces = Vec::with_capacity(ring_count * 2);
    for index in 0..ring_count {
        let next = (index + 1) % ring_count;
        faces.push(vec![top, index, next]);
        faces.push(vec![bottom, next, index]);
    }
    let face_refs = faces.iter().map(Vec::as_slice).collect::<Vec<_>>();
    closed_polyhedron(base, label, ring_count + 2, &face_refs)
}

fn closed_prism(base: u64, label: &str, side_count: usize) -> TopologyView {
    let top_offset = side_count;
    let mut faces = Vec::with_capacity(side_count + 2);
    faces.push((0..side_count).collect::<Vec<_>>());
    faces.push(
        (0..side_count)
            .rev()
            .map(|index| top_offset + index)
            .collect(),
    );
    for index in 0..side_count {
        let next = (index + 1) % side_count;
        faces.push(vec![index, top_offset + index, top_offset + next, next]);
    }
    let face_refs = faces.iter().map(Vec::as_slice).collect::<Vec<_>>();
    closed_polyhedron(base, label, side_count * 2, &face_refs)
}

fn closed_polyhedron(
    base: u64,
    label: &str,
    vertex_count: usize,
    face_cycles: &[&[usize]],
) -> TopologyView {
    let mut topology = base_container(base, label);
    let region_id = topology.regions[0].entity_id;
    let shell_id = entity(base + 4);
    topology
        .shells
        .push(shell(format!("{label} shell"), shell_id, region_id));
    topology.regions[0].shell_ids.push(shell_id);

    let vertex_ids = (0..vertex_count)
        .map(|index| entity(base + 10 + index as u64))
        .collect::<Vec<_>>();
    topology.vertices.extend(
        vertex_ids
            .iter()
            .enumerate()
            .map(|(index, id)| vertex(format!("{label} vertex {index}"), *id)),
    );

    let mut edge_map: BTreeMap<(usize, usize), (EntityId, Vec<EntityId>)> = BTreeMap::new();
    let mut half_edges = Vec::new();
    let mut next_half_edge_slot = base + 300;
    let mut next_edge_slot = base + 600;

    for (face_index, cycle) in face_cycles.iter().enumerate() {
        let face_id = entity(base + 100 + face_index as u64);
        let loop_id = entity(base + 200 + face_index as u64);
        let mut face_half_edge_ids = Vec::with_capacity(cycle.len());

        for local_index in 0..cycle.len() {
            let origin = cycle[local_index];
            let target = cycle[(local_index + 1) % cycle.len()];
            let key = ordered_edge_key(origin, target);
            let entry = edge_map.entry(key).or_insert_with(|| {
                let edge_id = entity(next_edge_slot);
                next_edge_slot += 1;
                (edge_id, Vec::new())
            });
            let half_edge_id = entity(next_half_edge_slot);
            next_half_edge_slot += 1;
            entry.1.push(half_edge_id);
            face_half_edge_ids.push(half_edge_id);
            half_edges.push((
                half_edge_id,
                loop_id,
                face_id,
                entry.0,
                vertex_ids[origin],
                vertex_ids[target],
                format!("{label} face {face_index} half-edge {local_index}"),
            ));
        }

        topology.faces.push(face(
            format!("{label} face {face_index}"),
            face_id,
            Some(shell_id),
            loop_id,
            face_half_edge_ids.clone(),
        ));
        topology.loops.push(loop_record(
            format!("{label} loop {face_index}"),
            loop_id,
            face_id,
            face_half_edge_ids,
        ));
        topology.shells[0].face_ids.push(face_id);
    }

    let radial = radial_pairs(&edge_map);
    for (index, (id, loop_id, face_id, edge_id, origin_id, target_id, label)) in
        half_edges.iter().enumerate()
    {
        let loop_half_edges =
            &topology.loops[index_for_loop(&topology.loops, *loop_id)].half_edge_ids;
        topology.half_edges.push(half_edge_with_cycle(
            index,
            *id,
            *loop_id,
            *face_id,
            *edge_id,
            *origin_id,
            *target_id,
            label,
            loop_half_edges,
            &radial,
        ));
    }

    topology.edges.extend(
        edge_map
            .values()
            .enumerate()
            .map(|(index, (id, _))| edge(format!("{label} edge {index}"), *id)),
    );
    topology
}

fn half_edge_with_cycle(
    index: usize,
    id: EntityId,
    loop_id: EntityId,
    face_id: EntityId,
    edge_id: EntityId,
    origin_id: EntityId,
    target_id: EntityId,
    label: &str,
    loop_half_edges: &[EntityId],
    radial: &BTreeMap<EntityId, EntityId>,
) -> TopologyHalfEdge {
    let local_index = loop_half_edges
        .iter()
        .position(|candidate| *candidate == id)
        .unwrap_or(index);
    let next_id = loop_half_edges[(local_index + 1) % loop_half_edges.len()];
    let prev_id =
        loop_half_edges[(local_index + loop_half_edges.len() - 1) % loop_half_edges.len()];
    half_edge(HalfEdgeRecordConstruction {
        label: label.to_string(),
        id,
        loop_id: Some(loop_id),
        wire_id: None,
        next_id: Some(next_id),
        prev_id: Some(prev_id),
        radial_next_id: radial.get(&id).copied(),
        edge_id,
        origin_id,
        target_id,
        face_id: Some(face_id),
    })
}

fn radial_pairs(
    edge_map: &BTreeMap<(usize, usize), (EntityId, Vec<EntityId>)>,
) -> BTreeMap<EntityId, EntityId> {
    let mut radial = BTreeMap::new();
    for (_, half_edge_ids) in edge_map.values() {
        if half_edge_ids.len() == 2 {
            radial.insert(half_edge_ids[0], half_edge_ids[1]);
            radial.insert(half_edge_ids[1], half_edge_ids[0]);
        }
    }
    radial
}

fn ordered_edge_key(a: usize, b: usize) -> (usize, usize) {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

fn index_for_loop(loops: &[crate::brep::topology_graph::TopologyLoop], loop_id: EntityId) -> usize {
    loops
        .iter()
        .position(|loop_record| loop_record.entity_id == loop_id)
        .expect("closed polyhedron builder only references loops it just created")
}
