//! Pre-stitch vertex identity repair step.
//!
//! DOMAIN: Cluster vertices by position and rewrite halfedge endpoints
//! to use the canonical (lowest-index) VertexId in each cluster.
//! This is an audited kernel step that logs TracedDecisions.
//!
//! DEPENDENCIES: shared_ops::copy::VertexWelder, GeometryState, ModelingContext.

use std::collections::BTreeMap;

use forge_core::KernelError;
use forge_topo::handles::VertexId;
use forge_topo::state::MutableDraft;

use crate::geometry_state::GeometryState;
use crate::shared_ops::copy::VertexWelder;

/// Pre-stitch identity repair: cluster vertices by position, rewrite
/// halfedge endpoints to use the canonical (lowest-index) VertexId in
/// each cluster.
///
/// This fixes "same position, different VertexId" defects that arise when
/// faces from different sources are copied into the same arena. It's local
/// surgery: face/edge structure is preserved, only vertex endpoints change.
///
/// Returns the number of vertices that were merged into canonical IDs.
pub fn repair_vertex_identity(
    draft: &mut MutableDraft,
    geom: &GeometryState,
    weld_tolerance: f64,
    ctx: &mut crate::core::ModelingContext,
) -> Result<usize, KernelError> {
    let mut welder = VertexWelder::with_linear_tolerance(weld_tolerance);

    for (vid, _) in draft.arena().iter_vertices() {
        if let Some(pos) = geom.get_vertex_position(vid) {
            welder.insert(vid, *pos);
        }
    }

    let mut remap: BTreeMap<u32, VertexId> = BTreeMap::new();
    for (vid, _) in draft.arena().iter_vertices() {
        if let Some(pos) = geom.get_vertex_position(vid) {
            if let Some(canonical) = welder.find_nearest(pos) {
                if canonical != vid {
                    remap.insert(vid.index(), canonical);
                }
            }
        }
    }

    if remap.is_empty() {
        return Ok(0);
    }

    let merged_count = remap.len();

    let all_he_ids: Vec<forge_topo::handles::HalfEdgeId> = draft.arena()
        .iter_half_edges()
        .map(|(id, _)| id)
        .collect();

    for he_id in &all_he_ids {
        let origin = draft.arena().get_half_edge(*he_id)?.origin();
        if let Some(&canonical) = remap.get(&origin.index()) {
            draft.arena_mut().get_half_edge_mut(*he_id)?.set_origin(canonical);
        }
    }

    let mut decision = forge_core::TracedDecision::new(
        forge_core::DecisionId(merged_count as u64),
        forge_core::DecisionKind::Forced {
            reason: format!("Identity repair: merged {} duplicate vertices", merged_count),
        },
        forge_core::DecisionTier::PolicyApplied,
        0.9,
        forge_core::DecisionContext::Degeneracy {
            description: format!(
                "Pre-stitch identity repair rewrote {} vertex references",
                merged_count,
            ),
        },
    );
    decision.set_entity_scope(forge_core::EntityRef::new(forge_core::EntityKind::Vertex, 0));
    ctx.get_decision_log_mut().record(decision);

    Ok(merged_count)
}
