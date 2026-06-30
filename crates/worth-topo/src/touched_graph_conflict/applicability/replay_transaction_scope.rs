use schema::facade::platform::authority::touched_graph_conflict::ConflictRoutingContract;

use crate::touched_graph_conflict::family_declaration::{
    TopologyConflictFamilyDeclaration, TopologyConflictPriorProofPosture,
};

pub(crate) fn matches_prior_proof_posture(
    declaration: &TopologyConflictFamilyDeclaration,
    contract: &ConflictRoutingContract,
) -> bool {
    match declaration.prior_proof_posture() {
        TopologyConflictPriorProofPosture::NoPriorProofRequired => {
            contract.prior_proof().identities().is_empty()
        }
        TopologyConflictPriorProofPosture::ReplayUndoOrTransactionRequired => {
            !contract.prior_proof().identities().is_empty()
        }
    }
}
