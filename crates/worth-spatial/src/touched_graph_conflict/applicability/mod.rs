#[cfg(test)]
mod evidence_pressure;
#[cfg(test)]
mod replay_transaction_scope;
#[cfg(test)]
mod routing_contract;

#[cfg(test)]
use schema::facade::platform::authority::touched_graph_conflict::ConflictRoutingContract;

#[cfg(test)]
use super::family_declaration::SpatialConflictFamilyDeclaration;

#[cfg(test)]
pub(crate) fn matches_declaration(
    declaration: &SpatialConflictFamilyDeclaration,
    contract: &ConflictRoutingContract,
) -> bool {
    routing_contract::matches_routing_contract(declaration, contract)
        && evidence_pressure::matches_evidence_pressure(declaration, contract)
        && replay_transaction_scope::matches_prior_proof_posture(declaration, contract)
}
