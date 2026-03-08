use forge_core::KernelError;

use crate::projection::data::ProjectedTopology;

use super::super::shared::vf;

pub fn validate_projected_no_dangling_refs(
    topology: &ProjectedTopology,
) -> Result<(), KernelError> {
    for (half_edge_index, half_edge) in topology.half_edges().iter().enumerate() {
        if half_edge.origin.index() >= topology.vertex_count() {
            return Err(vf(
                "projected_no_dangling_refs",
                format!(
                    "HalfEdge {} references missing vertex {}",
                    half_edge_index,
                    half_edge.origin.raw()
                ),
            ));
        }
        if half_edge.face.index() >= topology.face_count() {
            return Err(vf(
                "projected_no_dangling_refs",
                format!(
                    "HalfEdge {} references missing face {}",
                    half_edge_index,
                    half_edge.face.raw()
                ),
            ));
        }
        if half_edge.edge.index() >= topology.edge_count() {
            return Err(vf(
                "projected_no_dangling_refs",
                format!(
                    "HalfEdge {} references missing edge {}",
                    half_edge_index,
                    half_edge.edge.raw()
                ),
            ));
        }
        for (label, target) in [
            ("next", half_edge.next.raw()),
            ("prev", half_edge.prev.raw()),
            ("radial_next", half_edge.radial_next.raw()),
        ] {
            if target as usize >= topology.half_edge_count() {
                return Err(vf(
                    "projected_no_dangling_refs",
                    format!(
                        "HalfEdge {}.{} references missing halfedge {}",
                        half_edge_index, label, target
                    ),
                ));
            }
        }
    }

    for (loop_index, loop_data) in topology.loops().iter().enumerate() {
        if loop_data.face.index() >= topology.face_count() {
            return Err(vf(
                "projected_no_dangling_refs",
                format!(
                    "Loop {} references missing face {}",
                    loop_index,
                    loop_data.face.raw()
                ),
            ));
        }
        if loop_data.half_edge.index() >= topology.half_edge_count() {
            return Err(vf(
                "projected_no_dangling_refs",
                format!(
                    "Loop {} references missing halfedge {}",
                    loop_index,
                    loop_data.half_edge.raw()
                ),
            ));
        }
    }

    for (edge_index, edge_data) in topology.edges().iter().enumerate() {
        if edge_data.half_edge.index() >= topology.half_edge_count() {
            return Err(vf(
                "projected_no_dangling_refs",
                format!(
                    "Edge {} references missing halfedge {}",
                    edge_index,
                    edge_data.half_edge.raw()
                ),
            ));
        }
    }

    for (face_index, face_data) in topology.faces().iter().enumerate() {
        if face_data.shell.index() >= topology.shell_count() {
            return Err(vf(
                "projected_no_dangling_refs",
                format!(
                    "Face {} references missing shell {}",
                    face_index,
                    face_data.shell.raw()
                ),
            ));
        }
        if face_data.outer_loop.index() >= topology.loop_count() {
            return Err(vf(
                "projected_no_dangling_refs",
                format!(
                    "Face {} references missing outer loop {}",
                    face_index,
                    face_data.outer_loop.raw()
                ),
            ));
        }
        for inner_loop in &face_data.inner_loops {
            if inner_loop.index() >= topology.loop_count() {
                return Err(vf(
                    "projected_no_dangling_refs",
                    format!(
                        "Face {} references missing inner loop {}",
                        face_index,
                        inner_loop.raw()
                    ),
                ));
            }
        }
    }

    for (shell_index, shell_data) in topology.shells().iter().enumerate() {
        if shell_data.region.index() >= topology.region_count() {
            return Err(vf(
                "projected_no_dangling_refs",
                format!(
                    "Shell {} references missing region {}",
                    shell_index,
                    shell_data.region.raw()
                ),
            ));
        }
        for face in &shell_data.faces {
            if face.index() >= topology.face_count() {
                return Err(vf(
                    "projected_no_dangling_refs",
                    format!(
                        "Shell {} references missing face {}",
                        shell_index,
                        face.raw()
                    ),
                ));
            }
        }
    }

    for (region_index, region_data) in topology.regions().iter().enumerate() {
        if region_data.lump.index() >= topology.lump_count() {
            return Err(vf(
                "projected_no_dangling_refs",
                format!(
                    "Region {} references missing lump {}",
                    region_index,
                    region_data.lump.raw()
                ),
            ));
        }
        for shell in &region_data.shells {
            if shell.index() >= topology.shell_count() {
                return Err(vf(
                    "projected_no_dangling_refs",
                    format!(
                        "Region {} references missing shell {}",
                        region_index,
                        shell.raw()
                    ),
                ));
            }
        }
    }

    for (lump_index, lump_data) in topology.lumps().iter().enumerate() {
        if lump_data.body.index() >= topology.body_count() {
            return Err(vf(
                "projected_no_dangling_refs",
                format!(
                    "Lump {} references missing body {}",
                    lump_index,
                    lump_data.body.raw()
                ),
            ));
        }
        for region in &lump_data.regions {
            if region.index() >= topology.region_count() {
                return Err(vf(
                    "projected_no_dangling_refs",
                    format!(
                        "Lump {} references missing region {}",
                        lump_index,
                        region.raw()
                    ),
                ));
            }
        }
    }

    for (body_index, body_data) in topology.bodies().iter().enumerate() {
        for lump in &body_data.lumps {
            if lump.index() >= topology.lump_count() {
                return Err(vf(
                    "projected_no_dangling_refs",
                    format!(
                        "Body {} references missing lump {}",
                        body_index,
                        lump.raw()
                    ),
                ));
            }
        }
    }

    for (vertex_index, vertex_data) in topology.vertices().iter().enumerate() {
        if let Some(half_edge) = vertex_data.primary_half_edge {
            if half_edge.index() >= topology.half_edge_count() {
                return Err(vf(
                    "projected_no_dangling_refs",
                    format!(
                        "Vertex {} references missing primary halfedge {}",
                        vertex_index,
                        half_edge.raw()
                    ),
                ));
            }
        }
    }

    Ok(())
}
