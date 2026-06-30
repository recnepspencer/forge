mod evidence_pressure;
mod replay_transaction_scope;
mod routing_contract;

use schema::facade::platform::authority::touched_graph_conflict::ConflictRoutingContract;

use super::family_declaration::SpatialConflictFamilyDeclaration;

pub(crate) fn matches_declaration(
    declaration: &SpatialConflictFamilyDeclaration,
    contract: &ConflictRoutingContract,
) -> bool {
    routing_contract::matches_routing_contract(declaration, contract)
        && evidence_pressure::matches_evidence_pressure(declaration, contract)
        && replay_transaction_scope::matches_prior_proof_posture(declaration, contract)
}
