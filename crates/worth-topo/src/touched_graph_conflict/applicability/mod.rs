mod replay_transaction_scope;
mod routing_contract;
mod validator_pressure;

use schema::facade::platform::authority::touched_graph_conflict::ConflictRoutingContract;

use super::family_declaration::TopologyConflictFamilyDeclaration;

pub(crate) fn matches_declaration(
    declaration: &TopologyConflictFamilyDeclaration,
    contract: &ConflictRoutingContract,
) -> bool {
    routing_contract::matches_routing_contract(declaration, contract)
        && validator_pressure::matches_validator_pressure(declaration, contract)
        && replay_transaction_scope::matches_prior_proof_posture(declaration, contract)
}
