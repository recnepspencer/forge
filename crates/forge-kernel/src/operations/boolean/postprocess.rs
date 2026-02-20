//! Post-processing of boolean results.
//!
//! Includes simplification passes like merging coplanar faces to restore
//! a canonical representation and ensure associativity.

use std::collections::HashSet;

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
    // Use default config if not provided (internal utility)
    let config = crate::core::ToleranceConfig::default();
    
    let mut current_topo = topo;
    let mut total_merged = 0;

    loop {
        let (new_topo, merged_count) = run_merge_pass(current_topo, geom, &config, ctx)?;
        current_topo = new_topo;
        if merged_count == 0 {
            break;
        }
        total_merged += merged_count;
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
    
    // Track edges we've already processed (or their twins) to avoid double-counting
    // or attempting to delete invalid handles.
    
    let candidates = {
        let arena = draft.arena();
        let mut local_candidates = Vec::new(); // Shadow candidates

    // We scan all edges.
        for (he_id, he) in arena.iter_half_edges() {
            if he.twin() < he_id { 
                continue; 
            }

            let Ok(twin) = arena.get_half_edge(he.twin()) else { continue };

            let face_a = he.face();
            let face_b = twin.face();

            if face_a == face_b {
                continue; // Already same face
            }

            let Some(plane_a) = geom.get_face_plane(face_a) else { continue };
            let Some(plane_b) = geom.get_face_plane(face_b) else { continue };

            let exact_match = plane_a.raw_normal() == plane_b.raw_normal()
                && plane_a.raw_offset() == plane_b.raw_offset();

            if exact_match || forge_geom::primitives::plane::exact_eq(
                plane_a, 
                plane_b, 
            ) {
                local_candidates.push(he_id);
            }
        }
        local_candidates
    };

    // Now attempt to apply merges.
    let mut touched_faces = HashSet::new();
    
    for he_id in candidates {
        let (face_a, face_b) = {
            let Ok(he) = draft.arena().get_half_edge(he_id) else { continue };
            let Ok(twin) = draft.arena().get_half_edge(he.twin()) else { continue };
            (he.face(), twin.face())
        };
        
        if touched_faces.contains(&face_a) || touched_faces.contains(&face_b) {
            continue;
        }

        let shared_edge_count = {
            let arena = draft.arena();
            let mut count = 0u32;
            for (iter_he_id, iter_he) in arena.iter_half_edges() {
                if iter_he.face() == face_a {
                    if let Ok(tw) = arena.get_half_edge(iter_he.twin()) {
                        if tw.face() == face_b {
                            count += 1;
                        }
                    }
                }
            }
            count
        };

        if shared_edge_count > 1 {
            continue;
        }

        let op = JoinFaces { edge: he_id };
        
        match apply_op(&mut draft, op) {
            Ok(_) => {
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

                break;
            }
            Err(_) => {
                // Ignore failures (topology might have changed effectively)
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

    loop {
        let (new_topo, count) = run_vertex_cleanup_pass(current_topo, geom, &config, ctx)?;
        current_topo = new_topo;
        if count == 0 {
            break;
        }
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
        // Check degree.
        let he_first = v.outgoing();
        let Ok(_he_first_data) = draft.arena().get_half_edge(he_first) else { continue };
        
        let mut count = 0;
        let mut curr = he_first;
        let mut edges = Vec::new();
        let mut is_valid = true;

        loop {
            count += 1;
            edges.push(curr);
            
            let Ok(curr_data) = draft.arena().get_half_edge(curr) else { is_valid = false; break; };
            let Ok(twin_data) = draft.arena().get_half_edge(curr_data.twin()) else { is_valid = false; break; };
            
            let next_outgoing = twin_data.next();
            if next_outgoing == he_first {
                break;
            }
            curr = next_outgoing;
            
            if count > 100 { // Safety break
                is_valid = false; 
                break; 
            }
        }

        if !is_valid || count != 2 {
            continue;
        }

        // Degree 2. Check collinearity.
        let e1 = edges[0];
        let e2 = edges[1];
        
        let Ok(e1_data) = draft.arena().get_half_edge(e1) else { continue };
        let Ok(e2_data) = draft.arena().get_half_edge(e2) else { continue };
        
        // SAFE: Replaced unwraps with ? propagation or explicit error handling
        let p_v = geom.get_vertex_position(vid).ok_or_else(|| KernelError::InvalidInput { 
            message: format!("Missing position for vertex {}", vid), context: None 
        })?;
        
        let target_a = draft.arena().get_half_edge(e1_data.twin())
            .map_err(|_| KernelError::InternalError { message: "Broken twin link".into(), context: None })?
            .origin();
        let target_b = draft.arena().get_half_edge(e2_data.twin())
            .map_err(|_| KernelError::InternalError { message: "Broken twin link".into(), context: None })?
            .origin();
        
        let p_a = geom.get_vertex_position(target_a).ok_or_else(|| KernelError::InvalidInput { 
             message: format!("Missing position for vertex {}", target_a), context: None 
        })?;
        let p_b = geom.get_vertex_position(target_b).ok_or_else(|| KernelError::InvalidInput { 
             message: format!("Missing position for vertex {}", target_b), context: None 
        })?;
        
        let v_va = [p_a[0]-p_v[0], p_a[1]-p_v[1], p_a[2]-p_v[2]];
        let v_vb = [p_b[0]-p_v[0], p_b[1]-p_v[1], p_b[2]-p_v[2]];
        
        // Normalize
        let len_a = (v_va[0]*v_va[0] + v_va[1]*v_va[1] + v_va[2]*v_va[2]).sqrt();
        let len_b = (v_vb[0]*v_vb[0] + v_vb[1]*v_vb[1] + v_vb[2]*v_vb[2]).sqrt();
        
        // Use config tolerance
        let min_len = config.get_min_edge_length();
        if len_a < min_len || len_b < min_len {
            continue; // Degenerate edge
        }
        
        let dot = (v_va[0]*v_vb[0] + v_va[1]*v_vb[1] + v_va[2]*v_vb[2]) / (len_a * len_b);
        
        // If collinear and opposite, dot should be -1.0.
        // Use config tolerance
        let dot_tol = config.get_collinearity_dot_tolerance();

        if (dot + 1.0).abs() < dot_tol {
            // Found collinear vertex!
            candidates.push((vid, e1_data.twin()));
        }
    }
    
    candidates.sort_by_key(|k| k.0);
    
    let mut touched_verts = HashSet::new();
    
    for (vid, incoming_he) in candidates {
        if touched_verts.contains(&vid) {
            continue;
        }
        
        let op = KillEdgeVertex { edge: incoming_he };
        
        match apply_op(&mut draft, op) {
            Ok(_) => {
                touched_verts.insert(vid);
                removed += 1;
                
                // Log the cleanup
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

                break;
            }
            Err(_) => {
                // Ignore failures — handle may already be stale
            }
        }
    }
    
    let new_topo = draft.commit()?;
    Ok((new_topo, removed))
}
