use schema::facade::platform::authority::touched_graph_conflict::ConflictRoutingContract;

use crate::touched_graph_conflict::family_declaration::{
    SpatialConflictFamilyDeclaration, SpatialConflictPriorProofPosture,
};

pub(crate) fn matches_prior_proof_posture(
    declaration: &SpatialConflictFamilyDeclaration,
    contract: &ConflictRoutingContract,
) -> bool {
    match declaration.prior_proof_posture() {
        SpatialConflictPriorProofPosture::NoPriorProofRequired => {
            contract.prior_proof().identities().is_empty()
        }
        SpatialConflictPriorProofPosture::ReplayUndoOrTransactionRequired => {
            !contract.prior_proof().identities().is_empty()
        }
    }
}
