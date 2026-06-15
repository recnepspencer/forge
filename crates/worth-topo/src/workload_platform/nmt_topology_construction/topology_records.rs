use forge_relational::facade::identity::EntityId;

use super::denial::{unsupported_cardinality, NmtTopologyConstructionDenial};
use super::pattern_spec::{
    NmtTopologyPattern, OpenLayerPattern, OpenRadialFanSpec, OpenSheetPatchSpec, OpenWireChainSpec,
};
use crate::brep::topology_graph::{TopologyHalfEdge, TopologyView};
use crate::workload_platform::topology_seed_recipes::topology_record_constructors::{
    base_container, edge, entity, face, half_edge, loop_record, shell, vertex, wire,
    HalfEdgeRecordConstruction,
};

pub(crate) fn build_nmt_topology_view(
    pattern: &NmtTopologyPattern,
) -> Result<TopologyView, NmtTopologyConstructionDenial> {
    match pattern {
        NmtTopologyPattern::OpenWireChain(spec) => open_wire_chain_view(pattern, *spec, 70_000),
        NmtTopologyPattern::OpenSheetPatch(spec) => open_sheet_patch_view(pattern, *spec, 80_000),
        NmtTopologyPattern::OpenRadialFan(spec) => open_radial_fan_view(pattern, *spec, 90_000),
        NmtTopologyPattern::OpenLayerStack(spec) => {
            if !(2..=16).contains(&spec.layer_count()) {
                return Err(unsupported_cardinality(
                    pattern.clone(),
                    format!(
                        "open layer stack topology admits 2 through 16 layers today; requested {} layers",
                        spec.layer_count()
                    ),
                ));
            }
            let mut topology = base_container(100_000, pattern.human_name());
            for layer_index in 0..spec.layer_count() {
                append_layer_pattern(
                    &mut topology,
                    spec.pattern(),
                    100_000 + 10_000 * (layer_index as u64 + 1),
                    layer_index,
                    pattern,
                )?;
            }
            Ok(topology)
        }
    }
}

fn append_layer_pattern(
    topology: &mut TopologyView,
    layer_pattern: &OpenLayerPattern,
    base: u64,
    layer_index: usize,
    pattern: &NmtTopologyPattern,
) -> Result<(), NmtTopologyConstructionDenial> {
    let layer_topology = match layer_pattern {
        OpenLayerPattern::WireChain(spec) => open_wire_chain_view(pattern, *spec, base)?,
        OpenLayerPattern::SheetPatch(spec) => open_sheet_patch_view(pattern, *spec, base)?,
        OpenLayerPattern::RadialFan(spec) => open_radial_fan_view(pattern, *spec, base)?,
    };

    let region_id = topology.regions[0].entity_id;
    for mut shell_record in layer_topology.shells {
        shell_record.label = format!("layer {layer_index} {}", shell_record.label);
        shell_record.region_id = Some(region_id);
        topology.regions[0].shell_ids.push(shell_record.entity_id);
        topology.shells.push(shell_record);
    }
    topology.faces.extend(layer_topology.faces);
    topology.loops.extend(layer_topology.loops);
    topology.wires.extend(layer_topology.wires);
    topology.half_edges.extend(layer_topology.half_edges);
    topology.edges.extend(layer_topology.edges);
    topology.vertices.extend(layer_topology.vertices);
    Ok(())
}

fn open_wire_chain_view(
    pattern: &NmtTopologyPattern,
    spec: OpenWireChainSpec,
    base: u64,
) -> Result<TopologyView, NmtTopologyConstructionDenial> {
    if !(2..=128).contains(&spec.edge_count()) {
        return Err(unsupported_cardinality(
            pattern.clone(),
            format!(
                "open wire chain topology admits 2 through 128 edges today; requested {} edges",
                spec.edge_count()
            ),
        ));
    }

    let mut topology = base_container(base, "open wire chain");
    let wire_id = entity(base + 10);
    let vertex_ids = (0..=spec.edge_count())
        .map(|index| entity(base + 20 + index as u64))
        .collect::<Vec<_>>();
    let edge_ids = (0..spec.edge_count())
        .map(|index| entity(base + 200 + index as u64))
        .collect::<Vec<_>>();
    let half_edge_ids = (0..spec.edge_count())
        .map(|index| entity(base + 400 + index as u64))
        .collect::<Vec<_>>();

    for (index, vertex_id) in vertex_ids.iter().enumerate() {
        topology.vertices.push(vertex(
            format!("open wire chain vertex {index}"),
            *vertex_id,
        ));
    }
    for index in 0..spec.edge_count() {
        topology.edges.push(edge(
            format!("open wire chain edge {index}"),
            edge_ids[index],
        ));
        topology
            .half_edges
            .push(half_edge(HalfEdgeRecordConstruction {
                label: format!("open wire chain half-edge {index}"),
                id: half_edge_ids[index],
                loop_id: None,
                wire_id: Some(wire_id),
                next_id: Some(half_edge_ids[(index + 1) % half_edge_ids.len()]),
                prev_id: Some(
                    half_edge_ids[(index + half_edge_ids.len() - 1) % half_edge_ids.len()],
                ),
                radial_next_id: Some(half_edge_ids[index]),
                edge_id: edge_ids[index],
                origin_id: vertex_ids[index],
                target_id: vertex_ids[index + 1],
                face_id: None,
            }));
    }
    topology
        .wires
        .push(wire("open wire chain wire", wire_id, half_edge_ids));
    Ok(topology)
}

fn open_sheet_patch_view(
    pattern: &NmtTopologyPattern,
    spec: OpenSheetPatchSpec,
    base: u64,
) -> Result<TopologyView, NmtTopologyConstructionDenial> {
    if !(1..=64).contains(&spec.strip_count()) {
        return Err(unsupported_cardinality(
            pattern.clone(),
            format!(
                "open sheet patch topology admits 1 through 64 strips today; requested {} strips",
                spec.strip_count()
            ),
        ));
    }

    let mut topology = base_container(base, "open sheet patch");
    let region_id = topology.regions[0].entity_id;
    let shell_id = entity(base + 10);
    topology
        .shells
        .push(shell("open sheet patch shell", shell_id, region_id));
    topology.regions[0].shell_ids.push(shell_id);

    let mut half_edges_by_vertical_edge: Vec<Vec<EntityId>> =
        vec![Vec::new(); spec.strip_count().saturating_sub(1)];
    for strip_index in 0..spec.strip_count() {
        append_sheet_strip(
            &mut topology,
            base,
            shell_id,
            strip_index,
            &mut half_edges_by_vertical_edge,
        );
    }
    pair_shared_sheet_radials(&mut topology, &half_edges_by_vertical_edge);
    Ok(topology)
}

fn append_sheet_strip(
    topology: &mut TopologyView,
    base: u64,
    shell_id: EntityId,
    strip_index: usize,
    half_edges_by_vertical_edge: &mut [Vec<EntityId>],
) {
    let face_id = entity(base + 1_000 + strip_index as u64);
    let loop_id = entity(base + 2_000 + strip_index as u64);
    let half_edge_ids = (0..4)
        .map(|offset| entity(base + 3_000 + strip_index as u64 * 10 + offset))
        .collect::<Vec<_>>();
    let edge_ids = (0..4)
        .map(|offset| entity(base + 4_000 + strip_index as u64 * 10 + offset))
        .collect::<Vec<_>>();
    let vertex_ids = (0..4)
        .map(|offset| entity(base + 5_000 + strip_index as u64 * 10 + offset))
        .collect::<Vec<_>>();

    for (offset, vertex_id) in vertex_ids.iter().enumerate() {
        topology.vertices.push(vertex(
            format!("open sheet patch strip {strip_index} vertex {offset}"),
            *vertex_id,
        ));
    }
    for (offset, edge_id) in edge_ids.iter().enumerate() {
        topology.edges.push(edge(
            format!("open sheet patch strip {strip_index} edge {offset}"),
            *edge_id,
        ));
    }

    let edges = [
        (vertex_ids[0], vertex_ids[1]),
        (vertex_ids[1], vertex_ids[2]),
        (vertex_ids[2], vertex_ids[3]),
        (vertex_ids[3], vertex_ids[0]),
    ];
    for offset in 0..4 {
        topology
            .half_edges
            .push(half_edge(HalfEdgeRecordConstruction {
                label: format!("open sheet patch strip {strip_index} half-edge {offset}"),
                id: half_edge_ids[offset],
                loop_id: Some(loop_id),
                wire_id: None,
                next_id: Some(half_edge_ids[(offset + 1) % 4]),
                prev_id: Some(half_edge_ids[(offset + 3) % 4]),
                radial_next_id: Some(half_edge_ids[offset]),
                edge_id: edge_ids[offset],
                origin_id: edges[offset].0,
                target_id: edges[offset].1,
                face_id: Some(face_id),
            }));
    }
    if strip_index > 0 {
        half_edges_by_vertical_edge[strip_index - 1].push(half_edge_ids[3]);
    }
    if strip_index + 1 < half_edges_by_vertical_edge.len() + 1 {
        half_edges_by_vertical_edge[strip_index].push(half_edge_ids[1]);
    }
    topology.faces.push(face(
        format!("open sheet patch strip {strip_index} face"),
        face_id,
        Some(shell_id),
        loop_id,
        half_edge_ids.clone(),
    ));
    topology.shells[0].face_ids.push(face_id);
    topology.loops.push(loop_record(
        format!("open sheet patch strip {strip_index} loop"),
        loop_id,
        face_id,
        half_edge_ids,
    ));
}

fn pair_shared_sheet_radials(topology: &mut TopologyView, shared_edges: &[Vec<EntityId>]) {
    for pair in shared_edges {
        if pair.len() == 2 {
            set_radial_next(topology, pair[0], pair[1]);
            set_radial_next(topology, pair[1], pair[0]);
            let shared_edge_id = topology
                .half_edges
                .iter()
                .find(|half_edge| half_edge.entity_id == pair[0])
                .and_then(|half_edge| half_edge.edge_id)
                .expect("sheet patch shared half-edge has edge");
            set_edge_id(topology, pair[1], shared_edge_id);
        }
    }
}

fn open_radial_fan_view(
    pattern: &NmtTopologyPattern,
    spec: OpenRadialFanSpec,
    base: u64,
) -> Result<TopologyView, NmtTopologyConstructionDenial> {
    if !(3..=128).contains(&spec.incident_face_count()) {
        return Err(unsupported_cardinality(
            pattern.clone(),
            format!(
                "open radial fan topology admits 3 through 128 incident faces today; requested {} faces",
                spec.incident_face_count()
            ),
        ));
    }

    let mut topology = base_container(base, "open radial fan");
    let region_id = topology.regions[0].entity_id;
    let shell_id = entity(base + 10);
    let shared_edge_id = entity(base + 20);
    let shared_start = entity(base + 30);
    let shared_end = entity(base + 31);
    topology
        .shells
        .push(shell("open radial fan shell", shell_id, region_id));
    topology.regions[0].shell_ids.push(shell_id);
    topology
        .edges
        .push(edge("open radial fan shared edge", shared_edge_id));
    topology
        .vertices
        .push(vertex("open radial fan shared start", shared_start));
    topology
        .vertices
        .push(vertex("open radial fan shared end", shared_end));

    let mut shared_half_edge_ids = Vec::new();
    for face_index in 0..spec.incident_face_count() {
        append_radial_fan_face(
            &mut topology,
            base,
            shell_id,
            shared_edge_id,
            shared_start,
            shared_end,
            face_index,
            &mut shared_half_edge_ids,
        );
    }
    for index in 0..shared_half_edge_ids.len() {
        set_radial_next(
            &mut topology,
            shared_half_edge_ids[index],
            shared_half_edge_ids[(index + 1) % shared_half_edge_ids.len()],
        );
    }
    Ok(topology)
}

fn append_radial_fan_face(
    topology: &mut TopologyView,
    base: u64,
    shell_id: EntityId,
    shared_edge_id: EntityId,
    shared_start: EntityId,
    shared_end: EntityId,
    face_index: usize,
    shared_half_edge_ids: &mut Vec<EntityId>,
) {
    let face_id = entity(base + 1_000 + face_index as u64);
    let loop_id = entity(base + 2_000 + face_index as u64);
    let outer_vertex_id = entity(base + 3_000 + face_index as u64);
    let boundary_edge_a = entity(base + 4_000 + face_index as u64 * 2);
    let boundary_edge_b = entity(base + 4_001 + face_index as u64 * 2);
    let half_edge_ids = [
        entity(base + 5_000 + face_index as u64 * 3),
        entity(base + 5_001 + face_index as u64 * 3),
        entity(base + 5_002 + face_index as u64 * 3),
    ];

    topology.vertices.push(vertex(
        format!("open radial fan outer vertex {face_index}"),
        outer_vertex_id,
    ));
    topology.edges.push(edge(
        format!("open radial fan boundary edge {face_index} a"),
        boundary_edge_a,
    ));
    topology.edges.push(edge(
        format!("open radial fan boundary edge {face_index} b"),
        boundary_edge_b,
    ));
    let edge_ids = [shared_edge_id, boundary_edge_a, boundary_edge_b];
    let vertex_pairs = [
        (shared_start, shared_end),
        (shared_end, outer_vertex_id),
        (outer_vertex_id, shared_start),
    ];

    for offset in 0..3 {
        topology
            .half_edges
            .push(half_edge(HalfEdgeRecordConstruction {
                label: format!("open radial fan face {face_index} half-edge {offset}"),
                id: half_edge_ids[offset],
                loop_id: Some(loop_id),
                wire_id: None,
                next_id: Some(half_edge_ids[(offset + 1) % 3]),
                prev_id: Some(half_edge_ids[(offset + 2) % 3]),
                radial_next_id: Some(half_edge_ids[offset]),
                edge_id: edge_ids[offset],
                origin_id: vertex_pairs[offset].0,
                target_id: vertex_pairs[offset].1,
                face_id: Some(face_id),
            }));
    }
    shared_half_edge_ids.push(half_edge_ids[0]);
    topology.faces.push(face(
        format!("open radial fan face {face_index}"),
        face_id,
        Some(shell_id),
        loop_id,
        half_edge_ids.to_vec(),
    ));
    topology.shells[0].face_ids.push(face_id);
    topology.loops.push(loop_record(
        format!("open radial fan loop {face_index}"),
        loop_id,
        face_id,
        half_edge_ids.to_vec(),
    ));
}

fn set_radial_next(topology: &mut TopologyView, half_edge_id: EntityId, radial_next_id: EntityId) {
    if let Some(half_edge) = find_half_edge_mut(topology, half_edge_id) {
        half_edge.radial_next_half_edge_id = Some(radial_next_id);
    }
}

fn set_edge_id(topology: &mut TopologyView, half_edge_id: EntityId, edge_id: EntityId) {
    if let Some(half_edge) = find_half_edge_mut(topology, half_edge_id) {
        half_edge.edge_id = Some(edge_id);
    }
}

fn find_half_edge_mut(
    topology: &mut TopologyView,
    half_edge_id: EntityId,
) -> Option<&mut TopologyHalfEdge> {
    topology
        .half_edges
        .iter_mut()
        .find(|half_edge| half_edge.entity_id == half_edge_id)
}
