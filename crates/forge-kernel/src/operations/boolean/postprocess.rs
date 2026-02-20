//! Post-processing of boolean results.
//!
//! Includes simplification passes like merging coplanar faces to restore
//! a canonical representation and ensure associativity.

use std::collections::BTreeSet;

use forge_core::KernelError;
use forge_core::result::{TracedDecision, DecisionId, DecisionKind, DecisionTier, DecisionContext, EntityRef};
use forge_topo::state::{TopologyState};
use forge_topo::operator::apply_op;
use forge_topo::euler::join_faces::JoinFaces;

use crate::core::ModelingContext;
use crate::geometry_store::GeometryStore;

/// Merge adjacent coplanar faces to simplify the mesh.
///
/// Iteratively finds edges separating two faces that lie on the exact same plane
/// and removes them using the `JoinFaces` Euler operator. This is crucial for
/// achieving canonical results (e.g. `(A U B) U C == A U (B U C)`).
///
/// Returns the number of edges removed.
pub fn merge_coplanar_faces(
    topo: TopologyState,
    geom: &GeometryStore,
    ctx: &mut ModelingContext,
) -> Result<(TopologyState, usize), KernelError> {
    let config = crate::core::ToleranceConfig::default();

    let mut current_topo = topo;
    let mut total_merged = 0;
    let mut merged_this_pass = 1;

    while merged_this_pass > 0 {
        let (new_topo, count) = run_merge_pass(current_topo, geom, &config, ctx)?;
        current_topo = new_topo;
        merged_this_pass = count;
        total_merged += count;
    }

    Ok((current_topo, total_merged))
}

fn run_merge_pass(
    topo: TopologyState,
    geom: &GeometryStore,
    config: &crate::core::ToleranceConfig,
    ctx: &mut ModelingContext,
) -> Result<(TopologyState, usize), KernelError> {
    let mut draft = topo.into_mutation();
    let mut merged = 0;

    let candidates = {
        let arena = draft.arena();
        arena.iter_half_edges()
            .filter(|(he_id, he)| {
                he.twin() >= *he_id
            })
            .filter_map(|(he_id, he)| {
                let twin = arena.get_half_edge(he.twin()).ok()?;
                let face_a = he.face();
                let face_b = twin.face();
                if face_a == face_b {
                    return None;
                }
                let plane_a = geom.get_face_plane(face_a)?;
                let plane_b = geom.get_face_plane(face_b)?;

                if forge_geom::primitives::plane::exact_eq(plane_a, plane_b) {
                    Some(he_id)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    };

    let mut touched_faces = BTreeSet::new();
    let mut found_merge = false;

    for he_id in candidates {
        if found_merge {
            // Already merged one this pass — stop scanning.
            // The outer while loop will re-enter with fresh topology.
        } else {
            let face_pair = {
                let he = draft.arena().get_half_edge(he_id).ok();
                he.and_then(|h| {
                    let twin = draft.arena().get_half_edge(h.twin()).ok()?;
                    Some((h.face(), twin.face()))
                })
            };

            if let Some((face_a, face_b)) = face_pair {
                if !touched_faces.contains(&face_a) && !touched_faces.contains(&face_b) {
                    let shared_edge_count = {
                        let arena = draft.arena();
                        arena.iter_half_edges()
                            .filter(|(_, iter_he)| {
                                iter_he.face() == face_a
                                    && arena.get_half_edge(iter_he.twin())
                                        .map(|tw| tw.face() == face_b)
                                        .unwrap_or(false)
                            })
                            .count() as u32
                    };

                    if shared_edge_count <= 1 {
                        let op = JoinFaces { edge: he_id };

                        if let Ok(_) = apply_op(&mut draft, op) {
                            touched_faces.insert(face_a);
                            touched_faces.insert(face_b);
                            merged += 1;

                            let mut decision = TracedDecision::new(
                                DecisionId(he_id.index() as u64),
                                DecisionKind::PolicyApplied { policy: forge_core::PolicyKind::CoincidentGeometry, default_used: true },
                                DecisionTier::Deterministic,
                                1.0,
                                DecisionContext::Degeneracy {
                                    description: format!("Merged coplanar faces #{} and #{}", face_a.index(), face_b.index())
                                },
                            );
                            decision.set_entity_scope(EntityRef::new("HalfEdge", he_id.index()));
                            ctx.get_decision_log_mut().record(decision);

                            found_merge = true;
                        }
                    }
                }
            }
        }
    }

    let new_topo = draft.commit()?;
    Ok((new_topo, merged))
}

use forge_topo::euler::kill_edge_vertex::KillEdgeVertex;

/// Remove redundant vertices (valence 2, collinear edges).
///
/// Iteratively finds vertices that sit on straight lines and removes them
/// using the `KillEdgeVertex` operator.
pub fn remove_redundant_vertices(
    topo: TopologyState,
    geom: &GeometryStore,
    ctx: &mut ModelingContext,
) -> Result<(TopologyState, usize), KernelError> {
    let config = crate::core::ToleranceConfig::default();

    let mut current_topo = topo;
    let mut total_removed = 0;
    let mut removed_this_pass = 1;

    while removed_this_pass > 0 {
        let (new_topo, count) = run_vertex_cleanup_pass(current_topo, geom, &config, ctx)?;
        current_topo = new_topo;
        removed_this_pass = count;
        total_removed += count;
    }

    Ok((current_topo, total_removed))
}

fn run_vertex_cleanup_pass(
    topo: TopologyState,
    geom: &GeometryStore,
    config: &crate::core::ToleranceConfig,
    ctx: &mut ModelingContext,
) -> Result<(TopologyState, usize), KernelError> {
    let mut draft = topo.into_mutation();
    let mut removed = 0;

    let mut candidates = Vec::new();

    for (vid, v) in draft.arena().iter_vertices() {
        let he_first = v.outgoing();
        if let Ok(_he_first_data) = draft.arena().get_half_edge(he_first) {
            let degree_result = compute_vertex_degree(draft.arena(), he_first);

            if let Some((count, edges)) = degree_result {
                if count == 2 {
                    let e1 = edges[0];
                    let e2 = edges[1];

                    if let (Ok(e1_data), Ok(e2_data)) = (
                        draft.arena().get_half_edge(e1),
                        draft.arena().get_half_edge(e2),
                    ) {
                        if let Some(p_v) = geom.get_vertex_position(vid) {
                            let twin_a = draft.arena().get_half_edge(e1_data.twin());
                            let twin_b = draft.arena().get_half_edge(e2_data.twin());

                            if let (Ok(ta), Ok(tb)) = (twin_a, twin_b) {
                                let target_a = ta.origin();
                                let target_b = tb.origin();

                                if let (Some(p_a), Some(p_b)) = (
                                    geom.get_vertex_position(target_a),
                                    geom.get_vertex_position(target_b),
                                ) {
                                    let v_va = forge_math::linalg::sub(*p_a, *p_v);
                                    let v_vb = forge_math::linalg::sub(*p_b, *p_v);

                                    let len_a = forge_math::linalg::norm(v_va);
                                    let len_b = forge_math::linalg::norm(v_vb);

                                    let min_len = config.get_min_edge_length();
                                    if len_a >= min_len && len_b >= min_len {
                                        let dot = forge_math::linalg::dot(v_va, v_vb) / (len_a * len_b);
                                        let dot_tol = config.get_collinearity_dot_tolerance();

                                        if (dot + 1.0).abs() < dot_tol {
                                            candidates.push((vid, e1_data.twin()));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    candidates.sort_by_key(|k| k.0);

    let mut touched_verts = BTreeSet::new();
    let mut found_removal = false;

    for (vid, incoming_he) in candidates {
        if found_removal {
            // Already removed one this pass — stop.
        } else if !touched_verts.contains(&vid) {
            let op = KillEdgeVertex { edge: incoming_he };

            if let Ok(_) = apply_op(&mut draft, op) {
                touched_verts.insert(vid);
                removed += 1;

                let mut decision = TracedDecision::new(
                    DecisionId(vid.index() as u64),
                    DecisionKind::PolicyApplied { policy: forge_core::PolicyKind::CoincidentGeometry, default_used: true },
                    DecisionTier::Deterministic,
                    1.0,
                    DecisionContext::Degeneracy {
                        description: format!("Removed redundant collinear vertex #{}", vid.index())
                    },
                );
                decision.set_entity_scope(EntityRef::new("Vertex", vid.index()));
                ctx.get_decision_log_mut().record(decision);

                found_removal = true;
            }
        }
    }

    let new_topo = draft.commit()?;
    Ok((new_topo, removed))
}

/// Walk the half-edge ring around a vertex to compute its vertex degree.
///
/// Returns `None` if the ring is invalid (broken links, exceeds safety limit).
/// Returns `Some((count, edges))` with the degree and collected outgoing edges.
fn compute_vertex_degree(
    arena: &forge_topo::arena::TopologyArena,
    he_first: forge_topo::handles::HalfEdgeId,
) -> Option<(usize, Vec<forge_topo::handles::HalfEdgeId>)> {
    let mut count = 0;
    let mut curr = he_first;
    let mut edges = Vec::new();
    let mut completed = false;

    while !completed && count <= 100 {
        count += 1;
        edges.push(curr);

        let curr_data = arena.get_half_edge(curr).ok()?;
        let twin_data = arena.get_half_edge(curr_data.twin()).ok()?;

        let next_outgoing = twin_data.next();
        if next_outgoing == he_first {
            completed = true;
        } else {
            curr = next_outgoing;
        }
    }

    if completed {
        Some((count, edges))
    } else {
        None
    }
}
