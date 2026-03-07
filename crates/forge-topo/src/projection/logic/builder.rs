use std::collections::HashMap;

use forge_spec::facade::{RelationKind, SpecNodeId, SpecNodeKind, SpecState};

use crate::projection::data::{
    ProjectedBodyData, ProjectedBodyId, ProjectedEdgeData, ProjectedEdgeId, ProjectedFaceData,
    ProjectedFaceId, ProjectedHalfEdgeData, ProjectedHalfEdgeId, ProjectedLoopData,
    ProjectedLoopId, ProjectedLumpData, ProjectedLumpId, ProjectedRegionData, ProjectedRegionId,
    ProjectedShellData, ProjectedShellId, ProjectedTopology, ProjectedTopologyError,
    ProjectedVertexData, ProjectedVertexId,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct ProjectionBuilder;

impl ProjectionBuilder {
    pub fn build(state: &SpecState) -> Result<ProjectedTopology, ProjectedTopologyError> {
        let graph = state.graph();

        let body_map = collect_ids(graph, SpecNodeKind::Body)
            .into_iter()
            .enumerate()
            .map(|(idx, id)| (id, ProjectedBodyId::new(idx as u32)))
            .collect::<HashMap<_, _>>();
        let lump_map = collect_ids(graph, SpecNodeKind::Lump)
            .into_iter()
            .enumerate()
            .map(|(idx, id)| (id, ProjectedLumpId::new(idx as u32)))
            .collect::<HashMap<_, _>>();
        let region_map = collect_ids(graph, SpecNodeKind::Region)
            .into_iter()
            .enumerate()
            .map(|(idx, id)| (id, ProjectedRegionId::new(idx as u32)))
            .collect::<HashMap<_, _>>();
        let shell_map = collect_ids(graph, SpecNodeKind::Shell)
            .into_iter()
            .enumerate()
            .map(|(idx, id)| (id, ProjectedShellId::new(idx as u32)))
            .collect::<HashMap<_, _>>();
        let face_map = collect_ids(graph, SpecNodeKind::Face)
            .into_iter()
            .enumerate()
            .map(|(idx, id)| (id, ProjectedFaceId::new(idx as u32)))
            .collect::<HashMap<_, _>>();
        let loop_map = collect_ids(graph, SpecNodeKind::Loop)
            .into_iter()
            .enumerate()
            .map(|(idx, id)| (id, ProjectedLoopId::new(idx as u32)))
            .collect::<HashMap<_, _>>();
        let half_edge_map = collect_ids(graph, SpecNodeKind::HalfEdge)
            .into_iter()
            .enumerate()
            .map(|(idx, id)| (id, ProjectedHalfEdgeId::new(idx as u32)))
            .collect::<HashMap<_, _>>();
        let edge_map = collect_ids(graph, SpecNodeKind::Edge)
            .into_iter()
            .enumerate()
            .map(|(idx, id)| (id, ProjectedEdgeId::new(idx as u32)))
            .collect::<HashMap<_, _>>();
        let vertex_map = collect_ids(graph, SpecNodeKind::Vertex)
            .into_iter()
            .enumerate()
            .map(|(idx, id)| (id, ProjectedVertexId::new(idx as u32)))
            .collect::<HashMap<_, _>>();

        let mut topology = ProjectedTopology::default();

        for spec_id in collect_ids(graph, SpecNodeKind::Body) {
            topology.bodies.push(ProjectedBodyData {
                spec_id,
                lumps: outgoing_targets(graph, spec_id, RelationKind::BodyOwnsLump)
                    .into_iter()
                    .map(|id| lookup(&lump_map, id, "lump"))
                    .collect::<Result<_, _>>()?,
            });
        }

        for spec_id in collect_ids(graph, SpecNodeKind::Lump) {
            let body = incoming_single_required(graph, spec_id, RelationKind::BodyOwnsLump)?;
            topology.lumps.push(ProjectedLumpData {
                spec_id,
                body: lookup(&body_map, body, "body")?,
                regions: outgoing_targets(graph, spec_id, RelationKind::LumpOwnsRegion)
                    .into_iter()
                    .map(|id| lookup(&region_map, id, "region"))
                    .collect::<Result<_, _>>()?,
            });
        }

        for spec_id in collect_ids(graph, SpecNodeKind::Region) {
            let lump = incoming_single_required(graph, spec_id, RelationKind::LumpOwnsRegion)?;
            topology.regions.push(ProjectedRegionData {
                spec_id,
                lump: lookup(&lump_map, lump, "lump")?,
                shells: outgoing_targets(graph, spec_id, RelationKind::RegionOwnsShell)
                    .into_iter()
                    .map(|id| lookup(&shell_map, id, "shell"))
                    .collect::<Result<_, _>>()?,
            });
        }

        for spec_id in collect_ids(graph, SpecNodeKind::Shell) {
            let region = incoming_single_required(graph, spec_id, RelationKind::RegionOwnsShell)?;
            topology.shells.push(ProjectedShellData {
                spec_id,
                region: lookup(&region_map, region, "region")?,
                faces: outgoing_targets(graph, spec_id, RelationKind::ShellOwnsFace)
                    .into_iter()
                    .map(|id| lookup(&face_map, id, "face"))
                    .collect::<Result<_, _>>()?,
            });
        }

        for spec_id in collect_ids(graph, SpecNodeKind::Face) {
            let shell = incoming_single_required(graph, spec_id, RelationKind::ShellOwnsFace)?;
            let outer_loop = outgoing_single_required(graph, spec_id, RelationKind::FaceOuterLoop)?;
            topology.faces.push(ProjectedFaceData {
                spec_id,
                shell: lookup(&shell_map, shell, "shell")?,
                outer_loop: lookup(&loop_map, outer_loop, "loop")?,
                inner_loops: outgoing_targets(graph, spec_id, RelationKind::FaceInnerLoop)
                    .into_iter()
                    .map(|id| lookup(&loop_map, id, "loop"))
                    .collect::<Result<_, _>>()?,
                surface_binding: outgoing_single_optional(graph, spec_id, RelationKind::FaceUsesSurfaceBinding)?,
            });
        }

        for spec_id in collect_ids(graph, SpecNodeKind::Loop) {
            let face = incoming_face_for_loop(graph, spec_id)?;
            let half_edge = outgoing_single_required(graph, spec_id, RelationKind::LoopEntryHalfEdge)?;
            topology.loops.push(ProjectedLoopData {
                spec_id,
                face: lookup(&face_map, face, "face")?,
                half_edge: lookup(&half_edge_map, half_edge, "halfedge")?,
            });
        }

        let mut prev_by_half_edge = vec![None; half_edge_map.len()];
        for spec_id in collect_ids(graph, SpecNodeKind::HalfEdge) {
            let next = outgoing_single_required(graph, spec_id, RelationKind::HalfEdgeNext)?;
            let current = lookup(&half_edge_map, spec_id, "halfedge")?;
            let next_id = lookup(&half_edge_map, next, "halfedge")?;
            let slot = &mut prev_by_half_edge[next_id.index()];
            if slot.replace(current).is_some() {
                return Err(ProjectedTopologyError::new(format!(
                    "halfedge {} has multiple projected predecessors",
                    next
                )));
            }
        }

        for spec_id in collect_ids(graph, SpecNodeKind::HalfEdge) {
            let current = lookup(&half_edge_map, spec_id, "halfedge")?;
            topology.half_edges.push(ProjectedHalfEdgeData {
                spec_id,
                radial_next: lookup(
                    &half_edge_map,
                    outgoing_single_required(graph, spec_id, RelationKind::HalfEdgeRadialNext)?,
                    "halfedge",
                )?,
                next: lookup(
                    &half_edge_map,
                    outgoing_single_required(graph, spec_id, RelationKind::HalfEdgeNext)?,
                    "halfedge",
                )?,
                prev: prev_by_half_edge[current.index()].ok_or_else(|| {
                    ProjectedTopologyError::new(format!(
                        "halfedge {} is missing a projected predecessor",
                        spec_id
                    ))
                })?,
                face: lookup(
                    &face_map,
                    outgoing_single_required(graph, spec_id, RelationKind::HalfEdgeBoundsFace)?,
                    "face",
                )?,
                origin: lookup(
                    &vertex_map,
                    outgoing_single_required(graph, spec_id, RelationKind::HalfEdgeOriginVertex)?,
                    "vertex",
                )?,
                edge: lookup(
                    &edge_map,
                    outgoing_single_required(graph, spec_id, RelationKind::HalfEdgeUsesEdge)?,
                    "edge",
                )?,
                coedge_binding: outgoing_single_optional(graph, spec_id, RelationKind::HalfEdgeUsesCoedgeBinding)?,
            });
        }

        for spec_id in collect_ids(graph, SpecNodeKind::Edge) {
            let incoming = incoming_sources(graph, spec_id, RelationKind::HalfEdgeUsesEdge);
            let representative = incoming
                .first()
                .copied()
                .ok_or_else(|| ProjectedTopologyError::new(format!("edge {} has no incident halfedge", spec_id)))?;
            topology.edges.push(ProjectedEdgeData {
                spec_id,
                half_edge: lookup(&half_edge_map, representative, "halfedge")?,
                curve_binding: outgoing_single_optional(graph, spec_id, RelationKind::EdgeUsesCurveBinding)?,
            });
        }

        for spec_id in collect_ids(graph, SpecNodeKind::Vertex) {
            let primary_half_edge = incoming_sources(graph, spec_id, RelationKind::HalfEdgeOriginVertex)
                .first()
                .copied()
                .map(|id| lookup(&half_edge_map, id, "halfedge"))
                .transpose()?;
            topology.vertices.push(ProjectedVertexData {
                spec_id,
                primary_half_edge,
                geometry_binding: outgoing_single_optional(graph, spec_id, RelationKind::VertexUsesGeometryBinding)?,
            });
        }

        topology.rebuild_index();
        Ok(topology)
    }
}

fn collect_ids(graph: &forge_spec::facade::SpecGraph, kind: SpecNodeKind) -> Vec<SpecNodeId> {
    graph
        .iter_nodes()
        .filter(|node| node.kind == kind)
        .map(|node| node.id)
        .collect()
}

fn outgoing_targets(
    graph: &forge_spec::facade::SpecGraph,
    source: SpecNodeId,
    kind: RelationKind,
) -> Vec<SpecNodeId> {
    let mut relations = graph.outgoing_of_kind(source, kind);
    relations.sort_by_key(|relation| (relation.ordinal, relation.target, relation.id));
    relations.into_iter().map(|relation| relation.target).collect()
}

fn incoming_sources(
    graph: &forge_spec::facade::SpecGraph,
    target: SpecNodeId,
    kind: RelationKind,
) -> Vec<SpecNodeId> {
    let mut relations: Vec<_> = graph
        .incoming_relations(target)
        .into_iter()
        .filter(|relation| relation.kind == kind)
        .collect();
    relations.sort_by_key(|relation| (relation.source, relation.ordinal, relation.id));
    relations.into_iter().map(|relation| relation.source).collect()
}

fn outgoing_single_required(
    graph: &forge_spec::facade::SpecGraph,
    source: SpecNodeId,
    kind: RelationKind,
) -> Result<SpecNodeId, ProjectedTopologyError> {
    let targets = outgoing_targets(graph, source, kind);
    match targets.as_slice() {
        [target] => Ok(*target),
        [] => Err(ProjectedTopologyError::new(format!(
            "node {} is missing required outgoing {:?} relation",
            source, kind
        ))),
        _ => Err(ProjectedTopologyError::new(format!(
            "node {} has multiple outgoing {:?} relations where one was required",
            source, kind
        ))),
    }
}

fn outgoing_single_optional(
    graph: &forge_spec::facade::SpecGraph,
    source: SpecNodeId,
    kind: RelationKind,
) -> Result<Option<SpecNodeId>, ProjectedTopologyError> {
    let targets = outgoing_targets(graph, source, kind);
    match targets.as_slice() {
        [] => Ok(None),
        [target] => Ok(Some(*target)),
        _ => Err(ProjectedTopologyError::new(format!(
            "node {} has multiple outgoing {:?} relations where zero-or-one was required",
            source, kind
        ))),
    }
}

fn incoming_single_required(
    graph: &forge_spec::facade::SpecGraph,
    target: SpecNodeId,
    kind: RelationKind,
) -> Result<SpecNodeId, ProjectedTopologyError> {
    let sources = incoming_sources(graph, target, kind);
    match sources.as_slice() {
        [source] => Ok(*source),
        [] => Err(ProjectedTopologyError::new(format!(
            "node {} is missing required incoming {:?} relation",
            target, kind
        ))),
        _ => Err(ProjectedTopologyError::new(format!(
            "node {} has multiple incoming {:?} relations where one was required",
            target, kind
        ))),
    }
}

fn incoming_face_for_loop(
    graph: &forge_spec::facade::SpecGraph,
    loop_id: SpecNodeId,
) -> Result<SpecNodeId, ProjectedTopologyError> {
    let mut owners = incoming_sources(graph, loop_id, RelationKind::FaceOuterLoop);
    owners.extend(incoming_sources(graph, loop_id, RelationKind::FaceInnerLoop));
    owners.sort();
    owners.dedup();
    match owners.as_slice() {
        [face] => Ok(*face),
        [] => Err(ProjectedTopologyError::new(format!(
            "loop {} is not owned by any face",
            loop_id
        ))),
        _ => Err(ProjectedTopologyError::new(format!(
            "loop {} is owned by multiple faces",
            loop_id
        ))),
    }
}

fn lookup<T: Copy>(
    map: &HashMap<SpecNodeId, T>,
    spec_id: SpecNodeId,
    label: &str,
) -> Result<T, ProjectedTopologyError> {
    map.get(&spec_id).copied().ok_or_else(|| {
        ProjectedTopologyError::new(format!("missing projected {} for spec node {}", label, spec_id))
    })
}
