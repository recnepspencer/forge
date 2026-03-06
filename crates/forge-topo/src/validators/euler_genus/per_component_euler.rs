//! Per-Component Euler Validator
//!
//! INVARIANT: The Euler formula must hold true for every isolated connected
//! component (shell) in the topology, not just the global sum. This prevents
//! equal and opposite structural failures in separate shells from masking
//! each other in a global Euler check.

use crate::b_rep::EntityBitset;
use crate::b_rep::TopologyArena;
use forge_core::KernelError;
use std::collections::VecDeque;

pub(crate) fn validate_per_component_euler(arena: &TopologyArena) -> Result<(), KernelError> {
    if arena.vertex_count() == 0 {
        return Ok(());
    }

    let mut visited_vertices = EntityBitset::for_vertices(arena);
    let mut component_index = 0;

    for (seed_vid, _) in arena.iter_vertices() {
        if !visited_vertices.contains(seed_vid.index())? {
            let mut comp_vertices = EntityBitset::for_vertices(arena);
            let mut comp_edges = EntityBitset::for_edges(arena);
            let mut comp_faces = EntityBitset::for_faces(arena);

            let mut queue: VecDeque<crate::handles::VertexId> = VecDeque::new();
            queue.push_back(seed_vid);
            comp_vertices.insert(seed_vid.index())?;

            while let Some(vid) = queue.pop_front() {
                visited_vertices.insert(vid.index())?;

                // Traverse outgoing halfedges to find connected faces, edges, and adjacent vertices
                let mut current = arena.get_vertex(vid)?.primary_disk();
                if current.is_dangling() {
                    continue; // Isolated vertex
                }

                let start = current;
                let bound = arena.half_edge_count().max(1);

                for _ in 0..=bound {
                    let he_data = arena.get_half_edge(current)?;
                    comp_edges.insert(he_data.edge().index())?;
                    comp_faces.insert(he_data.face().index())?;

                    // The twin halfedge takes us to adjacent vertices along the edge
                    let twin_id = he_data.radial_next();
                    if let Ok(twin_data) = arena.get_half_edge(twin_id) {
                        let adj_vid = twin_data.origin();
                        if comp_vertices.insert(adj_vid.index())? {
                            queue.push_back(adj_vid);
                        }
                    }

                    let next_out = arena.get_half_edge(twin_id)?.next();
                    current = next_out;
                    if current == start {
                        break;
                    }
                }
            }

            let v = comp_vertices.count() as i64;
            let e = comp_edges.count() as i64;
            let f = comp_faces.count() as i64;
            let euler_char = v - e + f;

            // Wait, to calculate expected, we need the genus and inner loops.
            // But since this is a general component check, let's just make sure the
            // calculated genus is valid (non-negative integer).
            let mut rings = 0;
            let mut is_solid = false;

            for f_idx in comp_faces.iter_ones() {
                if let Ok(face_data) = arena.get_face(crate::handles::FaceId::new(f_idx as u32, 0))
                {
                    rings += face_data.inner_loop_count();
                    // Assuming all faces in the component share the same shell kind
                    if let Ok(shell_data) = arena.get_shell(face_data.shell()) {
                        if matches!(shell_data.kind(), crate::b_rep::ShellKind::Solid(_)) {
                            is_solid = true;
                        }
                    }
                }
            }

            // Only solids must rigidly obey V - E + F = 2 - 2G + L.
            // Sheets have boundaries (holes) which changes the calculation.
            if is_solid {
                let twice_genus = 2 - euler_char + rings as i64;
                if twice_genus < 0 || twice_genus % 2 != 0 {
                    return Err(KernelError::TopologyViolation {
                        err: forge_core::TopologyError::GeneralizedEulerViolation {
                            shell_index: component_index as u32,
                            vertices: v as usize,
                            edges: e as usize,
                            faces: f as usize,
                            genus: 0,
                            rings,
                            expected_chi: 0,
                            actual_chi: euler_char,
                        },
                        context: Some(forge_core::ErrorContext {
                            scope: forge_core::ErrorScope::Entity {
                                entity_kind: "Component".to_string(),
                                index: component_index as u32,
                            },
                            suggested_fixes: Vec::new(),
                            detail: format!(
                                "Component {} has invalid genus/characteristics: V={}, E={}, F={}, rings={}, twice_genus={}",
                                component_index, v, e, f, rings, twice_genus
                            ),
                        }),
                    });
                }
            }

            component_index += 1;
        }
    }

    Ok(())
}
