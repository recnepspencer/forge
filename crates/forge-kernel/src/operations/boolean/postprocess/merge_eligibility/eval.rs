//! Merge eligibility evaluation — kernel-side certification wrapper.
//!
//! DOMAIN: Orchestrate boundary extraction and certification for merge eligibility.
//! Wraps forge-geom certifier results in OperationResult<T> with traced decisions.
//!
//! DEPENDENCIES: `boundary_adapter`, `forge-geom::boundary_cert`, `GeometryStore`.
//! INVARIANTS: Wraps plain results in OperationResult. Policy here, not in geom.

use forge_core::{KernelError, OperationResult};
use forge_core::tracing::{
    DecisionId, DecisionKind, DecisionTier, DecisionContext, TracedDecision,
};
use forge_topo::arena::TopologyArena;
use forge_topo::bitset::EntityBitset;

use forge_geom::algorithms::boundary_cert::eval::{
    certify_boundary, project_boundary_to_2d,
};
use forge_geom::algorithms::boundary_cert::schema::WeakSimpleCertificate;

use crate::geometry_store::GeometryStore;

use super::boundary_adapter::{extract_boundary_candidate, get_group_plane_normal};

/// Certify whether a face-group boundary is eligible for merge.
///
/// Orchestrates: extract candidate → get plane normal → project to 2D → certify.
/// Wraps the result in `OperationResult<WeakSimpleCertificate>` with traced decisions.
pub fn certify_merge_boundary(
    arena: &TopologyArena,
    group: &EntityBitset,
    geom: &GeometryStore,
) -> Result<OperationResult<WeakSimpleCertificate>, KernelError> {
    let candidate = extract_boundary_candidate(arena, group, geom)?;

    let normal = get_group_plane_normal(arena, group, geom)?;

    let segments_3d: Vec<([f64; 3], [f64; 3], u64)> = candidate
        .get_segments_3d()
        .iter()
        .map(|seg| (seg.get_start(), seg.get_end(), seg.get_provenance()))
        .collect();

    let boundary_2d = project_boundary_to_2d(&segments_3d, normal);

    let certificate = certify_boundary(&boundary_2d);

    let mut result = OperationResult::new(certificate);

    let decision_desc = match result.get_value() {
        WeakSimpleCertificate::Simple => "Boundary certified Simple",
        WeakSimpleCertificate::WeaklySimple { .. } => "Boundary certified WeaklySimple",
        WeakSimpleCertificate::Rejected { .. } => "Boundary rejected",
    };

    let group_hash = {
        let mut h: u64 = 0xcbf29ce484222325;
        for idx in 0..group.capacity() {
            if group.contains(idx).unwrap_or(false) {
                h = h.wrapping_mul(0x100000001b3) ^ (idx as u64);
            }
        }
        h
    };

    let decision = TracedDecision::new(
        DecisionId(group_hash),
        DecisionKind::PolicyApplied {
            policy: forge_core::PolicyKind::CoincidentGeometry,
            default_used: true,
        },
        DecisionTier::Deterministic,
        1.0,
        DecisionContext::Degeneracy {
            description: decision_desc.to_string(),
        },
    );
    result.get_decision_log_mut().record(decision);

    Ok(result)
}
