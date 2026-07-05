use schema::facade::platform::authority::touched_graph_conflict::ConflictOverlapCategory;
use schema::facade::platform::authority::touched_graph_conflict::ConflictRoutingContract;

use crate::touched_graph_conflict::family_declaration::SpatialConflictFamilyDeclaration;

pub(crate) fn matches_evidence_pressure(
    declaration: &SpatialConflictFamilyDeclaration,
    contract: &ConflictRoutingContract,
) -> bool {
    if declaration.primary_overlap_category() == ConflictOverlapCategory::Evidence {
        return !contract.overlap_identity().participants().is_empty();
    }
    true
}
