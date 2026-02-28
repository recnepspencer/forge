//! Fragment ambiguity resolution.
//!
//! DOMAIN: Detect split fragments that are classified incorrectly due to
//! coplanar degeneracy and mark them as `Ambiguous`.

use std::collections::BTreeMap;

use forge_core::{DecisionContext, DecisionId, DecisionKind, DecisionTier, TracedDecision, PolicyKind, EntityRef, EntityKind};
use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::FaceId;
use forge_topo::transactions::TopologyState;

use crate::core::ModelingContext;
use crate::operations::boolean::schema::BooleanOp;
use crate::operations::boolean::classify_schema::{ClassifiedFace, FaceClassification};

pub(crate) fn resolve_fragment_ambiguities(
    target_topo: &TopologyState,
    tool_topo: &TopologyState,
    operation: BooleanOp,
    target_classified: &mut [ClassifiedFace],
    tool_classified: &mut [ClassifiedFace],
    ctx: &mut ModelingContext,
) {
    if operation != BooleanOp::Subtraction {
        return;
    }
    if std::env::var("FORGE_ENABLE_FRAGMENT_AMBIGUITY").ok().as_deref() != Some("1") {
        return;
    }
    mark_outside_split_fragments_ambiguous(tool_topo.arena(), tool_classified, "tool", ctx);
    let _ = target_topo;
    let _ = target_classified;
}

fn mark_outside_split_fragments_ambiguous(
    arena: &TopologyArena,
    classified: &mut [ClassifiedFace],
    label: &str,
    ctx: &mut ModelingContext,
) {
    let class_map: BTreeMap<FaceId, FaceClassification> =
        classified.iter().map(|f| (f.face(), f.classification())).collect();

    for face in classified.iter_mut() {
        if face.classification() != FaceClassification::Outside {
            continue;
        }
        if !crate::shared_ops::assembly::fragment::is_make_edge_face_fragment(arena, face.face()) {
            continue;
        }
        let (inside_neighbors, split_neighbors) = count_split_face_neighbors(arena, face.face(), &class_map);
        if std::env::var("FORGE_DEBUG_AMBIGUITY").ok().as_deref() == Some("1")
            && matches!(face.face().index(), 14 | 15)
        {
            eprintln!(
                "[ambiguity] probe {} F#{} class={:?} inside_neighbors={} split_neighbors={}",
                label,
                face.face().index(),
                face.classification(),
                inside_neighbors,
                split_neighbors,
            );
        }
        let bridge_like = (inside_neighbors >= 2 && split_neighbors >= 2)
            || (inside_neighbors >= 1 && split_neighbors >= 3);
        if !bridge_like {
            continue;
        }
        face.set_classification(FaceClassification::Ambiguous);

        if std::env::var("FORGE_DEBUG_SELECT_PROVENANCE").ok().as_deref() == Some("1") {
            let lineage = arena
                .get_face(face.face())
                .ok()
                .and_then(|f| f.lineage())
                .map(|lin| format!("{}#{}", lin.get_creation_op().get_name(), lin.get_creation_op().get_invocation_id()))
                .unwrap_or_else(|| "no-lineage".to_string());
            eprintln!(
                "[ambiguity] {} F#{} Outside -> Ambiguous (inside_neighbors={}, split_neighbors={}) {}",
                label,
                face.face().index(),
                inside_neighbors,
                split_neighbors,
                lineage,
            );
        }

        let mut decision = TracedDecision::new(
            DecisionId(50_000 + face.face().index() as u64),
            DecisionKind::PolicyApplied { policy: PolicyKind::CoincidentGeometry, default_used: true },
            DecisionTier::Deterministic,
            1.0,
            DecisionContext::Classification {
                point: [0.0; 3],
                result: format!(
                    "Promote {}:Face#{} Outside -> Ambiguous (split-fragment closure safeguard)",
                    label,
                    face.face().index()
                ),
            },
        );
        decision.set_entity_scope(EntityRef::new(EntityKind::Face, face.face().index()));
        ctx.get_decision_log_mut().record(decision);
    }
}

fn count_split_face_neighbors(
    arena: &TopologyArena,
    face_id: FaceId,
    class_map: &BTreeMap<FaceId, FaceClassification>,
) -> (usize, usize) {
    let neighbors: std::collections::BTreeSet<FaceId> =
        forge_topo::queries::classification::face_adjacent_faces(arena, face_id)
            .unwrap_or_default()
            .into_iter()
            .collect();

    let mut inside_neighbors = 0usize;
    let mut split_neighbors = 0usize;
    for nface in neighbors {
        if crate::shared_ops::assembly::fragment::is_make_edge_face_fragment(arena, nface) {
            split_neighbors += 1;
        }
        if matches!(class_map.get(&nface), Some(FaceClassification::Inside | FaceClassification::OnBoundary | FaceClassification::OppositeBoundary)) {
            inside_neighbors += 1;
        }
    }
    (inside_neighbors, split_neighbors)
}
