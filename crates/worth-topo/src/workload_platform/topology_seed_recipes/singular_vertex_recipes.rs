use std::collections::BTreeMap;

use forge_relational::facade::identity::EntityId;

use super::topology_record_constructors::{
    base_container, edge, entity, face, half_edge, loop_record, shell, vertex,
    HalfEdgeRecordConstruction,
};
use crate::brep::topology_graph::{TopologyHalfEdge, TopologyView};
use crate::workload_platform::topology_seed::TopologySeedNeighborhoodReceipt;

pub(crate) fn high_valence_vertex_topology_view_with_valence(
    valence: usize,
) -> (TopologyView, TopologySeedNeighborhoodReceipt) {
    let mut topology = high_valence_planar_fan_container();
    let center_id = entity(70_010);
    let outer_vertex_ids =
        insert_high_valence_planar_fan_vertices(&mut topology, center_id, valence);
    let fan_faces = build_high_valence_planar_fan_faces(center_id, &outer_vertex_ids, valence);

    insert_high_valence_planar_fan_faces(&mut topology, &fan_faces);
    insert_high_valence_planar_fan_half_edges(&mut topology, &fan_faces);
    insert_high_valence_planar_fan_edges(&mut topology, &fan_faces.edge_map);

    (
        topology,
        TopologySeedNeighborhoodReceipt::new(center_id, fan_faces.incident_half_edges),
    )
}

fn high_valence_planar_fan_container() -> TopologyView {
    let mut topology = base_container(70_000, "high valence planar fan");
    let region_id = topology.regions[0].entity_id;
    let shell_id = high_valence_planar_fan_shell_id();
    topology
        .shells
        .push(shell("high valence planar fan shell", shell_id, region_id));
    topology.regions[0].shell_ids.push(shell_id);
    topology
}

fn insert_high_valence_planar_fan_vertices(
    topology: &mut TopologyView,
    center_id: EntityId,
    valence: usize,
) -> Vec<EntityId> {
    let outer_vertex_ids = (0..valence)
        .map(|index| entity(70_020 + index as u64))
        .collect::<Vec<_>>();
    topology
        .vertices
        .push(vertex("high valence singular center", center_id));
    for (index, outer_id) in outer_vertex_ids.iter().enumerate() {
        topology.vertices.push(vertex(
            format!("high valence planar fan outer {index}"),
            *outer_id,
        ));
    }
    outer_vertex_ids
}

fn build_high_valence_planar_fan_faces(
    center_id: EntityId,
    outer_vertex_ids: &[EntityId],
    valence: usize,
) -> HighValencePlanarFanFaces {
    let mut next_edge_slot = 70_100;
    let mut next_half_edge_slot = 70_300;
    let mut edge_map: BTreeMap<(usize, usize), (EntityId, Vec<EntityId>)> = BTreeMap::new();
    let mut half_edge_inputs = Vec::new();
    let mut incident_half_edges = Vec::with_capacity(valence);
    let mut face_loop_half_edges = Vec::new();

    for face_index in 0..valence {
        let face_id = entity(70_200 + face_index as u64);
        let loop_id = entity(70_250 + face_index as u64);
        let face_cycle = [0, face_index + 1, ((face_index + 1) % valence) + 1];
        let mut face_half_edge_ids = Vec::with_capacity(face_cycle.len());

        for local_index in 0..face_cycle.len() {
            let origin = face_cycle[local_index];
            let target = face_cycle[(local_index + 1) % face_cycle.len()];
            let edge_entry = edge_map
                .entry(ordered_edge_key(origin, target))
                .or_insert_with(|| {
                    let edge_id = entity(next_edge_slot);
                    next_edge_slot += 1;
                    (edge_id, Vec::new())
                });
            let half_edge_id = entity(next_half_edge_slot);
            next_half_edge_slot += 1;
            edge_entry.1.push(half_edge_id);
            face_half_edge_ids.push(half_edge_id);

            let origin_id = fan_vertex_id(center_id, outer_vertex_ids, origin);
            let target_id = fan_vertex_id(center_id, outer_vertex_ids, target);
            if origin_id == center_id {
                incident_half_edges.push(half_edge_id);
            }
            half_edge_inputs.push(HighValencePlanarFanHalfEdgeInput {
                id: half_edge_id,
                loop_id,
                face_id,
                edge_id: edge_entry.0,
                origin_id,
                target_id,
                label: format!("high valence planar fan face {face_index} half-edge {local_index}"),
            });
        }
        face_loop_half_edges.push(HighValencePlanarFanLoop {
            face_id,
            loop_id,
            half_edge_ids: face_half_edge_ids,
        });
    }

    HighValencePlanarFanFaces {
        face_loop_half_edges,
        half_edge_inputs,
        edge_map,
        incident_half_edges,
    }
}

fn insert_high_valence_planar_fan_faces(
    topology: &mut TopologyView,
    fan_faces: &HighValencePlanarFanFaces,
) {
    let shell_id = high_valence_planar_fan_shell_id();
    for (face_index, face_loop) in fan_faces.face_loop_half_edges.iter().enumerate() {
        topology.faces.push(face(
            format!("high valence planar fan face {face_index}"),
            face_loop.face_id,
            Some(shell_id),
            face_loop.loop_id,
            face_loop.half_edge_ids.clone(),
        ));
        topology.loops.push(loop_record(
            format!("high valence planar fan loop {face_index}"),
            face_loop.loop_id,
            face_loop.face_id,
            face_loop.half_edge_ids.clone(),
        ));
        topology.shells[0].face_ids.push(face_loop.face_id);
    }
}

fn insert_high_valence_planar_fan_half_edges(
    topology: &mut TopologyView,
    fan_faces: &HighValencePlanarFanFaces,
) {
    let radial_pairs = radial_pairs(&fan_faces.edge_map);
    for (index, input) in fan_faces.half_edge_inputs.iter().enumerate() {
        let loop_half_edges =
            &topology.loops[index_for_loop(topology, input.loop_id)].half_edge_ids;
        topology.half_edges.push(half_edge_with_cycle(
            index,
            input,
            loop_half_edges,
            &radial_pairs,
        ));
    }
}

fn insert_high_valence_planar_fan_edges(
    topology: &mut TopologyView,
    edge_map: &BTreeMap<(usize, usize), (EntityId, Vec<EntityId>)>,
) {
    topology.edges.extend(
        edge_map
            .values()
            .enumerate()
            .map(|(index, (id, _))| edge(format!("high valence planar fan edge {index}"), *id)),
    );
}

fn half_edge_with_cycle(
    index: usize,
    input: &HighValencePlanarFanHalfEdgeInput,
    loop_half_edges: &[EntityId],
    radial_pairs: &BTreeMap<EntityId, EntityId>,
) -> TopologyHalfEdge {
    let local_index = loop_half_edges
        .iter()
        .position(|candidate| *candidate == input.id)
        .unwrap_or(index);
    half_edge(HalfEdgeRecordConstruction {
        label: input.label.clone(),
        id: input.id,
        loop_id: Some(input.loop_id),
        wire_id: None,
        next_id: Some(loop_half_edges[(local_index + 1) % loop_half_edges.len()]),
        prev_id: Some(
            loop_half_edges[(local_index + loop_half_edges.len() - 1) % loop_half_edges.len()],
        ),
        radial_next_id: radial_pairs.get(&input.id).copied(),
        edge_id: input.edge_id,
        origin_id: input.origin_id,
        target_id: input.target_id,
        face_id: Some(input.face_id),
    })
}

fn high_valence_planar_fan_shell_id() -> EntityId {
    entity(70_004)
}

fn fan_vertex_id(center_id: EntityId, outer_vertex_ids: &[EntityId], fan_index: usize) -> EntityId {
    if fan_index == 0 {
        center_id
    } else {
        outer_vertex_ids[fan_index - 1]
    }
}

fn ordered_edge_key(a: usize, b: usize) -> (usize, usize) {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

fn radial_pairs(
    edge_map: &BTreeMap<(usize, usize), (EntityId, Vec<EntityId>)>,
) -> BTreeMap<EntityId, EntityId> {
    let mut radial_pairs = BTreeMap::new();
    for (_, half_edge_ids) in edge_map.values() {
        match half_edge_ids.as_slice() {
            [boundary] => {
                radial_pairs.insert(*boundary, *boundary);
            }
            [first, second] => {
                radial_pairs.insert(*first, *second);
                radial_pairs.insert(*second, *first);
            }
            _ => {}
        }
    }
    radial_pairs
}

fn index_for_loop(topology: &TopologyView, loop_id: EntityId) -> usize {
    topology
        .loops
        .iter()
        .position(|loop_record| loop_record.entity_id == loop_id)
        .expect("high-valence fan builder only references loops it just created")
}

struct HighValencePlanarFanFaces {
    face_loop_half_edges: Vec<HighValencePlanarFanLoop>,
    half_edge_inputs: Vec<HighValencePlanarFanHalfEdgeInput>,
    edge_map: BTreeMap<(usize, usize), (EntityId, Vec<EntityId>)>,
    incident_half_edges: Vec<EntityId>,
}

struct HighValencePlanarFanLoop {
    face_id: EntityId,
    loop_id: EntityId,
    half_edge_ids: Vec<EntityId>,
}

struct HighValencePlanarFanHalfEdgeInput {
    id: EntityId,
    loop_id: EntityId,
    face_id: EntityId,
    edge_id: EntityId,
    origin_id: EntityId,
    target_id: EntityId,
    label: String,
}
