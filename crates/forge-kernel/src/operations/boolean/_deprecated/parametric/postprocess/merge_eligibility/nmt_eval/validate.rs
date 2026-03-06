use std::collections::BTreeSet;

use forge_core::tracing::{CandidateValueSummary, DecisionId};
use forge_core::{KernelError, PolicyKind, PolicyQuery};

use crate::core::ModelingContext;
use crate::geom_facade::WeakSimpleCertificate;

use super::super::eval::compute_group_hash;
use super::super::schema::MergeRegionSelection;
use forge_core::errors::MergeError;

use super::plan::find_face_by_index;

pub(crate) fn apply_boundary_cert_gate_policy(
    cert: &WeakSimpleCertificate,
    selection: &MergeRegionSelection,
    ctx: &mut ModelingContext,
) -> Result<(), KernelError> {
    match cert {
        WeakSimpleCertificate::Rejected {
            reason,
            witness,
        } => Err(KernelError::MergeFailure(
            MergeError::BoundaryCertificationFailed {
                reason: format!("{:?}", reason),
                witness: Some(*witness),
            },
        )),
        WeakSimpleCertificate::WeaklySimple {
            touch_count,
        } => {
            let group_hash = compute_group_hash(selection.get_selected_faces())?;
            let policy_decision_id = DecisionId(group_hash ^ 0x9e37_79b9_7f4a_7c15);

            let policy_query = PolicyQuery {
                kind: PolicyKind::CoincidentGeometry,
                location: [0.0, 0.0, 0.0],
                margin: *touch_count as f64,
                overridable: true,
            };
            let resolved_result = ctx.resolve_policy_query(
                policy_decision_id,
                &policy_query,
                Some(0.0),
                CandidateValueSummary::EnumTag {
                    type_name: "WeakSimpleCertificate".to_string(),
                    variant: "WeaklySimple".to_string(),
                },
            );

            let resolved = resolved_result?;
            if !resolved.accept_potential_value {
                return Err(KernelError::MergeFailure(
                    MergeError::BoundaryCertificationFailed {
                        reason: "CoincidentGeometry policy rejected WeaklySimple boundary"
                            .to_string(),
                        witness: None,
                    },
                ));
            }
            Ok(())
        }
        WeakSimpleCertificate::Simple => Ok(()),
    }
}

/// Reject if any face appears in both `selected_faces` and `protected_faces`.
///
/// This is a deterministic input validation: the two sets must be disjoint.
/// If overlap exists, no merge can proceed without violating protection semantics.
pub(crate) fn validate_protected_faces(selection: &MergeRegionSelection) -> Result<(), KernelError> {
    let selected = selection.get_selected_faces();
    let protected = selection.get_protected_faces();

    for idx in selected.iter_ones() {
        if protected.contains(idx)? {
            return Err(KernelError::MergeFailure(
                MergeError::ProtectedUseConflict {
                    face_index: idx,
                    edge_index: None,
                },
            ));
        }
    }

    Ok(())
}

/// Validate that all selected faces form a connected subgraph.
///
/// Uses BFS via shared edges: two selected faces are connected if they
/// share at least one edge. All selected faces must be reachable from
/// the surviving face.
pub(crate) fn validate_connectivity(
    arena: &forge_topo::b_rep::TopologyArena,
    selection: &MergeRegionSelection,
) -> Result<(), KernelError> {
    let selected = selection.get_selected_faces();

    let mut selected_indices: Vec<u32> = Vec::new();
    for (face_id, _) in arena.iter_faces() {
        let idx = face_id.index();
        if selected.contains(idx)? {
            selected_indices.push(idx);
        }
    }

    if selected_indices.is_empty() {
        return Err(KernelError::MergeFailure(
            MergeError::WouldDisconnectSheet { face_index: 0 },
        ));
    }

    let start_idx = selection.get_surviving_face().index();
    if !selected_indices.contains(&start_idx) {
        return Err(KernelError::InvalidInput {
            message: "Surviving face is not in selected_faces set".into(),
            context: None,
        });
    }

    let mut visited: BTreeSet<u32> = BTreeSet::new();
    let mut queue: Vec<u32> = vec![start_idx];
    visited.insert(start_idx);

    while let Some(current_face_idx) = queue.pop() {
        let current_face_id = find_face_by_index(arena, current_face_idx)?;

        let outer_loop = arena.get_face(current_face_id)?.loops.outer();
        let loop_he = arena.get_loop(outer_loop)?.half_edge();

        let mut he = loop_he;
        loop {
            let twin = arena.get_half_edge(he)?.radial_next();
            if twin != he {
                let neighbor_face = arena.get_half_edge(twin)?.face();
                let neighbor_idx = neighbor_face.index();

                if selected.contains(neighbor_idx)? && !visited.contains(&neighbor_idx) {
                    visited.insert(neighbor_idx);
                    queue.push(neighbor_idx);
                }

                let mut radial_cur = arena.get_half_edge(twin)?.radial_next();
                while radial_cur != he {
                    let rf = arena.get_half_edge(radial_cur)?.face();
                    let ri = rf.index();
                    if selected.contains(ri)? && !visited.contains(&ri) {
                        visited.insert(ri);
                        queue.push(ri);
                    }
                    radial_cur = arena.get_half_edge(radial_cur)?.radial_next();
                }
            }

            he = arena.get_half_edge(he)?.next();
            if he == loop_he {
                break;
            }
        }
    }

    if visited.len() != selected_indices.len() {
        let disconnected = selected_indices
            .iter()
            .find(|idx| !visited.contains(idx))
            .copied()
            .unwrap_or(0);
        return Err(KernelError::MergeFailure(
            MergeError::WouldDisconnectSheet {
                face_index: disconnected,
            },
        ));
    }

    Ok(())
}
