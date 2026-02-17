//! Face selection logic.

use forge_topo::handles::FaceId;
use crate::boolean::schema::{BooleanOp, FaceOrigin, FaceClassification, ClassifiedFace};

/// Select faces to keep based on the Boolean operation type.
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
) -> Vec<FaceId> {
    classified
        .iter()
        .filter(|f| match (origin, operation, f.classification()) {
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
            
            // All other cases (Inside for Union, Outside for Intersection, etc.) -> Drop
            _ => false,
        })
        .map(|f| f.face())
        .collect()
}
