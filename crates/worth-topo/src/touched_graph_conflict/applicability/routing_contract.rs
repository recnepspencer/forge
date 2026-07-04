use schema::facade::platform::authority::replay_undo_semantic_graph::ReplayUndoSemanticGraphLocalityScope;
use schema::facade::platform::authority::touched_graph_conflict::{
    ConflictOverlapCategory, ConflictRoutingContract,
};

use crate::touched_graph_conflict::family_declaration::TopologyConflictFamilyDeclaration;

pub(crate) fn matches_routing_contract(
    declaration: &TopologyConflictFamilyDeclaration,
    contract: &ConflictRoutingContract,
) -> bool {
    let overlap = contract.overlap_identity();
    let locality = match overlap.locality_identity() {
        Some(locality) => locality,
        None => return false,
    };
    locality.scope() == ReplayUndoSemanticGraphLocalityScope::TopologyTouchedClosure
        && contract.posture() == declaration.routing_posture()
        && matches_category(declaration, overlap.category())
}

fn matches_category(
    declaration: &TopologyConflictFamilyDeclaration,
    category: ConflictOverlapCategory,
) -> bool {
    category == declaration.primary_overlap_category()
        || declaration.secondary_overlap_category() == Some(category)
}
