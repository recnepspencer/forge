use schema::facade::platform::authority::touched_graph_conflict::{
    ConflictOverlapCategory, ConflictRoutingPosture,
};

use super::family_declaration::{
    SpatialConflictDiagnosticWitness, SpatialConflictFamilyDeclaration,
    SpatialConflictLocalityAuthorityRequirement, SpatialConflictPriorProofPosture,
    SpatialConflictSelectionProductPosture,
};
use super::family_identity::SpatialConflictFamilyIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpatialConflictFamilyDeclarationInput {
    pub identity: SpatialConflictFamilyIdentity,
    pub locality_authority_requirement: SpatialConflictLocalityAuthorityRequirement,
    pub primary_overlap_category: ConflictOverlapCategory,
    pub secondary_overlap_category: Option<ConflictOverlapCategory>,
    pub routing_posture: ConflictRoutingPosture,
    pub prior_proof_posture: SpatialConflictPriorProofPosture,
    pub diagnostic_witness: SpatialConflictDiagnosticWitness,
    pub selection_product_posture: SpatialConflictSelectionProductPosture,
}

pub fn admit_spatial_conflict_family_declaration(
    input: SpatialConflictFamilyDeclarationInput,
) -> SpatialConflictFamilyDeclaration {
    SpatialConflictFamilyDeclaration::new(
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
