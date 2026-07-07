#[cfg(test)]
mod replay_transaction_scope;
#[cfg(test)]
mod routing_contract;
#[cfg(test)]
mod validator_pressure;

#[cfg(test)]
use schema::facade::platform::authority::touched_graph_conflict::ConflictRoutingContract;

#[cfg(test)]
use super::family_declaration::TopologyConflictFamilyDeclaration;

#[cfg(test)]
pub(crate) fn matches_declaration(
    declaration: &TopologyConflictFamilyDeclaration,
    contract: &ConflictRoutingContract,
) -> bool {
    routing_contract::matches_routing_contract(declaration, contract)
        && validator_pressure::matches_validator_pressure(declaration, contract)
        && replay_transaction_scope::matches_prior_proof_posture(declaration, contract)
}
