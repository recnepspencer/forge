//! Merge eligibility evaluation — kernel-side certification wrapper.
//!
//! DOMAIN: Orchestrate boundary extraction and certification for merge eligibility.
//! Wraps forge-geom certifier results in OperationResult<T> with traced decisions.
//!
//! DEPENDENCIES: `boundary_adapter`, `forge-geom::boundary_cert`, `GeometryState`.
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

use crate::geometry_state::GeometryView;

use super::boundary_adapter::{extract_boundary_candidate, get_group_plane_normal};

pub(crate) fn compute_group_hash(group: &EntityBitset) -> Result<u64, KernelError> {
    let mut h: u64 = 0xcbf29ce484222325;
    for idx in 0..group.capacity() {
        if group.contains(idx)? {
            h = h.wrapping_mul(0x100000001b3) ^ (idx as u64);
        }
    }
    Ok(h)
}

fn build_certification_decision(
    group_hash: u64,
    cert: &WeakSimpleCertificate,
) -> TracedDecision {
    match cert {
        WeakSimpleCertificate::Simple => TracedDecision::new(
            DecisionId(group_hash),
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            1.0,
            DecisionContext::Degeneracy {
                description: "Boundary certified Simple".to_string(),
            },
        ),
        WeakSimpleCertificate::WeaklySimple { touch_count } => TracedDecision::new(
            DecisionId(group_hash),
            DecisionKind::NearBoundary {
                threshold: 0.0,
            },
            DecisionTier::NearBoundary,
            *touch_count as f64,
            DecisionContext::Tolerance {
                measured: *touch_count as f64,
                threshold: 0.0,
            },
        ),
        WeakSimpleCertificate::Rejected { reason, witness } => TracedDecision::new(
            DecisionId(group_hash),
            DecisionKind::Forced {
                reason: "BoundaryCertificationRejected".to_string(),
            },
            DecisionTier::Escalated,
            0.0,
            DecisionContext::Degeneracy {
                description: format!(
                    "Boundary rejected: reason={:?} witness={:?}",
                    reason, witness
                ),
            },
        ),
    }
}

/// Certify whether a face-group boundary is eligible for merge.
///
/// Orchestrates: extract candidate → get plane normal → project to 2D → certify.
/// Wraps the result in `OperationResult<WeakSimpleCertificate>` with traced decisions.
pub fn certify_merge_boundary(
    arena: &TopologyArena,
    group: &EntityBitset,
    geom: &dyn GeometryView,
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

    let group_hash = compute_group_hash(group)?;
    let decision = build_certification_decision(group_hash, result.get_value());
    result.get_decision_log_mut().record(decision);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use forge_geom::algorithms::boundary_cert::schema::BoundaryRejectReason;

    #[test]
    fn simple_certificate_traces_as_exact_deterministic() {
        let d = build_certification_decision(42, &WeakSimpleCertificate::Simple);
        assert!(matches!(d.get_kind(), DecisionKind::Exact));
        assert_eq!(d.get_tier(), DecisionTier::Deterministic);
        assert_eq!(d.get_id().0, 42);
    }

    #[test]
    fn weakly_simple_certificate_traces_as_near_boundary_pending_policy_resolution() {
        let d = build_certification_decision(
            7,
            &WeakSimpleCertificate::WeaklySimple { touch_count: 3 },
        );
        assert!(matches!(
            d.get_kind(),
            DecisionKind::NearBoundary { .. }
        ));
        assert_eq!(d.get_tier(), DecisionTier::NearBoundary);
        assert_eq!(d.get_margin(), 3.0);
    }

    #[test]
    fn rejected_certificate_traces_as_escalated_forced_with_reason() {
        let d = build_certification_decision(
            9,
            &WeakSimpleCertificate::Rejected {
                reason: BoundaryRejectReason::SelfCrossing,
                witness: [1.25, -0.5],
            },
        );
        assert!(matches!(d.get_kind(), DecisionKind::Forced { .. }));
        assert_eq!(d.get_tier(), DecisionTier::Escalated);
        match d.get_context() {
            DecisionContext::Degeneracy { description } => {
                assert!(description.contains("SelfCrossing"));
                assert!(description.contains("witness"));
            }
            other => panic!("expected Degeneracy context, got {:?}", other),
        }
    }
}
