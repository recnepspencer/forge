use schema::facade::platform::authority::touched_graph_conflict::{
    ConflictOverlapCategory, ConflictRoutingPosture,
};

use super::family_declaration::{
    TopologyConflictDiagnosticWitness, TopologyConflictFamilyDeclaration,
    TopologyConflictLocalityAuthorityRequirement, TopologyConflictPriorProofPosture,
    TopologyConflictSelectionProductPosture,
};
use super::family_identity::TopologyConflictFamilyIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopologyConflictFamilyDeclarationInput {
    pub identity: TopologyConflictFamilyIdentity,
    pub locality_authority_requirement: TopologyConflictLocalityAuthorityRequirement,
    pub primary_overlap_category: ConflictOverlapCategory,
    pub secondary_overlap_category: Option<ConflictOverlapCategory>,
    pub routing_posture: ConflictRoutingPosture,
    pub prior_proof_posture: TopologyConflictPriorProofPosture,
    pub diagnostic_witness: TopologyConflictDiagnosticWitness,
    pub selection_product_posture: TopologyConflictSelectionProductPosture,
}

pub fn admit_topology_conflict_family_declaration(
    input: TopologyConflictFamilyDeclarationInput,
) -> TopologyConflictFamilyDeclaration {
    TopologyConflictFamilyDeclaration::new(
        input.identity,
        input.locality_authority_requirement,
        input.primary_overlap_category,
        input.secondary_overlap_category,
        input.routing_posture,
        input.prior_proof_posture,
        input.diagnostic_witness,
        input.selection_product_posture,
    )
}
