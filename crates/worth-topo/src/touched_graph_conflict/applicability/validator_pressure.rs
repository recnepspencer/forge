use schema::facade::platform::authority::touched_graph_conflict::ConflictRoutingContract;
use schema::facade::platform::authority::touched_graph_conflict::ConflictOverlapCategory;

use crate::touched_graph_conflict::family_declaration::TopologyConflictFamilyDeclaration;

pub(crate) fn matches_validator_pressure(
    declaration: &TopologyConflictFamilyDeclaration,
    contract: &ConflictRoutingContract,
) -> bool {
    if declaration.primary_overlap_category() == ConflictOverlapCategory::Validator {
        return !contract.overlap_identity().participants().is_empty();
    }
    true
}
