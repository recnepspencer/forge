//! Coplanar face merging (legacy iterative approach).
//!
//! DOMAIN: Simplification pass — merge adjacent coplanar faces
//! by iteratively removing shared edges via `JoinFaces`.
//!
//! DEPENDENCIES: forge_topo (JoinFaces), GeometryState.
//! INVARIANTS: Each pass removes at most one entity, then re-enters
//! with fresh topology. This avoids invalidating arena handles mid-pass.

use forge_core::KernelError;
use forge_core::{TracedDecision, DecisionId, DecisionKind, DecisionTier, DecisionContext, EntityRef};
use forge_core::tracing::TopologyDelta;
use forge_topo::state::TopologyState;
use forge_topo::handles::HalfEdgeId;
use forge_topo::operator::apply_op;
use forge_topo::euler::join_faces::JoinFaces;
use forge_topo::validate::{validate_topology, ValidationLevel};

use crate::core::{ModelingContext, ArenaSnapshot, compute_topology_delta, KernelState, KernelDraft};
use crate::geometry_state::{GeometryState, GeometryPatch};

use super::run_iterative_pass;

/// Merge adjacent coplanar faces to simplify the mesh.
///
/// Iteratively removes edges separating faces on the exact same plane
/// via `JoinFaces`. Critical for canonical results: `(A ∪ B) ∪ C == A ∪ (B ∪ C)`.
pub fn merge_coplanar_faces(
    state: KernelState,
    ctx: &mut ModelingContext,
) -> Result<(KernelState, usize), KernelError> {
    run_iterative_pass(state, |current| run_merge_pass(current, ctx))
}

/// Find and merge one pair of coplanar faces.
fn run_merge_pass(
    state: KernelState,
    ctx: &mut ModelingContext,
) -> Result<(KernelState, usize), KernelError> {
    let (topo, mut geom) = state.into_parts();
    let candidate = find_coplanar_merge_candidate(topo.arena(), &geom);
    let Some((he_id, face_a, face_b)) = candidate else {
        return Ok((KernelState::new(topo, geom), 0));
    };

    let mut pair_group = forge_topo::bitset::EntityBitset::for_faces(topo.arena());
    let _ = pair_group.insert(face_a.index());
    let _ = pair_group.insert(face_b.index());

    let cert_ok = match super::merge_eligibility::eval::certify_merge_boundary(
        topo.arena(),
        &pair_group,
        &geom,
    ) {
        Ok(mut op_result) => {
            let cert_log = op_result.take_decision_log();
            ctx.get_decision_log_mut().merge(cert_log);
            let cert = op_result.into_value();
            !matches!(cert, forge_geom::algorithms::boundary_cert::schema::WeakSimpleCertificate::Rejected { .. })
        }
        Err(_) => false,
    };

    if !cert_ok {
        return Ok((KernelState::new(topo, geom), 0));
    }

    let mut draft = KernelDraft::new(KernelState::new(topo, geom));

    let pre_snapshot = ArenaSnapshot::capture(draft.arena());
    let (mut_draft, mut_geom) = draft.as_parts_mut();
    match apply_op(mut_draft, JoinFaces { edge: he_id }) {
        Ok(_) => {
            if let Err(err) = validate_topology(mut_draft.arena(), ValidationLevel::Full) {
                eprintln!(
                    "[postprocess/coplanar] skip invalid merge on he#{} faces {}+{}: {}",
                    he_id.index(),
                    face_a.index(),
                    face_b.index(),
                    err
                );
                return Ok((draft.rollback(), 0));
            }
            let delta = compute_topology_delta(&pre_snapshot, mut_draft.arena());
            log_merge(he_id, face_a, face_b, delta, ctx);
            mut_geom.remove_face_plane(face_b);
            Ok((draft.commit()?, 1))
        }
        Err(_) => Ok((draft.commit()?, 0)),
    }
}

/// Find the first edge separating two coplanar faces.
fn find_coplanar_merge_candidate(
    arena: &forge_topo::arena::TopologyArena,
    geom: &GeometryState,
) -> Option<(HalfEdgeId, forge_topo::handles::FaceId, forge_topo::handles::FaceId)> {
    arena.iter_half_edges()
        .filter(|(he_id, he)| he.radial_next() >= *he_id)
        .find_map(|(he_id, he)| {
            let twin = arena.get_half_edge(he.radial_next()).ok()?;
            let face_a = he.face();
            let face_b = twin.face();
            if face_a == face_b { return None; }

            let plane_a = geom.get_face_plane(face_a)?;
            let plane_b = geom.get_face_plane(face_b)?;
            if forge_geom::primitives::plane::exact_eq(plane_a, plane_b) {
                Some((he_id, face_a, face_b))
            } else {
                None
            }
        })
}

/// Log a coplanar face merge decision.
fn log_merge(
    he_id: HalfEdgeId,
    face_a: forge_topo::handles::FaceId,
    face_b: forge_topo::handles::FaceId,
    delta: TopologyDelta,
    ctx: &mut ModelingContext,
) {
    let mut decision = TracedDecision::new(
        DecisionId(he_id.index() as u64),
        DecisionKind::PolicyApplied { policy: forge_core::PolicyKind::CoincidentGeometry, default_used: true },
        DecisionTier::Deterministic, 1.0,
        DecisionContext::Degeneracy {
            description: format!("Merged coplanar faces #{} and #{}", face_a.index(), face_b.index()),
        },
    );
    decision.set_entity_scope(EntityRef::new(forge_core::EntityKind::HalfEdge, he_id.index()));
    decision.set_topology_delta(delta);
    ctx.get_decision_log_mut().record(decision);
}
