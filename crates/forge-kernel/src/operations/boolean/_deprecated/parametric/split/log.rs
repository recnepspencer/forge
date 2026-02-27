//! Decision logging for the face-splitting phase.
//!
//! DOMAIN: Emit TracedDecision records for cut rejections and successes.
//! DEPENDENCIES: forge_core tracing types, ModelingContext.

use forge_core::{
    DecisionContext, DecisionId, DecisionKind, DecisionTier, EntityRef, TracedDecision,
};
use forge_topo::handles::FaceId;

use crate::core::ModelingContext;

/// Log a face-cut rejection decision.
pub(super) fn log_rejection(
    face: FaceId,
    cut_plane_idx: usize,
    reason: &str,
    ctx: &mut ModelingContext,
) {
    let mut decision = TracedDecision::new(
        DecisionId(face.index() as u64),
        DecisionKind::Exact,
        DecisionTier::Deterministic,
        1.0,
        DecisionContext::Degeneracy {
            description: format!("Face #{} {reason} (plane #{cut_plane_idx})", face.index()),
        },
    );
    decision.set_entity_scope(EntityRef::new(forge_core::EntityKind::Face, face.index()));
    ctx.get_decision_log_mut().record(decision);
}

/// Log a successful face split decision.
pub(super) fn log_split_success(
    face: FaceId,
    cut_plane_idx: usize,
    new_face: FaceId,
    ctx: &mut ModelingContext,
) {
    let mut decision = TracedDecision::new(
        DecisionId(face.index() as u64),
        DecisionKind::PolicyApplied {
            policy: forge_core::PolicyKind::CoincidentGeometry,
            default_used: true,
        },
        DecisionTier::Deterministic,
        1.0,
        DecisionContext::Degeneracy {
            description: format!(
                "Split face #{} by plane #{} -> new face #{}",
                face.index(),
                cut_plane_idx,
                new_face.index()
            ),
        },
    );
    decision.set_entity_scope(EntityRef::new(forge_core::EntityKind::Face, face.index()));
    ctx.get_decision_log_mut().record(decision);
}
