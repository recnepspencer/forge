//! Coplanar face merging (legacy iterative approach).
//!
//! DOMAIN: Simplification pass — merge adjacent coplanar faces
//! by iteratively removing shared edges via `JoinFaces`.
//!
//! DEPENDENCIES: forge_topo (JoinFaces), GeometryStore.
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

use crate::core::{ModelingContext, ArenaSnapshot, compute_topology_delta};
use crate::geometry_store::GeometryStore;

use super::run_iterative_pass;

/// Merge adjacent coplanar faces to simplify the mesh.
///
/// Iteratively removes edges separating faces on the exact same plane
/// via `JoinFaces`. Critical for canonical results: `(A ∪ B) ∪ C == A ∪ (B ∪ C)`.
pub fn merge_coplanar_faces(
    topo: TopologyState,
    geom: &mut GeometryStore,
    ctx: &mut ModelingContext,
) -> Result<(TopologyState, usize), KernelError> {
    run_iterative_pass(topo, |current| run_merge_pass(current, geom, ctx))
}

/// Find and merge one pair of coplanar faces.
fn run_merge_pass(
    topo: TopologyState,
    geom: &mut GeometryStore,
    ctx: &mut ModelingContext,
) -> Result<(TopologyState, usize), KernelError> {
    let candidate = find_coplanar_merge_candidate(topo.arena(), geom);
    let Some((he_id, face_a, face_b)) = candidate else {
        return Ok((topo, 0));
    };

    let mut pair_group = forge_topo::bitset::EntityBitset::for_faces(topo.arena());
    let _ = pair_group.insert(face_a.index());
    let _ = pair_group.insert(face_b.index());

    let cert_ok = match super::merge_eligibility::eval::certify_merge_boundary(
        topo.arena(),
        &pair_group,
        geom,
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
        return Ok((topo, 0));
    }

    let original = topo.clone();
    let mut draft = topo.into_mutation();

    let pre_snapshot = ArenaSnapshot::capture(draft.arena());
    match apply_op(&mut draft, JoinFaces { edge: he_id }) {
        Ok(_) => {
            if let Err(err) = validate_topology(draft.arena(), ValidationLevel::Full) {
                eprintln!(
                    "[postprocess/coplanar] skip invalid merge on he#{} faces {}+{}: {}",
                    he_id.index(),
                    face_a.index(),
                    face_b.index(),
                    err
                );
                return Ok((original, 0));
            }
            let delta = compute_topology_delta(&pre_snapshot, draft.arena());
            log_merge(he_id, face_a, face_b, delta, ctx);
            let committed = draft.commit()?;
            geom.remove_face_plane(face_b);
            Ok((committed, 1))
        }
        Err(_) => Ok((draft.commit()?, 0)),
    }
}

/// Find the first edge separating two coplanar faces.
fn find_coplanar_merge_candidate(
    arena: &forge_topo::arena::TopologyArena,
    geom: &GeometryStore,
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
