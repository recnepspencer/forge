//! Face selection logic.
//!
//! Every selection decision is recorded into the `ModelingContext`'s
//! decision log, providing full traceability of which faces
//! were kept or dropped and why.

use forge_topo::handles::FaceId;
use forge_core::{TracedDecision, DecisionId, DecisionKind, DecisionContext, DecisionTier, EntityRef};
use crate::core::ModelingContext;
use crate::operations::boolean::schema::{BooleanOp, FaceOrigin, FaceClassification, ClassifiedFace};

/// Select faces to keep based on the Boolean operation type.
///
/// Records why each face was kept or dropped into the context's decision log.
///
/// | Operation     | Origin | Classification   | Action | Reason |
/// |---------------|--------|------------------|--------|--------|
/// | **Union**     | Target | Outside          | Keep   | Part of sum |
/// |               | Target | OnBoundary       | Keep   | Surface of sum |
/// |               | Target | OppositeBoundary | Drop   | Internal (merged) |
/// |               | Tool   | Outside          | Keep   | Part of sum |
/// |               | Tool   | OnBoundary       | Drop   | Redundant with Target |
/// |               | Tool   | OppositeBoundary | Drop   | Internal (merged) |
/// | **Intersect** | Target | Inside           | Keep   | Common volume |
/// |               | Target | OnBoundary       | Keep   | Common boundary |
/// |               | Target | OppositeBoundary | Drop   | Disjoint boundary |
/// |               | Tool   | Inside           | Keep   | Common volume |
/// |               | Tool   | OnBoundary       | Drop   | Redundant with Target |
/// |               | Tool   | OppositeBoundary | Drop   | Disjoint boundary |
/// | **Subtract**  | Target | Outside          | Keep   | Main volume |
/// |               | Target | OnBoundary       | Drop   | Removed by tool |
/// |               | Target | OppositeBoundary | Keep   | Touching tool (safe) |
/// |               | Tool   | Inside           | Keep   | Wall of hole (inverted) |
/// |               | Tool   | OnBoundary       | Drop   | Removed surface |
/// |               | Tool   | OppositeBoundary | Drop   | Touching surface |
pub fn select_faces(
    classified: &[ClassifiedFace],
    origin: FaceOrigin,
    operation: BooleanOp,
    ctx: &mut ModelingContext,
) -> Vec<FaceId> {
    let mut selected = Vec::new();
    let origin_label = match origin {
        FaceOrigin::Target => "Target",
        FaceOrigin::Tool => "Tool",
    };

    for f in classified {
        let keep = match (origin, operation, f.classification()) {
            // UNION
            (FaceOrigin::Target, BooleanOp::Union, FaceClassification::Outside) => true,
            (FaceOrigin::Target, BooleanOp::Union, FaceClassification::OnBoundary) => true,
            (FaceOrigin::Tool,   BooleanOp::Union, FaceClassification::Outside) => true,
            
            // INTERSECTION
            (FaceOrigin::Target, BooleanOp::Intersection, FaceClassification::Inside) => true,
            (FaceOrigin::Target, BooleanOp::Intersection, FaceClassification::OnBoundary) => true,
            (FaceOrigin::Tool,   BooleanOp::Intersection, FaceClassification::Inside) => true,
            
            // SUBTRACTION
            (FaceOrigin::Target, BooleanOp::Subtraction, FaceClassification::Outside) => true,
            (FaceOrigin::Target, BooleanOp::Subtraction, FaceClassification::OppositeBoundary) => true,
            (FaceOrigin::Tool,   BooleanOp::Subtraction, FaceClassification::Inside) => true,
            
            _ => false,
        };

        let action = if keep { "Keep" } else { "Drop" };
        let mut decision = TracedDecision::new(
            DecisionId(1000 + f.face().index() as u64),
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            1.0,
            DecisionContext::Classification {
                point: [0.0; 3],
                result: format!("Select {}:Face#{} {:?} for {:?} → {}",
                    origin_label, f.face().index(), f.classification(), operation, action),
            },
        );
        decision.set_entity_scope(EntityRef::new("Face", f.face().index()));
        ctx.get_decision_log_mut().record(decision);

        if keep {
            selected.push(f.face());
        }
    }

    selected
}
